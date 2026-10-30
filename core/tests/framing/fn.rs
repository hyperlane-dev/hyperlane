use super::*;

#[test]
fn declared_length_accepts_only_ascii_digits_after_trimming() {
    for (input, expected) in [("0", 0), ("42", 42), (" 7\t", 7), ("0009", 9)] {
        assert_eq!(parse_ascii_length(input).unwrap(), expected);
    }
    assert_eq!(
        parse_ascii_length(&usize::MAX.to_string()).unwrap(),
        usize::MAX
    );
}

#[test]
fn declared_length_rejects_signs_empty_values_and_non_ascii_digits() {
    for input in ["", " ", "+1", "-1", "1.0", "1x", "1 2", "\u{0661}"] {
        assert!(matches!(
            parse_ascii_length(input),
            Err(RequestError::InvalidContentLength(HttpStatus::BadRequest))
        ));
    }
}

#[test]
fn declared_length_rejects_numeric_overflow() {
    assert!(parse_ascii_length(&format!("{}0", usize::MAX)).is_err());
}

#[test]
fn status_validator_checks_both_three_digit_boundaries() {
    for status in [100, 101, 200, 204, 304, 599, 999] {
        assert!(validate_status_code(status).is_ok());
    }
    for status in [0, 99, 1000, usize::MAX] {
        assert!(validate_status_code(status).is_err());
    }
}

#[test]
fn response_budget_handles_exact_limits_and_saturating_arithmetic() {
    assert!(!check_body_budget(Some(8), 5, 3));
    assert!(check_body_budget(Some(8), 5, 4));
    assert!(!check_body_budget(Some(0), 0, 0));
    assert!(check_body_budget(Some(0), 0, 1));
    assert!(check_body_budget(Some(8), usize::MAX, 1));
    assert!(!check_body_budget(None, usize::MAX, 1));
}

#[test]
fn entity_length_and_body_suppression_preserve_head_and_304_semantics() {
    for is_head in [false, true] {
        assert!(hides_entity_length(100, is_head));
        assert!(hides_entity_length(204, is_head));
        assert!(!hides_entity_length(200, is_head));
        assert!(!hides_entity_length(304, is_head));
    }
    assert!(is_bodyless_status(200, true));
    assert!(is_bodyless_status(304, false));
    assert!(!is_bodyless_status(200, false));
}

#[test]
fn response_head_preserves_duplicate_headers_and_matching_lengths() {
    let (status, headers): (usize, ProtocolHeaders) = parse_response_head(b"HTTP/2 200 OK\r\nSet-Cookie: one=1\r\nSet-Cookie: two=2\r\nContent-Length: 7\r\nContent-Length: 7\r\n\r\n").unwrap();
    assert_eq!(status, 200);
    assert_eq!(headers.get_all("set-cookie").iter().count(), 2);
    assert_eq!(parse_content_length(&headers).unwrap(), Some(7));
    assert!(!has_transfer_encoding(&headers));
}

#[test]
fn conflicting_and_malformed_response_lengths_are_rejected() {
    for head in [
        b"HTTP/1.1 200 OK\r\nContent-Length: 7\r\nContent-Length: 8\r\n\r\n".as_slice(),
        b"HTTP/1.1 200 OK\r\nContent-Length: +7\r\n\r\n".as_slice(),
    ] {
        let (_, headers): (usize, ProtocolHeaders) = parse_response_head(head).unwrap();
        assert!(parse_content_length(&headers).is_err());
    }
    let (_, headers): (usize, ProtocolHeaders) =
        parse_response_head(b"HTTP/3 204 No Content\r\n\r\n").unwrap();
    assert_eq!(parse_content_length(&headers).unwrap(), None);
}

#[test]
fn malformed_response_heads_are_not_silently_accepted() {
    for head in [
        b"not-http 200 OK\r\n\r\n".as_slice(),
        b"HTTP/1.1\r\n\r\n".as_slice(),
        b"HTTP/1.1 200 OK\r\nbroken\r\n\r\n".as_slice(),
        b"HTTP/1.1 200 OK\r\nBad Name: value\r\n\r\n".as_slice(),
        b"HTTP/1.1 200 OK\r\nName: \xff\r\n\r\n".as_slice(),
    ] {
        assert!(parse_response_head(head).is_err());
    }
}

#[test]
fn connection_tokens_are_case_insensitive_and_transfer_encoding_is_detected() {
    let (_, headers): (usize, ProtocolHeaders) = parse_response_head(b"HTTP/1.1 200 OK\r\nConnection: keep-alive, X-Private\r\nTransfer-Encoding: chunked\r\n\r\n").unwrap();
    assert!(connection_nominates(&headers, "x-private"));
    assert!(connection_nominates(&headers, "CONNECTION"));
    assert!(!connection_nominates(&headers, "x-public"));
    assert!(has_transfer_encoding(&headers));
}

#[test]
fn response_head_boundary_detection_waits_for_a_complete_blank_line() {
    assert_eq!(find_head_end(b"HTTP/1.1 200 OK\r\nName: value\r\n\r"), None);
    let complete: &[u8] = b"HTTP/1.1 200 OK\r\n\r\nbody";
    assert_eq!(find_head_end(complete), Some(complete.len() - 4));
    assert_eq!(find_line_end(b"line\r\nrest"), Some(6));
    assert_eq!(find_line_end(b"unfinished\r"), None);
}

