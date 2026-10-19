use super::*;

#[tokio::test]
async fn http2_head_response_carries_headers_without_a_body() {
    let _guard: tokio::sync::MutexGuard<'_, ()> = limits_test_lock().lock().await;
    reset_signals();
    let (control, port): (ServerControlHook, u16) = start_limits_server(
        ServerProtocol::Http2,
        limits_request_config(MAX_BODY_SIZE, READ_TIMEOUT_MS),
    )
    .await
    .expect("http2 limits server must start");
    let (mut sender, connection) = h2_plaintext_session(port)
        .await
        .expect("h2 plaintext handshake must succeed");
    let driving: JoinHandle<()> = spawn(async move {
        let _result: Result<(), h2::Error> = connection.await;
    });
    let request: http::Request<()> =
        h2_build_request("HEAD", &h2_target(port, HEAD_ROUTE_PATH), None)
            .expect("h2 head request must build");
    let (response, _send_stream) =
        h2_open_stream(&mut sender, request, true).expect("h2 head stream must open");
    let (status, _headers, body): (u16, http::HeaderMap, Vec<u8>) = h2_await_response(response)
        .await
        .unwrap_or_else(|error: String| panic!("h2 HEAD response failed: {error}"));
    assert_eq!(status, WIRE_OK_STATUS);
    assert!(
        body.is_empty(),
        "a HEAD response must carry no body bytes, got {} bytes",
        body.len()
    );
    let after: (u16, http::HeaderMap, Vec<u8>) = h2_get(&mut sender, port, BODY_ECHO_ROUTE_PATH)
        .await
        .expect("the same connection must serve a later valid request");
    assert_eq!(after.0, WIRE_OK_STATUS);
    drop(sender);
    driving.abort();
    control.shutdown().await;
}

#[tokio::test]
async fn http2_no_content_and_not_modified_strip_the_body_bytes() {
    let _guard: tokio::sync::MutexGuard<'_, ()> = limits_test_lock().lock().await;
    reset_signals();
    let (control, port): (ServerControlHook, u16) = start_limits_server(
        ServerProtocol::Http2,
        limits_request_config(MAX_BODY_SIZE, READ_TIMEOUT_MS),
    )
    .await
    .expect("http2 limits server must start");
    let (mut sender, connection) = h2_plaintext_session(port)
        .await
        .expect("h2 plaintext handshake must succeed");
    let driving: JoinHandle<()> = spawn(async move {
        let _result: Result<(), h2::Error> = connection.await;
    });
    for (path, expected) in [
        (NO_CONTENT_ROUTE_PATH, WIRE_NO_CONTENT_STATUS),
        (NOT_MODIFIED_ROUTE_PATH, WIRE_NOT_MODIFIED_STATUS),
    ] {
        let request: http::Request<()> =
            h2_build_request("GET", &h2_target(port, path), None).expect("h2 request must build");
        let (response, _send_stream) =
            h2_open_stream(&mut sender, request, true).expect("h2 stream must open");
        let (status, headers, body): (u16, http::HeaderMap, Vec<u8>) = h2_await_response(response)
            .await
            .unwrap_or_else(|error: String| panic!("h2 {expected} response failed: {error}"));
        assert_eq!(status, expected);
        assert!(
            body.is_empty(),
            "status {expected} must not carry body bytes, got {:?}",
            String::from_utf8_lossy(&body)
        );
        if expected == WIRE_NO_CONTENT_STATUS {
            assert!(
                header_value(&headers, CONTENT_LENGTH).is_none(),
                "204 must not announce a content length, got headers: {headers:?}"
            );
        }
    }
    drop(sender);
    driving.abort();
    control.shutdown().await;
}

