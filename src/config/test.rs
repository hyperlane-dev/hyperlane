use crate::*;

#[tokio::test]
async fn server_config_from_json() {
    let config_str: &'static str = r#"
        {
            "host": "0.0.0.0",
            "port": 80,
            "nodelay": true,
            "ttl": 64
        }
    "#;
    let config: ServerConfig = ServerConfig::from_json(config_str).unwrap();
    let new_config: ServerConfig = ServerConfig::new().await;
    new_config
        .host("0.0.0.0")
        .await
        .port(80)
        .await
        .enable_nodelay()
        .await
        .ttl(64)
        .await;
    assert_eq!(config, new_config);
}