#[test]
fn bounded_body_rejects_excess_without_mutating_existing_bytes() {
    let mut body: Vec<u8> = vec![1, 2];
    push_bounded_body(3, &mut body, Bytes::from_static(&[3])).unwrap();
    assert_eq!(body, vec![1, 2, 3]);
    assert!(push_bounded_body(3, &mut body, Bytes::from_static(&[4])).is_err());
    assert_eq!(body, vec![1, 2, 3]);
}

#[test]
fn request_target_adds_only_nonempty_queries() {
    assert_eq!(request_target("/path", Some("q=a%20b")), "/path?q=a%20b");
    assert_eq!(request_target("/path", Some("")), "/path");
    assert_eq!(request_target("/path", None), "/path");
}

fn authority_headers(pairs: &[(&str, &str)]) -> ProtocolHeaders {
    let mut headers: ProtocolHeaders = ProtocolHeaders::new();
    for (name, value) in pairs {
        headers.append(
            http::HeaderName::try_from(*name).expect("the header name must be valid"),
            http::HeaderValue::try_from(*value).expect("the header value must be valid"),
        );
    }
    headers
}

fn authority_uri(authority: &str) -> http::Uri {
    http::Uri::try_from(format!("http://{authority}/path")).expect("the authority must build")
}

#[test]
fn an_authority_that_disagrees_with_the_host_header_is_rejected() {
    let headers: ProtocolHeaders = authority_headers(&[("host", "other.example")]);
    let uri: http::Uri = authority_uri("authority.example");
    assert!(matches!(
        check_request_authority(&headers, &uri),
        Err(RequestError::InvalidUrlHost(HttpStatus::BadRequest))
    ));
}

#[test]
fn an_authority_that_agrees_with_the_host_header_is_accepted() {
    let headers: ProtocolHeaders = authority_headers(&[("host", "authority.example")]);
    let uri: http::Uri = authority_uri("authority.example");
    assert!(check_request_authority(&headers, &uri).is_ok());
}

#[test]
fn an_authority_that_differs_from_the_host_header_only_by_case_is_accepted() {
    let headers: ProtocolHeaders = authority_headers(&[("host", "AUTHORITY.example")]);
    let uri: http::Uri = authority_uri("authority.example");
    assert!(check_request_authority(&headers, &uri).is_ok());
}

#[test]
fn an_authority_and_a_host_header_that_name_different_ports_are_rejected() {
    let headers: ProtocolHeaders = authority_headers(&[("host", "authority.example:8080")]);
    let uri: http::Uri = authority_uri("authority.example");
    assert!(matches!(
        check_request_authority(&headers, &uri),
        Err(RequestError::InvalidUrlHost(HttpStatus::BadRequest))
    ));
}

#[test]
fn a_default_port_matches_an_authority_that_carries_no_port() {
    let headers: ProtocolHeaders = authority_headers(&[("host", "authority.example:443")]);
    let uri: http::Uri =
        http::Uri::try_from("https://authority.example/path").expect("the uri must build");
    assert!(check_request_authority(&headers, &uri).is_ok());
}

#[test]
fn an_authority_that_carries_userinfo_is_rejected() {
    let uri: http::Uri = authority_uri("user@authority.example");
    let headers: ProtocolHeaders = authority_headers(&[("host", "user@authority.example")]);
    assert!(matches!(
        check_request_authority(&headers, &uri),
        Err(RequestError::InvalidUrlHost(HttpStatus::BadRequest))
    ));
}

#[test]
fn a_host_header_that_carries_userinfo_is_rejected() {
    let uri: http::Uri = authority_uri("authority.example");
    let headers: ProtocolHeaders = authority_headers(&[("host", "user@authority.example")]);
    assert!(matches!(
        check_request_authority(&headers, &uri),
        Err(RequestError::InvalidUrlHost(HttpStatus::BadRequest))
    ));
}

#[test]
fn two_host_headers_are_rejected() {
    let uri: http::Uri = authority_uri("authority.example");
    let headers: ProtocolHeaders =
        authority_headers(&[("host", "authority.example"), ("host", "authority.example")]);
    assert!(matches!(
        check_request_authority(&headers, &uri),
        Err(RequestError::InvalidUrlHost(HttpStatus::BadRequest))
    ));
}

#[test]
fn a_request_without_a_host_header_is_accepted() {
    let uri: http::Uri = authority_uri("authority.example");
    let headers: ProtocolHeaders = ProtocolHeaders::new();
    assert!(check_request_authority(&headers, &uri).is_ok());
}

#[test]
fn an_authority_is_split_into_its_host_and_its_optional_port() {
    assert_eq!(split_authority_port("example.com"), ("example.com", None));
    assert_eq!(
        split_authority_port("example.com:8080"),
        ("example.com", Some("8080"))
    );
    assert_eq!(split_authority_port("[::1]:8080"), ("[::1]", Some("8080")));
    assert_eq!(split_authority_port("[::1]"), ("[::1]", None));
    assert_eq!(split_authority_port(" example.com "), ("example.com", None));
}

#[test]
fn an_authority_is_normalized_by_case_and_by_the_scheme_default_port() {
    assert_eq!(
        normalize_authority("EXAMPLE.com", Some("http")),
        ("example.com".to_string(), None)
    );
    assert_eq!(
        normalize_authority("example.com:80", Some("http")),
        ("example.com".to_string(), None)
    );
    assert_eq!(
        normalize_authority("example.com:443", Some("https")),
        ("example.com".to_string(), None)
    );
    assert_eq!(
        normalize_authority("example.com:80", Some("https")),
        ("example.com".to_string(), Some("80".to_string()))
    );
    assert_eq!(
        normalize_authority("example.com:8080", None),
        ("example.com".to_string(), Some("8080".to_string()))
    );
}