#[tokio::test]
async fn http3_head_and_no_body_statuses_carry_no_body() {
    let _guard: tokio::sync::MutexGuard<'_, ()> = limits_test_lock().lock().await;
    reset_signals();
    let (control, port): (ServerControlHook, u16) = start_limits_server(
        ServerProtocol::Http3,
        limits_request_config(MAX_BODY_SIZE, READ_TIMEOUT_MS),
    )
    .await
    .expect("http3 limits server must start");
    for (method, path, expected) in [
        ("HEAD", HEAD_ROUTE_PATH, WIRE_OK_STATUS),
        ("GET", NO_CONTENT_ROUTE_PATH, WIRE_NO_CONTENT_STATUS),
        ("GET", NOT_MODIFIED_ROUTE_PATH, WIRE_NOT_MODIFIED_STATUS),
    ] {
        let (status, headers, body): (u16, http::HeaderMap, Vec<u8>) =
            h3_exchange(port, method, &h3_target(port, path), &[])
                .await
                .unwrap_or_else(|error: String| panic!("h3 {method} {path} failed: {error}"));
        assert_eq!(status, expected, "h3 {method} {path} answered {status}");
        assert!(
            body.is_empty(),
            "h3 {method} {path} must carry no body bytes, got {}",
            body.len()
        );
        if expected == WIRE_NO_CONTENT_STATUS {
            assert!(
                header_value(&headers, CONTENT_LENGTH).is_none(),
                "h3 204 must not announce a content length, got headers: {headers:?}"
            );
        }
    }
    control.shutdown().await;
}

#[tokio::test]
async fn http2_raw_chunked_framing_decodes_or_is_rejected_explicitly() {
    let _guard: tokio::sync::MutexGuard<'_, ()> = limits_test_lock().lock().await;
    reset_signals();
    let (control, port): (ServerControlHook, u16) = start_limits_server(
        ServerProtocol::Http2,
        limits_request_config(MAX_BODY_SIZE, READ_TIMEOUT_MS),
    )
    .await
    .expect("http2 limits server must start");
    let (mut sender, connection) = h2_plaintext_session(port)
        .await
        .expect("h2 plaintext handshake must succeed");
    let driving: JoinHandle<()> = spawn(async move {
        let _result: Result<(), h2::Error> = connection.await;
    });
    let request: http::Request<()> =
        h2_build_request("GET", &h2_target(port, RAW_CHUNKED_ROUTE_PATH), None)
            .expect("h2 chunked request must build");
    let (response, _send_stream) =
        h2_open_stream(&mut sender, request, true).expect("h2 stream must open");
    match h2_await_outcome(response).await {
        LimitsOutcome::Replied(status, headers, body) => {
            assert_eq!(
                status, WIRE_OK_STATUS,
                "a decodable chunked response must not be downgraded, got status {status}"
            );
            assert!(
                !headers.contains_key(CHUNKED_TRANSFER_HEADER),
                "a transfer coding is hop by hop on http2 and must be stripped, got: {headers:?}"
            );
            let text: String = String::from_utf8_lossy(&body).to_string();
            assert!(
                text.contains(CHUNKED_PAYLOAD) && !text.contains(CHUNK_SIZE_LINE),
                "a chunked body must reach the client decoded, never as chunk size bytes, got: {text:?}"
            );
        }
        LimitsOutcome::H2Reset(error) => {
            assert!(
                error.reason() == Some(h2::Reason::INTERNAL_ERROR),
                "an undecodable chunked body must be refused as an explicit internal reset, got: {error:?}"
            );
        }
        LimitsOutcome::H3Reset(error) => {
            panic!("an http2 request must never be answered with an http3 reset: {error:?}");
        }
        LimitsOutcome::TimedOut => {
            panic!("the raw chunked request must be answered or explicitly rejected, never stall");
        }
        LimitsOutcome::SetupFailed => {
            panic!("the raw chunked request must reach the server, never fail in the fixture");
        }
    }
    let after: (u16, http::HeaderMap, Vec<u8>) = h2_get(&mut sender, port, BODY_ECHO_ROUTE_PATH)
        .await
        .expect("the same connection must serve a later valid request");
    assert_eq!(after.0, WIRE_OK_STATUS);
    drop(sender);
    driving.abort();
    control.shutdown().await;
}

