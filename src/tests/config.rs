use crate::*;

#[tokio::test]
async fn config_from_str() {
    let config_str: &'static str = r#"
        {
            "host": "0.0.0.0",
            "port": 80,
            "ws_buffer": 4096,
            "http_buffer": 4096,
            "nodelay": true,
            "linger": { "secs": 64, "nanos": 0 },
            "ttl": 64
        }
    "#;
    let config: ServerConfig = ServerConfig::from_str(config_str).unwrap();
    assert_eq!(config.get_host(), "0.0.0.0");
    assert_eq!(*config.get_port(), 80);
    assert_eq!(*config.get_ws_buffer(), 4096);
    assert_eq!(*config.get_http_buffer(), 4096);
    assert_eq!(*config.get_nodelay(), Some(true));
    assert_eq!(
        *config.get_linger(),
        Some(std::time::Duration::from_secs(64))
    );
    assert_eq!(*config.get_ttl(), Some(64));
}
