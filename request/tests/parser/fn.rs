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
        parse_chunked_body(b"3;name=value\r\na\0\xff\r\n2\r\nbc\r\n0\r\n\r\n"),
        b"a\0\xffbc"
    );
    assert_eq!(parse_chunked_body(b"2\r\nok\r\n4\r\nno"), b"ok");
    assert!(parse_chunked_body(b"invalid\r\nbytes").is_empty());
}

#[test]
fn response_header_helpers_accept_case_insensitive_wire_names() {
    let headers: &[u8] =
        b"HTTP/1.1 200 OK\r\nCoNtEnT-LeNgTh: 12\r\nTransfer-Encoding: Chunked\r\n\r\n";
    assert_eq!(get_content_length(headers), 12);
    assert!(is_chunked_encoding(headers));
    assert_eq!(get_content_length(b"HTTP/1.1 200 OK\r\n\r\n"), 0);
    assert!(!is_chunked_encoding(b"Content-Length: 0\r\n"));
}

#[test]
fn decimal_and_status_parsers_cover_valid_and_invalid_ascii() {
    assert_eq!(parse_decimal_bytes(b" \t123rest"), 123);
    assert_eq!(parse_decimal_bytes(b"0007"), 7);
    assert_eq!(parse_decimal_bytes(b"x123"), 0);
    assert_eq!(parse_decimal_bytes(b""), 0);
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
    parse_response_headers(b"HTTP/1.1 302 Found\r\nLocation:/next\r\nContent-Length: 4\r\nTransfer-Encoding: chunked\r\n\r\n", b"http/1.1", b"location:", &mut length, &mut redirect, &mut chunked).unwrap();
    assert_eq!(length, 4);
    assert_eq!(redirect, Some(b"/next".to_vec()));
    assert!(chunked);
}
