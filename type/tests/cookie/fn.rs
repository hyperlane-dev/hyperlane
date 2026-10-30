use super::*;

#[test]
fn test_cookie_builder_new() {
    let cookie: CookieBuilder = CookieBuilder::new("session_id", "abc123");
    assert_eq!(cookie.get_name(), "session_id");
    assert_eq!(cookie.get_value(), "abc123");
    assert_eq!(*cookie.try_get_expires(), None);
    assert_eq!(cookie.try_get_max_age(), None);
    assert_eq!(*cookie.try_get_domain(), None);
    assert_eq!(*cookie.try_get_path(), None);
    assert!(cookie.try_get_secure().is_none());
    assert!(cookie.try_get_http_only().is_none());
    assert_eq!(*cookie.try_get_same_site(), None);
}

#[test]
fn test_cookie_builder_default() {
    let cookie: CookieBuilder = CookieBuilder::default();
    assert_eq!(cookie.get_name(), "");
    assert_eq!(cookie.get_value(), "");
    assert_eq!(*cookie.try_get_expires(), None);
    assert_eq!(cookie.try_get_max_age(), None);
    assert_eq!(*cookie.try_get_domain(), None);
    assert_eq!(*cookie.try_get_path(), None);
    assert!(cookie.try_get_secure().is_none());
    assert!(cookie.try_get_http_only().is_none());
    assert_eq!(*cookie.try_get_same_site(), None);
}

#[test]
fn test_cookie_builder_parse_basic() {
    let cookie: CookieBuilder = CookieBuilder::parse("session_id=abc123");
    assert_eq!(cookie.get_name(), "session_id");
    assert_eq!(cookie.get_value(), "abc123");
    assert_eq!(*cookie.try_get_expires(), None);
    assert_eq!(cookie.try_get_max_age(), None);
    assert_eq!(*cookie.try_get_domain(), None);
    assert_eq!(*cookie.try_get_path(), None);
    assert!(cookie.try_get_secure().is_none());
    assert!(cookie.try_get_http_only().is_none());
    assert_eq!(*cookie.try_get_same_site(), None);
}

#[test]
fn test_cookie_builder_parse_with_expires() {
    let cookie: CookieBuilder =
        CookieBuilder::parse("session_id=abc123; expires=Wed, 21 Oct 2015 07:28:00 GMT");
    assert_eq!(cookie.get_name(), "session_id");
    assert_eq!(cookie.get_value(), "abc123");
    assert_eq!(
        *cookie.try_get_expires(),
        Some("Wed, 21 Oct 2015 07:28:00 GMT".to_string())
    );
    assert_eq!(cookie.try_get_max_age(), None);
    assert_eq!(*cookie.try_get_domain(), None);
    assert_eq!(*cookie.try_get_path(), None);
    assert!(cookie.try_get_secure().is_none());
    assert!(cookie.try_get_http_only().is_none());
    assert_eq!(*cookie.try_get_same_site(), None);
}

#[test]
fn test_cookie_builder_parse_with_max_age() {
    let cookie: CookieBuilder = CookieBuilder::parse("session_id=abc123; max-age=3600");
    assert_eq!(cookie.get_name(), "session_id");
    assert_eq!(cookie.get_value(), "abc123");
    assert_eq!(*cookie.try_get_expires(), None);
    assert_eq!(cookie.try_get_max_age(), Some(3600));
    assert_eq!(*cookie.try_get_domain(), None);
    assert_eq!(*cookie.try_get_path(), None);
    assert!(cookie.try_get_secure().is_none());
    assert!(cookie.try_get_http_only().is_none());
    assert_eq!(*cookie.try_get_same_site(), None);
}

#[test]
fn test_cookie_builder_parse_with_domain() {
    let cookie: CookieBuilder = CookieBuilder::parse("session_id=abc123; domain=example.com");
    assert_eq!(cookie.get_name(), "session_id");
    assert_eq!(cookie.get_value(), "abc123");
    assert_eq!(*cookie.try_get_expires(), None);
    assert_eq!(cookie.try_get_max_age(), None);
    assert_eq!(*cookie.try_get_domain(), Some("example.com".to_string()));
    assert_eq!(*cookie.try_get_path(), None);
    assert!(cookie.try_get_secure().is_none());
    assert!(cookie.try_get_http_only().is_none());
    assert_eq!(*cookie.try_get_same_site(), None);
}

