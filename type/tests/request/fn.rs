use super::*;

#[test]
fn request_config_from_json() {
    let request_config_json: &'static str = r#"
    {
        "buffer_size": 8192,
        "max_path_size": 8192,
        "max_header_count": 100,
        "max_header_key_size": 8192,
        "max_header_value_size": 8192,
        "max_body_size": 2097152,
        "read_timeout_ms": 6000
    }
    "#;
    let request_config: RequestConfig = RequestConfig::from_json(request_config_json).unwrap();
    let mut new_request_config: RequestConfig = RequestConfig::default();
    new_request_config
        .set_buffer_size(8192)
        .set_max_path_size(8192)
        .set_max_header_count(100)
        .set_max_header_key_size(8192)
        .set_max_header_value_size(8192)
        .set_max_body_size(2097152)
        .set_read_timeout_ms(6000);
    assert_eq!(request_config, new_request_config);
}

#[test]
fn request_config_security_levels() {
    let default_config: RequestConfig = RequestConfig::default();
    let low_config: RequestConfig = RequestConfig::low_security();
    let high_config: RequestConfig = RequestConfig::high_security();
    assert!(low_config.get_max_body_size() > default_config.get_max_body_size());
    assert!(high_config.get_max_body_size() < default_config.get_max_body_size());
    assert!(low_config.get_read_timeout_ms() > default_config.get_read_timeout_ms());
    assert!(high_config.get_read_timeout_ms() < default_config.get_read_timeout_ms());
}

#[test]
fn request_default() {
    let request: Request = Request::default();
    assert!(request.get_method().is_unknown());
    assert!(request.get_host().is_empty());
    assert!(request.get_path().is_empty());
    assert!(request.get_querys().is_empty());
    assert!(request.get_headers().is_empty());
    assert!(request.get_body().is_empty());
}

#[test]
fn request_query_operations() {
    let mut request: Request = Request::default();
    let mut first: VecDeque<String> = VecDeque::new();
    first.push_back("value1".to_string());
    request.get_mut_querys().insert("key1".to_string(), first);
    let mut second: VecDeque<String> = VecDeque::new();
    second.push_back("value2".to_string());
    request.get_mut_querys().insert("key2".to_string(), second);
    assert_eq!(
        request.try_get_query_front("key1"),
        Some("value1".to_string())
    );
    assert_eq!(
        request.try_get_query_front("key2"),
        Some("value2".to_string())
    );
    assert_eq!(request.try_get_query_front("key3"), None);
    assert_eq!(request.get_query_front("key1"), "value1".to_string());
}

#[test]
fn request_query_keeps_every_value_of_a_repeated_key() {
    let mut request: Request = Request::default();
    let mut values: VecDeque<String> = VecDeque::new();
    values.push_back("a".to_string());
    values.push_back("b".to_string());
    values.push_back("c".to_string());
    request.get_mut_querys().insert("tag".to_string(), values);
    let all: RequestQuerysValue = request.get_query("tag");
    assert_eq!(all.len(), 3);
    assert_eq!(request.try_get_query_size("tag"), Some(3));
    assert_eq!(request.try_get_query_front("tag"), Some("a".to_string()));
    assert_eq!(request.try_get_query_back("tag"), Some("c".to_string()));
    assert!(request.has_query("tag"));
    assert!(!request.has_query("missing"));
    assert_eq!(request.get_querys_size(), 1);
}

#[test]
fn request_header_lookup_is_case_insensitive() {
    let mut request: Request = Request::default();
    let mut values: VecDeque<String> = VecDeque::new();
    values.push_back("text/plain".to_string());
    request
        .get_mut_headers()
        .insert("content-type".to_string(), values);
    assert_eq!(
        request.try_get_header("Content-Type"),
        request.try_get_header("content-type")
    );
    assert!(request.has_header("Content-Type"));
    assert_eq!(
        request.try_get_header_front("CONTENT-TYPE"),
        Some("text/plain".to_string())
    );
    assert_eq!(request.try_get_header_size("Content-Type"), Some(1));
}

