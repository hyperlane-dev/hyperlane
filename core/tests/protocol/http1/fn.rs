use super::*;

#[tokio::test]
async fn http1_plaintext_serves_default_protocol_over_tcp() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http1, None)
            .await
            .expect("http1 plaintext server must start");
    let request: Vec<u8> = http1_close_request("GET", VERSION_ROUTE_PATH);
    let raw: Vec<u8> = raw_http1_exchange(port, &request)
        .await
        .expect("plaintext http1 exchange must complete");
    let head: String = response_head(&raw);
    assert!(
        head.starts_with("HTTP/1.1 200"),
        "expected an HTTP/1.1 200 head, got: {head}"
    );
    let (_, body): (String, Vec<u8>) = response_head_and_body(&raw);
    assert_eq!(
        String::from_utf8_lossy(&body),
        format!("{:?}", HttpVersion::Http1_1)
    );
    control.shutdown().await;
}

#[tokio::test]
async fn http1_plaintext_reads_path_query_and_header_through_middleware() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http1, None)
            .await
            .expect("http1 plaintext server must start");
    let target: String = format!("{PROBE_PATH}?{}", PROBE_QUERY);
    let request: Vec<u8> = http1_request_with_headers("GET", &target, PROBE_REPEATED_HEADERS, &[]);
    let raw: Vec<u8> = raw_http1_exchange(port, &request)
        .await
        .expect("plaintext http1 exchange must complete");
    let head: String = response_head(&raw);
    assert_eq!(
        response_header_value(&head, PROBE_METHOD_HEADER),
        PROBE_METHOD,
        "the handler must observe the exact request method, got: {head}"
    );
    assert_eq!(
        response_header_value(&head, PROBE_PATH_HEADER),
        PROBE_PATH,
        "the handler must observe the exact request path, got: {head}"
    );
    assert_eq!(
        response_header_value(&head, PROBE_QUERY_HEADER),
        PROBE_QUERY_EXPECTED,
        "every query key must be read back with its value, got: {head}"
    );
    assert_eq!(
        response_header_value(&head, PROBE_HEADER_ECHO_HEADER),
        PROBE_HEADER_EXPECTED,
        "a repeated request header must keep every value in order, got: {head}"
    );
    assert_eq!(
        response_header_value(&head, PROBE_VERSION_HEADER),
        format!("{:?}", HttpVersion::Http1_1),
        "the handler must observe HTTP/1.1, got: {head}"
    );
    control.shutdown().await;
}

#[tokio::test]
async fn http1_tls_negotiates_http11_alpn_against_validated_chain() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http1, Some(trusted_tls_config()))
            .await
            .expect("http1 tls server must start");
    let stream: ClientTlsStream = connect_tls(port, ALPN_HTTP1)
        .await
        .expect("validating client must complete the handshake");
    let alpn: Option<Vec<u8>> = negotiated_alpn(&stream);
    assert_eq!(
        alpn,
        Some(ALPN_HTTP1.to_vec()),
        "the server must select the http/1.1 ALPN identifier"
    );
    let request: Vec<u8> = http1_close_request("GET", VERSION_ROUTE_PATH);
    let raw: Vec<u8> = tls_exchange(stream, request)
        .await
        .expect("tls http1 exchange must complete");
    let head: String = response_head(&raw);
    assert!(
        head.starts_with("HTTP/1.1 200"),
        "expected HTTP/1.1 over TLS, got: {head}"
    );
    control.shutdown().await;
}

#[tokio::test]
async fn http1_tls_rejects_a_client_whose_alpn_never_matches() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http1, Some(trusted_tls_config()))
            .await
            .expect("http1 tls server must start");
    let outcome: io::Result<ClientTlsStream> = connect_tls(port, ALPN_HTTP2).await;
    assert!(
        outcome.is_err(),
        "an http1 listener must refuse an h2-only client instead of silently downgrading"
    );
    let valid: io::Result<ClientTlsStream> = connect_tls(port, ALPN_HTTP1).await;
    assert!(
        valid.is_ok(),
        "a rejected client must not disturb a later valid connection"
    );
    control.shutdown().await;
}

