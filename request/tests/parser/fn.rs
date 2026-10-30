use super::*;

#[test]
fn split_multi_byte_preserves_non_overlapping_segments() {
    assert_eq!(
        split_multi_byte(b"left::middle::right", b"::"),
        vec![
            b"left".as_slice(),
            b"middle".as_slice(),
            b"right".as_slice()
        ]
    );
    assert_eq!(
        split_multi_byte(b"::left::::", b"::"),
        vec![b"".as_slice(), b"left".as_slice(), b"".as_slice()]
    );
}

#[test]
fn split_multi_byte_skips_overlapping_matches() {
    assert_eq!(
        split_multi_byte(b"aaaaa", b"aa"),
        vec![b"".as_slice(), b"".as_slice(), b"a".as_slice()]
    );
}

#[test]
fn split_multi_byte_handles_empty_delimiters_and_inputs() {
    assert_eq!(
        split_multi_byte(b"payload", b""),
        vec![b"payload".as_slice()]
    );
    assert!(split_multi_byte(b"", b"::").is_empty());
    assert_eq!(split_multi_byte(b"one", b"longer"), vec![b"one".as_slice()]);
}

#[test]
fn split_http_head_preserves_body_leading_crlf() {
    assert_eq!(
        split_multi_byte(b"HTTP/1.1 200 OK\r\n\r\n\r\n", b"\r\n\r\n"),
        vec![b"HTTP/1.1 200 OK".as_slice(), b"\r\n".as_slice()]
    );
}

#[test]
fn whitespace_split_ignores_empty_space_and_tab_runs() {
    assert_eq!(
        split_whitespace(b" \tGET\t /path  HTTP/1.1 \t"),
        vec![
            b"GET".as_slice(),
            b"/path".as_slice(),
            b"HTTP/1.1".as_slice()
        ]
    );
    assert!(split_whitespace(b" \t ").is_empty());
    assert!(split_whitespace(b"").is_empty());
}

#[test]
fn wire_request_builder_keeps_binary_body_and_delimiters() {
    let request: Vec<u8> = build_http_request(
        "POST",
        "/upload".to_owned(),
        b"Content-Length: 3\r\n".to_vec(),
        Some(vec![0, 255, 13]),
        "HTTP/1.1".to_owned(),
    );
    assert_eq!(
        request,
        b"POST /upload HTTP/1.1\r\nContent-Length: 3\r\n\r\n\0\xff\r"
    );
    let empty: Vec<u8> = build_http_request(
        "GET",
        "/".to_owned(),
        Vec::new(),
        None,
        "HTTP/1.1".to_owned(),
    );
    assert_eq!(empty, b"GET / HTTP/1.1\r\n\r\n");
}

#[test]
fn line_delimiter_search_respects_offsets_and_partial_input() {
    let data: &[u8] = b"one\r\n\r\ntwo\r\n";
    assert_eq!(find_crlf(data, 0), Some(3));
    assert_eq!(find_crlf(data, 5), Some(5));
    assert_eq!(find_double_crlf(data, 0), Some(3));
    assert_eq!(find_double_crlf(data, 4), None);
    assert_eq!(find_crlf(data, data.len()), None);
    assert_eq!(find_double_crlf(b"\r\n\r", 0), None);
}

#[test]
fn case_insensitive_search_handles_binary_and_empty_patterns() {
    assert_eq!(
        find_pattern_case_insensitive(b"\xffContent-TYPE: bin", b"content-type:"),
        Some(1)
    );
    assert_eq!(find_pattern_case_insensitive(b"abAB", b"AB"), Some(0));
    assert_eq!(find_pattern_case_insensitive(b"short", b"longer"), None);
    assert_eq!(find_pattern_case_insensitive(b"body", b""), None);
}

#[test]
fn chunked_body_parser_decodes_extensions_and_binary_payloads() {
    assert_eq!(
        parse_chunked_body(b"3;name=value\r\na\0\xff\r\n2\r\nbc\r\n0\r\n\r\n").unwrap(),
        b"a\0\xffbc"
    );
    assert!(parse_chunked_body(b"2\r\nok\r\n4\r\nno").is_err());
    assert!(parse_chunked_body(b"invalid\r\nbytes").is_err());
}

#[test]
fn response_header_helpers_accept_case_insensitive_wire_names() {
    let headers: &[u8] =
        b"HTTP/1.1 200 OK\r\nCoNtEnT-LeNgTh: 12\r\nTransfer-Encoding: Chunked\r\n\r\n";
    assert_eq!(get_content_length(headers).unwrap(), 12);
    assert!(is_chunked_encoding(headers).unwrap());
    assert_eq!(get_content_length(b"HTTP/1.1 200 OK\r\n\r\n").unwrap(), 0);
    assert!(!is_chunked_encoding(b"Content-Length: 0\r\n").unwrap());
}