#[tokio::test]
async fn http3_raw_chunked_framing_decodes_or_is_rejected_explicitly() {
    let _guard: tokio::sync::MutexGuard<'_, ()> = limits_test_lock().lock().await;
    reset_signals();
    let (control, port): (ServerControlHook, u16) = start_limits_server(
        ServerProtocol::Http3,
        limits_request_config(MAX_BODY_SIZE, READ_TIMEOUT_MS),
    )
    .await
    .expect("http3 limits server must start");
    match h3_exchange_outcome(port, "GET", &h3_target(port, RAW_CHUNKED_ROUTE_PATH), &[]).await {
        LimitsOutcome::Replied(status, headers, body) => {
            assert_eq!(
                status, WIRE_OK_STATUS,
                "a decodable chunked response must not be downgraded, got status {status}"
            );
            assert!(
                !headers.contains_key(RAW_MARKER_HEADER),
                "the raw marker header does not exist on an http3 response, got: {headers:?}"
            );
            let text: String = String::from_utf8_lossy(&body).to_string();
            assert!(
                text.contains(CHUNKED_PAYLOAD) && !text.contains(CHUNK_SIZE_LINE),
                "a chunked body must reach the client decoded, never as chunk size bytes, got: {text:?}"
            );
        }
        LimitsOutcome::H3Reset(error) => {
            assert!(
                h3_reset_is_internal(&error),
                "an undecodable chunked body must be refused as an explicit internal reset, got: {error:?}"
            );
        }
        LimitsOutcome::H2Reset(error) => {
            panic!("an http3 request must never be answered with an http2 reset: {error:?}");
        }
        LimitsOutcome::TimedOut => {
            panic!("the raw chunked request must be answered or explicitly rejected, never stall");
        }
        LimitsOutcome::SetupFailed => {
            panic!("the raw chunked request must reach the server, never fail in the fixture");
        }
    }
    let after: (u16, http::HeaderMap, Vec<u8>) =
        h3_exchange(port, "GET", &h3_target(port, BODY_ECHO_ROUTE_PATH), &[])
            .await
            .unwrap_or_else(|error: String| panic!("the later valid request failed: {error}"));
    assert_eq!(after.0, WIRE_OK_STATUS);
    control.shutdown().await;
}

#[tokio::test]
async fn http2_oversized_body_hits_max_body_size_and_the_request_error_hook() {
    let _guard: tokio::sync::MutexGuard<'_, ()> = limits_test_lock().lock().await;
    reset_signals();
    let (control, port): (ServerControlHook, u16) = start_limits_server(
        ServerProtocol::Http2,
        limits_request_config(MAX_BODY_SIZE, READ_TIMEOUT_MS),
    )
    .await
    .expect("http2 limits server must start");
    let (mut sender, connection) = h2_plaintext_session(port)
        .await
        .expect("h2 plaintext handshake must succeed");
    let driving: JoinHandle<()> = spawn(async move {
        let _result: Result<(), h2::Error> = connection.await;
    });
    let before: (u16, http::HeaderMap, Vec<u8>) = h2_get(&mut sender, port, BODY_ECHO_ROUTE_PATH)
        .await
        .expect("a valid request before the oversized one must succeed");
    assert_eq!(before.0, WIRE_OK_STATUS);
    let payload: Vec<u8> = oversized_payload();
    let request: http::Request<()> = h2_build_request(
        "POST",
        &h2_target(port, BODY_ECHO_ROUTE_PATH),
        Some(payload.len()),
    )
    .expect("h2 oversized request must build");
    let (response, mut send_stream) =
        h2_open_stream(&mut sender, request, false).expect("h2 stream must open");
    match h2_push_body_typed(&mut send_stream, &payload, true) {
        Ok(()) => {}
        Err(error) => {
            assert!(
                error.is_reset() || error.is_go_away(),
                "an oversized body must be rejected as a stream reset, not a local failure, got: {error}"
            );
        }
    }
    if let LimitsOutcome::Replied(status, _headers, body) = h2_await_outcome(response).await {
        assert_ne!(
            status, WIRE_OK_STATUS,
            "a body above max_body_size must never be accepted as a normal 200"
        );
        assert!(
            body.len() < payload.len(),
            "an oversized body must never be echoed whole, got {} of {}",
            body.len(),
            payload.len()
        );
    }
    assert!(
        request_error_seen(),
        "an oversized body must reach the request_error hook"
    );
    drop(sender);
    driving.abort();
    control.shutdown().await;
}