#[tokio::test]
async fn http1_plaintext_binary_post_round_trips_hostile_bytes() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http1, None)
            .await
            .expect("http1 plaintext server must start");
    let payload: Vec<u8> = hostile_binary_payload();
    let request: Vec<u8> = http1_request("POST", BINARY_ROUTE_PATH, &payload);
    let raw: Vec<u8> = raw_http1_exchange(port, &request)
        .await
        .expect("binary post must complete");
    let (_, body): (String, Vec<u8>) = response_head_and_body(&raw);
    assert_eq!(
        body, payload,
        "every hostile byte must survive the http1 body round trip"
    );
    control.shutdown().await;
}

#[tokio::test]
async fn http1_plaintext_refuses_a_truncated_content_length_body() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http1, None)
            .await
            .expect("http1 plaintext server must start");
    let head: Vec<u8> = format!(
        "POST {BINARY_ROUTE_PATH} HTTP/1.1{HTTP_BR}Host: {TLS_SERVER_NAME}{HTTP_BR}Content-Length: 100{HTTP_BR}{HTTP_BR}"
    )
    .into_bytes();
    let mut request: Vec<u8> = head;
    request.extend_from_slice(b"only-ten!!");
    if let Ok(raw) = raw_http1_exchange(port, &request).await {
        let text: String = response_head(&raw);
        assert!(
            !text.contains(" 200"),
            "a truncated body must not be answered as a complete request, got: {text}"
        );
    }
    control.shutdown().await;
}

#[tokio::test]
async fn http1_plaintext_streams_split_writes_before_the_handler_finishes() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http1, None)
            .await
            .expect("http1 plaintext server must start");
    let request: Vec<u8> = http1_close_request("GET", SPLIT_ROUTE_PATH);
    let raw: Vec<u8> = raw_http1_exchange(port, &request)
        .await
        .expect("split write exchange must complete");
    let head: String = response_head(&raw);
    assert!(
        !head.to_lowercase().contains(CONTENT_LENGTH),
        "an event-stream response must not be framed with a content-length, got: {head}"
    );
    let (_, body): (String, Vec<u8>) = response_head_and_body(&raw);
    let text: String = String::from_utf8_lossy(&body).to_string();
    assert_eq!(
        text,
        format!("{SPLIT_HEAD_BODY}{SPLIT_TAIL_BODY}"),
        "every separately written fragment must reach the client in order, got: {text:?}"
    );
    control.shutdown().await;
}

#[tokio::test]
async fn http1_tls_streams_split_writes_through_the_tls_listener() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http1, Some(trusted_tls_config()))
            .await
            .expect("http1 tls server must start");
    let stream: ClientTlsStream = connect_tls(port, ALPN_HTTP1)
        .await
        .expect("validating client must complete the handshake");
    let request: Vec<u8> = http1_close_request("GET", SPLIT_ROUTE_PATH);
    let raw: Vec<u8> = tls_exchange(stream, request)
        .await
        .expect("split write exchange must complete");
    let head: String = response_head(&raw);
    assert!(
        !head.to_lowercase().contains(CONTENT_LENGTH),
        "an event-stream response must not be framed with a content-length, got: {head}"
    );
    let (_, body): (String, Vec<u8>) = response_head_and_body(&raw);
    let text: String = String::from_utf8_lossy(&body).to_string();
    assert_eq!(
        text,
        format!("{SPLIT_HEAD_BODY}{SPLIT_TAIL_BODY}"),
        "every separately written fragment must survive TLS in order, got: {text:?}"
    );
    control.shutdown().await;
}

