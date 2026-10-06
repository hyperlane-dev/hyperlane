use super::*;

#[test]
fn test_proxy_http_sets_http_variant() {
    let proxy: Proxy = Proxy::http("127.0.0.1", 7890u16);
    assert_eq!(proxy.proxy_type, ProxyType::Http);
    assert_eq!(proxy.host, "127.0.0.1");
    assert_eq!(proxy.port, 7890u16);
    assert!(proxy.username.is_none());
    assert!(proxy.password.is_none());
}

#[test]
fn test_proxy_https_sets_https_variant() {
    let proxy: Proxy = Proxy::https("proxy.example.com", 443u16);
    assert_eq!(proxy.proxy_type, ProxyType::Https);
    assert_eq!(proxy.host, "proxy.example.com");
    assert_eq!(proxy.port, 443u16);
    assert!(proxy.username.is_none());
    assert!(proxy.password.is_none());
}

#[test]
fn test_proxy_socks5_sets_socks5_variant() {
    let proxy: Proxy = Proxy::socks5("127.0.0.1", 1080u16);
    assert_eq!(proxy.proxy_type, ProxyType::Socks5);
    assert_eq!(proxy.host, "127.0.0.1");
    assert_eq!(proxy.port, 1080u16);
    assert!(proxy.username.is_none());
    assert!(proxy.password.is_none());
}

#[test]
fn test_proxy_accepts_owned_string_host() {
    let proxy: Proxy = Proxy::http(String::from("owned.host"), 1u16);
    assert_eq!(proxy.host, "owned.host");
}

#[test]
fn test_proxy_auth_sets_username_and_password() {
    let proxy: Proxy = Proxy::socks5("127.0.0.1", 1080u16).auth("user", "pass");
    assert_eq!(proxy.username, Some("user".to_string()));
    assert_eq!(proxy.password, Some("pass".to_string()));
    assert_eq!(proxy.proxy_type, ProxyType::Socks5);
}

#[test]
fn test_proxy_auth_overwrites_previous_credentials() {
    let proxy: Proxy = Proxy::http("h", 1u16)
        .auth("first", "one")
        .auth("second", "two");
    assert_eq!(proxy.username, Some("second".to_string()));
    assert_eq!(proxy.password, Some("two".to_string()));
}

#[test]
fn test_proxy_type_is_copy_and_eq() {
    let proxy: Proxy = Proxy::https("h", 2u16);
    let copied: ProxyType = proxy.proxy_type;
    assert_eq!(copied, ProxyType::Https);
    assert_eq!(proxy.proxy_type, copied);
}

#[test]
fn test_proxy_type_variants_are_distinct() {
    assert_ne!(ProxyType::Http, ProxyType::Https);
    assert_ne!(ProxyType::Https, ProxyType::Socks5);
    assert_ne!(ProxyType::Http, ProxyType::Socks5);
}

#[test]
fn test_proxy_equality_compares_every_field() {
    assert_eq!(Proxy::http("h", 1u16), Proxy::http("h", 1u16));
    assert_ne!(Proxy::http("h", 1u16), Proxy::https("h", 1u16));
    assert_ne!(Proxy::http("h", 1u16), Proxy::http("h2", 1u16));
    assert_ne!(Proxy::http("h", 1u16), Proxy::http("h", 2u16));
    assert_ne!(
        Proxy::http("h", 1u16),
        Proxy::http("h", 1u16).auth("u", "p")
    );
}

#[test]
fn test_proxy_clone_preserves_credentials() {
    let proxy: Proxy = Proxy::socks5("h", 5u16).auth("u", "p");
    let cloned: Proxy = proxy.clone();
    assert_eq!(cloned, proxy);
    assert_eq!(cloned.username, Some("u".to_string()));
    assert_eq!(cloned.password, Some("p".to_string()));
}
