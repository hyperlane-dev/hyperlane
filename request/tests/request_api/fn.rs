use super::*;

#[test]
fn test_http_request_get_sets_method_and_url() {
    let request: HttpRequest = HttpRequest::get("http://example.com/a");
    assert_eq!(request.get_method(), Method::Get);
    assert!(request.get_method().is_get());
    assert_eq!(request.get_url(), "http://example.com/a");
    assert_eq!(request.get_url_ref(), "http://example.com/a");
    assert!(request.get_headers().is_empty());
    assert!(request.get_body_ref().get_bytes_ref().is_empty());
}

#[test]
fn test_http_request_post_sets_method_and_url() {
    let request: HttpRequest = HttpRequest::post("https://example.com/b");
    assert_eq!(request.get_method(), Method::Post);
    assert!(request.get_method().is_post());
    assert_eq!(request.get_url_ref(), "https://example.com/b");
}

#[test]
fn test_http_request_accepts_owned_url() {
    let request: HttpRequest = HttpRequest::get(String::from("http://owned.test/"));
    assert_eq!(request.get_url_ref(), "http://owned.test/");
}

#[test]
fn test_http_request_default_is_empty_request() {
    let request: HttpRequest = HttpRequest::default();
    assert_eq!(request.get_url_ref(), "");
    assert_eq!(request.url, "");
    assert!(request.get_method().is_unknown());
    assert!(request.get_headers_ref().is_empty());
    assert!(request.get_body_ref().get_bytes_ref().is_empty());
    assert_eq!(
        *request.get_config_ref().get_http_version(),
        HttpVersion::Http1_1
    );
}

#[test]
fn test_http_request_get_sets_url_matches_field() {
    let request: HttpRequest = HttpRequest::get("http://example.com/");
    assert_eq!(request.url, "http://example.com/");
    assert_eq!(request.method, Method::Get);
}

#[test]
fn test_http_request_set_method_overrides_default() {
    let mut request: HttpRequest = HttpRequest::get("http://example.com/");
    request.set_method(Method::Put);
    assert_eq!(request.get_method(), Method::Put);
    assert!(!request.get_method().is_get());
    assert_eq!(request.method, Method::Put);
}

#[test]
fn test_http_request_set_url_overrides_previous() {
    let mut request: HttpRequest = HttpRequest::get("http://first.test/");
    request.set_url("http://second.test/");
    assert_eq!(request.get_url_ref(), "http://second.test/");
}

#[test]
fn test_http_request_set_header_normalizes_key_to_lowercase() {
    let mut request: HttpRequest = HttpRequest::get("http://example.com/");
    request.set_header("Content-Type", "application/json");
    let headers: &HashMap<String, String> = request.get_headers_ref();
    assert_eq!(headers.len(), 1usize);
    assert_eq!(
        headers.get("content-type").map(String::as_str),
        Some("application/json")
    );
    assert!(headers.get("Content-Type").is_none());
}

#[test]
fn test_http_request_set_header_last_write_wins_across_cases() {
    let mut request: HttpRequest = HttpRequest::get("http://example.com/");
    request.set_header("X-Token", "first");
    request.set_header("x-token", "second");
    let headers: &HashMap<String, String> = request.get_headers_ref();
    assert_eq!(headers.len(), 1usize);
    assert_eq!(headers.get("x-token").map(String::as_str), Some("second"));
    assert!(headers.get("X-Token").is_none());
}

#[test]
fn test_http_request_remove_header_is_case_insensitive() {
    let mut request: HttpRequest = HttpRequest::get("http://example.com/");
    request.set_header("X-Token", "value");
    request.remove_header("x-token");
    assert!(request.get_headers_ref().is_empty());
    request.set_header("X-Token", "value");
    request.remove_header("X-TOKEN");
    assert!(request.get_headers_ref().is_empty());
}

#[test]
fn test_http_request_remove_missing_header_is_noop() {
    let mut request: HttpRequest = HttpRequest::get("http://example.com/");
    request.set_header("X-Token", "value");
    request.remove_header("absent");
    assert_eq!(request.get_headers_ref().len(), 1usize);
}

#[test]
fn test_http_request_clear_headers_drops_all() {
    let mut request: HttpRequest = HttpRequest::get("http://example.com/");
    request.set_header("A", "1");
    request.set_header("B", "2");
    assert_eq!(request.get_headers().len(), 2usize);
    request.clear_headers();
    assert!(request.get_headers_ref().is_empty());
}