#[tokio::test]
async fn http1_plaintext_reports_the_real_local_and_peer_addresses_of_the_socket() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http1, None)
            .await
            .expect("http1 plaintext server must start");
    let mut stream: TcpStream = connect_tcp(port).await.expect("socket must connect");
    let client_local: SocketAddr = stream
        .local_addr()
        .expect("a connected socket must report its local address");
    let request: Vec<u8> = http1_close_request("GET", SOCKET_METADATA_ROUTE_PATH);
    let raw: Vec<u8> = exchange_on(&mut stream, &request).await;
    let (_, body): (String, Vec<u8>) = response_head_and_body(&raw);
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
        "the handler must see the bound local address, not a fabricated one"
    );
    assert_eq!(
        reported_peer,
        client_local.to_string(),
        "the handler must see this connection's own peer address"
    );
    control.shutdown().await;
}

#[tokio::test]
async fn http1_sse_chunk_reaches_the_client_before_the_handler_finishes() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http1, None)
            .await
            .expect("http1 plaintext server must start");
    let _lease: GateLease = gate_lease(GateSlot::SseHttp1).await;
    let mut stream: TcpStream = connect_tcp(port).await.expect("socket must connect");
    let request: Vec<u8> = http1_close_request("GET", SSE_ROUTE_PATH);
    write_bounded(&mut stream, &request)
        .await
        .expect("sse request must be written");
    let mut received: Vec<u8> = Vec::new();
    let first_chunk_deadline: Instant = Instant::now() + Duration::from_secs(REQUEST_TIMEOUT_SECS);
    while !String::from_utf8_lossy(&received).contains(SSE_FIRST_CHUNK) {
        assert!(
            Instant::now() < first_chunk_deadline,
            "the first sse chunk must arrive while the handler is still blocked, got: {:?}",
            String::from_utf8_lossy(&received)
        );
        read_more(&mut stream, &mut received).await;
    }
    assert!(
        !String::from_utf8_lossy(&received).contains(SSE_SECOND_CHUNK),
        "the second chunk must not appear before the handler is released"
    );
    route_gate(GateSlot::SseHttp1).release_sse();
    let completion_deadline: Instant = Instant::now() + Duration::from_secs(REQUEST_TIMEOUT_SECS);
    while !String::from_utf8_lossy(&received).contains(SSE_SECOND_CHUNK) {
        assert!(
            Instant::now() < completion_deadline,
            "the second sse chunk must arrive once the handler is released"
        );
        read_more(&mut stream, &mut received).await;
    }
    control.shutdown().await;
}

#[tokio::test]
async fn http1_tls_sse_chunk_reaches_the_client_before_the_handler_finishes() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http1, Some(trusted_tls_config()))
            .await
            .expect("http1 tls server must start");
    let _lease: GateLease = gate_lease(GateSlot::SseHttp1).await;
    let mut stream: ClientTlsStream = connect_tls(port, ALPN_HTTP1)
        .await
        .expect("validating client must complete the handshake");
    let request: Vec<u8> = http1_close_request("GET", SSE_ROUTE_PATH);
    write_bounded(&mut stream, &request)
        .await
        .expect("sse request must be written");
    let first: String = http1_head_until(&mut stream, SSE_FIRST_CHUNK).await;
    assert!(
        first.contains(SSE_FIRST_CHUNK),
        "the first chunk must arrive over tls while the handler is blocked, got: {first:?}"
    );
    assert!(
        !first.contains(SSE_SECOND_CHUNK),
        "the second chunk must wait for the handler to be released, got: {first:?}"
    );
    route_gate(GateSlot::SseHttp1).release_sse();
    let second: String = http1_head_until(&mut stream, SSE_SECOND_CHUNK).await;
    assert!(
        second.contains(SSE_SECOND_CHUNK),
        "the second chunk must arrive once the tls handler is released, got: {second:?}"
    );
    control.shutdown().await;
}

