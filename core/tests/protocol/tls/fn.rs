use super::*;

#[test]
fn tls_config_new_keeps_the_paths_it_was_given() {
    let chain: String = fixture_path(LEAF_CHAIN_PEM_NAME)
        .to_string_lossy()
        .to_string();
    let key: String = fixture_path(LEAF_KEY_NAME).to_string_lossy().to_string();
    let config: TlsConfig = TlsConfig::new(chain.clone(), key.clone());
    assert_eq!(config.get_certificate_path(), &chain);
    assert_eq!(config.get_private_key_path(), &key);
}

#[test]
fn server_protocol_defaults_to_http1() {
    let protocol: ServerProtocol = ServerProtocol::default();
    assert_eq!(protocol, ServerProtocol::Http1);
}

fn protocol_from_wire_name(name: &str) -> Option<ServerProtocol> {
    let json: String = format!("{{\"address\":\"127.0.0.1:8080\",\"protocol\":\"{name}\"}}");
    match ServerConfig::from_json(json) {
        Ok(config) => Some(config.get_protocol()),
        Err(_) => None,
    }
}

#[test]
fn server_protocol_wire_names_are_http1_http2_and_http3() {
    assert_eq!(
        protocol_from_wire_name(WIRE_HTTP1),
        Some(ServerProtocol::Http1)
    );
    assert_eq!(
        protocol_from_wire_name(WIRE_HTTP2),
        Some(ServerProtocol::Http2)
    );
    assert_eq!(
        protocol_from_wire_name(WIRE_HTTP3),
        Some(ServerProtocol::Http3)
    );
}

#[test]
fn an_omitted_protocol_field_loads_as_http1() {
    let config: ServerConfig =
        ServerConfig::from_json("{\"address\":\"127.0.0.1:8080\"}").expect("legacy json must load");
    assert_eq!(config.get_protocol(), ServerProtocol::Http1);
}

#[test]
fn a_legacy_config_without_protocol_or_tls_still_loads_as_plaintext_http1() {
    let legacy: &str = "{\"address\":\"127.0.0.1:8080\"}";
    let config: ServerConfig = ServerConfig::from_json(legacy).expect("legacy json must load");
    assert_eq!(config.get_address(), "127.0.0.1:8080");
    assert_eq!(
        config.get_protocol(),
        ServerProtocol::Http1,
        "an omitted protocol must resolve to the historical default"
    );
    assert!(
        config.get_tls().is_none(),
        "a legacy config must carry no tls material"
    );
    assert_eq!(
        config.get_alpn_protocol(),
        ALPN_HTTP1,
        "a legacy config must advertise the http/1.1 alpn"
    );
}

#[test]
fn the_three_argument_constructor_keeps_speaking_http1_without_tls() {
    let legacy: String = SERVER_CONFIG_LEGACY_ADDRESS.to_string();
    let config: ServerConfig = ServerConfig::new(legacy, None, None);
    assert_eq!(*config.get_address(), SERVER_CONFIG_LEGACY_ADDRESS);
    assert_eq!(config.get_protocol(), ServerProtocol::Http1);
    assert!(config.get_tls().is_none());
}

#[test]
fn a_config_built_from_json_and_rebuilt_through_the_constructor_agree() {
    let from_json: ServerConfig =
        ServerConfig::from_json("{\"address\":\"127.0.0.1:9090\",\"protocol\":\"http2\"}")
            .expect("json config must load");
    let other: String = "127.0.0.1:9090".to_string();
    let built: ServerConfig = ServerConfig::new(other, None, None);
    assert_eq!(from_json.get_protocol(), ServerProtocol::Http2);
    assert_eq!(
        built.get_protocol(),
        ServerProtocol::Http1,
        "the old constructor cannot select a protocol, so it stays on http1"
    );
    assert_eq!(from_json.get_address(), built.get_address());
}

#[test]
fn an_unknown_protocol_wire_name_is_rejected() {
    assert_eq!(
        protocol_from_wire_name(WIRE_UNKNOWN),
        None,
        "an unknown protocol name must not silently fall back to http1"
    );
}

