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
    let io_error: std::io::Error = std::io::Error::other("test error");
    let response_error: ResponseError = ResponseError::from(io_error);
    assert!(matches!(response_error, ResponseError::Send(_)));
}
