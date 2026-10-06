use super::*;

#[test]
fn test_response_default_is_unknown() {
    let response: HttpResponse = HttpResponse::default();
    assert_eq!(response.status_code, HttpStatus::Unknown.code());
    assert_eq!(response.status_code, 0usize);
    assert_eq!(response.reason_phrase, "Unknown");
    assert_eq!(response.version, HttpVersion::Http1_1);
    assert!(response.headers.is_empty());
    assert!(response.body.is_empty());
    assert!(!response.is_success());
    assert!(!response.is_redirect());
}

#[test]
fn test_response_from_bytes_parses_status_headers_body() {
    let response: HttpResponse = HttpResponse::from_bytes(
        b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nX-Empty:\r\n\r\nhello",
    );
    assert_eq!(response.status_code, 200usize);
    assert_eq!(response.reason_phrase, "OK");
    assert_eq!(response.version, HttpVersion::Http1_1);
    assert_eq!(response.headers.len(), 1usize);
    assert_eq!(
        response.get_header("content-type"),
        Some("application/json")
    );
    assert_eq!(response.bytes(), b"hello");
    assert_eq!(response.text(), "hello");
    assert!(response.is_success());
    assert!(!response.is_redirect());
}

#[test]
fn test_response_get_header_is_case_insensitive() {
    let response: HttpResponse = HttpResponse::from_bytes(
        b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nX-Empty:\r\n\r\nhello",
    );
    assert_eq!(
        response.get_header("Content-Type"),
        Some("application/json")
    );
    assert_eq!(
        response.get_header("CONTENT-TYPE"),
        Some("application/json")
    );
    assert_eq!(response.get_header("absent"), None);
}

#[test]
fn test_response_from_bytes_skips_valueless_header() {
    let response: HttpResponse = HttpResponse::from_bytes(
        b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nX-Empty:\r\n\r\nhello",
    );
    assert!(response.get_header("x-empty").is_none());
    assert!(!response.headers.contains_key("x-empty"));
}

#[test]
fn test_response_from_bytes_parses_redirect() {
    let raw: &[u8] = b"HTTP/1.1 302 Found\r\nLocation: http://target.test/\r\n\r\n";
    let response: HttpResponse = HttpResponse::from_bytes(raw);
    assert_eq!(response.status_code, 302usize);
    assert_eq!(response.reason_phrase, "Found");
    assert_eq!(response.get_header("location"), Some("http://target.test/"));
    assert!(response.is_redirect());
    assert!(!response.is_success());
}

#[test]
fn test_response_from_bytes_joins_multi_word_reason_and_body() {
    let raw: &[u8] = b"HTTP/1.1 500 Internal Server Error\r\nX-Trace: t1\r\n\r\nline1\r\nline2";
    let response: HttpResponse = HttpResponse::from_bytes(raw);
    assert_eq!(response.status_code, 500usize);
    assert_eq!(response.reason_phrase, "Internal Server Error");
    assert_eq!(response.get_header("x-trace"), Some("t1"));
    assert_eq!(response.bytes(), b"line1\nline2");
    assert!(!response.is_success());
    assert!(!response.is_redirect());
}

#[test]
fn test_response_from_bytes_falls_back_on_unparsable_status_line() {
    let raw: &[u8] = b"garbage\r\n\r\n";
    let response: HttpResponse = HttpResponse::from_bytes(raw);
    assert_eq!(response.status_code, HttpStatus::Unknown.code());
    assert_eq!(response.reason_phrase, "Unknown");
    assert!(response.version.is_unknown());
    assert!(response.body.is_empty());
}

#[test]
fn test_response_from_bytes_handles_empty_input() {
    let response: HttpResponse = HttpResponse::from_bytes(&[]);
    assert_eq!(response.status_code, HttpStatus::Unknown.code());
    assert_eq!(response.reason_phrase, "Unknown");
    assert!(response.body.is_empty());
    assert!(response.headers.is_empty());
}

