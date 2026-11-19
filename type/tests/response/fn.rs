use super::*;

#[test]
fn response_default() {
    let response: Response = Response::default();
    assert_eq!(response.get_status_code(), 200);
    assert_eq!(response.get_reason_phrase(), "OK");
    assert!(response.get_headers().is_empty());
    assert!(response.get_body().is_empty());
}

#[test]
fn response_reason_phrase_follows_the_status_code() {
    let mut response: Response = Response::default();
    response.set_status_code(404);
    let _: Vec<u8> = response.build();
    assert_eq!(response.get_reason_phrase(), "Not Found");
}

#[test]
fn response_keeps_the_reason_phrase_that_was_set() {
    let mut response: Response = Response::default();
    response.set_reason_phrase("Gone");
    response.set_status_code(410);
    let _: Vec<u8> = response.build();
    assert_eq!(response.get_reason_phrase(), "Gone");
}

#[test]
fn response_reset_restores_the_phrase_of_the_default_status() {
    let mut response: Response = Response::default();
    response.set_status_code(404);
    let _: Vec<u8> = response.build();
    assert_eq!(response.get_reason_phrase(), "Not Found");
    response.reset();
    assert_eq!(response.get_reason_phrase(), "OK");
}

#[test]
fn response_build_writes_the_status_line_and_the_headers() {
    let mut response: Response = Response::default();
    response.set_header("connection", "keep-alive");
    let data: Vec<u8> = response.build();
    let text: String = String::from_utf8_lossy(&data).into_owned();
    assert!(text.starts_with("HTTP/1.1 200 OK\r\n"), "{text:?}");
    assert!(text.contains("connection:keep-alive\r\n"), "{text:?}");
    assert!(text.contains("content-length:0\r\n"), "{text:?}");
    assert!(text.ends_with("\r\n\r\n"), "{text:?}");
}

#[test]
fn response_build_writes_the_exact_bytes() {
    let mut response: Response = Response::default();
    response.set_header("connection", "keep-alive");
    response.set_status_code(200);
    let data: Vec<u8> = response.build();
    assert_eq!(
        String::from_utf8_lossy(&data),
        "HTTP/1.1 200 OK\r\ncontent-length:0\r\nconnection:keep-alive\r\n\
         content-type:text/html; charset=utf-8\r\n\r\n"
    );
}

#[test]
fn response_build_writes_the_exact_bytes_of_a_response_with_a_body() {
    let mut response: Response = Response::default();
    response.set_status_code(404);
    response.set_body("hello");
    let data: Vec<u8> = response.build();
    assert_eq!(
        String::from_utf8_lossy(&data),
        "HTTP/1.1 404 Not Found\r\ncontent-length:5\r\nconnection:keep-alive\r\n\
         content-type:text/html; charset=utf-8\r\n\r\nhello"
    );
}

#[test]
fn response_build_writes_the_exact_bytes_of_a_response_with_a_long_length() {
    let mut response: Response = Response::default();
    response.set_body(vec![b'x'; 1234]);
    let data: Vec<u8> = response.build();
    let text: String = String::from_utf8_lossy(&data).into_owned();
    assert!(
        text.starts_with(
            "HTTP/1.1 200 OK\r\ncontent-length:1234\r\nconnection:keep-alive\r\n\
             content-type:text/html; charset=utf-8\r\n\r\n"
        ),
        "{text:?}"
    );
    assert_eq!(data.len(), 1336);
}

fn response_wire_parts(data: &[u8]) -> (&str, &[u8]) {
    let separator: &[u8] = b"\r\n\r\n";
    let end: usize = data
        .windows(separator.len())
        .position(|window: &[u8]| window == separator)
        .expect("response has no header terminator");
    let head: &str = str::from_utf8(&data[..end]).expect("response head is not UTF-8");
    (head, &data[end + separator.len()..])
}