#[test]
fn a_protocol_and_tls_config_round_trips_through_json() {
    let mut config: ServerConfig = ServerConfig::default();
    config
        .set_address("127.0.0.1:8443")
        .set_protocol(ServerProtocol::Http2)
        .set_tls(Some(TlsConfig::new(
            fixture_path(LEAF_CHAIN_PEM_NAME)
                .to_string_lossy()
                .to_string(),
            fixture_path(LEAF_KEY_NAME).to_string_lossy().to_string(),
        )));
    let encoded: String = format!(
        "{{\"address\":\"127.0.0.1:8443\",\"protocol\":\"{WIRE_HTTP2}\",\"tls\":{{\"certificate_path\":\"{}\",\"private_key_path\":\"{}\"}}}}",
        fixture_path(LEAF_CHAIN_PEM_NAME).to_string_lossy(),
        fixture_path(LEAF_KEY_NAME).to_string_lossy()
    );
    let decoded: ServerConfig = ServerConfig::from_json(encoded).expect("config must decode");
    assert_eq!(decoded.get_protocol(), ServerProtocol::Http2);
    let decoded_tls: Option<TlsConfig> = decoded.get_tls().clone();
    match decoded_tls {
        Some(value) => {
            assert_eq!(
                *value.get_certificate_path(),
                fixture_path(LEAF_CHAIN_PEM_NAME)
                    .to_string_lossy()
                    .to_string()
            );
        }
        None => panic!("the tls block must survive the round trip"),
    }
}

#[test]
fn clearing_tls_leaves_the_protocol_choice_untouched() {
    let mut config: ServerConfig = ServerConfig::default();
    config
        .set_protocol(ServerProtocol::Http3)
        .set_tls(Some(TlsConfig::new("a".to_string(), "b".to_string())))
        .set_tls(None);
    assert_eq!(config.get_protocol(), ServerProtocol::Http3);
    assert!(config.get_tls().is_none());
}

#[tokio::test]
async fn a_truncated_certificate_is_rejected_before_the_listener_binds() {
    let port: u16 = reserve_tcp_port();
    let mut config: ServerConfig = ServerConfig::default();
    config
        .set_address(format!("{LOOPBACK_HOST}:{port}"))
        .set_protocol(ServerProtocol::Http2)
        .set_tls(Some(TlsConfig::new(
            fixture_path(TRUNCATED_CERTIFICATE_PEM_NAME)
                .to_string_lossy()
                .to_string(),
            fixture_path(LEAF_KEY_NAME).to_string_lossy().to_string(),
        )));
    let mut server: Server = Server::default();
    server.server_config(config);
    register_probe_hooks(&mut server);
    let outcome: Result<ServerControlHook, Box<ServerError>> = server.run().await;
    assert!(
        outcome.is_err(),
        "a truncated certificate must fail configuration, not start a listener"
    );
    let rebound: io::Result<TcpListener> = TcpListener::bind(("127.0.0.1", port));
    assert!(
        rebound.is_ok(),
        "a rejected certificate must leave port {port} unbound"
    );
}

#[tokio::test]
async fn a_malformed_certificate_is_rejected_before_the_listener_binds() {
    let port: u16 = reserve_tcp_port();
    let mut config: ServerConfig = ServerConfig::default();
    config
        .set_address(format!("{LOOPBACK_HOST}:{port}"))
        .set_protocol(ServerProtocol::Http2)
        .set_tls(Some(TlsConfig::new(
            fixture_path(MALFORMED_CERTIFICATE_PEM_NAME)
                .to_string_lossy()
                .to_string(),
            fixture_path(LEAF_KEY_NAME).to_string_lossy().to_string(),
        )));
    let mut server: Server = Server::default();
    server.server_config(config);
    register_probe_hooks(&mut server);
    let outcome: Result<ServerControlHook, Box<ServerError>> = server.run().await;
    assert!(
        outcome.is_err(),
        "a malformed certificate must fail configuration, not start a listener"
    );
    let rebound: io::Result<TcpListener> = TcpListener::bind(("127.0.0.1", port));
    assert!(
        rebound.is_ok(),
        "a rejected certificate must leave port {port} unbound"
    );
}