#[tokio::test]
async fn http1_sse_response_carries_the_event_stream_content_type() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http1, None)
            .await
            .expect("http1 plaintext server must start");
    let _lease: GateLease = gate_lease(GateSlot::SseHttp1).await;
    let mut stream: TcpStream = connect_tcp(port).await.expect("socket must connect");
    let request: Vec<u8> = http1_close_request("GET", SSE_ROUTE_PATH);
    write_bounded(&mut stream, &request)
        .await
        .expect("sse request must be written");
    let head: String = http1_head_until(&mut stream, HTTP_DOUBLE_BR).await;
    assert!(
        head.to_lowercase().contains(TEXT_EVENT_STREAM),
        "the sse route must send an event-stream content type, got: {head}"
    );
    route_gate(GateSlot::SseHttp1).release_sse();
    let completed: String = http1_head_until(&mut stream, SSE_SECOND_CHUNK).await;
    assert!(
        completed.contains(SSE_SECOND_CHUNK),
        "the sse response must run to its end once released, got: {completed:?}"
    );
    control.shutdown().await;
}

#[tokio::test]
async fn http1_plaintext_repeats_a_duplicate_response_cookie() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http1, None)
            .await
            .expect("http1 plaintext server must start");
    let request: Vec<u8> = http1_close_request("GET", COOKIE_ROUTE_PATH);
    let raw: Vec<u8> = raw_http1_exchange(port, &request)
        .await
        .expect("cookie exchange must complete");
    let head: String = response_head(&raw);
    let cookies: usize = head.to_lowercase().matches(SET_COOKIE).count();
    assert_eq!(
        cookies, 2,
        "both set-cookie headers must survive, got: {head}"
    );
    assert!(
        head.contains("dup=one") && head.contains("dup2=two"),
        "both cookie values must be present, got: {head}"
    );
    control.shutdown().await;
}

#[tokio::test]
async fn http1_plaintext_answers_four_pipelined_requests_on_one_connection() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http1, None)
            .await
            .expect("http1 plaintext server must start");
    sleep(Duration::from_millis(SERVER_BOOT_MILLIS)).await;
    let mut stream: TcpStream = connect_tcp(port).await.expect("socket must connect");
    let mut batch: Vec<u8> = Vec::new();
    for _ in 0..4 {
        batch.extend_from_slice(&http1_request("GET", VERSION_ROUTE_PATH, &[]));
    }
    write_bounded(&mut stream, &batch)
        .await
        .expect("pipelined batch must be written");
    let mut received: Vec<u8> = Vec::new();
    let deadline: Instant = Instant::now() + Duration::from_secs(REQUEST_TIMEOUT_SECS);
    let mut chunk: [u8; 16 * 1024] = [0; 16 * 1024];
    while Instant::now() < deadline && count_http1_responses(&received) < 4 {
        match stream.read(&mut chunk).await {
            Ok(0) => break,
            Ok(read) => received.extend_from_slice(&chunk[..read]),
            Err(_) => break,
        }
    }
    assert_eq!(
        count_http1_responses(&received),
        4,
        "every pipelined request must be answered on the same connection"
    );
    control.shutdown().await;
}

#[tokio::test]
async fn http1_plaintext_shutdown_releases_the_bound_tcp_port() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http1, None)
            .await
            .expect("http1 plaintext server must start");
    assert_wait_pending_then_complete(&control).await;
    let rebound: io::Result<TcpListener> = TcpListener::bind(format!("{LOOPBACK_HOST}:{port}"));
    assert!(
        rebound.is_ok(),
        "port {port} must be free again after shutdown and wait"
    );
}

#[tokio::test]
async fn http1_tls_shutdown_releases_the_bound_tcp_port() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http1, Some(trusted_tls_config()))
            .await
            .expect("http1 tls server must start");
    assert_wait_pending_then_complete(&control).await;
    let rebound: io::Result<TcpListener> = TcpListener::bind(format!("{LOOPBACK_HOST}:{port}"));
    assert!(
        rebound.is_ok(),
        "port {port} must be free again after a tls shutdown and wait"
    );
}
