use super::*;

#[tokio::test]
async fn http3_over_quic_reports_the_real_local_and_peer_addresses_of_the_connection() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http3, Some(trusted_tls_config()))
            .await
            .expect("http3 quic server must start");
    let (endpoint, connection): (quinn::Endpoint, quinn::Connection) =
        connect_quic(port, ALPN_HTTP3)
            .await
            .expect("a quic client must connect");
    let client_local: SocketAddr = endpoint
        .local_addr()
        .expect("the client endpoint must report its own udp address");
    let (status, _headers, body, _alpn): H3HeadReply = h3_exchange_on(
        connection,
        None,
        "GET",
        &format!("https://{TLS_SERVER_NAME}:{port}{SOCKET_METADATA_ROUTE_PATH}"),
        &[],
    )
    .await
    .expect("h3 socket metadata request must complete");
    assert_eq!(status, 200);
    let text: String = String::from_utf8_lossy(&body).to_string();
    let (reported_local, reported_peer): (String, String) =
        match text.split_once(SOCKET_METADATA_SEPARATOR) {
            Some(pair) => (pair.0.to_string(), pair.1.to_string()),
            None => panic!("the route must report both addresses, got: {text:?}"),
        };
    let expected_local: SocketAddr = format!("{LOOPBACK_HOST}:{port}")
        .parse()
        .expect("the bind address must parse");
    assert_eq!(
        reported_local,
        expected_local.to_string(),
        "an h3 stream must carry the connection's real local address, not a fabricated one"
    );
    assert_eq!(
        reported_peer,
        client_local.to_string(),
        "an h3 stream must carry the connection's real peer address"
    );
    control.shutdown().await;
}

#[tokio::test]
async fn http3_streams_a_body_that_never_announced_a_content_length() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http3, Some(trusted_tls_config()))
            .await
            .expect("http3 quic server must start");
    let (status, headers, body, _alpn): H3HeadReply = h3_request_with_headers(
        port,
        "GET",
        &format!("https://{TLS_SERVER_NAME}:{port}{UNFRAMED_BODY_ROUTE_PATH}"),
        &[],
    )
    .await
    .expect("an unframed h3 body must be accepted");
    assert_eq!(status, 200);
    assert!(
        !headers.contains_key(CONTENT_LENGTH),
        "the head must not announce a length for a body framed by data frames"
    );
    assert_eq!(
        String::from_utf8_lossy(&body),
        UNFRAMED_BODY,
        "a body with no content-length must still reach the client in full"
    );
    control.shutdown().await;
}

#[tokio::test]
async fn http3_over_quic_negotiates_h3_alpn_and_tls13() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http3, Some(trusted_tls_config()))
            .await
            .expect("http3 quic server must start");
    let (status, version, body, alpn): H3Reply = h3_request(
        port,
        "GET",
        &format!("https://{TLS_SERVER_NAME}:{port}{VERSION_ROUTE_PATH}"),
        &[],
    )
    .await
    .expect("h3 request must complete");
    assert_eq!(status, 200);
    assert_eq!(
        version,
        http::Version::HTTP_3,
        "the response must be framed as HTTP/3, not downgraded"
    );
    assert_eq!(
        alpn,
        Some(ALPN_HTTP3.to_vec()),
        "QUIC must negotiate the h3 ALPN identifier"
    );
    assert_eq!(
        String::from_utf8_lossy(&body),
        format!("{:?}", HttpVersion::Http3)
    );
    control.shutdown().await;
}

#[tokio::test]
async fn http3_over_quic_reads_path_query_and_header() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http3, Some(trusted_tls_config()))
            .await
            .expect("http3 quic server must start");
    let target: String = format!(
        "https://{TLS_SERVER_NAME}:{port}{PROBE_PATH}?{}",
        PROBE_QUERY
    );
    let (status, version, _body, _alpn): H3Reply = h3_request(port, "GET", &target, &[])
        .await
        .expect("h3 probe request must complete");
    assert_eq!(status, 200);
    assert_eq!(version, http::Version::HTTP_3);
    control.shutdown().await;
}

#[tokio::test]
async fn http3_over_quic_binary_post_round_trips_hostile_bytes() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http3, Some(trusted_tls_config()))
            .await
            .expect("http3 quic server must start");
    let payload: Vec<u8> = hostile_binary_payload();
    let (status, version, body, _alpn): H3Reply = h3_request(
        port,
        "POST",
        &format!("https://{TLS_SERVER_NAME}:{port}{BINARY_ROUTE_PATH}"),
        &payload,
    )
    .await
    .expect("h3 binary post must complete");
    assert_eq!(status, 200);
    assert_eq!(version, http::Version::HTTP_3);
    assert_eq!(
        body, payload,
        "every hostile byte must survive the h3 body round trip"
    );
    control.shutdown().await;
}

