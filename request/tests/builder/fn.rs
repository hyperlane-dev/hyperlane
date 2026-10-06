use super::*;

#[test]
fn test_builder_new_builds_empty_request() {
    let mut builder: RequestBuilder = RequestBuilder::new();
    let request: HttpRequest = builder.build();
    assert_eq!(request.get_url_ref(), "");
    assert!(request.get_method().is_unknown());
    assert!(request.get_headers_ref().is_empty());
    assert!(request.get_body_ref().get_bytes_ref().is_empty());
}

#[test]
fn test_builder_get_sets_method_and_url() {
    let mut builder: RequestBuilder = RequestBuilder::new();
    let request: HttpRequest = builder.get("http://get.test/").build();
    assert!(request.get_method().is_get());
    assert_eq!(request.get_url_ref(), "http://get.test/");
}

#[test]
fn test_builder_post_sets_method_and_url() {
    let mut builder: RequestBuilder = RequestBuilder::new();
    let request: HttpRequest = builder.post("http://post.test/").build();
    assert!(request.get_method().is_post());
    assert_eq!(request.get_url_ref(), "http://post.test/");
}

#[test]
fn test_builder_method_and_url_overrides_shortcuts() {
    let mut builder: RequestBuilder = RequestBuilder::new();
    let request: HttpRequest = builder
        .get("http://first.test/")
        .method(Method::Put)
        .url("http://second.test/")
        .build();
    assert!(request.get_method() == Method::Put);
    assert_eq!(request.get_url_ref(), "http://second.test/");
}

#[test]
fn test_builder_header_normalizes_key() {
    let mut builder: RequestBuilder = RequestBuilder::new();
    let request: HttpRequest = builder
        .get("http://a.test/")
        .header("Accept", "application/json")
        .build();
    assert_eq!(
        request.get_headers_ref().get("accept").map(String::as_str),
        Some("application/json")
    );
}

#[test]
fn test_builder_headers_inserts_every_pair() {
    let mut headers: HashMap<String, String> = HashMap::new();
    headers.insert("A".to_string(), "1".to_string());
    headers.insert("B".to_string(), "2".to_string());
    let mut builder: RequestBuilder = RequestBuilder::new();
    let request: HttpRequest = builder.get("http://a.test/").headers(headers).build();
    assert_eq!(request.get_headers_ref().len(), 2usize);
    assert_eq!(
        request.get_headers_ref().get("a").map(String::as_str),
        Some("1")
    );
    assert_eq!(
        request.get_headers_ref().get("b").map(String::as_str),
        Some("2")
    );
}

#[test]
fn test_builder_remove_and_clear_headers() {
    let mut builder: RequestBuilder = RequestBuilder::new();
    let request: HttpRequest = builder
        .get("http://a.test/")
        .header("A", "1")
        .header("B", "2")
        .remove_header("a")
        .build();
    assert_eq!(request.get_headers_ref().len(), 1usize);
    assert!(request.get_headers_ref().get("a").is_none());
    let mut second: RequestBuilder = RequestBuilder::new();
    let cleared: HttpRequest = second
        .get("http://a.test/")
        .header("A", "1")
        .clear_headers()
        .build();
    assert!(cleared.get_headers_ref().is_empty());
}

#[test]
fn test_builder_body_accepts_raw_bytes() {
    let mut builder: RequestBuilder = RequestBuilder::new();
    let request: HttpRequest = builder.post("http://a.test/").body(vec![1u8, 2, 3]).build();
    assert_eq!(request.get_body_ref().get_bytes_ref(), &[1u8, 2, 3]);
}

#[test]
fn test_builder_body_text_stores_utf8_bytes() {
    let mut builder: RequestBuilder = RequestBuilder::new();
    let request: HttpRequest = builder.post("http://a.test/").body_text("héllo").build();
    assert_eq!(request.get_body_ref().as_str(), Some("héllo"));
    assert_eq!(request.get_body_ref().get_bytes_ref().len(), 6usize);
}

#[test]
fn test_builder_body_json_serialises_value() {
    let body: serde_json::Value = json!({ "code": 1 });
    let mut builder: RequestBuilder = RequestBuilder::new();
    let request: HttpRequest = builder.post("http://a.test/").body_json(&body).build();
    assert_eq!(request.get_body_ref().as_str(), Some("{\"code\":1}"));
}

#[test]
fn test_builder_body_json_replaces_previous_body() {
    let first: serde_json::Value = json!({ "a": 1 });
    let second: serde_json::Value = json!({ "b": 2 });
    let mut builder: RequestBuilder = RequestBuilder::new();
    let request: HttpRequest = builder
        .post("http://a.test/")
        .body_json(&first)
        .body_json(&second)
        .build();
    assert_eq!(request.get_body_ref().as_str(), Some("{\"b\":2}"));
}

#[test]
fn test_builder_timeout_and_buffer_size() {
    let mut builder: RequestBuilder = RequestBuilder::new();
    let request: HttpRequest = builder
        .get("http://a.test/")
        .timeout(6000u64)
        .buffer_size(8192usize)
        .build();
    assert_eq!(request.get_config_ref().get_timeout(), 6000u64);
    assert_eq!(request.get_config_ref().get_buffer_size(), 8192usize);
}