#[tokio::test]
async fn http3_oversized_body_hits_max_body_size_and_the_request_error_hook() {
    let _guard: tokio::sync::MutexGuard<'_, ()> = limits_test_lock().lock().await;
    reset_signals();
    let (control, port): (ServerControlHook, u16) = start_limits_server(
        ServerProtocol::Http3,
        limits_request_config(MAX_BODY_SIZE, READ_TIMEOUT_MS),
    )
    .await
    .expect("http3 limits server must start");
    let before: (u16, http::HeaderMap, Vec<u8>) =
        h3_exchange(port, "GET", &h3_target(port, BODY_ECHO_ROUTE_PATH), &[])
            .await
            .unwrap_or_else(|error: String| panic!("the baseline h3 request failed: {error}"));
    assert_eq!(before.0, WIRE_OK_STATUS);
    let payload: Vec<u8> = oversized_payload();
    let outcome: Result<(u16, http::HeaderMap, Vec<u8>), String> = h3_exchange(
        port,
        "POST",
        &h3_target(port, BODY_ECHO_ROUTE_PATH),
        &payload,
    )
    .await;
    if let Ok((status, _headers, body)) = outcome {
        assert_ne!(
            status, WIRE_OK_STATUS,
            "a body above max_body_size must never be accepted as a normal 200"
        );
        assert!(
            body.len() < payload.len(),
            "an oversized body must never be echoed whole, got {} of {}",
            body.len(),
            payload.len()
        );
    }
    assert!(
        request_error_seen(),
        "an oversized body must reach the request_error hook"
    );
    control.shutdown().await;
}

#[tokio::test]
async fn http2_incomplete_body_hits_the_read_timeout_without_blocking_a_valid_request() {
    let _guard: tokio::sync::MutexGuard<'_, ()> = limits_test_lock().lock().await;
    reset_signals();
    let (control, port): (ServerControlHook, u16) = start_limits_server(
        ServerProtocol::Http2,
        limits_request_config(MAX_BODY_SIZE, READ_TIMEOUT_MS),
    )
    .await
    .expect("http2 limits server must start");
    let (mut sender, connection) = h2_plaintext_session(port)
        .await
        .expect("h2 plaintext handshake must succeed");
    let driving: JoinHandle<()> = spawn(async move {
        let _result: Result<(), h2::Error> = connection.await;
    });
    let payload: Vec<u8> = declared_short_payload();
    let stalled: http::Request<()> = h2_build_request(
        "POST",
        &h2_target(port, BODY_ECHO_ROUTE_PATH),
        Some(WRONG_CONTENT_LENGTH),
    )
    .expect("h2 incomplete request must build");
    let (stalled_response, mut stalled_send) =
        h2_open_stream(&mut sender, stalled, false).expect("stalled stream must open");
    h2_push_body(&mut stalled_send, &payload, false).expect("the partial body must be accepted");
    assert!(
        h2_still_pending(stalled_response, CONCURRENT_BUDGET_MS).await,
        "an incomplete body must stay pending until the read timeout fires"
    );
    let valid: http::Request<()> =
        h2_build_request("GET", &h2_target(port, BODY_ECHO_ROUTE_PATH), None)
            .expect("h2 valid request must build");
    let (valid_response, _valid_send) =
        h2_open_stream(&mut sender, valid, true).expect("valid stream must open");
    let (status, headers, body): (u16, http::HeaderMap, Vec<u8>) = timeout(
        Duration::from_millis(CONCURRENT_BUDGET_MS),
        h2_await_response(valid_response),
    )
    .await
    .unwrap_or_else(|_: tokio::time::error::Elapsed| {
        panic!("the valid request was blocked by the stalled one")
    })
    .expect("the valid request must complete");
    assert_eq!(status, WIRE_OK_STATUS);
    assert!(body.is_empty());
    assert_eq!(
        header_value(&headers, BODY_ECHO_HEADER).as_deref(),
        Some("0"),
        "the valid request must be answered from its own empty body, got headers: {headers:?}"
    );
    drop(stalled_send);
    drop(sender);
    driving.abort();
    control.shutdown().await;
}

