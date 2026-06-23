use crate::*;

#[test]
fn server_config_from_json() {
    let server_config_json: &'static str = r#"
    {
        "address": "0.0.0.0:90",
        "nodelay": true,
        "ttl": 64,
        "enable_http2": true,
        "enable_http3": true,
        "cert_path": "cert.pem",
        "key_path": "key.pem",
        "http3_bind_address": "0.0.0.0:100"
    }
    "#;
    let server_config: ServerConfig = ServerConfig::from_json(server_config_json).unwrap();
    let mut new_server_config: ServerConfig = ServerConfig::default();
    new_server_config
        .set_address("0.0.0.0:90")
        .set_nodelay(Some(true))
        .set_ttl(Some(64))
        .set_enable_http2(true)
        .set_enable_http3(true)
        .set_cert_path(Some("cert.pem".to_string()))
        .set_key_path(Some("key.pem".to_string()))
        .set_http3_bind_address(Some("0.0.0.0:100".to_string()));
    assert_eq!(server_config, new_server_config);
}

#[test]
fn server_config_http2_http3() {
    let mut config: ServerConfig = ServerConfig::default();
    assert!(config.get_enable_http2());
    assert!(!config.get_enable_http3());
    config.set_enable_http2(false).set_enable_http3(true);
    assert!(!config.get_enable_http2());
    assert!(config.get_enable_http3());
}

#[test]
fn server_config_tls_paths() {
    let mut config: ServerConfig = ServerConfig::default();
    assert!(config.get_cert_path().is_none());
    assert!(config.get_key_path().is_none());
    config
        .set_cert_path(Some("cert.pem".to_string()))
        .set_key_path(Some("key.pem".to_string()));
    assert_eq!(config.get_cert_path().as_deref(), Some("cert.pem"));
    assert_eq!(config.get_key_path().as_deref(), Some("key.pem"));
    config.set_cert_path(None).set_key_path(None);
    assert!(config.get_cert_path().is_none());
    assert!(config.get_key_path().is_none());
}

#[test]
fn server_config_http3_bind_address() {
    let mut config: ServerConfig = ServerConfig::default();
    config.set_address("0.0.0.0:90");
    assert!(config.get_http3_bind_address().is_none());
    assert_eq!(config.get_effective_http3_bind_address(), "0.0.0.0:443");
    config.set_http3_bind_address(Some("0.0.0.0:100".to_string()));
    assert_eq!(config.get_effective_http3_bind_address(), "0.0.0.0:100");
    config.set_http3_bind_address(None);
    assert_eq!(config.get_effective_http3_bind_address(), "0.0.0.0:443");
}
