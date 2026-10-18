use super::*;

#[tokio::test]
async fn http1_and_http2_plaintext_listeners_never_claim_the_same_tcp_port() {
    let (first, first_port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http1, None)
            .await
            .expect("http1 server must start");
    let (second, second_port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http2, None)
            .await
            .expect("http2 server must start");
    assert_ne!(
        first_port, second_port,
        "each listener must own a distinct tcp port"
    );
    let request: Vec<u8> = http1_close_request("GET", VERSION_ROUTE_PATH);
    let raw: Vec<u8> = raw_http1_exchange(first_port, &request)
        .await
        .expect("the http1 listener must answer a plain http1 request");
    assert!(response_head(&raw).starts_with("HTTP/1.1 200"));
    first.shutdown().await;
    second.shutdown().await;
}

#[tokio::test]
async fn an_http1_listener_refuses_a_prior_knowledge_http2_client() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http1, None)
            .await
            .expect("http1 server must start");
    let (mut sender, connection) = h2_plaintext_session(port)
        .await
        .expect("the client handshake future resolves before the wire is settled");
    let driving: JoinHandle<()> = spawn(async move {
        let _result: Result<(), h2::Error> = connection.await;
    });
    let target: String = format!("http://{TLS_SERVER_NAME}:{port}{VERSION_ROUTE_PATH}");
    let outcome: Result<H2Reply, String> = h2_request(&mut sender, "GET", &target, &[]).await;
    assert!(
        outcome.is_err(),
        "an http1 listener must not answer an http2 request on the wire, got: {outcome:?}"
    );
    drop(sender);
    driving.abort();
    let request: Vec<u8> = http1_close_request("GET", VERSION_ROUTE_PATH);
    let raw: Vec<u8> = raw_http1_exchange(port, &request)
        .await
        .expect("the http1 listener must still serve plain http1");
    assert!(response_head(&raw).starts_with("HTTP/1.1 200"));
    control.shutdown().await;
}

#[tokio::test]
async fn an_http2_listener_does_not_answer_a_plain_http1_request() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http2, None)
            .await
            .expect("http2 server must start");
    let request: &[u8] = b"GET /version HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n";
    if let Ok(raw) = raw_http1_exchange(port, request).await {
        let head: String = response_head(&raw);
        assert!(
            !head.starts_with("HTTP/1.1 200"),
            "an http2 listener must not answer an http1 request, got: {head}"
        );
    }
    control.shutdown().await;
}

#[tokio::test]
async fn an_http3_listener_does_not_answer_a_tcp_request() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http3, Some(trusted_tls_config()))
            .await
            .expect("http3 server must start");
    let request: &[u8] = b"GET /version HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n";
    if let Ok(raw) = raw_http1_exchange(port, request).await {
        let head: String = response_head(&raw);
        assert!(
            !head.starts_with("HTTP/1.1 200"),
            "an http3 listener binds udp only and must not answer tcp, got: {head}"
        );
    }
    control.shutdown().await;
}

#[tokio::test]
async fn http2_strips_every_hop_by_hop_header_but_keeps_end_to_end_ones() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http2, None)
            .await
            .expect("http2 server must start");
    let (mut sender, connection) = h2_plaintext_session(port)
        .await
        .expect("h2 handshake must succeed");
    let driving: JoinHandle<()> = spawn(async move {
        let _result: Result<(), h2::Error> = connection.await;
    });
    let (status, headers, body): (u16, http::HeaderMap, Vec<u8>) = h2_request_with_headers(
        &mut sender,
        "GET",
        &format!("http://{TLS_SERVER_NAME}:{port}{HOP_BY_HOP_CLEAN_ROUTE_PATH}"),
        &[],
    )
    .await
    .expect("the hop-by-hop route must answer");
    assert_eq!(status, 200);
    for name in [
        CONNECTION,
        KEEP_ALIVE,
        UPGRADE,
        TE,
        INTERNAL_CONNECTION_HEADER,
    ] {
        assert!(
            !headers.contains_key(name),
            "{name} is hop by hop on http2 and must not reach the client, got: {headers:?}"
        );
    }
    assert_eq!(
        headers
            .get(PRESERVED_HEADER_KEY)
            .map(|value: &http::HeaderValue| value.to_str().unwrap_or_default()),
        Some(PRESERVED_HEADER_VALUE),
        "an end-to-end header must survive the stripping, got: {headers:?}"
    );
    assert_eq!(String::from_utf8_lossy(&body), HOP_BY_HOP_SENT_BODY);
    let (status, _version, body): (u16, http::Version, Vec<u8>) = h2_request(
        &mut sender,
        "GET",
        &format!("http://{TLS_SERVER_NAME}:{port}{HOP_BY_HOP_CLEAN_ROUTE_PATH}"),
        &[],
    )
    .await
    .expect("the hop-by-hop route must answer a second time on the same connection");
    assert_eq!(status, 200);
    assert_eq!(String::from_utf8_lossy(&body), HOP_BY_HOP_SENT_BODY);
    drop(sender);
    driving.abort();
    control.shutdown().await;
}