fn response_wire_header_values<'a>(head: &'a str, key: &str) -> Vec<&'a str> {
    head.split("\r\n")
        .skip(1)
        .filter_map(|line: &str| line.split_once(':'))
        .filter_map(|(name, value): (&str, &str)| {
            name.eq_ignore_ascii_case(key).then_some(value.trim())
        })
        .collect()
}

#[test]
fn response_build_early_hints_omits_body_and_framing_headers() {
    let mut response: Response = Response::default();
    response.set_status_code(103);
    response.set_header(TRANSFER_ENCODING, CHUNKED);
    response.set_body("discard me");

    let data: ResponseData = response.build();
    let (head, body): (&str, &[u8]) = response_wire_parts(&data);
    let lengths: Vec<&str> = response_wire_header_values(head, CONTENT_LENGTH);
    let encodings: Vec<&str> = response_wire_header_values(head, TRANSFER_ENCODING);

    assert!(head.starts_with("HTTP/1.1 103 Early Hints\r\n"), "{head:?}");
    assert_eq!(
        (lengths, encodings, body),
        (Vec::<&str>::new(), Vec::<&str>::new(), b"".as_slice()),
        "{head:?}"
    );
}

#[test]
fn response_build_no_content_omits_body_and_framing_headers() {
    let mut response: Response = Response::default();
    response.set_status_code(204);
    response
        .get_mut_headers()
        .insert(CONTENT_LENGTH.to_owned(), VecDeque::from(["10".to_owned()]));
    response.set_header(TRANSFER_ENCODING, CHUNKED);
    response.set_body("discard me");

    let data: ResponseData = response.build();
    let (head, body): (&str, &[u8]) = response_wire_parts(&data);
    let lengths: Vec<&str> = response_wire_header_values(head, CONTENT_LENGTH);
    let encodings: Vec<&str> = response_wire_header_values(head, TRANSFER_ENCODING);

    assert!(head.starts_with("HTTP/1.1 204 No Content\r\n"), "{head:?}");
    assert_eq!(
        (lengths, encodings, body),
        (Vec::<&str>::new(), Vec::<&str>::new(), b"".as_slice()),
        "{head:?}"
    );
}

#[test]
fn response_build_reset_content_writes_zero_length_even_for_event_stream() {
    let mut response: Response = Response::default();
    response.set_status_code(205);
    response.set_header(CONTENT_TYPE, TEXT_EVENT_STREAM);
    response.set_header(TRANSFER_ENCODING, CHUNKED);
    response.set_body("data: ignored\n\n");

    let data: ResponseData = response.build();
    let (head, body): (&str, &[u8]) = response_wire_parts(&data);
    let lengths: Vec<&str> = response_wire_header_values(head, CONTENT_LENGTH);
    let encodings: Vec<&str> = response_wire_header_values(head, TRANSFER_ENCODING);

    assert!(
        head.starts_with("HTTP/1.1 205 Reset Content\r\n"),
        "{head:?}"
    );
    assert_eq!(
        (lengths, encodings, body),
        (vec!["0"], Vec::<&str>::new(), b"".as_slice()),
        "{head:?}"
    );
}

#[test]
fn response_build_not_modified_preserves_etag_without_body_or_generated_length() {
    let mut response: Response = Response::default();
    response.set_status_code(304);
    response.set_header(ETAG, "\"revision-1\"");
    response.set_body("discard me");

    let data: ResponseData = response.build();
    let (head, body): (&str, &[u8]) = response_wire_parts(&data);
    let lengths: Vec<&str> = response_wire_header_values(head, CONTENT_LENGTH);
    let etags: Vec<&str> = response_wire_header_values(head, ETAG);

    assert!(
        head.starts_with("HTTP/1.1 304 Not Modified\r\n"),
        "{head:?}"
    );
    assert_eq!(
        (lengths, etags, body),
        (Vec::<&str>::new(), vec!["\"revision-1\""], b"".as_slice()),
        "{head:?}"
    );
}