#[test]
fn decimal_and_status_parsers_cover_valid_and_invalid_ascii() {
    assert!(parse_decimal_bytes(b" \t123rest").is_err());
    assert_eq!(parse_decimal_bytes(b"0007").unwrap(), 7);
    assert!(parse_decimal_bytes(b"x123").is_err());
    assert!(parse_decimal_bytes(b"").is_err());
    assert_eq!(parse_status_code(b"204"), 204);
    for invalid in [
        b"".as_slice(),
        b"20".as_slice(),
        b"2000".as_slice(),
        b"2x0".as_slice(),
    ] {
        assert_eq!(parse_status_code(invalid), 0);
    }
}

#[test]
fn response_buffer_growth_covers_each_capacity_branch() {
    assert_eq!(calculate_buffer_capacity(b"data", 4, 8), 0);
    assert_eq!(calculate_buffer_capacity(b"data", 1, 0), 1024);
    assert_eq!(calculate_buffer_capacity(b"data", 8, 8), 16);
    assert_eq!(calculate_buffer_capacity(b"data", 20, 8), 36);
}

#[test]
fn response_header_parser_extracts_redirect_length_and_chunked_state() {
    let mut length: usize = 0;
    let mut redirect: Option<Vec<u8>> = None;
    let mut chunked: bool = false;
    parse_response_headers(
        b"HTTP/1.1 302 Found\r\nLocation:/next\r\nContent-Length: 4\r\n\r\n",
        b"http/1.1",
        b"location:",
        &mut length,
        &mut redirect,
        &mut chunked,
    )
    .unwrap();
    assert_eq!(length, 4);
    assert_eq!(redirect, Some(b"/next".to_vec()));
    assert!(!chunked);
    parse_response_headers(
        b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\n\r\n",
        b"http/1.1",
        b"location:",
        &mut length,
        &mut redirect,
        &mut chunked,
    )
    .unwrap();
    assert_eq!(length, 0);
    assert!(redirect.is_none());
    assert!(chunked);
}

#[test]
fn response_header_lookup_ignores_prefixed_names_and_body_fields() {
    for response in [
        b"HTTP/1.1 200 OK\r\nX-Content-Length: 999\r\nX-Transfer-Encoding: chunked\r\n\r\n"
            .as_slice(),
        b"HTTP/1.1 200 OK\r\nX-Note: Content-Length: 999; Transfer-Encoding: chunked\r\n\r\n"
            .as_slice(),
        b"HTTP/1.1 200 OK\r\n\r\nContent-Length: 999\r\nTransfer-Encoding: chunked\r\n".as_slice(),
    ] {
        assert_eq!(get_content_length(response).unwrap(), 0);
        assert!(!is_chunked_encoding(response).unwrap());
    }
    assert_eq!(
        get_content_length(
            b"HTTP/1.1 200 OK\r\nX-Content-Length: 999\r\nContent-Length: 5\r\n\r\n"
        )
        .unwrap(),
        5
    );
}

#[test]
fn response_content_length_rejects_non_decimal_values() {
    for value in ["5x", "+5", "-5", "0x5", "", "5 5", "5\t5", "5, 5", "\x005"] {
        let headers: String = format!("HTTP/1.1 200 OK\r\nContent-Length: {value}\r\n\r\n");
        assert!(get_content_length(headers.as_bytes()).is_err(), "{value:?}");
    }
    assert_eq!(
        get_content_length(b"Content-Length:\t 0005 \t\r\n\r\n").unwrap(),
        5
    );
}

#[test]
fn response_content_length_rejects_overflow_without_panicking() {
    let overflow: String = format!("{}0", usize::MAX);
    let headers: String = format!("HTTP/1.1 200 OK\r\nContent-Length: {overflow}\r\n\r\n");
    assert!(matches!(
        parse_decimal_bytes(overflow.as_bytes()),
        Err(RequestError::ContentLengthTooLarge(HttpStatus::BadRequest))
    ));
    assert!(matches!(
        get_content_length(headers.as_bytes()),
        Err(RequestError::ContentLengthTooLarge(HttpStatus::BadRequest))
    ));
    let maximum: String = format!("Content-Length: {}\r\n\r\n", usize::MAX);
    assert!(matches!(
        get_content_length(maximum.as_bytes()),
        Err(RequestError::ContentLengthTooLarge(HttpStatus::BadRequest))
    ));
}