#[tokio::test]
async fn http2_refuses_a_response_announcing_a_chunked_transfer_encoding() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http2, None)
            .await
            .expect("http2 server must start");
    let (mut sender, connection) = h2_plaintext_session(port)
        .await
        .expect("h2 handshake must succeed");
    let driving: JoinHandle<()> = spawn(async move {
        let _result: Result<(), h2::Error> = connection.await;
    });
    let outcome: Result<H2Reply, String> = h2_request(
        &mut sender,
        "GET",
        &format!("http://{TLS_SERVER_NAME}:{port}{HOP_BY_HOP_ROUTE_PATH}"),
        &[],
    )
    .await;
    assert!(
        outcome.is_err(),
        "a head announcing a transfer coding must be refused, never delivered as a success, got: {outcome:?}"
    );
    drop(sender);
    driving.abort();
    control.shutdown().await;
}

#[tokio::test]
async fn http3_refuses_a_response_announcing_a_chunked_transfer_encoding() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http3, Some(trusted_tls_config()))
            .await
            .expect("http3 quic server must start");
    let outcome: Result<H3Reply, String> = h3_request(
        port,
        "GET",
        &format!("https://{TLS_SERVER_NAME}:{port}{HOP_BY_HOP_ROUTE_PATH}"),
        &[],
    )
    .await;
    assert!(
        outcome.is_err(),
        "a head announcing a transfer coding must be refused, never delivered as a success, got: {outcome:?}"
    );
    control.shutdown().await;
}

#[tokio::test]
async fn http1_keeps_its_connection_header_because_it_is_end_to_end_on_http1() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http1, None)
            .await
            .expect("http1 server must start");
    let request: Vec<u8> = http1_close_request("GET", HOP_BY_HOP_ROUTE_PATH);
    let raw: Vec<u8> = raw_http1_exchange(port, &request)
        .await
        .expect("http1 exchange must complete");
    let head: String = response_head(&raw).to_lowercase();
    assert!(
        head.contains(CONNECTION),
        "an http1 response keeps its connection header, got: {head}"
    );
    control.shutdown().await;
}

#[tokio::test]
async fn http1_split_writes_reassemble_into_one_body_across_several_writes() {
    let (control, port): (ServerControlHook, u16) =
        start_protocol_server(ServerProtocol::Http1, None)
            .await
            .expect("http1 server must start");
    let mut stream: TcpStream = connect_tcp(port).await.expect("socket must connect");
    let request: Vec<u8> = http1_close_request("GET", SPLIT_ROUTE_PATH);
    write_bounded(&mut stream, &request)
        .await
        .expect("request must be written");
    let (head, body): (String, Vec<u8>) = read_http1_head(&mut stream, SPLIT_BODY_LEN)
        .await
        .expect("the split response head must be readable");
    assert!(head.starts_with("HTTP/1.1 200"), "got: {head}");
    let text: String = String::from_utf8_lossy(&body).to_string();
    assert_eq!(
        text,
        format!("{SPLIT_HEAD_BODY}{SPLIT_TAIL_BODY}"),
        "fragments written separately must reassemble into one body"
    );
    control.shutdown().await;
}
