use super::*;

#[tokio::test]
async fn http1_plaintext_keeps_the_exact_request_target_bytes() {
    let (control, port): (ServerControlHook, u16) =
        start_target_server(ServerProtocol::Http1, None)
            .await
            .expect("the http1 target server must start");
    for (name, target) in TARGET_CASES {
        let mut stream: TcpStream = connect_tcp(port).await.expect("socket must connect");
        let (head, body): (String, Vec<u8>) = http1_target_reply(&mut stream, target).await;
        assert!(
            head.starts_with("HTTP/1.1 200"),
            "{name}: the response must be framed as HTTP/1.1, got: {head}"
        );
        assert_eq!(
            response_header_value(&head, PROBE_VERSION_HEADER),
            format!("{:?}", HttpVersion::Http1_1),
            "{name}: the handler must observe the HTTP/1.1 request version"
        );
        assert_eq!(
            response_header_value(&head, TARGET_ECHO_HEADER),
            target,
            "{name}: the echoed target header must carry the exact target bytes"
        );
        assert_eq!(
            String::from_utf8_lossy(&body),
            target,
            "{name}: the echoed body must carry the exact target bytes"
        );
    }
    control.shutdown().await;
}

#[tokio::test]
async fn http1_plaintext_preserves_a_trailing_question_mark() {
    let (control, port): (ServerControlHook, u16) =
        start_target_server(ServerProtocol::Http1, None)
            .await
            .expect("the http1 target server must start");
    let mut stream: TcpStream = connect_tcp(port).await.expect("socket must connect");
    let (head, body): (String, Vec<u8>) =
        http1_target_reply(&mut stream, TRAILING_QUESTION_TARGET).await;
    assert_eq!(
        response_header_value(&head, TARGET_ECHO_HEADER),
        TRAILING_QUESTION_TARGET,
        "a trailing question mark must survive the http1 target"
    );
    assert_eq!(
        String::from_utf8_lossy(&body),
        TRAILING_QUESTION_TARGET,
        "a trailing question mark must survive the http1 target"
    );
    control.shutdown().await;
}

#[tokio::test]
async fn http1_keep_alive_next_request_replaces_the_previous_target() {
    let (control, port): (ServerControlHook, u16) =
        start_target_server(ServerProtocol::Http1, None)
            .await
            .expect("the http1 target server must start");
    let mut stream: TcpStream = connect_tcp(port).await.expect("socket must connect");
    let (first_head, first_body): (String, Vec<u8>) =
        http1_target_reply(&mut stream, TARGET_CASES[0].1).await;
    let (second_head, second_body): (String, Vec<u8>) =
        http1_target_reply(&mut stream, TARGET_CASES[3].1).await;
    assert_eq!(
        response_header_value(&first_head, TARGET_ECHO_HEADER),
        TARGET_CASES[0].1,
        "the first request must report its own target"
    );
    assert_eq!(
        response_header_value(&second_head, TARGET_ECHO_HEADER),
        TARGET_CASES[3].1,
        "the second request on the same connection must not retain the first target"
    );
    assert_eq!(String::from_utf8_lossy(&first_body), TARGET_CASES[0].1);
    assert_eq!(String::from_utf8_lossy(&second_body), TARGET_CASES[3].1);
    control.shutdown().await;
}

#[tokio::test]
async fn http2_prior_knowledge_keeps_the_exact_request_target_bytes() {
    let (control, port): (ServerControlHook, u16) =
        start_target_server(ServerProtocol::Http2, None)
            .await
            .expect("the http2 target server must start");
    let (mut sender, connection) = h2_plaintext_session(port)
        .await
        .expect("h2 prior-knowledge handshake must succeed");
    let driving: JoinHandle<()> = spawn(async move {
        let _result: Result<(), h2::Error> = connection.await;
    });
    for (name, target) in TARGET_CASES {
        let uri: String = format!("http://{TLS_SERVER_NAME}:{port}{target}");
        let (status, version, body): H2Reply = h2_request(&mut sender, "GET", &uri, &[])
            .await
            .expect("the h2 target request must complete");
        assert_eq!(status, 200, "{name}: the h2 target request must succeed");
        assert_eq!(
            version,
            http::Version::HTTP_2,
            "{name}: the response must be framed as HTTP/2"
        );
        assert_eq!(
            String::from_utf8_lossy(&body),
            target,
            "{name}: the echoed body must carry the exact target bytes"
        );
    }
    drop(sender);
    driving.abort();
    control.shutdown().await;
}

#[tokio::test]
async fn http2_prior_knowledge_preserves_a_trailing_question_mark() {
    let (control, port): (ServerControlHook, u16) =
        start_target_server(ServerProtocol::Http2, None)
            .await
            .expect("the http2 target server must start");
    let (mut sender, connection) = h2_plaintext_session(port)
        .await
        .expect("h2 prior-knowledge handshake must succeed");
    let driving: JoinHandle<()> = spawn(async move {
        let _result: Result<(), h2::Error> = connection.await;
    });
    let uri: String = format!("http://{TLS_SERVER_NAME}:{port}{TRAILING_QUESTION_TARGET}");
    let (status, headers, body): H2HeadReply =
        h2_request_with_headers(&mut sender, "GET", &uri, &[])
            .await
            .expect("the h2 target request must complete");
    assert_eq!(status, 200);
    assert_eq!(
        headers
            .get(PROBE_VERSION_HEADER)
            .and_then(|value: &http::HeaderValue| value.to_str().ok()),
        Some(format!("{:?}", HttpVersion::Http2).as_str()),
        "the handler must observe the HTTP/2 request version"
    );
    assert_eq!(
        headers
            .get(TARGET_ECHO_HEADER)
            .and_then(|value: &http::HeaderValue| value.to_str().ok()),
        Some(TRAILING_QUESTION_TARGET),
        "a trailing question mark must survive the h2 target"
    );
    assert_eq!(
        String::from_utf8_lossy(&body),
        TRAILING_QUESTION_TARGET,
        "a trailing question mark must survive the h2 target"
    );
    drop(sender);
    driving.abort();
    control.shutdown().await;
}

