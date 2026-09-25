use super::*;

#[test]
fn test_http_url_components_parse_http() {
    let result: Result<HttpUrlComponents, HttpUrlError> =
        HttpUrlComponents::parse("http://example.com/path?query=1#frag");
    assert!(result.is_ok());
    let components: HttpUrlComponents = result.unwrap();
    assert_eq!(components.protocol, "http");
    assert_eq!(components.host, Some("example.com".to_string()));
    assert_eq!(components.port, None);
    assert_eq!(components.path, Some("/path".to_string()));
    assert_eq!(components.query, Some("query=1".to_string()));
    assert_eq!(components.fragment, Some("frag".to_string()));
}

#[test]
fn test_http_url_components_parse_https_with_port() {
    let result: Result<HttpUrlComponents, HttpUrlError> =
        HttpUrlComponents::parse("https://example.com:8443/");
    assert!(result.is_ok());
    let components: HttpUrlComponents = result.unwrap();
    assert_eq!(components.protocol, "https");
    assert_eq!(components.host, Some("example.com".to_string()));
    assert_eq!(components.port, Some(8443));
    assert_eq!(components.path, Some("/".to_string()));
}

#[test]
fn test_http_url_components_default() {
    let components: HttpUrlComponents = HttpUrlComponents::default();
    assert_eq!(components.protocol, "");
    assert_eq!(components.host, None);
    assert_eq!(components.port, None);
    assert_eq!(components.path, None);
    assert_eq!(components.query, None);
    assert_eq!(components.fragment, None);
}

#[test]
fn test_http_url_components_clone() {
    let components: HttpUrlComponents = HttpUrlComponents {
        protocol: "https".to_string(),
        host: Some("example.com".to_string()),
        ..Default::default()
    };
    let cloned: HttpUrlComponents = components.clone();
    assert_eq!(cloned.protocol, "https");
    assert_eq!(cloned.host, Some("example.com".to_string()));
}

#[test]
fn test_http_url_components_eq() {
    let left: HttpUrlComponents = HttpUrlComponents {
        protocol: "http".to_string(),
        ..Default::default()
    };
    let right: HttpUrlComponents = HttpUrlComponents {
        protocol: "http".to_string(),
        ..Default::default()
    };
    assert_eq!(left, right);
}

#[test]
fn test_http_url_error_display_invalid_url() {
    let error: HttpUrlError = HttpUrlError::InvalidUrl;
    assert_eq!(format!("{error}"), "Invalid URL");
}

#[test]
fn test_http_url_error_display_unknown() {
    let error: HttpUrlError = HttpUrlError::Unknown;
    assert_eq!(format!("{error}"), "Unknown error");
}

#[test]
fn test_http_url_error_default() {
    let error: HttpUrlError = HttpUrlError::default();
    assert_eq!(error, HttpUrlError::Unknown);
}

#[test]
fn test_http_url_error_from_parse_error() {
    let parse_error: ParseError = "not a url".parse::<Url>().unwrap_err();
    let error: HttpUrlError = HttpUrlError::from(parse_error);
    assert_eq!(error, HttpUrlError::InvalidUrl);
}