#[test]
fn test_cookie_builder_parse_with_path() {
    let cookie: CookieBuilder = CookieBuilder::parse("session_id=abc123; path=/admin");
    assert_eq!(cookie.get_name(), "session_id");
    assert_eq!(cookie.get_value(), "abc123");
    assert_eq!(*cookie.try_get_expires(), None);
    assert_eq!(cookie.try_get_max_age(), None);
    assert_eq!(*cookie.try_get_domain(), None);
    assert_eq!(*cookie.try_get_path(), Some("/admin".to_string()));
    assert!(cookie.try_get_secure().is_none());
    assert!(cookie.try_get_http_only().is_none());
    assert_eq!(*cookie.try_get_same_site(), None);
}

#[test]
fn test_cookie_builder_parse_with_secure() {
    let cookie: CookieBuilder = CookieBuilder::parse("session_id=abc123; secure");
    assert_eq!(cookie.get_name(), "session_id");
    assert_eq!(cookie.get_value(), "abc123");
    assert_eq!(*cookie.try_get_expires(), None);
    assert_eq!(cookie.try_get_max_age(), None);
    assert_eq!(*cookie.try_get_domain(), None);
    assert_eq!(*cookie.try_get_path(), None);
    assert_eq!(cookie.try_get_secure(), Some(true));
    assert!(cookie.try_get_http_only().is_none());
    assert_eq!(*cookie.try_get_same_site(), None);
}

#[test]
fn test_cookie_builder_parse_with_http_only() {
    let cookie: CookieBuilder = CookieBuilder::parse("session_id=abc123; httponly");
    assert_eq!(cookie.get_name(), "session_id");
    assert_eq!(cookie.get_value(), "abc123");
    assert_eq!(*cookie.try_get_expires(), None);
    assert_eq!(cookie.try_get_max_age(), None);
    assert_eq!(*cookie.try_get_domain(), None);
    assert_eq!(*cookie.try_get_path(), None);
    assert!(cookie.try_get_secure().is_none());
    assert_eq!(cookie.try_get_http_only(), Some(true));
    assert_eq!(*cookie.try_get_same_site(), None);
}

#[test]
fn test_cookie_builder_parse_with_same_site() {
    let cookie: CookieBuilder = CookieBuilder::parse("session_id=abc123; samesite=strict");
    assert_eq!(cookie.get_name(), "session_id");
    assert_eq!(cookie.get_value(), "abc123");
    assert_eq!(*cookie.try_get_expires(), None);
    assert_eq!(cookie.try_get_max_age(), None);
    assert_eq!(*cookie.try_get_domain(), None);
    assert_eq!(*cookie.try_get_path(), None);
    assert!(cookie.try_get_secure().is_none());
    assert!(cookie.try_get_http_only().is_none());
    assert_eq!(*cookie.try_get_same_site(), Some("strict".to_string()));
}

#[test]
fn test_cookie_builder_parse_complex() {
    let cookie: CookieBuilder = CookieBuilder::parse(
        "session_id=abc123; expires=Wed, 21 Oct 2015 07:28:00 GMT; max-age=3600; domain=example.com; path=/admin; secure; httponly; samesite=lax",
    );
    assert_eq!(cookie.get_name(), "session_id");
    assert_eq!(cookie.get_value(), "abc123");
    assert_eq!(
        *cookie.try_get_expires(),
        Some("Wed, 21 Oct 2015 07:28:00 GMT".to_string())
    );
    assert_eq!(cookie.try_get_max_age(), Some(3600));
    assert_eq!(*cookie.try_get_domain(), Some("example.com".to_string()));
    assert_eq!(*cookie.try_get_path(), Some("/admin".to_string()));
    assert_eq!(cookie.try_get_secure(), Some(true));
    assert_eq!(cookie.try_get_http_only(), Some(true));
    assert_eq!(*cookie.try_get_same_site(), Some("lax".to_string()));
}

