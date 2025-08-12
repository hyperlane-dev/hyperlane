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
    let new_config: ServerConfig = ServerConfig::new();
    new_config.host("0.0.0.0").await;
    new_config.port(80).await;
    new_config.ws_buffer(4096).await;
    new_config.http_buffer(4096).await;
    new_config.enable_nodelay().await;
    new_config
        .enable_linger(std::time::Duration::from_secs(64))
        .await;
    new_config.enable_ttl(64).await;
    assert_eq!(config, new_config);
}
