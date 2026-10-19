use super::*;

#[tokio::test]
async fn http2_plaintext_reports_the_real_local_and_peer_addresses_of_the_connection() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http2, None)
            .await
            .expect("http2 plaintext server must start");
    let socket: TcpStream = connect_tcp(port).await.expect("socket must connect");
    let client_local: SocketAddr = socket
        .local_addr()
        .expect("a connected socket must report its local address");
    let (mut sender, connection) = timeout(
        Duration::from_secs(CONNECT_TIMEOUT_SECS),
        h2::client::handshake(socket),
    )
    .await
    .expect("h2 prior-knowledge handshake must not time out")
    .expect("h2 prior-knowledge handshake must succeed");
    let driving: JoinHandle<()> = spawn(async move {
        let _result: Result<(), h2::Error> = connection.await;
    });
    let (status, _version, body): H2Reply = h2_request(
        &mut sender,
        "GET",
        &format!("http://{TLS_SERVER_NAME}:{port}{SOCKET_METADATA_ROUTE_PATH}"),
        &[],
    )
    .await
    .expect("h2 socket metadata request must complete");
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
        "an h2 stream must carry the connection's real local address"
    );
    assert_eq!(
        reported_peer,
        client_local.to_string(),
        "an h2 stream must carry the connection's real peer address, not a fabricated one"
    );
    drop(sender);
    driving.abort();
    control.shutdown().await;
}

#[tokio::test]
async fn http2_streams_a_body_that_never_announced_a_content_length() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http2, None)
            .await
            .expect("http2 plaintext server must start");
    let (mut sender, connection) = h2_plaintext_session(port)
        .await
        .expect("h2 prior-knowledge handshake must succeed");
    let driving: JoinHandle<()> = spawn(async move {
        let _result: Result<(), h2::Error> = connection.await;
    });
    let (status, headers, body): H2HeadReply = h2_request_with_headers(
        &mut sender,
        "GET",
        &format!("http://{TLS_SERVER_NAME}:{port}{UNFRAMED_BODY_ROUTE_PATH}"),
        &[],
    )
    .await
    .expect("an unframed h2 body must be accepted");
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
    drop(sender);
    driving.abort();
    control.shutdown().await;
}

#[tokio::test]
async fn http2_plaintext_prior_knowledge_serves_http2_over_tcp() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http2, None)
            .await
            .expect("http2 plaintext server must start");
    let (mut sender, connection) = h2_plaintext_session(port)
        .await
        .expect("h2 prior-knowledge handshake must succeed");
    let driving: JoinHandle<()> = spawn(async move {
        let _result: Result<(), h2::Error> = connection.await;
    });
    let (status, version, body): H2Reply = h2_request(
        &mut sender,
        "GET",
        &format!("http://{TLS_SERVER_NAME}:{port}{VERSION_ROUTE_PATH}"),
        &[],
    )
    .await
    .expect("h2 plaintext request must complete");
    assert_eq!(status, 200);
    assert_eq!(
        version,
        http::Version::HTTP_2,
        "the response must be framed as HTTP/2, not downgraded"
    );
    assert_eq!(
        String::from_utf8_lossy(&body),
        format!("{:?}", HttpVersion::Http2)
    );
    drop(sender);
    driving.abort();
    control.shutdown().await;
}

#[tokio::test]
async fn http2_plaintext_prior_knowledge_reads_path_query_and_header() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http2, None)
            .await
            .expect("http2 plaintext server must start");
    let (mut sender, connection) = h2_plaintext_session(port)
        .await
        .expect("h2 prior-knowledge handshake must succeed");
    let driving: JoinHandle<()> = spawn(async move {
        let _result: Result<(), h2::Error> = connection.await;
    });
    let target: String = format!(
        "http://{TLS_SERVER_NAME}:{port}{PROBE_PATH}?{}",
        PROBE_QUERY
    );
    let (status, _version, _body): H2Reply = h2_request(&mut sender, "GET", &target, &[])
        .await
        .expect("h2 probe request must complete");
    assert_eq!(status, 200);
    drop(sender);
    driving.abort();
    control.shutdown().await;
}