#[tokio::test]
async fn http3_incomplete_body_hits_the_read_timeout_without_blocking_a_valid_request() {
    let _guard: tokio::sync::MutexGuard<'_, ()> = limits_test_lock().lock().await;
    reset_signals();
    let (control, port): (ServerControlHook, u16) = start_limits_server(
        ServerProtocol::Http3,
        limits_request_config(MAX_BODY_SIZE, READ_TIMEOUT_MS),
    )
    .await
    .expect("http3 limits server must start");
    let (_endpoint, connection): (quinn::Endpoint, quinn::Connection) =
        connect_quic(port, ALPN_HTTP3)
            .await
            .expect("quic connect must succeed");
    let (mut driver, mut client) = h3::client::new(h3_quinn::Connection::new(connection.clone()))
        .await
        .expect("h3 handshake must succeed");
    let driving: JoinHandle<()> = spawn(async move {
        let _final: h3::error::ConnectionError = driver.wait_idle().await;
    });
    let payload: Vec<u8> = declared_short_payload();
    let stalled: http::Request<()> = http::Request::builder()
        .method("POST")
        .uri(h3_target(port, BODY_ECHO_ROUTE_PATH))
        .header(CONTENT_LENGTH, WRONG_CONTENT_LENGTH.to_string())
        .body(())
        .expect("h3 incomplete request must build");
    let mut stalled_stream: h3::client::RequestStream<h3_quinn::BidiStream<Bytes>, Bytes> = client
        .send_request(stalled)
        .await
        .expect("stalled stream must open");
    stalled_stream
        .send_data(Bytes::copy_from_slice(&payload))
        .await
        .expect("the partial body must be accepted");
    let still_pending: bool = timeout(
        Duration::from_millis(CONCURRENT_BUDGET_MS),
        stalled_stream.recv_response(),
    )
    .await
    .is_err();
    assert!(
        still_pending,
        "an incomplete body must stay pending until the read timeout fires"
    );
    let valid: http::Request<()> = http::Request::builder()
        .method("GET")
        .uri(h3_target(port, BODY_ECHO_ROUTE_PATH))
        .body(())
        .expect("h3 valid request must build");
    let mut valid_stream: h3::client::RequestStream<h3_quinn::BidiStream<Bytes>, Bytes> = client
        .send_request(valid)
        .await
        .expect("valid stream must open");
    valid_stream
        .finish()
        .await
        .expect("the valid request must finish");
    let (status, headers, body): (u16, http::HeaderMap, Vec<u8>) = timeout(
        Duration::from_millis(CONCURRENT_BUDGET_MS),
        h3_collect(&mut valid_stream),
    )
    .await
    .unwrap_or_else(|_: tokio::time::error::Elapsed| {
        panic!("the valid request was blocked by the stalled one")
    })
    .expect("the valid request must complete");
    assert_eq!(status, WIRE_OK_STATUS);
    assert!(body.is_empty());
    assert_eq!(
        header_value(&headers, BODY_ECHO_HEADER).as_deref(),
        Some("0"),
        "the valid request must be answered from its own empty body, got headers: {headers:?}"
    );
    drop(client);
    driving.abort();
    control.shutdown().await;
}