#[tokio::test]
async fn http2_next_stream_replaces_the_previous_target() {
    let (control, port): (ServerControlHook, u16) =
        start_target_server(ServerProtocol::Http2, None)
            .await
            .expect("the http2 target server must start");
    let (mut sender, connection) = h2_plaintext_session(port)
        .await
        .expect("h2 prior-knowledge handshake must succeed");
    let driving: JoinHandle<()> = spawn(async move {
        let _result: Result<(), h2::Error> = connection.await;
    });
    let first_uri: String = format!("http://{TLS_SERVER_NAME}:{port}{}", TARGET_CASES[0].1);
    let second_uri: String = format!("http://{TLS_SERVER_NAME}:{port}{}", TARGET_CASES[5].1);
    let (first_status, _, first_body): H2HeadReply =
        h2_request_with_headers(&mut sender, "GET", &first_uri, &[])
            .await
            .expect("the first h2 target request must complete");
    let (second_status, _, second_body): H2HeadReply =
        h2_request_with_headers(&mut sender, "GET", &second_uri, &[])
            .await
            .expect("the second h2 target request must complete");
    assert_eq!(first_status, 200);
    assert_eq!(second_status, 200);
    assert_eq!(
        String::from_utf8_lossy(&first_body),
        TARGET_CASES[0].1,
        "the first stream must report its own target"
    );
    assert_eq!(
        String::from_utf8_lossy(&second_body),
        TARGET_CASES[5].1,
        "the second stream on the same connection must not retain the first target"
    );
    drop(sender);
    driving.abort();
    control.shutdown().await;
}

#[tokio::test]
async fn http3_over_quic_keeps_the_exact_request_target_bytes() {
    let (control, port): (ServerControlHook, u16) =
        start_target_server(ServerProtocol::Http3, Some(trusted_tls_config()))
            .await
            .expect("the http3 target server must start");
    for (name, target) in TARGET_CASES {
        let uri: String = format!("https://{TLS_SERVER_NAME}:{port}{target}");
        let (status, version, body, alpn): H3Reply = h3_request(port, "GET", &uri, &[])
            .await
            .expect("the h3 target request must complete");
        assert_eq!(status, 200, "{name}: the h3 target request must succeed");
        assert_eq!(
            version,
            http::Version::HTTP_3,
            "{name}: the response must be framed as HTTP/3"
        );
        assert_eq!(
            alpn,
            Some(ALPN_HTTP3.to_vec()),
            "{name}: the connection must carry the h3 ALPN identifier"
        );
        assert_eq!(
            String::from_utf8_lossy(&body),
            target,
            "{name}: the echoed body must carry the exact target bytes"
        );
    }
    control.shutdown().await;
}

#[tokio::test]
async fn http3_over_quic_preserves_a_trailing_question_mark() {
    let (control, port): (ServerControlHook, u16) =
        start_target_server(ServerProtocol::Http3, Some(trusted_tls_config()))
            .await
            .expect("the http3 target server must start");
    let uri: String = format!("https://{TLS_SERVER_NAME}:{port}{TRAILING_QUESTION_TARGET}");
    let (status, headers, body, _alpn): H3HeadReply =
        h3_request_with_headers(port, "GET", &uri, &[])
            .await
            .expect("the h3 target request must complete");
    assert_eq!(status, 200);
    assert_eq!(
        headers
            .get(PROBE_VERSION_HEADER)
            .and_then(|value: &http::HeaderValue| value.to_str().ok()),
        Some(format!("{:?}", HttpVersion::Http3).as_str()),
        "the handler must observe the HTTP/3 request version"
    );
    assert_eq!(
        headers
            .get(TARGET_ECHO_HEADER)
            .and_then(|value: &http::HeaderValue| value.to_str().ok()),
        Some(TRAILING_QUESTION_TARGET),
        "a trailing question mark must survive the h3 target"
    );
    assert_eq!(
        String::from_utf8_lossy(&body),
        TRAILING_QUESTION_TARGET,
        "a trailing question mark must survive the h3 target"
    );
    control.shutdown().await;
}

#[tokio::test]
async fn http3_next_stream_on_one_connection_replaces_the_previous_target() {
    let (control, port): (ServerControlHook, u16) =
        start_target_server(ServerProtocol::Http3, Some(trusted_tls_config()))
            .await
            .expect("the http3 target server must start");
    let replies: Vec<(u16, http::Version, Vec<u8>)> =
        h3_targets_on_one_connection(port, [TARGET_CASES[0].1, TARGET_CASES[5].1])
            .await
            .expect("the h3 target requests must complete on one connection");
    assert_eq!(replies.len(), 2);
    assert_eq!(replies[0].0, 200);
    assert_eq!(replies[1].0, 200);
    assert_eq!(replies[0].1, http::Version::HTTP_3);
    assert_eq!(replies[1].1, http::Version::HTTP_3);
    assert_eq!(
        String::from_utf8_lossy(&replies[0].2),
        TARGET_CASES[0].1,
        "the first stream must report its own target"
    );
    assert_eq!(
        String::from_utf8_lossy(&replies[1].2),
        TARGET_CASES[5].1,
        "the second stream on the same connection must not retain the first target"
    );
    control.shutdown().await;
}