#[tokio::test]
async fn http2_tls_negotiates_h2_alpn_against_validated_chain() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http2, Some(trusted_tls_config()))
            .await
            .expect("http2 tls server must start");
    let (alpn, (mut sender, connection)): (Option<Vec<u8>>, H2TlsSession) = h2_tls_alpn(port)
        .await
        .expect("h2 tls handshake must succeed");
    assert_eq!(
        alpn,
        Some(ALPN_HTTP2.to_vec()),
        "the server must select the h2 ALPN identifier"
    );
    let driving: JoinHandle<()> = spawn(async move {
        let _result: Result<(), h2::Error> = connection.await;
    });
    let (status, version, body): H2Reply = h2_request(
        &mut sender,
        "GET",
        &format!("https://{TLS_SERVER_NAME}:{port}{VERSION_ROUTE_PATH}"),
        &[],
    )
    .await
    .expect("h2 tls request must complete");
    assert_eq!(status, 200);
    assert_eq!(version, http::Version::HTTP_2);
    assert_eq!(
        String::from_utf8_lossy(&body),
        format!("{:?}", HttpVersion::Http2)
    );
    drop(sender);
    driving.abort();
    control.shutdown().await;
}

#[tokio::test]
async fn http2_tls_rejects_a_client_offering_only_http11() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http2, Some(trusted_tls_config()))
            .await
            .expect("http2 tls server must start");
    let outcome: io::Result<ClientTlsStream> = connect_tls(port, ALPN_HTTP1).await;
    assert!(
        outcome.is_err(),
        "an http2 listener must refuse an http/1.1-only client instead of downgrading"
    );
    let valid: io::Result<ClientTlsStream> = connect_tls(port, ALPN_HTTP2).await;
    assert!(
        valid.is_ok(),
        "a rejected client must not disturb a later valid connection"
    );
    control.shutdown().await;
}

#[tokio::test]
async fn http2_plaintext_binary_post_round_trips_hostile_bytes() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http2, None)
            .await
            .expect("http2 plaintext server must start");
    let (mut sender, connection) = h2_plaintext_session(port)
        .await
        .expect("h2 prior-knowledge handshake must succeed");
    let driving: JoinHandle<()> = spawn(async move {
        let _result: Result<(), h2::Error> = connection.await;
    });
    let payload: Vec<u8> = hostile_binary_payload();
    let (status, version, body): H2Reply = h2_request(
        &mut sender,
        "POST",
        &format!("http://{TLS_SERVER_NAME}:{port}{BINARY_ROUTE_PATH}"),
        &payload,
    )
    .await
    .expect("h2 binary post must complete");
    assert_eq!(status, 200);
    assert_eq!(version, http::Version::HTTP_2);
    assert_eq!(
        body, payload,
        "every hostile byte must survive the h2 body round trip"
    );
    drop(sender);
    driving.abort();
    control.shutdown().await;
}

#[tokio::test]
async fn http2_plaintext_delivers_a_response_above_the_flow_control_window() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http2, None)
            .await
            .expect("http2 plaintext server must start");
    let (mut sender, connection) = h2_plaintext_session(port)
        .await
        .expect("h2 prior-knowledge handshake must succeed");
    let driving: JoinHandle<()> = spawn(async move {
        let _result: Result<(), h2::Error> = connection.await;
    });
    let expected: Vec<u8> = large_binary_payload();
    let (status, _version, body): H2Reply = h2_request(
        &mut sender,
        "GET",
        &format!("http://{TLS_SERVER_NAME}:{port}{LARGE_ROUTE_PATH}"),
        &[],
    )
    .await
    .expect("large h2 response must complete");
    assert_eq!(status, 200);
    assert_eq!(
        body.len(),
        expected.len(),
        "the whole multi-megabyte body must arrive"
    );
    assert!(
        body == expected,
        "the large response must be byte identical past the flow control window"
    );
    drop(sender);
    driving.abort();
    control.shutdown().await;
}

