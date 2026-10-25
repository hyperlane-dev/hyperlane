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