#[test]
fn response_build_head_preserves_encoding_without_generating_a_length() {
    let mut response: Response = Response::default();
    response.set_body(b"representation");
    let _: ResponseData = response.build();
    response.set_header(CONTENT_ENCODING, GZIP);
    let data: ResponseData = response.build_head();
    let (head, body): (&str, &[u8]) = response_wire_parts(&data);
    assert!(body.is_empty());
    assert!(response_wire_header_values(head, CONTENT_LENGTH).is_empty());
    assert_eq!(response_wire_header_values(head, CONTENT_ENCODING), [GZIP]);
    assert_eq!(response.get_body(), b"representation");
}

#[test]
fn response_build_head_omits_length_when_no_representation_was_provided() {
    let data: ResponseData = Response::default().build_head();
    let (head, body): (&str, &[u8]) = response_wire_parts(&data);
    assert!(body.is_empty());
    assert!(response_wire_header_values(head, CONTENT_LENGTH).is_empty());
}

#[test]
fn response_build_announces_the_length_of_the_body() {
    let mut response: Response = Response::default();
    response.set_body("hello");
    let data: Vec<u8> = response.build();
    let text: String = String::from_utf8_lossy(&data).into_owned();
    assert!(text.contains("content-length:5\r\n"), "{text:?}");
    assert!(text.ends_with("hello"), "{text:?}");
}

#[test]
fn response_build_announces_the_length_of_a_large_body() {
    let mut response: Response = Response::default();
    let body: Vec<u8> = vec![b'x'; 1234];
    response.set_body(body);
    let data: Vec<u8> = response.build();
    let text: String = String::from_utf8_lossy(&data).into_owned();
    assert!(text.contains("content-length:1234\r\n"), "{text:?}");
}

#[test]
fn response_header_operations() {
    let mut response: Response = Response::default();
    response.set_header("content-type", "application/json");
    assert!(response.has_header("content-type"));
    assert!(!response.has_header("authorization"));
    assert_eq!(response.get_header_size("content-type"), 1);
    assert_eq!(
        response.try_get_header_front("content-type"),
        Some("application/json".to_string())
    );
    assert_eq!(
        response.try_get_header_back("content-type"),
        Some("application/json".to_string())
    );
    assert_eq!(response.get_headers_size(), 1);
}

#[test]
fn response_set_header_reuses_entry_and_replaces_repeated_values() {
    let mut response: Response = Response::default();
    response.add_header("x-custom", "first");
    response.add_header("x-custom", "second");
    response.set_header("x-custom", "replacement");
    assert_eq!(response.get_header_size("x-custom"), 1);
    assert_eq!(response.get_header_back("x-custom"), "replacement");
    response.set_header("x-custom", "again");
    assert_eq!(response.get_header_back("x-custom"), "again");
    response.set_header("content-length", "incorrect");
    assert!(!response.has_header("content-length"));
    response.reset();
    response.set_header("x-custom", "next request");
    assert_eq!(response.get_header_back("x-custom"), "next request");
}

#[test]
fn response_build_fills_empty_default_headers() {
    let mut response: Response = Response::default();
    response
        .get_mut_headers()
        .insert("connection".to_owned(), VecDeque::new());
    response
        .get_mut_headers()
        .insert("content-type".to_owned(), VecDeque::new());
    let _: Vec<u8> = response.build();
    assert_eq!(response.get_header_back("connection"), "keep-alive");
    assert_eq!(
        response.get_header_back("content-type"),
        "text/html; charset=utf-8"
    );
}

#[test]
fn response_add_header() {
    let mut response: Response = Response::default();
    response.add_header("x-custom", "value1");
    response.add_header("x-custom", "value2");
    assert_eq!(response.get_header_size("x-custom"), 2);
    assert!(response.has_header_value("x-custom", "value1"));
    assert!(response.has_header_value("x-custom", "value2"));
}