#[test]
fn test_http_request_get_mut_headers_writes_directly() {
    let mut request: HttpRequest = HttpRequest::get("http://example.com/");
    let headers: &mut HashMap<String, String> = request.get_mut_headers();
    headers.insert("X-raw".to_string(), "v".to_string());
    assert_eq!(
        request.get_headers_ref().get("X-raw").map(String::as_str),
        Some("v")
    );
}

#[test]
fn test_http_request_set_body_replaces_previous() {
    let mut request: HttpRequest = HttpRequest::post("http://example.com/");
    request.set_body(Body::from_bytes("data"));
    assert_eq!(request.get_body().get_bytes_ref(), b"data");
    assert_eq!(request.get_body_ref().as_str(), Some("data"));
    request.set_body(Body::empty());
    assert!(request.get_body_ref().get_bytes_ref().is_empty());
    assert_eq!(request.body, Body::empty());
}

#[test]
fn test_http_request_config_defaults() {
    let request: HttpRequest = HttpRequest::get("http://example.com/");
    assert_eq!(request.get_config_ref().get_buffer_size(), 0usize);
    assert_eq!(request.get_config_ref().get_timeout(), 0u64);
    assert_eq!(request.get_config_ref().get_max_redirect_times(), 0usize);
    assert_eq!(
        *request.get_config_ref().get_http_version(),
        HttpVersion::Http1_1
    );
    assert!(!request.get_config_ref().get_redirect());
    assert!(!request.get_config_ref().get_decode());
    assert!(request.get_config_ref().get_proxy().is_none());
}

#[test]
fn test_http_request_config_setters_are_chainable() {
    let mut request: HttpRequest = HttpRequest::get("http://example.com/");
    request
        .get_config_mut()
        .set_buffer_size(4096usize)
        .set_timeout(1500u64)
        .set_max_redirect_times(3usize)
        .set_http_version(HttpVersion::Http2)
        .set_redirect(true)
        .set_decode(true);
    assert_eq!(request.get_config_ref().get_buffer_size(), 4096usize);
    assert_eq!(request.get_config_ref().get_timeout(), 1500u64);
    assert_eq!(request.get_config_ref().get_max_redirect_times(), 3usize);
    assert_eq!(
        *request.get_config_ref().get_http_version(),
        HttpVersion::Http2
    );
    assert!(request.get_config_ref().get_redirect());
    assert!(request.get_config_ref().get_decode());
}

#[test]
fn test_http_request_config_proxy_setter() {
    let mut request: HttpRequest = HttpRequest::get("http://example.com/");
    request
        .get_config_mut()
        .set_proxy(Some(Proxy::http("h", 1u16)));
    let proxy: &Option<Proxy> = request.get_config_ref().get_proxy();
    assert_eq!(
        proxy.as_ref().map(|item: &Proxy| item.host.clone()),
        Some("h".to_string())
    );
    request.get_config_mut().set_proxy(None);
    assert!(request.get_config_ref().get_proxy().is_none());
}

#[test]
fn test_http_request_set_config_replaces_whole_config() {
    let mut source: HttpRequest = HttpRequest::get("http://source.test/");
    source.get_config_mut().set_timeout(1234u64);
    source.get_config_mut().set_buffer_size(64usize);
    let mut target: HttpRequest = HttpRequest::post("http://target.test/");
    target.get_config_mut().set_timeout(9999u64);
    target.set_config(source.get_config());
    assert_eq!(target.get_config(), source.get_config());
    assert_eq!(target.get_config_ref().get_timeout(), 1234u64);
    assert_eq!(target.get_config_ref().get_buffer_size(), 64usize);
    assert_eq!(target.config, source.config);
}

#[test]
fn test_http_request_get_config_returns_owned_copy() {
    let request: HttpRequest = HttpRequest::get("http://example.com/");
    assert_eq!(request.get_config(), request.get_config());
    assert_eq!(request.config, request.get_config());
}

#[test]
fn test_http_request_clone_copies_headers_and_body() {
    let mut request: HttpRequest = HttpRequest::post("http://example.com/");
    request.set_header("X-Token", "v");
    request.set_body(Body::from_bytes("payload"));
    let cloned: HttpRequest = request.clone();
    assert_eq!(cloned.get_url_ref(), request.get_url_ref());
    assert_eq!(cloned.get_method(), request.get_method());
    assert_eq!(cloned.get_headers_ref(), request.get_headers_ref());
    assert_eq!(cloned.get_body_ref(), request.get_body_ref());
}

#[test]
fn test_http_request_debug_lists_url() {
    let request: HttpRequest = HttpRequest::get("http://debug.test/");
    let debugged: String = format!("{request:?}");
    assert!(debugged.contains("http://debug.test/"));
}
