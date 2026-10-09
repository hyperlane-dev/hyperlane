use super::*;

#[test]
fn test_url_components_parse_full_url() {
    let components: HttpUrlComponents =
        HttpUrlComponents::parse("https://host.test:8443/a/b?x=1&y=2#frag").unwrap_or_default();
    assert_eq!(components.protocol, "https");
    assert_eq!(components.host, Some("host.test".to_string()));
    assert_eq!(components.port, Some(8443u16));
    assert_eq!(components.path, Some("/a/b".to_string()));
    assert_eq!(components.query, Some("x=1&y=2".to_string()));
    assert_eq!(components.fragment, Some("frag".to_string()));
}

#[test]
fn test_url_components_parse_bare_host_uses_root_path() {
    let components: HttpUrlComponents =
        HttpUrlComponents::parse("http://host.test").unwrap_or_default();
    assert_eq!(components.protocol, "http");
    assert_eq!(components.host, Some("host.test".to_string()));
    assert_eq!(components.port, None);
    assert_eq!(components.path, Some("/".to_string()));
    assert_eq!(components.query, None);
    assert_eq!(components.fragment, None);
}

#[test]
fn test_url_components_parse_keeps_empty_query() {
    let components: HttpUrlComponents =
        HttpUrlComponents::parse("http://host.test/p?").unwrap_or_default();
    assert_eq!(components.query, Some(String::new()));
    assert_eq!(components.path, Some("/p".to_string()));
}

#[test]
fn test_url_components_parse_rejects_relative_url() {
    assert!(HttpUrlComponents::parse("not a url").is_err());
    assert!(HttpUrlComponents::parse("").is_err());
}

#[test]
fn test_url_components_default_is_all_empty() {
    let components: HttpUrlComponents = HttpUrlComponents::default();
    assert_eq!(components.protocol, "");
    assert_eq!(components.host, None);
    assert_eq!(components.port, None);
    assert_eq!(components.path, None);
    assert_eq!(components.query, None);
    assert_eq!(components.fragment, None);
}

#[test]
fn test_url_components_equality_and_clone() {
    let first: HttpUrlComponents = HttpUrlComponents::parse("http://a.test/x").unwrap_or_default();
    let second: HttpUrlComponents = first.clone();
    assert_eq!(first, second);
    let other: HttpUrlComponents = HttpUrlComponents::parse("http://a.test/y").unwrap_or_default();
    assert_ne!(first, other);
}

#[test]
fn test_url_parse_error_display_message() {
    let message: String = match HttpUrlComponents::parse("not a url") {
        Ok(parsed) => format!("ok:{parsed:?}"),
        Err(error) => format!("{error}"),
    };
    assert_eq!(message, "Invalid URL");
}

#[test]
fn test_request_headers_alias_is_single_valued_map() {
    let mut headers: RequestHeaders = hash_map_xx_hash3_64();
    assert!(headers.is_empty());
    headers.insert("accept".to_string(), "*/*".to_string());
    assert_eq!(headers.len(), 1usize);
    assert_eq!(headers.get("accept").map(String::as_str), Some("*/*"));
}

#[test]
fn test_request_result_alias_carries_request_error() {
    let result: RequestResult = Err(RequestError::Request("boom".to_string()));
    assert!(result.is_err());
    let ok: RequestResult = Ok(HttpResponse::default());
    assert!(ok.is_ok());
}

#[test]
fn test_request_error_display_matches_debug() {
    let error: RequestError = RequestError::Request("boom".to_string());
    let displayed: String = format!("{error}");
    let debugged: String = format!("{error:?}");
    assert!(displayed.contains("Request"));
    assert_eq!(displayed, debugged);
}

#[test]
fn test_request_error_clone_and_equality() {
    let error: RequestError = RequestError::Request("boom".to_string());
    let cloned: RequestError = error.clone();
    assert_eq!(cloned, error);
    assert_ne!(cloned, RequestError::Request("other".to_string()));
}

#[test]
fn test_header_name_constants() {
    assert_eq!(ACCEPT, "accept");
    assert_eq!(HOST, "host");
    assert_eq!(LOCATION, "location");
    assert_eq!(CONTENT_LENGTH, "content-length");
    assert_eq!(CONTENT_TYPE, "content-type");
    assert_eq!(USER_AGENT, "user-agent");
    assert_eq!(ACCEPT_ANY, "*/*");
    assert_eq!(HTTPS_LOWERCASE, "https");
    assert_eq!(QUERY, "?");
}

#[test]
fn test_byte_constants() {
    assert_eq!(SPACE_U8, b' ');
    assert_eq!(TAB_U8, b'\t');
    assert_eq!(COLON_U8, b':');
    assert_eq!(BR_BYTES, b"\n");
    assert_eq!(HTTP_BR_BYTES, b"\r\n");
}