#[test]
fn response_remove_header() {
    let mut response: Response = Response::default();
    response.set_header("x-custom", "value");
    assert!(response.has_header("x-custom"));
    response.remove_header("x-custom");
    assert!(!response.has_header("x-custom"));
}

#[test]
fn response_remove_header_value() {
    let mut response: Response = Response::default();
    response.add_header("x-custom", "value1");
    response.add_header("x-custom", "value2");
    response.remove_header_value("x-custom", "value1");
    assert!(response.has_header("x-custom"));
    assert!(!response.has_header_value("x-custom", "value1"));
    assert!(response.has_header_value("x-custom", "value2"));
}

#[test]
fn response_clear_headers() {
    let mut response: Response = Response::default();
    response.set_header("header1", "value1");
    response.set_header("header2", "value2");
    assert_eq!(response.get_headers_size(), 2);
    response.clear_headers();
    assert_eq!(response.get_headers_size(), 0);
}

#[test]
fn response_body_operations() {
    let mut response: Response = Response::default();
    response.set_body(b"hello world");
    assert_eq!(response.get_body(), b"hello world");
    assert_eq!(response.get_body_string(), "hello world");
}

#[test]
fn response_body_json() {
    let json: &'static str = r#"{"name":"test","value":123}"#;
    let mut response: Response = Response::default();
    response.set_body(json.as_bytes());
    #[derive(Debug, Deserialize, PartialEq)]
    struct TestData {
        name: String,
        value: i32,
    }
    let result: TestData = response.try_get_body_json().unwrap();
    assert_eq!(result.name, "test");
    assert_eq!(result.value, 123);
}

#[test]
fn response_cookie_operations() {
    let mut response: Response = Response::default();
    response.set_header("set-cookie", "session_id=abc123");
    let cookies: Cookies = response.get_cookies();
    assert_eq!(cookies.len(), 1);
    assert_eq!(cookies.get("session_id"), Some(&"abc123".to_string()));
}

#[test]
fn response_try_get_cookies() {
    let response: Response = Response::default();
    assert!(response.try_get_cookies().is_none());
    let mut response: Response = Response::default();
    response.set_header("set-cookie", "user_id=xyz789");
    let cookies_opt: Option<Cookies> = response.try_get_cookies();
    assert!(cookies_opt.is_some());
    let cookies: Cookies = cookies_opt.unwrap();
    assert_eq!(cookies.get("user_id"), Some(&"xyz789".to_string()));
}

#[test]
fn response_try_get_cookie() {
    let mut response: Response = Response::default();
    response.set_header("set-cookie", "token=secret123; secure");
    assert_eq!(
        response.try_get_cookie("token"),
        Some("secret123".to_string())
    );
    assert_eq!(response.try_get_cookie("not_exist"), None);
}

#[test]
fn response_get_cookie() {
    let mut response: Response = Response::default();
    response.set_header("set-cookie", "theme=dark; path=/");
    assert_eq!(response.get_cookie("theme"), "dark".to_string());
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn response_get_cookie_panic() {
    let response: Response = Response::default();
    let _: CookieValue = response.get_cookie("not_exist");
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn response_get_cookies_panic() {
    let response: Response = Response::default();
    let _: Cookies = response.get_cookies();
}

#[test]
fn response_error_display() {
    let error: ResponseError = ResponseError::NotFoundStream;
    assert_eq!(format!("{error}"), "Not found stream");
    let error: ResponseError = ResponseError::ConnectionClosed;
    assert_eq!(format!("{error}"), "Connection has been closed");
    let error: ResponseError = ResponseError::Terminated;
    assert_eq!(format!("{error}"), "Current processing has been terminated");
    let error: ResponseError = ResponseError::Send("network error".to_string());
    assert_eq!(format!("{error}"), "Send error: network error");
}

#[test]
fn response_error_from_io() {
    let io_error: io::Error = io::Error::other("test error");
    let response_error: ResponseError = ResponseError::from(io_error);
    assert!(matches!(response_error, ResponseError::Send(_)));
}