#[tokio::test]
async fn http2_wrong_declared_content_length_never_yields_a_truncated_success() {
    let _guard: tokio::sync::MutexGuard<'_, ()> = limits_test_lock().lock().await;
    reset_signals();
    let (control, port): (ServerControlHook, u16) = start_limits_server(
        ServerProtocol::Http2,
        limits_request_config(MAX_BODY_SIZE, READ_TIMEOUT_MS),
    )
    .await
    .expect("http2 limits server must start");
    let (mut sender, connection) = h2_plaintext_session(port)
        .await
        .expect("h2 plaintext handshake must succeed");
    let driving: JoinHandle<()> = spawn(async move {
        let _result: Result<(), h2::Error> = connection.await;
    });
    let payload: Vec<u8> = declared_short_payload();
    let request: http::Request<()> = h2_build_request(
        "POST",
        &h2_target(port, BODY_ECHO_ROUTE_PATH),
        Some(WRONG_CONTENT_LENGTH),
    )
    .expect("h2 truncated request must build");
    let (response, mut send_stream) =
        h2_open_stream(&mut sender, request, false).expect("h2 stream must open");
    h2_push_body(&mut send_stream, &payload, false).expect("the partial body must be accepted");
    if let Ok((status, headers, body)) = h2_await_response(response).await {
        assert_ne!(
            status, WIRE_OK_STATUS,
            "a short body against a larger declared length must never succeed"
        );
        assert!(
            body.is_empty() || body.len() != payload.len(),
            "a truncated body must not be handed to the handler as if complete, got {} bytes",
            body.len()
        );
        assert_ne!(
            header_value(&headers, BODY_ECHO_HEADER).as_deref(),
            Some(DECLARED_BODY_TEXT),
            "the handler must not observe the declared length as received, got headers: {headers:?}"
        );
    }
    drop(send_stream);
    drop(sender);
    driving.abort();
    control.shutdown().await;
}

#[tokio::test]
async fn http2_panicking_route_reaches_the_task_panic_hook() {
    let _guard: tokio::sync::MutexGuard<'_, ()> = limits_test_lock().lock().await;
    reset_signals();
    let (control, port): (ServerControlHook, u16) = start_limits_server(
        ServerProtocol::Http2,
        limits_request_config(MAX_BODY_SIZE, READ_TIMEOUT_MS),
    )
    .await
    .expect("http2 limits server must start");
    let (mut sender, connection) = h2_plaintext_session(port)
        .await
        .expect("h2 plaintext handshake must succeed");
    let driving: JoinHandle<()> = spawn(async move {
        let _result: Result<(), h2::Error> = connection.await;
    });
    let request: http::Request<()> =
        h2_build_request("GET", &h2_target(port, PANIC_ROUTE_PATH), None)
            .expect("h2 panic request must build");
    let (response, _send_stream) =
        h2_open_stream(&mut sender, request, true).expect("h2 stream must open");
    if let Ok((status, headers, _body)) = h2_await_response(response).await {
        assert_eq!(
            status, WIRE_SERVER_ERROR_STATUS,
            "a panicking route must answer 500, got headers: {headers:?}"
        );
        assert_eq!(
            header_value(&headers, TASK_PANIC_MARKER_HEADER).as_deref(),
            Some("seen"),
            "the task_panic hook must own the response, got headers: {headers:?}"
        );
    }
    assert!(
        task_panic_seen(),
        "a panicking route must reach the task_panic hook"
    );
    drop(sender);
    driving.abort();
    control.shutdown().await;
}

#[tokio::test]
async fn http3_panicking_route_reaches_the_task_panic_hook() {
    let _guard: tokio::sync::MutexGuard<'_, ()> = limits_test_lock().lock().await;
    reset_signals();
    let (control, port): (ServerControlHook, u16) = start_limits_server(
        ServerProtocol::Http3,
        limits_request_config(MAX_BODY_SIZE, READ_TIMEOUT_MS),
    )
    .await
    .expect("http3 limits server must start");
    if let Ok((status, headers, _body)) =
        h3_exchange(port, "GET", &h3_target(port, PANIC_ROUTE_PATH), &[]).await
    {
        assert_eq!(
            status, WIRE_SERVER_ERROR_STATUS,
            "a panicking route must answer 500, got headers: {headers:?}"
        );
        assert_eq!(
            header_value(&headers, TASK_PANIC_MARKER_HEADER).as_deref(),
            Some("seen"),
            "the task_panic hook must own the response, got headers: {headers:?}"
        );
    }
    assert!(
        task_panic_seen(),
        "a panicking route must reach the task_panic hook"
    );
    control.shutdown().await;
}
