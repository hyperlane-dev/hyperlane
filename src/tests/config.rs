use crate::*;

#[tokio::test]
async fn config_from_str() {
    let config_str: &'static str = r#"
        {
            "host": "0.0.0.0",
            "port": 80,           
            "request_config": {
                "buffer_size": 8192,
                "max_request_line_length": 8192,
                "max_path_length": 8192,
                "max_query_length": 8192,
                "max_header_line_length": 8192,
                "max_header_count": 100,
                "max_header_key_length": 8192,
                "max_header_value_length": 8192,
                "max_body_size": 2097152,
                "max_ws_frame_size": 65536,
                "max_ws_frames": 6000,
                "http_read_timeout_ms": 6000,
                "ws_read_timeout_ms": 1800000
            },
            "nodelay": true,            
            "ttl": 64
        }
    "#;
    let config: ServerConfig = ServerConfig::from_json_str(config_str).unwrap();
    let new_config: ServerConfig = ServerConfig::new();
    new_config
        .host("0.0.0.0")
        .port(80)
        .request_config(RequestConfig::default())
        .enable_nodelay()
        .ttl(64);
    assert_eq!(config, new_config);
}