#[test]
fn test_response_from_bytes_status_code_needs_three_digits() {
    let raw: &[u8] = b"HTTP/1.1 20 OK\r\n\r\n";
    let response: HttpResponse = HttpResponse::from_bytes(raw);
    assert_eq!(response.status_code, 20usize);
}

#[test]
fn test_response_getters_read_public_fields() {
    let response: HttpResponse = HttpResponse::from_bytes(
        b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nX-Empty:\r\n\r\nhello",
    );
    assert_eq!(response.get_status_code(), response.status_code);
    assert_eq!(response.get_reason_phrase(), &response.reason_phrase);
    assert_eq!(response.get_headers(), &response.headers);
    assert_eq!(response.get_body(), &response.body);
    assert_eq!(response.get_version(), &response.version);
}

#[test]
fn test_response_setters_write_public_fields() {
    let mut response: HttpResponse = HttpResponse::default();
    response.set_status_code(201usize);
    response.set_reason_phrase("Created");
    response.set_version(HttpVersion::Http2);
    response.set_body(vec![b'a', b'b', b'c']);
    assert_eq!(response.status_code, 201usize);
    assert_eq!(response.reason_phrase, "Created");
    assert_eq!(response.version, HttpVersion::Http2);
    assert_eq!(response.bytes(), b"abc");
}

#[test]
fn test_response_set_headers_replaces_map() {
    let mut response: HttpResponse = HttpResponse::from_bytes(
        b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nX-Empty:\r\n\r\nhello",
    );
    let mut replacement: HttpResponseHeaders = new_response_headers();
    replacement.insert("x-only".to_string(), "1".to_string());
    response.set_headers(replacement);
    assert_eq!(response.headers.len(), 1usize);
    assert_eq!(response.get_header("x-only"), Some("1"));
    assert!(response.get_header("content-type").is_none());
}

#[test]
fn test_response_clone_is_equal_by_content() {
    let response: HttpResponse = HttpResponse::from_bytes(
        b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nX-Empty:\r\n\r\nhello",
    );
    let cloned: HttpResponse = response.clone();
    assert_eq!(cloned.status_code, response.status_code);
    assert_eq!(cloned.reason_phrase, response.reason_phrase);
    assert_eq!(cloned.headers, response.headers);
    assert_eq!(cloned.body, response.body);
}

#[test]
fn test_response_decode_without_content_encoding_keeps_body() {
    let response: HttpResponse = HttpResponse::from_bytes(
        b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nX-Empty:\r\n\r\nhello",
    );
    let decoded: HttpResponse = response.decode(4096usize);
    assert_eq!(decoded.body, response.body);
    assert_eq!(decoded.status_code, response.status_code);
    assert_eq!(decoded.reason_phrase, response.reason_phrase);
    assert_eq!(decoded.headers, response.headers);
    assert_eq!(decoded.version, response.version);
}

#[test]
fn test_response_text_replaces_invalid_utf8() {
    let mut response: HttpResponse = HttpResponse::default();
    response.set_body(vec![b'a', 0xff, b'b']);
    assert_eq!(response.bytes(), &[b'a', 0xff, b'b']);
    assert_eq!(response.text(), "a\u{fffd}b");
    assert_eq!(response.text().len(), 5usize);
}

#[test]
fn test_new_response_headers_starts_empty() {
    let mut headers: HttpResponseHeaders = new_response_headers();
    assert!(headers.is_empty());
    headers.insert("k".to_string(), "v".to_string());
    assert_eq!(headers.len(), 1usize);
    assert_eq!(headers.get("k").map(String::as_str), Some("v"));
}

#[test]
fn test_response_debug_lists_status() {
    let response: HttpResponse = HttpResponse::from_bytes(
        b"HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nX-Empty:\r\n\r\nhello",
    );
    let debugged: String = format!("{response:?}");
    assert!(debugged.contains("200"));
    assert!(debugged.contains("OK"));
}