#[tokio::test]
async fn an_invalid_private_key_is_rejected_before_the_listener_binds() {
    let port: u16 = reserve_tcp_port();
    let mut config: ServerConfig = ServerConfig::default();
    config
        .set_address(format!("{LOOPBACK_HOST}:{port}"))
        .set_protocol(ServerProtocol::Http2)
        .set_tls(Some(TlsConfig::new(
            fixture_path(LEAF_CHAIN_PEM_NAME)
                .to_string_lossy()
                .to_string(),
            fixture_path(INVALID_PRIVATE_KEY_PEM_NAME)
                .to_string_lossy()
                .to_string(),
        )));
    let mut server: Server = Server::default();
    server.server_config(config);
    register_probe_hooks(&mut server);
    let outcome: Result<ServerControlHook, Box<ServerError>> = server.run().await;
    assert!(
        outcome.is_err(),
        "an invalid private key must fail configuration, not start a listener"
    );
    let rebound: io::Result<TcpListener> = TcpListener::bind(("127.0.0.1", port));
    assert!(
        rebound.is_ok(),
        "a rejected key must leave port {port} unbound"
    );
}

#[tokio::test]
async fn a_certificate_and_key_from_different_chains_are_rejected() {
    let port: u16 = reserve_tcp_port();
    let mut config: ServerConfig = ServerConfig::default();
    config
        .set_address(format!("{LOOPBACK_HOST}:{port}"))
        .set_protocol(ServerProtocol::Http2)
        .set_tls(Some(TlsConfig::new(
            fixture_path(LEAF_CHAIN_PEM_NAME)
                .to_string_lossy()
                .to_string(),
            fixture_path(MISMATCHED_LEAF_KEY_NAME)
                .to_string_lossy()
                .to_string(),
        )));
    let mut server: Server = Server::default();
    server.server_config(config);
    register_probe_hooks(&mut server);
    let outcome: Result<ServerControlHook, Box<ServerError>> = server.run().await;
    assert!(
        outcome.is_err(),
        "a certificate and key from different chains must not pair up"
    );
    let rebound: io::Result<TcpListener> = TcpListener::bind(("127.0.0.1", port));
    assert!(
        rebound.is_ok(),
        "a rejected key pair must leave port {port} unbound"
    );
}

#[tokio::test]
async fn a_missing_certificate_file_is_rejected_before_the_listener_binds() {
    let port: u16 = reserve_tcp_port();
    let mut config: ServerConfig = ServerConfig::default();
    config
        .set_address(format!("{LOOPBACK_HOST}:{port}"))
        .set_protocol(ServerProtocol::Http2)
        .set_tls(Some(TlsConfig::new(
            fixture_path(MISSING_PEM_NAME).to_string_lossy().to_string(),
            fixture_path(LEAF_KEY_NAME).to_string_lossy().to_string(),
        )));
    let mut server: Server = Server::default();
    server.server_config(config);
    register_probe_hooks(&mut server);
    let outcome: Result<ServerControlHook, Box<ServerError>> = server.run().await;
    assert!(
        outcome.is_err(),
        "a missing certificate file must fail configuration"
    );
}

#[tokio::test]
async fn http3_without_tls_is_refused_and_never_binds_the_udp_port() {
    let port: u16 = reserve_udp_port();
    let mut config: ServerConfig = ServerConfig::default();
    config
        .set_address(format!("{LOOPBACK_HOST}:{port}"))
        .set_protocol(ServerProtocol::Http3)
        .set_tls(None);
    let mut server: Server = Server::default();
    server.server_config(config);
    register_probe_hooks(&mut server);
    let outcome: Result<ServerControlHook, Box<ServerError>> = server.run().await;
    assert!(
        outcome.is_err(),
        "http3 over quic without tls must be refused, tls is mandatory"
    );
    let rebound: io::Result<UdpSocket> = UdpSocket::bind(format!("{LOOPBACK_HOST}:{port}"));
    assert!(
        rebound.is_ok(),
        "a refused http3 start must leave udp port {port} unbound"
    );
}

