use super::*;

#[tokio::test]
async fn http1_default_config_serves_over_plaintext_tcp() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http1, None)
            .await
            .expect("the default plaintext server must start");
    let raw: Vec<u8> = raw_http1_exchange(port, &http1_close_request("GET", VERSION_ROUTE_PATH))
        .await
        .expect("the plaintext exchange must complete");
    let head: String = response_head(&raw);
    assert!(
        head.starts_with("HTTP/1.1 200"),
        "the default protocol must answer over HTTP/1.1, got: {head}"
    );
    control.shutdown().await;
    control.wait().await;
}

#[tokio::test]
async fn http3_without_tls_is_rejected_before_binding() {
    let address: String = format!("{LOOPBACK_HOST}:{}", reserve_tcp_port());
    let mut config: ServerConfig = ServerConfig::default();
    config
        .set_address(address.clone())
        .set_protocol(ServerProtocol::Http3);
    let mut server: Server = Server::default();
    server.server_config(config);
    let result: Result<ServerControlHook, Box<ServerError>> = server.run().await;
    let Err(error) = result else {
        panic!("HTTP/3 without TLS must fail before the socket is bound");
    };
    assert!(
        matches!(*error, ServerError::MissingTlsForHttp3),
        "the rejection must name the missing TLS material, got: {error:?}"
    );
    let rebound: io::Result<TcpListener> = TcpListener::bind(&address);
    assert!(rebound.is_ok(), "the rejected run must leave the port free");
}

#[tokio::test]
async fn shutdown_releases_the_port_before_wait_returns() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http1, None)
            .await
            .expect("a plaintext server must start");
    let raw: Vec<u8> = raw_http1_exchange(port, &http1_close_request("GET", VERSION_ROUTE_PATH))
        .await
        .expect("the exchange before shutdown must complete");
    assert!(
        response_head(&raw).starts_with("HTTP/1.1 200"),
        "the server must serve before it is shut down"
    );
    assert_wait_pending_then_complete(&control).await;
    let rebound: io::Result<TcpListener> = TcpListener::bind(format!("{LOOPBACK_HOST}:{port}"));
    assert!(
        rebound.is_ok(),
        "the port must be released by the time wait returns"
    );
}

#[tokio::test]
async fn dropping_the_caller_server_keeps_the_listener_serving() {
    let (control, port): (ServerControlHook, u16) = start_dropped_caller_server()
        .await
        .expect("a server whose caller is dropped must still serve");
    let raw: Vec<u8> = raw_http1_exchange(port, &http1_close_request("GET", VERSION_ROUTE_PATH))
        .await
        .expect("the exchange must complete after the caller was dropped");
    let head: String = response_head(&raw);
    assert!(
        head.starts_with("HTTP/1.1 200"),
        "the accepted tasks must own the server, got: {head}"
    );
    control.shutdown().await;
    control.wait().await;
}

#[tokio::test]
async fn unreadable_tls_material_is_rejected_before_binding() {
    let address: String = format!("{LOOPBACK_HOST}:{}", reserve_tcp_port());
    let mut config: ServerConfig = ServerConfig::default();
    config
        .set_address(address.clone())
        .set_protocol(ServerProtocol::Http1)
        .set_tls(Some(TlsConfig::new(
            MISSING_PEM_NAME.to_string(),
            MISSING_PEM_NAME.to_string(),
        )));
    let mut server: Server = Server::default();
    server.server_config(config);
    let result: Result<ServerControlHook, Box<ServerError>> = server.run().await;
    let Err(error) = result else {
        panic!("unreadable TLS material must fail before the socket is bound");
    };
    assert!(
        matches!(*error, ServerError::Tls(_)),
        "the failure must be reported as a TLS error, got: {error:?}"
    );
    let rebound: io::Result<TcpListener> = TcpListener::bind(&address);
    assert!(rebound.is_ok(), "the rejected run must leave the port free");
}