#[tokio::test]
async fn http3_over_quic_delivers_a_response_above_the_flow_control_window() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http3, Some(trusted_tls_config()))
            .await
            .expect("http3 quic server must start");
    let expected: Vec<u8> = large_binary_payload();
    let (status, version, body, _alpn): H3Reply = h3_request(
        port,
        "GET",
        &format!("https://{TLS_SERVER_NAME}:{port}{LARGE_ROUTE_PATH}"),
        &[],
    )
    .await
    .expect("large h3 response must complete");
    assert_eq!(status, 200);
    assert_eq!(version, http::Version::HTTP_3);
    assert_eq!(body.len(), expected.len());
    assert!(
        body == expected,
        "the large response must be byte identical past the flow control window"
    );
    control.shutdown().await;
}

#[tokio::test]
async fn http3_over_quic_shutdown_releases_the_bound_udp_port() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http3, Some(trusted_tls_config()))
            .await
            .expect("http3 quic server must start");
    assert_wait_pending_then_complete(&control).await;
    let rebound: io::Result<UdpSocket> = UdpSocket::bind(format!("{LOOPBACK_HOST}:{port}"));
    assert!(
        rebound.is_ok(),
        "udp port {port} must be free again after shutdown and wait"
    );
}

#[tokio::test]
async fn http3_over_quic_rejects_a_client_offering_only_h2() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http3, Some(trusted_tls_config()))
            .await
            .expect("http3 quic server must start");
    let outcome: Result<(quinn::Endpoint, quinn::Connection), String> =
        connect_quic(port, ALPN_HTTP2).await;
    assert!(
        outcome.is_err(),
        "an http3 listener must refuse an h2-only QUIC client instead of downgrading"
    );
    control.shutdown().await;
}

#[tokio::test]
async fn h3_sse_chunk_reaches_the_client_before_the_handler_finishes() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http3, Some(trusted_tls_config()))
            .await
            .expect("http3 quic server must start");
    let _lease: GateLease = gate_lease(GateSlot::SseHttp3).await;
    let target: String = format!("https://{TLS_SERVER_NAME}:{port}{SSE_ROUTE_PATH_HTTP3}");
    let reading: JoinHandle<(Vec<u8>, bool)> =
        spawn(async move { h3_stream_until(port, &target, SSE_FIRST_CHUNK).await });
    route_gate(GateSlot::SseHttp3)
        .await_first_chunk_sent()
        .await;
    let (collected, matched): (Vec<u8>, bool) =
        timeout(Duration::from_secs(FAST_STREAM_BUDGET_MILLIS), reading)
            .await
            .expect("the first sse chunk must be readable while the handler is still blocked")
            .expect("the reader task must not panic");
    let text: String = String::from_utf8_lossy(&collected).to_string();
    assert!(
        matched,
        "the first sse chunk must be readable while the handler is still blocked, got: {text:?}"
    );
    assert!(
        !text.contains(SSE_SECOND_CHUNK),
        "the second chunk must wait for the handler to be released, got: {text:?}"
    );
    route_gate(GateSlot::SseHttp3).release_sse();
    control.shutdown().await;
}

#[tokio::test]
async fn http3_one_connection_answers_a_fast_stream_while_a_slow_stream_is_blocked() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http3, Some(trusted_tls_config()))
            .await
            .expect("http3 quic server must start");
    let _lease: GateLease = gate_lease(GateSlot::SlowHttp3).await;
    let order: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(Vec::new()));

    let slow_order: Arc<Mutex<Vec<&'static str>>> = Arc::clone(&order);
    let slow: JoinHandle<Result<H3Reply, String>> = spawn(async move {
        let outcome: Result<H3Reply, String> = h3_request(
            port,
            "GET",
            &format!("https://{TLS_SERVER_NAME}:{port}{SLOW_ROUTE_PATH_HTTP3}"),
            &[],
        )
        .await;
        slow_order.lock().expect("order lock").push("slow");
        outcome
    });

    route_gate(GateSlot::SlowHttp3).await_slow_entered().await;

    let fast_order: Arc<Mutex<Vec<&'static str>>> = Arc::clone(&order);
    let fast: JoinHandle<Result<H3Reply, String>> = spawn(async move {
        let outcome: Result<H3Reply, String> = h3_request(
            port,
            "GET",
            &format!("https://{TLS_SERVER_NAME}:{port}{VERSION_ROUTE_PATH}"),
            &[],
        )
        .await;
        fast_order.lock().expect("order lock").push("fast");
        outcome
    });

    let budget: Duration = Duration::from_millis(FAST_STREAM_BUDGET_MILLIS);
    let fast_result: Result<H3Reply, String> = timeout(budget, fast)
        .await
        .expect("the fast stream must complete while the slow handler is still blocked")
        .expect("the fast stream task must not panic");
    assert!(
        fast_result.is_ok(),
        "the fast request must succeed while the slow handler is held"
    );
    assert_eq!(
        *order.lock().expect("order lock"),
        vec!["fast"],
        "only the fast stream may complete while the slow handler is blocked"
    );

    route_gate(GateSlot::SlowHttp3).release_slow();
    let slow_result: Result<H3Reply, String> =
        timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS), slow)
            .await
            .expect("the released slow stream must finish inside its timeout")
            .expect("the slow stream task must not panic");
    assert!(slow_result.is_ok());
    assert_eq!(*order.lock().expect("order lock"), vec!["fast", "slow"]);
    control.shutdown().await;
}