#[test]
fn test_cookie_builder_parse_empty_string() {
    let cookie: CookieBuilder = CookieBuilder::parse("");
    assert_eq!(cookie.get_name(), "");
    assert_eq!(cookie.get_value(), "");
    assert_eq!(*cookie.try_get_expires(), None);
    assert_eq!(cookie.try_get_max_age(), None);
    assert_eq!(*cookie.try_get_domain(), None);
    assert_eq!(*cookie.try_get_path(), None);
    assert!(cookie.try_get_secure().is_none());
    assert!(cookie.try_get_http_only().is_none());
    assert_eq!(*cookie.try_get_same_site(), None);
}

#[test]
fn test_cookie_builder_parse_case_insensitive() {
    let cookie: CookieBuilder = CookieBuilder::parse(
        "session_id=abc123; DOMAIN=example.com; SECURE; HTTPONLY; SAMESITE=Strict",
    );
    assert_eq!(cookie.get_name(), "session_id");
    assert_eq!(cookie.get_value(), "abc123");
    assert_eq!(*cookie.try_get_expires(), None);
    assert_eq!(cookie.try_get_max_age(), None);
    assert_eq!(*cookie.try_get_domain(), Some("example.com".to_string()));
    assert_eq!(*cookie.try_get_path(), None);
    assert_eq!(cookie.try_get_secure(), Some(true));
    assert_eq!(cookie.try_get_http_only(), Some(true));
    assert_eq!(*cookie.try_get_same_site(), Some("Strict".to_string()));
}

#[test]
fn test_cookie_builder_expires() {
    let mut cookie: CookieBuilder = CookieBuilder::new("test", "value");
    cookie.set_expires("Wed, 21 Oct 2015 07:28:00 GMT");
    assert_eq!(
        *cookie.try_get_expires(),
        Some("Wed, 21 Oct 2015 07:28:00 GMT".to_string())
    );
}

#[test]
fn test_cookie_builder_max_age() {
    let mut cookie: CookieBuilder = CookieBuilder::new("test", "value");
    cookie.set_max_age(3600);
    assert_eq!(cookie.try_get_max_age(), Some(3600));
}

#[test]
fn test_cookie_builder_domain() {
    let mut cookie: CookieBuilder = CookieBuilder::new("test", "value");
    cookie.set_domain("example.com");
    assert_eq!(*cookie.try_get_domain(), Some("example.com".to_string()));
}

#[test]
fn test_cookie_builder_path() {
    let mut cookie: CookieBuilder = CookieBuilder::new("test", "value");
    cookie.set_path("/admin");
    assert_eq!(*cookie.try_get_path(), Some("/admin".to_string()));
}

#[test]
fn test_cookie_builder_secure() {
    let mut cookie: CookieBuilder = CookieBuilder::new("test", "value");
    cookie.secure();
    assert_eq!(cookie.try_get_secure(), Some(true));
}

#[test]
fn test_cookie_builder_http_only() {
    let mut cookie: CookieBuilder = CookieBuilder::new("test", "value");
    cookie.http_only();
    assert_eq!(cookie.try_get_http_only(), Some(true));
}

#[test]
fn test_cookie_builder_same_site() {
    let mut cookie: CookieBuilder = CookieBuilder::new("test", "value");
    cookie.set_same_site("Strict");
    assert_eq!(*cookie.try_get_same_site(), Some("Strict".to_string()));
}

#[test]
fn test_cookie_builder_chaining() {
    let mut cookie: CookieBuilder = CookieBuilder::new("session_id", "abc123");
    cookie
        .set_expires("Wed, 21 Oct 2015 07:28:00 GMT")
        .set_max_age(3600)
        .set_domain("example.com")
        .set_path("/admin")
        .secure()
        .http_only()
        .set_same_site("Strict");
    assert_eq!(cookie.get_name(), "session_id");
    assert_eq!(cookie.get_value(), "abc123");
    assert_eq!(
        *cookie.try_get_expires(),
        Some("Wed, 21 Oct 2015 07:28:00 GMT".to_string())
    );
    assert_eq!(cookie.try_get_max_age(), Some(3600));
    assert_eq!(*cookie.try_get_domain(), Some("example.com".to_string()));
    assert_eq!(*cookie.try_get_path(), Some("/admin".to_string()));
    assert_eq!(cookie.try_get_secure(), Some(true));
    assert_eq!(cookie.try_get_http_only(), Some(true));
    assert_eq!(*cookie.try_get_same_site(), Some("Strict".to_string()));
}