#[tokio::test]
async fn an_http2_tls_client_that_offers_no_alpn_is_not_served() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http2, Some(trusted_tls_config()))
            .await
            .expect("http2 tls server must start");
    let mut stream: ClientTlsStream = connect_tls_without_alpn(port)
        .await
        .expect("the handshake itself must still complete");
    let mut observed: Vec<u8> = Vec::new();
    let mut chunk: [u8; 512] = [0; 512];
    if let Ok(Ok(read)) = timeout(
        Duration::from_millis(SHUTDOWN_PROBE_MILLIS),
        stream.read(&mut chunk),
    )
    .await
    {
        observed.extend_from_slice(&chunk[..read]);
    }
    assert!(
        observed.is_empty(),
        "a client that negotiated no protocol must not receive an http2 response, got: {:?}",
        String::from_utf8_lossy(&observed)
    );
    control.shutdown().await;
    control.wait().await;
}

#[tokio::test]
async fn an_http1_tls_client_that_offers_no_alpn_is_still_served() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http1, Some(trusted_tls_config()))
            .await
            .expect("http1 tls server must start");
    let mut stream: ClientTlsStream = connect_tls_without_alpn(port)
        .await
        .expect("a legacy client without alpn must complete the handshake");
    let request: String = format!(
        "GET {VERSION_ROUTE_PATH} HTTP/1.1\r\nHost: {TLS_SERVER_NAME}\r\nConnection: close\r\n\r\n"
    );
    let written: io::Result<()> = write_bounded(&mut stream, request.as_bytes()).await;
    assert!(written.is_ok(), "the request must reach the server");
    let mut collected: Vec<u8> = Vec::new();
    let mut chunk: [u8; 512] = [0; 512];
    if let Ok(Ok(read)) = timeout(
        Duration::from_millis(SHUTDOWN_PROBE_MILLIS),
        stream.read(&mut chunk),
    )
    .await
    {
        collected.extend_from_slice(&chunk[..read]);
    }
    let (head, body): (String, Vec<u8>) = response_head_and_body(&collected);
    assert!(
        head.starts_with("HTTP/1.1 200"),
        "a no-alpn client must still be served over http1, got: {head}"
    );
    assert_eq!(
        String::from_utf8_lossy(&body),
        format!("{:?}", HttpVersion::Http1_1)
    );
    control.shutdown().await;
    control.wait().await;
}

#[tokio::test]
async fn a_client_that_does_not_trust_the_server_ca_is_refused() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http2, Some(trusted_tls_config()))
            .await
            .expect("http2 tls server must start");
    let outcome: Result<(), String> = connect_tls_untrusted(port).await;
    assert!(
        outcome.is_err(),
        "a client without the fixture ca in its trust store must be refused"
    );
    let (sender, connection) = h2_plaintext_session(port)
        .await
        .expect("the server must keep serving a properly trusting client");
    drop(sender);
    drop(connection);
    control.shutdown().await;
}

#[tokio::test]
async fn an_infinite_read_timeout_tls_close_still_delivers_close_notify() {
    let port: u16 = reserve_tcp_port();
    let address: String = format!("{LOOPBACK_HOST}:{port}");
    let mut server: Server = Server::default();
    server
        .server_config(protocol_config(
            &address,
            ServerProtocol::Http1,
            Some(trusted_tls_config()),
        ))
        .request_config(RequestConfig {
            read_timeout_ms: u64::MAX,
            ..RequestConfig::default()
        });
    register_probe_hooks(&mut server);
    let control: ServerControlHook = server
        .run()
        .await
        .expect("http1 tls server must start with an infinite read timeout");
    drop(server);
    sleep(Duration::from_millis(SERVER_BOOT_MILLIS)).await;
    let stream: ClientTlsStream = connect_tls(port, ALPN_HTTP1)
        .await
        .expect("validating client must complete the handshake");
    let request: Vec<u8> = http1_close_request("GET", VERSION_ROUTE_PATH);
    let exchanged: Vec<u8> = timeout(
        Duration::from_secs(REQUEST_TIMEOUT_SECS),
        tls_exchange(stream, request),
    )
    .await
    .expect("the tls exchange must not outlive the client timeout")
    .expect("the tls exchange must complete without a transport error");
    let (head, body): (String, Vec<u8>) = response_head_and_body(&exchanged);
    assert!(
        head.starts_with("HTTP/1.1 200"),
        "an infinite read timeout must still serve the request, got: {head}"
    );
    assert_eq!(
        String::from_utf8_lossy(&body),
        format!("{:?}", HttpVersion::Http1_1),
        "the body must arrive complete before the session is closed"
    );
    control.shutdown().await;
}