#[tokio::test]
async fn http2_one_connection_answers_a_fast_stream_while_a_slow_stream_is_blocked() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http2, None)
            .await
            .expect("http2 plaintext server must start");
    let (sender, connection) = h2_plaintext_session(port)
        .await
        .expect("h2 prior-knowledge handshake must succeed");
    let driving: JoinHandle<()> = spawn(async move {
        let _result: Result<(), h2::Error> = connection.await;
    });
    let _lease: GateLease = gate_lease(GateSlot::SlowHttp2).await;
    let order: Arc<Mutex<Vec<&'static str>>> = Arc::new(Mutex::new(Vec::new()));
    let slow_target: String = format!("http://{TLS_SERVER_NAME}:{port}{SLOW_ROUTE_PATH}");
    let slow_order: Arc<Mutex<Vec<&'static str>>> = Arc::clone(&order);
    let mut slow_sender: h2::client::SendRequest<Bytes> = sender.clone();
    let slow: JoinHandle<Result<H2Reply, String>> = spawn(async move {
        let outcome: Result<H2Reply, String> =
            h2_request(&mut slow_sender, "GET", &slow_target, &[]).await;
        slow_order.lock().expect("order lock").push("slow");
        outcome
    });
    route_gate(GateSlot::SlowHttp2).await_slow_entered().await;
    let fast_target: String = format!("http://{TLS_SERVER_NAME}:{port}{VERSION_ROUTE_PATH}");
    let fast_order: Arc<Mutex<Vec<&'static str>>> = Arc::clone(&order);
    let mut fast_sender: h2::client::SendRequest<Bytes> = sender.clone();
    let fast: JoinHandle<Result<H2Reply, String>> = spawn(async move {
        let outcome: Result<H2Reply, String> =
            h2_request(&mut fast_sender, "GET", &fast_target, &[]).await;
        fast_order.lock().expect("order lock").push("fast");
        outcome
    });
    let budget: Duration = Duration::from_millis(FAST_STREAM_BUDGET_MILLIS);
    let fast_result: Result<H2Reply, String> = timeout(budget, fast)
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
    route_gate(GateSlot::SlowHttp2).release_slow();
    let slow_result: Result<H2Reply, String> =
        timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS), slow)
            .await
            .expect("the released slow stream must finish inside its timeout")
            .expect("the slow stream task must not panic");
    assert!(slow_result.is_ok());
    assert_eq!(
        *order.lock().expect("order lock"),
        vec!["fast", "slow"],
        "the fast stream must complete before the released slow one"
    );
    drop(sender);
    driving.abort();
    control.shutdown().await;
}

#[tokio::test]
async fn http2_plaintext_shutdown_releases_the_bound_tcp_port() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http2, None)
            .await
            .expect("http2 plaintext server must start");
    assert_wait_pending_then_complete(&control).await;
    let rebound: io::Result<TcpListener> = TcpListener::bind(format!("{LOOPBACK_HOST}:{port}"));
    assert!(
        rebound.is_ok(),
        "port {port} must be free again after shutdown and wait"
    );
}

#[tokio::test]
async fn http2_sse_chunk_reaches_the_client_before_the_handler_finishes() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http2, None)
            .await
            .expect("http2 plaintext server must start");
    let _lease: GateLease = gate_lease(GateSlot::SseHttp2).await;
    let target: String = format!("http://{TLS_SERVER_NAME}:{port}{SSE_ROUTE_PATH_HTTP2}");
    let reading: JoinHandle<(Vec<u8>, bool)> =
        spawn(async move { h2_stream_until(port, &target, SSE_FIRST_CHUNK).await });
    route_gate(GateSlot::SseHttp2)
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
    route_gate(GateSlot::SseHttp2).release_sse();
    control.shutdown().await;
}