#[test]
fn request_header_operations() {
    let mut request: Request = Request::default();
    let mut values: VecDeque<String> = VecDeque::new();
    values.push_back("value1".to_string());
    values.push_back("value2".to_string());
    request.headers.insert("content-type".to_string(), values);
    assert!(request.has_header("content-type"));
    assert!(!request.has_header("authorization"));
    assert_eq!(request.get_header_size("content-type"), 2);
    assert_eq!(
        request.try_get_header_front("content-type"),
        Some("value1".to_string())
    );
    assert_eq!(
        request.try_get_header_back("content-type"),
        Some("value2".to_string())
    );
    assert_eq!(request.get_headers_size(), 1);
    assert_eq!(request.get_headers_values_size(), 2);
}

#[test]
fn request_has_header_value() {
    let mut request: Request = Request::default();
    let mut values: VecDeque<String> = VecDeque::new();
    values.push_back("application/json".to_string());
    values.push_back("text/html".to_string());
    request.headers.insert("accept".to_string(), values);
    assert!(request.has_header_value("accept", "application/json"));
    assert!(request.has_header_value("accept", "text/html"));
    assert!(!request.has_header_value("accept", "text/xml"));
    assert!(!request.has_header_value("content-type", "application/json"));
}

#[test]
fn request_body_operations() {
    let request: Request = Request {
        body: b"hello world".to_vec(),
        ..Default::default()
    };
    assert_eq!(request.get_body(), b"hello world");
    assert_eq!(request.get_body_string(), "hello world");
}

#[test]
fn request_body_string_utf8() {
    let body: Vec<u8> = "你好世界".as_bytes().to_vec();
    let request: Request = Request {
        body: body.clone(),
        ..Default::default()
    };
    assert_eq!(request.get_body_string(), "你好世界");
}

#[test]
fn request_body_json() {
    let json: &'static str = r#"{"name":"test","value":123}"#;
    let request: Request = Request {
        body: json.as_bytes().to_vec(),
        ..Default::default()
    };
    #[derive(Debug, Deserialize, PartialEq)]
    struct TestData {
        name: String,
        value: i32,
    }
    let result: TestData = request.try_get_body_json().unwrap();
    assert_eq!(result.name, "test");
    assert_eq!(result.value, 123);
}

#[test]
fn request_method_checks() {
    let request: Request = Request {
        method: Method::Get,
        ..Default::default()
    };
    assert!(request.get_method().is_get());
    assert!(!request.get_method().is_post());
    assert!(!request.get_method().is_put());
    assert!(!request.get_method().is_delete());
    assert!(!request.get_method().is_patch());
    assert!(!request.get_method().is_head());
    assert!(!request.get_method().is_options());
    assert!(!request.get_method().is_connect());
    assert!(!request.get_method().is_trace());
    assert!(!request.get_method().is_unknown());
}

#[test]
fn request_version_checks() {
    let request: Request = Request {
        version: HttpVersion::Http1_1,
        ..Default::default()
    };
    assert!(request.get_version().is_http1_1());
    assert!(request.get_version().is_http1_1_or_higher());
    assert!(request.get_version().is_http());
    assert!(!request.get_version().is_http0_9());
    assert!(!request.get_version().is_http1_0());
    assert!(!request.get_version().is_http2());
    assert!(!request.get_version().is_http3());
    assert!(!request.get_version().is_unknown());
}

#[test]
fn request_upgrade_type_checks() {
    let mut request: Request = Request::default();
    let mut values: VecDeque<String> = VecDeque::new();
    values.push_back("websocket".to_string());
    request.headers.insert("upgrade".to_string(), values);
    assert!(request.is_ws_upgrade_type());
    assert!(!request.is_h2c_upgrade_type());
    assert!(!request.is_tls_upgrade_type());
    assert!(!request.is_unknown_upgrade_type());
}

#[test]
fn request_keep_alive_checks() {
    let mut request: Request = Request {
        version: HttpVersion::Http1_1,
        ..Default::default()
    };
    assert!(request.is_enable_keep_alive());
    assert!(!request.is_disable_keep_alive());
    let mut values: VecDeque<String> = VecDeque::new();
    values.push_back("close".to_string());
    request.headers.insert("connection".to_string(), values);
    assert!(!request.is_enable_keep_alive());
    assert!(request.is_disable_keep_alive());
}

