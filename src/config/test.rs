use crate::*;

#[tokio::test]
async fn server_config_from_json() {
    let server_config_json: &'static str = r#"
    {
        "address": "0.0.0.0:80",
        "nodelay": true,
        "ttl": 64
    }
    "#;
    let server_config: ServerConfig = ServerConfig::from_json(server_config_json).unwrap();
    let mut new_server_config: ServerConfig = ServerConfig::default();
    new_server_config
        .set_address("0.0.0.0:80")
        .set_nodelay(Some(true))
        .set_ttl(Some(64));
    assert_eq!(server_config, new_server_config);
}