#[test]
fn test_cookie_builder_build_basic() {
    let cookie: CookieBuilder = CookieBuilder::new("session_id", "abc123");
    let result: String = cookie.build();
    assert_eq!(result, "session_id=abc123");
}

#[test]
fn test_cookie_builder_build_empty_name() {
    let cookie: CookieBuilder = CookieBuilder::new("", "abc123");
    let result: String = cookie.build();
    assert_eq!(result, "");
}

#[test]
fn test_cookie_builder_build_with_expires() {
    let mut cookie: CookieBuilder = CookieBuilder::new("session_id", "abc123");
    cookie.set_expires("Wed, 21 Oct 2015 07:28:00 GMT");
    let result: String = cookie.build();
    assert_eq!(
        result,
        "session_id=abc123; expires=Wed, 21 Oct 2015 07:28:00 GMT"
    );
}

#[test]
fn test_cookie_builder_build_with_max_age() {
    let mut cookie: CookieBuilder = CookieBuilder::new("session_id", "abc123");
    cookie.set_max_age(3600);
    let result: String = cookie.build();
    assert_eq!(result, "session_id=abc123; max-age=3600");
}

#[test]
fn test_cookie_builder_build_with_domain() {
    let mut cookie: CookieBuilder = CookieBuilder::new("session_id", "abc123");
    cookie.set_domain("example.com");
    let result: String = cookie.build();
    assert_eq!(result, "session_id=abc123; domain=example.com");
}

#[test]
fn test_cookie_builder_build_with_path() {
    let mut cookie: CookieBuilder = CookieBuilder::new("session_id", "abc123");
    cookie.set_path("/admin");
    let result: String = cookie.build();
    assert_eq!(result, "session_id=abc123; path=/admin");
}

#[test]
fn test_cookie_builder_build_with_secure() {
    let mut cookie: CookieBuilder = CookieBuilder::new("session_id", "abc123");
    cookie.secure();
    let result: String = cookie.build();
    assert_eq!(result, "session_id=abc123; secure");
}

#[test]
fn test_cookie_builder_build_with_http_only() {
    let mut cookie: CookieBuilder = CookieBuilder::new("session_id", "abc123");
    cookie.http_only();
    let result: String = cookie.build();
    assert_eq!(result, "session_id=abc123; httponly");
}

#[test]
fn test_cookie_builder_build_with_same_site() {
    let mut cookie: CookieBuilder = CookieBuilder::new("session_id", "abc123");
    cookie.set_same_site("strict");
    let result: String = cookie.build();
    assert_eq!(result, "session_id=abc123; samesite=strict");
}

#[test]
fn test_cookie_builder_build_all_attributes() {
    let mut cookie: CookieBuilder = CookieBuilder::new("session_id", "abc123");
    cookie
        .set_expires("Wed, 21 Oct 2015 07:28:00 GMT")
        .set_max_age(3600)
        .set_domain("example.com")
        .set_path("/admin")
        .secure()
        .http_only()
        .set_same_site("lax");
    let result: String = cookie.build();
    assert_eq!(
        result,
        "session_id=abc123; expires=Wed, 21 Oct 2015 07:28:00 GMT; max-age=3600; domain=example.com; path=/admin; secure; httponly; samesite=lax"
    );
}

#[test]
fn test_cookie_builder_refuses_a_name_that_is_not_a_token() {
    for name in [
        "session id",
        "session;id",
        "session=id",
        "session\tid",
        "session\r\nid",
    ] {
        let cookie: CookieBuilder = CookieBuilder::new(name, "abc123");
        assert!(
            matches!(cookie.try_build(), Err(CookieError::InvalidName)),
            "the name {name:?} must be refused"
        );
        assert!(
            cookie.build().is_empty(),
            "the name {name:?} must not produce a cookie"
        );
    }
}

