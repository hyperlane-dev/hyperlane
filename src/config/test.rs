use crate::*;

#[tokio::test]
async fn server_config_from_json() {
    let server_config_json: &'static str = r#"
    {
        "host": "0.0.0.0",
        "port": 80,
        "nodelay": true,
        "ttl": 64
    }
    "#;
    let server_config: ServerConfig = ServerConfig::from_json(server_config_json).unwrap();
    let new_server_config: ServerConfig = ServerConfig::new().await;
    new_server_config
        .host("0.0.0.0")
        .await
        .port(80)
        .await
        .enable_nodelay()
        .await
        .ttl(64)
        .await;
    assert_eq!(server_config, new_server_config);
}