#[test]
fn test_builder_http_version_toggles() {
    let mut builder: RequestBuilder = RequestBuilder::new();
    let http2: HttpRequest = builder.get("http://a.test/").http2_only().build();
    assert_eq!(
        *http2.get_config_ref().get_http_version(),
        HttpVersion::Http2
    );
    let mut other: RequestBuilder = RequestBuilder::new();
    let http1: HttpRequest = other
        .get("http://a.test/")
        .http2_only()
        .http1_1_only()
        .build();
    assert_eq!(
        *http1.get_config_ref().get_http_version(),
        HttpVersion::Http1_1
    );
}

#[test]
fn test_builder_redirect_toggles() {
    let mut builder: RequestBuilder = RequestBuilder::new();
    let request: HttpRequest = builder.get("http://a.test/").build();
    assert!(!request.get_config_ref().get_redirect());
    let mut enabled: RequestBuilder = RequestBuilder::new();
    let redirected: HttpRequest = enabled.get("http://a.test/").redirect().build();
    assert!(redirected.get_config_ref().get_redirect());
    let mut disabled: RequestBuilder = RequestBuilder::new();
    let plain: HttpRequest = disabled
        .get("http://a.test/")
        .redirect()
        .no_redirect()
        .build();
    assert!(!plain.get_config_ref().get_redirect());
}

#[test]
fn test_builder_max_redirect_times() {
    let mut builder: RequestBuilder = RequestBuilder::new();
    let request: HttpRequest = builder
        .get("http://a.test/")
        .max_redirect_times(5usize)
        .build();
    assert_eq!(request.get_config_ref().get_max_redirect_times(), 5usize);
}

#[test]
fn test_builder_decode_toggles() {
    let mut builder: RequestBuilder = RequestBuilder::new();
    let request: HttpRequest = builder.get("http://a.test/").build();
    assert!(!request.get_config_ref().get_decode());
    let mut enabled: RequestBuilder = RequestBuilder::new();
    let decoded: HttpRequest = enabled.get("http://a.test/").decode().build();
    assert!(decoded.get_config_ref().get_decode());
    let mut disabled: RequestBuilder = RequestBuilder::new();
    let plain: HttpRequest = disabled.get("http://a.test/").decode().no_decode().build();
    assert!(!plain.get_config_ref().get_decode());
}

#[test]
fn test_builder_proxy_and_no_proxy() {
    let mut builder: RequestBuilder = RequestBuilder::new();
    let request: HttpRequest = builder
        .get("http://a.test/")
        .proxy(Proxy::socks5("127.0.0.1", 1080u16))
        .build();
    let proxy: &Option<Proxy> = request.get_config_ref().get_proxy();
    assert_eq!(
        proxy.as_ref().map(|item: &Proxy| item.proxy_type),
        Some(ProxyType::Socks5)
    );
    assert_eq!(proxy.as_ref().map(|item: &Proxy| item.port), Some(1080u16));
    let mut cleared: RequestBuilder = RequestBuilder::new();
    let direct: HttpRequest = cleared
        .get("http://a.test/")
        .proxy(Proxy::http("h", 1u16))
        .no_proxy()
        .build();
    assert!(direct.get_config_ref().get_proxy().is_none());
}

#[test]
fn test_builder_build_resets_the_builder() {
    let mut builder: RequestBuilder = RequestBuilder::new();
    builder.get("http://a.test/");
    let first: HttpRequest = builder.build();
    let second: HttpRequest = builder.build();
    assert_eq!(first.get_url_ref(), "http://a.test/");
    assert_eq!(second.get_url_ref(), "");
    assert!(second.get_headers_ref().is_empty());
}

#[test]
fn test_builder_get_request_mut_exposes_underlying_request() {
    let mut builder: RequestBuilder = RequestBuilder::new();
    let inner: &mut HttpRequest = builder.get_request_mut();
    inner.set_url("http://inner.test/");
    inner.set_header("X-Token", "v");
    let request: HttpRequest = builder.build();
    assert_eq!(request.get_url_ref(), "http://inner.test/");
    assert_eq!(
        request.get_headers_ref().get("x-token").map(String::as_str),
        Some("v")
    );
}

#[test]
fn test_builder_default_and_clone_preserve_state() {
    let mut base: RequestBuilder = RequestBuilder::default();
    base.get("http://base.test/").header("A", "1");
    let mut cloned: RequestBuilder = base.clone();
    let from_clone: HttpRequest = cloned.build();
    let from_base: HttpRequest = base.build();
    assert_eq!(from_clone.get_url_ref(), "http://base.test/");
    assert_eq!(from_base.get_url_ref(), "http://base.test/");
    assert_eq!(
        from_clone.get_headers_ref().get("a").map(String::as_str),
        Some("1")
    );
    assert_eq!(
        from_base.get_headers_ref().get("a").map(String::as_str),
        Some("1")
    );
}

#[test]
fn test_builder_debug_lists_url() {
    let mut builder: RequestBuilder = RequestBuilder::new();
    builder.get("http://debug.test/");
    let debugged: String = format!("{builder:?}");
    assert!(debugged.contains("http://debug.test/"));
}