#[test]
fn response_framing_rejects_transfer_encoding_with_content_length() {
    for headers in [
        b"HTTP/1.1 200 OK\r\nContent-Length: 0\r\nTransfer-Encoding: chunked\r\n\r\n".as_slice(),
        b"HTTP/1.1 200 OK\r\nTRANSFER-ENCODING: CHUNKED\r\ncontent-length: 5\r\n\r\n".as_slice(),
    ] {
        let mut length: usize = 0;
        let mut redirect: Option<Vec<u8>> = None;
        let mut chunked: bool = false;
        assert!(
            parse_response_headers(
                headers,
                b"http/1.1",
                b"location:",
                &mut length,
                &mut redirect,
                &mut chunked
            )
            .is_err()
        );
    }
}

#[test]
fn response_framing_rejects_unsupported_transfer_codings() {
    for value in [
        "gzip",
        "chunked, gzip",
        "gzip, chunked",
        "xchunked",
        "chunkedx",
        "chunked;foo=bar",
        "",
        "identity",
    ] {
        let headers: String = format!("HTTP/1.1 200 OK\r\nTransfer-Encoding: {value}\r\n\r\n");
        assert!(
            is_chunked_encoding(headers.as_bytes()).is_err(),
            "{value:?}"
        );
    }
    assert!(is_chunked_encoding(b"Transfer-Encoding:\t ChUnKeD \t\r\n\r\n").unwrap());
}

#[test]
fn response_framing_rejects_duplicate_length_and_transfer_fields() {
    for headers in [
        b"HTTP/1.1 200 OK\r\nContent-Length: 1\r\ncontent-length: 2\r\n\r\n".as_slice(),
        b"HTTP/1.1 200 OK\r\nContent-Length: 1\r\nContent-Length: 1\r\n\r\n".as_slice(),
        b"HTTP/1.1 200 OK\r\nTransfer-Encoding: chunked\r\ntransfer-encoding: chunked\r\n\r\n"
            .as_slice(),
    ] {
        let mut length: usize = 0;
        let mut redirect: Option<Vec<u8>> = None;
        let mut chunked: bool = false;
        assert!(
            parse_response_headers(
                headers,
                b"http/1.1",
                b"location:",
                &mut length,
                &mut redirect,
                &mut chunked
            )
            .is_err()
        );
    }
}

#[test]
fn response_headers_reject_invalid_names_and_control_values() {
    for field in [
        "Content-Length : 5",
        " Content-Length: 5",
        "\tContent-Length: 5",
        "X-Test: a\x00b",
        "X-Test: a\x7fb",
        "X-Test: a\rb",
        "X-Test: a\nb",
    ] {
        let headers: String = format!("HTTP/1.1 200 OK\r\n{field}\r\n\r\n");
        assert!(get_content_length(headers.as_bytes()).is_err(), "{field:?}");
    }
}

#[test]
fn chunked_body_rejects_invalid_or_excessive_chunk_sizes() {
    for size in [
        "+1",
        "-1",
        "0x1",
        " 1",
        "1 ",
        "1\t",
        "g",
        "",
        "ffffffffffffffffffffffffffffffff",
        "ffffffffffffffff",
        "4000001",
    ] {
        let body: String = format!("{size}\r\nx\r\n0\r\n\r\n");
        assert!(chunked_body_end(body.as_bytes()).is_err(), "{size:?}");
        assert!(parse_chunked_body(body.as_bytes()).is_err(), "{size:?}");
    }
    assert_eq!(parse_chunk_size(b"aBc;name=value").unwrap(), 0xabc);
}

#[test]
fn chunked_body_requires_data_and_final_trailer_delimiters() {
    for body in [
        b"1\r\nxXX0\r\n\r\n".as_slice(),
        b"1\r\nx\r\n0\r\n".as_slice(),
        b"1\r\nx\r\n0\r\nX-Trailer: value\r\n".as_slice(),
    ] {
        assert!(parse_chunked_body(body).is_err());
    }
    assert_eq!(chunked_body_end(b"0\r\n").unwrap(), None);
    assert_eq!(chunked_body_end(b"0\r\n\r\n").unwrap(), Some(5));
    let body: &[u8] = b"1\r\nx\r\n0\r\nX-Trailer: value\r\n\r\n";
    assert_eq!(chunked_body_end(body).unwrap(), Some(body.len()));
    assert_eq!(parse_chunked_body(body).unwrap(), b"x");
}

#[test]
fn response_redirect_uses_exact_location_header() {
    let mut length: usize = 0;
    let mut redirect: Option<Vec<u8>> = None;
    let mut chunked: bool = false;
    parse_response_headers(b"HTTP/1.1 302 Found\r\nX-Location: http://evil.test/\r\nLocation: http://safe.test/\r\nContent-Length: 0\r\n\r\n", b"http/1.1", b"location:", &mut length, &mut redirect, &mut chunked).unwrap();
    assert_eq!(redirect, Some(b"http://safe.test/".to_vec()));
}