#[test]
fn request_error_http_status() {
    let error: RequestError = RequestError::RequestTooLong(HttpStatus::BadRequest);
    assert_eq!(error.get_http_status(), HttpStatus::BadRequest);
    assert_eq!(error.get_http_status_code(), 400);
    let error: RequestError = RequestError::ReadTimeout(HttpStatus::RequestTimeout);
    assert_eq!(error.get_http_status(), HttpStatus::RequestTimeout);
    assert_eq!(error.get_http_status_code(), 408);
}

#[test]
fn request_error_default() {
    let error: RequestError = RequestError::default();
    assert_eq!(error.get_http_status(), HttpStatus::InternalServerError);
}

#[test]
fn request_error_from_io_error() {
    let io_error: io::Error = io::Error::new(ErrorKind::ConnectionReset, "reset");
    let request_error: RequestError = RequestError::from(io_error);
    assert!(matches!(request_error, RequestError::ClientDisconnected(_)));
    let io_error: io::Error = io::Error::other("other");
    let request_error: RequestError = RequestError::from(io_error);
    assert!(matches!(request_error, RequestError::ReadConnection(_)));
}

#[tokio::test]
async fn request_error_from_elapsed() {
    let elapsed: Elapsed = timeout(Duration::from_millis(0), async {
        sleep(Duration::from_secs(10)).await;
    })
    .await
    .unwrap_err();
    let request_error: RequestError = RequestError::from(elapsed);
    assert!(matches!(request_error, RequestError::ReadTimeout(_)));
}

#[test]
fn request_error_from_parse_int_error() {
    let parse_error: ParseIntError = "abc".parse::<usize>().unwrap_err();
    let request_error: RequestError = RequestError::from(parse_error);
    assert!(matches!(
        request_error,
        RequestError::InvalidContentLength(_)
    ));
}

#[test]
fn request_config_default() {
    let config: RequestConfig = RequestConfig::default();
    assert!(config.get_buffer_size() > 0);
    assert!(config.get_max_body_size() > 0);
    assert!(config.get_read_timeout_ms() > 0);
}

#[test]
fn request_config_security_presets() {
    let low: RequestConfig = RequestConfig::low_security();
    let high: RequestConfig = RequestConfig::high_security();
    let default: RequestConfig = RequestConfig::default();
    assert!(low.get_max_body_size() > default.get_max_body_size());
    assert!(high.get_max_body_size() < default.get_max_body_size());
}

#[test]
fn request_cookie_operations() {
    let mut request: Request = Request::default();
    let mut values: VecDeque<String> = VecDeque::new();
    values.push_back("session_id=abc123; user_id=xyz789".to_string());
    request.headers.insert("cookie".to_string(), values);
    let cookies: Cookies = request.get_cookies();
    assert_eq!(cookies.len(), 2);
    assert_eq!(cookies.get("session_id"), Some(&"abc123".to_string()));
    assert_eq!(cookies.get("user_id"), Some(&"xyz789".to_string()));
}

#[test]
fn request_try_get_cookies() {
    let request: Request = Request::default();
    assert!(request.try_get_cookies().is_none());
    let mut request: Request = Request::default();
    let mut values: VecDeque<String> = VecDeque::new();
    values.push_back("theme=dark".to_string());
    request.headers.insert("cookie".to_string(), values);
    let cookies_opt: Option<Cookies> = request.try_get_cookies();
    assert!(cookies_opt.is_some());
    let cookies: Cookies = cookies_opt.unwrap();
    assert_eq!(cookies.get("theme"), Some(&"dark".to_string()));
}

#[test]
fn request_try_get_cookie() {
    let mut request: Request = Request::default();
    let mut values: VecDeque<String> = VecDeque::new();
    values.push_back("token=secret123; lang=en".to_string());
    request.headers.insert("cookie".to_string(), values);
    assert_eq!(
        request.try_get_cookie("token"),
        Some("secret123".to_string())
    );
    assert_eq!(request.try_get_cookie("lang"), Some("en".to_string()));
    assert_eq!(request.try_get_cookie("not_exist"), None);
}