#[test]
fn test_cookie_builder_refuses_a_value_that_carries_a_control_byte() {
    for value in [
        "abc\r\nX-Injected: 1",
        "abc\rX",
        "abc\nX",
        "abc\u{0}X",
        "abc\u{7f}X",
    ] {
        let cookie: CookieBuilder = CookieBuilder::new("session_id", value);
        assert!(
            matches!(cookie.try_build(), Err(CookieError::InvalidValue)),
            "the value {value:?} must be refused"
        );
        assert!(
            cookie.build().is_empty(),
            "the value {value:?} must not produce a cookie"
        );
    }
}

#[test]
fn test_cookie_builder_refuses_a_value_that_would_end_the_cookie_pair() {
    for value in ["abc; domain=evil.example", "abc, other=1", "abc\"quoted\""] {
        let cookie: CookieBuilder = CookieBuilder::new("session_id", value);
        assert!(
            matches!(cookie.try_build(), Err(CookieError::InvalidValue)),
            "the value {value:?} must be refused"
        );
    }
}

#[test]
fn test_cookie_builder_refuses_an_attribute_that_carries_a_control_byte() {
    let mut cookie: CookieBuilder = CookieBuilder::new("session_id", "abc123");
    cookie.set_path("/admin\r\nX-Injected: 1");
    assert!(
        matches!(cookie.try_build(), Err(CookieError::InvalidAttribute)),
        "a path that carries a control byte must be refused"
    );
}

#[test]
fn test_cookie_builder_refuses_an_attribute_that_would_start_another_attribute() {
    let mut cookie: CookieBuilder = CookieBuilder::new("session_id", "abc123");
    cookie.set_domain("evil.example; secure");
    assert!(
        matches!(cookie.try_build(), Err(CookieError::InvalidAttribute)),
        "a domain that carries a semicolon must be refused"
    );
}

#[test]
fn test_cookie_builder_builds_a_cookie_whose_parts_are_all_valid() {
    let mut cookie: CookieBuilder = CookieBuilder::new("session_id", "abc123");
    cookie
        .set_expires("Wed, 21 Oct 2015 07:28:00 GMT")
        .set_max_age(3600)
        .set_domain("example.com")
        .set_path("/admin")
        .secure()
        .http_only()
        .set_same_site("lax");
    assert_eq!(
        cookie.try_build(),
        Ok("session_id=abc123; expires=Wed, 21 Oct 2015 07:28:00 GMT; max-age=3600; domain=example.com; path=/admin; secure; httponly; samesite=lax".to_string())
    );
}

#[test]
fn test_cookie_builder_parse_skips_a_name_that_is_not_a_token() {
    let cookie: CookieBuilder = CookieBuilder::parse("session id=abc123");
    assert_eq!(cookie.get_name(), "");
    assert_eq!(cookie.get_value(), "");
}

#[test]
fn test_cookie_builder_parse_skips_a_value_that_carries_a_control_byte() {
    let cookie: CookieBuilder = CookieBuilder::parse("session_id=abc\r\nX-Injected: 1");
    assert_eq!(cookie.get_name(), "");
    assert_eq!(cookie.get_value(), "");
}

#[test]
fn test_cookie_builder_parse_skips_an_attribute_that_carries_a_control_byte() {
    let cookie: CookieBuilder =
        CookieBuilder::parse("session_id=abc123; path=/admin\r\nX-Injected: 1");
    assert_eq!(cookie.get_name(), "session_id");
    assert_eq!(*cookie.try_get_path(), None);
}

#[test]
fn test_cookie_parse_skips_a_pair_that_carries_a_control_byte() {
    let cookies: Cookies = Cookie::parse("session_id=abc123; bad=va\r\nlue");
    assert_eq!(cookies.len(), 1);
    assert_eq!(cookies.get("session_id"), Some(&"abc123".to_string()));
    assert_eq!(cookies.get("bad"), None);
}

#[test]
fn test_cookie_parse_skips_a_pair_whose_name_is_not_a_token() {
    let cookies: Cookies = Cookie::parse("session id=abc123; ok=1");
    assert_eq!(cookies.len(), 1);
    assert_eq!(cookies.get("ok"), Some(&"1".to_string()));
    assert_eq!(cookies.get("session id"), None);
}