#[test]
fn request_get_cookie() {
    let mut request: Request = Request::default();
    let mut values: VecDeque<String> = VecDeque::new();
    values.push_back("session=value123".to_string());
    request.headers.insert("cookie".to_string(), values);
    assert_eq!(request.get_cookie("session"), "value123".to_string());
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn request_get_cookie_panic() {
    let request: Request = Request::default();
    let _: CookieValue = request.get_cookie("not_exist");
}

#[test]
#[should_panic(expected = "called `Option::unwrap()` on a `None` value")]
fn request_get_cookies_panic() {
    let request: Request = Request::default();
    let _: Cookies = request.get_cookies();
}

fn request_with_if_none_match(values: &[&str]) -> Request {
    let mut request: Request = Request::default();
    let headers: RequestHeadersValue = values
        .iter()
        .map(|value: &&str| (*value).to_owned())
        .collect();
    request
        .get_mut_headers()
        .insert("if-none-match".to_owned(), headers);
    request
}

#[test]
fn request_if_none_match_requires_a_header_and_nonempty_tag_list() {
    let request: Request = Request::default();
    assert!(!request.matches_if_none_match("\"v1\""));
    for values in [&[][..], &[""][..], &[" \t,"][..], &[",,", "\t"][..]] {
        let request: Request = request_with_if_none_match(values);
        assert!(!request.matches_if_none_match("\"v1\""), "{values:?}");
    }
}

#[test]
fn request_if_none_match_uses_case_sensitive_weak_comparison() {
    for (header, current, expected) in [
        ("\"v1\"", "\"v1\"", true),
        ("\"v1\"", "\"v2\"", false),
        ("\"V1\"", "\"v1\"", false),
        ("W/\"v1\"", "\"v1\"", true),
        ("\"v1\"", "W/\"v1\"", true),
        ("W/\"v1\"", "W/\"v1\"", true),
        ("\"\"", "\"\"", true),
        ("W/\"\"", "\"\"", true),
        ("\"\"", "W/\"\"", true),
        ("\"a,b\"", "W/\"a,b\"", true),
        ("\"a,b\"", "\"a\"", false),
        ("\"a\\b\"", "W/\"a\\b\"", true),
        ("\"é\"", "W/\"é\"", true),
        ("\"*\"", "\"v1\"", false),
        ("\"*\"", "\"*\"", true),
    ] {
        let request: Request = request_with_if_none_match(&[header]);
        assert_eq!(
            request.matches_if_none_match(current),
            expected,
            "{header:?} {current:?}"
        );
    }
}

#[test]
fn request_if_none_match_combines_repeated_fields_and_preserves_quoted_commas() {
    for values in [
        &["\"old\", W/\"v1\", \"next\""][..],
        &["\"old\"", "W/\"v1\"", "\"next\""][..],
        &[" \t\"old\"\t,\t W/\"v1\" \t"][..],
        &[",, \t\"old\",,", "", "\t, W/\"v1\", ,"][..],
        &["\"before,after\", W/\"v1\""][..],
        &["\"old\",", ", \"v1\",", "\t"][..],
    ] {
        let request: Request = request_with_if_none_match(values);
        assert!(request.matches_if_none_match("\"v1\""), "{values:?}");
        assert!(!request.matches_if_none_match("\"missing\""), "{values:?}");
    }
}

#[test]
fn request_if_none_match_wildcard_requires_a_standalone_complete_field() {
    for header in ["*", " \t*\t "] {
        let request: Request = request_with_if_none_match(&[header]);
        assert!(request.matches_if_none_match("\"v1\""));
        assert!(request.matches_if_none_match("W/\"\""));
    }
    for values in [
        &["*, \"v1\""][..],
        &["\"v1\", *"][..],
        &["*", "\"v1\""][..],
        &["\"v1\"", "*"][..],
        &["*", "*"][..],
        &["*,*"][..],
        &["*,"][..],
        &[",*"][..],
        &["*", ""][..],
        &["", "*"][..],
        &["**"][..],
        &["W/*"][..],
    ] {
        let request: Request = request_with_if_none_match(values);
        assert!(!request.matches_if_none_match("\"v1\""), "{values:?}");
    }
}

#[test]
fn request_if_none_match_rejects_malformed_tags_and_bad_matching_suffixes() {
    for header in [
        "v1",
        "w/\"v1\"",
        "W/ \"v1\"",
        "\"v1",
        "\"v1\"junk",
        "\"v1\" \"v2\"",
        "\"v1\", junk",
        "\"v1\", \"unfinished",
        "\"v1\", w/\"v2\"",
        "\"v1\", W/ \"v2\"",
        "\"v1\", \"bad space\"",
        "\"v1\", \"bad\tvalue\"",
        "\"v1\", \"bad\rvalue\"",
        "\"v1\", \"bad\nvalue\"",
        "\"v1\", \"bad\0value\"",
        "\"v1\", \"bad\u{7f}value\"",
        "\"v1\"\r",
        "\n\"v1\"",
        "\"v1\"\u{a0}",
        "\"v1\", \"escaped\\\"quote\"",
    ] {
        let request: Request = request_with_if_none_match(&[header]);
        assert!(!request.matches_if_none_match("\"v1\""), "{header:?}");
    }
    for values in [
        &["\"v1\"", "junk"][..],
        &["\"v1\"", "\"unfinished"][..],
        &["junk", "\"v1\""][..],
        &["\"v", "1\""][..],
    ] {
        let request: Request = request_with_if_none_match(values);
        assert!(!request.matches_if_none_match("\"v1\""), "{values:?}");
    }
}

#[test]
fn request_if_none_match_validates_current_entity_tag_even_with_a_wildcard() {
    for current in [
        "",
        "*",
        "v1",
        "w/\"v1\"",
        "W/ \"v1\"",
        "\"v1",
        "\"v1\"junk",
        "\"v1\", \"v2\"",
        " \"v1\"",
        "\"v1\"\t",
        "\"bad space\"",
        "\"bad\tvalue\"",
        "\"bad\0value\"",
        "\"bad\rvalue\"",
        "\"bad\nvalue\"",
        "\"bad\u{7f}value\"",
    ] {
        for header in ["*", "\"v1\""] {
            let request: Request = request_with_if_none_match(&[header]);
            assert!(
                !request.matches_if_none_match(current),
                "{header:?} {current:?}"
            );
        }
    }
}

#[test]
fn request_if_none_match_validates_every_ascii_opaque_byte() {
    for byte in 0u8..=127 {
        let tag: String = format!("\"{}\"", char::from(byte));
        let expected: bool = byte == 0x21 || (0x23..=0x7e).contains(&byte);
        let request: Request = request_with_if_none_match(&[&tag]);
        assert_eq!(request.matches_if_none_match(&tag), expected, "{byte}");
        let wildcard: Request = request_with_if_none_match(&["*"]);
        assert_eq!(wildcard.matches_if_none_match(&tag), expected, "{byte}");
    }
}

#[test]
fn request_if_none_match_is_explicit_and_independent_of_request_method() {
    for method in [
        Method::Get,
        Method::Head,
        Method::Post,
        Method::Put,
        Method::Delete,
    ] {
        let mut request: Request = request_with_if_none_match(&["\"v1\""]);
        request.set_method(method.clone());
        assert!(request.matches_if_none_match("\"v1\""));
        assert_eq!(request.get_method(), &method);
        assert_eq!(request.get_header_front("if-none-match"), "\"v1\"");
        assert!(request.get_body().is_empty());
    }
}

#[test]
fn request_if_none_match_handles_long_lists_tags_and_invalid_tails() {
    let opaque: String = "x".repeat(1024 * 1024);
    let current: String = format!("\"{opaque}\"");
    let header: String = format!("W/{current}");
    let request: Request = request_with_if_none_match(&[&header]);
    assert!(request.matches_if_none_match(&current));
    let header: String = format!("{}{current},", ", \t\"other\", ".repeat(20_000));
    let request: Request = request_with_if_none_match(&[&header]);
    assert!(request.matches_if_none_match(&current));
    let malformed: String = format!("{header}W/\"unfinished");
    let request: Request = request_with_if_none_match(&[&malformed]);
    assert!(!request.matches_if_none_match(&current));
    let empties: String = ", \t".repeat(100_000);
    let request: Request = request_with_if_none_match(&[&empties]);
    assert!(!request.matches_if_none_match(&current));
}
