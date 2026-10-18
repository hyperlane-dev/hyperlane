use super::*;

pub(crate) fn limits_request_config(max_body_size: usize, read_timeout_ms: u64) -> RequestConfig {
    RequestConfig {
        max_body_size,
        read_timeout_ms,
        ..RequestConfig::default()
    }
}

fn register_limits_hooks(server: &mut Server) -> &mut Server {
    server.route::<HeadBodyRoute>(HEAD_ROUTE_PATH);
    server.route::<NoContentRoute>(NO_CONTENT_ROUTE_PATH);
    server.route::<NotModifiedRoute>(NOT_MODIFIED_ROUTE_PATH);
    server.route::<RawChunkedRoute>(RAW_CHUNKED_ROUTE_PATH);
    server.route::<PanicRoute>(PANIC_ROUTE_PATH);
    server.route::<BodyEchoRoute>(BODY_ECHO_ROUTE_PATH);
    server.response_middleware::<LimitsResponseSender>();
    server.task_panic::<TaskPanicRecorder>();
    server.request_error::<RequestErrorRecorder>();
    server
}

pub(crate) async fn start_limits_server(
    protocol: ServerProtocol,
    request_config: RequestConfig,
) -> Result<(ServerControlHook, u16), Box<ServerError>> {
    let mut server: Server = Server::default();
    let address: String = if protocol.is_quic() {
        format!("{LOOPBACK_HOST}:{}", reserve_udp_port())
    } else {
        format!("{LOOPBACK_HOST}:{}", reserve_tcp_port())
    };
    let tls: Option<TlsConfig> = if protocol.is_quic() {
        Some(trusted_tls_config())
    } else {
        None
    };
    server.server_config(protocol_config(&address, protocol, tls));
    server.request_config(request_config);
    register_limits_hooks(&mut server);
    let control: ServerControlHook = server.run().await?;
    drop(server);
    sleep(Duration::from_millis(SERVER_BOOT_MILLIS)).await;
    let port: u16 = address
        .rsplit(':')
        .next()
        .and_then(|value: &str| value.parse::<u16>().ok())
        .expect("bind address must carry a numeric port");
    Ok((control, port))
}

pub(crate) fn h2_build_request(
    method: &str,
    target: &str,
    content_length: Option<usize>,
) -> Result<http::Request<()>, String> {
    let mut builder: http::request::Builder = http::Request::builder()
        .method(method)
        .uri(target)
        .header(PROBE_HEADER_KEY, PROBE_HEADER_VALUE);
    if let Some(length) = content_length {
        builder = builder.header(CONTENT_LENGTH, length.to_string());
    }
    builder
        .body(())
        .map_err(|error: http::Error| format!("h2 request build failed: {error}"))
}

pub(crate) fn h2_open_stream(
    sender: &mut h2::client::SendRequest<Bytes>,
    request: http::Request<()>,
    end_of_stream: bool,
) -> Result<(h2::client::ResponseFuture, h2::SendStream<Bytes>), String> {
    sender
        .send_request(request, end_of_stream)
        .map_err(|error: h2::Error| format!("h2 send_request failed: {error}"))
}

pub(crate) fn h2_push_body(
    send_stream: &mut h2::SendStream<Bytes>,
    body: &[u8],
    end_of_stream: bool,
) -> Result<(), String> {
    send_stream
        .send_data(Bytes::copy_from_slice(body), end_of_stream)
        .map_err(|error: h2::Error| format!("h2 send_data failed: {error}"))
}

pub(crate) fn h2_push_body_typed(
    send_stream: &mut h2::SendStream<Bytes>,
    body: &[u8],
    end_of_stream: bool,
) -> Result<(), h2::Error> {
    send_stream.send_data(Bytes::copy_from_slice(body), end_of_stream)
}

pub(crate) async fn h2_await_response(
    response: h2::client::ResponseFuture,
) -> Result<(u16, http::HeaderMap, Vec<u8>), String> {
    match h2_await_outcome(response).await {
        LimitsOutcome::Replied(status, headers, body) => Ok((status, headers, body)),
        LimitsOutcome::H2Reset(error) => Err(format!("h2 stream reset: {error:?}")),
        LimitsOutcome::H3Reset(error) => Err(format!("h3 stream reset: {error:?}")),
        LimitsOutcome::TimedOut => Err("h2 exchange timed out".to_string()),
        LimitsOutcome::SetupFailed => Err("h2 response future must resolve".to_string()),
    }
}

pub(crate) async fn h2_await_outcome(response: h2::client::ResponseFuture) -> LimitsOutcome {
    let received: Result<http::Response<h2::RecvStream>, h2::Error> =
        match timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS), response).await {
            Ok(received) => received,
            Err(_) => return LimitsOutcome::TimedOut,
        };
    let received: http::Response<h2::RecvStream> = match received {
        Ok(received) => received,
        Err(error) => return LimitsOutcome::H2Reset(error),
    };
    let status: u16 = received.status().as_u16();
    let headers: http::HeaderMap = received.headers().clone();
    let mut body_stream: h2::RecvStream = received.into_body();
    let mut collected: Vec<u8> = Vec::new();
    let deadline: Instant = Instant::now() + Duration::from_secs(REQUEST_TIMEOUT_SECS);
    loop {
        let remaining: Duration = deadline.saturating_duration_since(Instant::now());
        let chunk: Result<Option<Result<Bytes, h2::Error>>, tokio::time::error::Elapsed> =
            timeout(remaining, body_stream.data()).await;
        match chunk {
            Err(_) => return LimitsOutcome::TimedOut,
            Ok(None) => break,
            Ok(Some(Err(error))) => return LimitsOutcome::H2Reset(error),
            Ok(Some(Ok(chunk))) => {
                let released: Result<(), h2::Error> =
                    body_stream.flow_control().release_capacity(chunk.len());
                collected.extend_from_slice(&chunk);
                let _: Result<(), h2::Error> = released;
            }
        }
    }
    LimitsOutcome::Replied(status, headers, collected)
}

pub(crate) async fn h2_still_pending(response: h2::client::ResponseFuture, budget_ms: u64) -> bool {
    let outcome: Result<
        Result<http::Response<h2::RecvStream>, h2::Error>,
        tokio::time::error::Elapsed,
    > = timeout(Duration::from_millis(budget_ms), response).await;
    outcome.is_err()
}

pub(crate) async fn h2_get(
    sender: &mut h2::client::SendRequest<Bytes>,
    port: u16,
    path: &str,
) -> Result<(u16, http::HeaderMap, Vec<u8>), String> {
    let request: http::Request<()> =
        h2_build_request("GET", &h2_target(port, path), None).map_err(|error: String| error)?;
    let (response, _send_stream) =
        h2_open_stream(sender, request, true).map_err(|error: String| error)?;
    h2_await_response(response).await
}

pub(crate) fn h2_target(port: u16, path: &str) -> String {
    format!("http://{TLS_SERVER_NAME}:{port}{path}")
}

pub(crate) fn h3_target(port: u16, path: &str) -> String {
    format!("https://{TLS_SERVER_NAME}:{port}{path}")
}

pub(crate) fn h3_reset_is_internal(error: &h3::error::StreamError) -> bool {
    match error {
        h3::error::StreamError::StreamError { code, .. } => *code == Code::H3_INTERNAL_ERROR,
        h3::error::StreamError::RemoteTerminate { code, .. } => *code == Code::H3_INTERNAL_ERROR,
        _ => false,
    }
}

pub(crate) fn header_value(headers: &http::HeaderMap, key: &str) -> Option<String> {
    headers
        .get(key)
        .map(|value: &http::HeaderValue| value.to_str().unwrap_or_default().to_owned())
}

pub(crate) fn oversized_payload() -> Vec<u8> {
    let mut payload: Vec<u8> = Vec::with_capacity(OVERSIZED_BODY_LEN);
    let mut seed: u32 = 0x2468_1357;
    while payload.len() < OVERSIZED_BODY_LEN {
        seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
        payload.extend_from_slice(&seed.to_be_bytes());
    }
    payload.truncate(OVERSIZED_BODY_LEN);
    payload
}

pub(crate) fn declared_short_payload() -> Vec<u8> {
    let mut payload: Vec<u8> = INCOMPLETE_BODY_PREFIX.to_vec();
    payload.resize(DECLARED_BODY_LEN, b'a');
    payload
}

pub(crate) async fn h3_collect(
    stream: &mut h3::client::RequestStream<h3_quinn::BidiStream<Bytes>, Bytes>,
) -> Result<(u16, http::HeaderMap, Vec<u8>), String> {
    match h3_collect_outcome(stream).await {
        LimitsOutcome::Replied(status, headers, body) => Ok((status, headers, body)),
        LimitsOutcome::H3Reset(error) => Err(format!("h3 stream reset: {error:?}")),
        LimitsOutcome::TimedOut => Err("h3 exchange timed out".to_string()),
        LimitsOutcome::H2Reset(error) => Err(format!("h2 reset on an h3 exchange: {error:?}")),
        LimitsOutcome::SetupFailed => Err("h3 response must resolve".to_string()),
    }
}

async fn h3_collect_outcome(
    stream: &mut h3::client::RequestStream<h3_quinn::BidiStream<Bytes>, Bytes>,
) -> LimitsOutcome {
    let received: Result<http::Response<()>, h3::error::StreamError> = match timeout(
        Duration::from_secs(REQUEST_TIMEOUT_SECS),
        stream.recv_response(),
    )
    .await
    {
        Ok(received) => received,
        Err(_) => return LimitsOutcome::TimedOut,
    };
    let received: http::Response<()> = match received {
        Ok(received) => received,
        Err(error) => return LimitsOutcome::H3Reset(error),
    };
    let status: u16 = received.status().as_u16();
    let headers: http::HeaderMap = received.headers().clone();
    let mut collected: Vec<u8> = Vec::new();
    let deadline: Instant = Instant::now() + Duration::from_secs(REQUEST_TIMEOUT_SECS);
    loop {
        let remaining: Duration = deadline.saturating_duration_since(Instant::now());
        match timeout(remaining, stream.recv_data()).await {
            Err(_) => return LimitsOutcome::TimedOut,
            Ok(Err(error)) => return LimitsOutcome::H3Reset(error),
            Ok(Ok(None)) => break,
            Ok(Ok(Some(mut chunk))) => {
                collected.extend_from_slice(&chunk.copy_to_bytes(chunk.remaining()));
            }
        }
    }
    LimitsOutcome::Replied(status, headers, collected)
}

pub(crate) async fn h3_exchange(
    port: u16,
    method: &str,
    target: &str,
    body: &[u8],
) -> Result<(u16, http::HeaderMap, Vec<u8>), String> {
    match h3_exchange_outcome(port, method, target, body).await {
        LimitsOutcome::Replied(status, headers, body) => Ok((status, headers, body)),
        LimitsOutcome::H3Reset(error) => Err(format!("h3 stream reset: {error:?}")),
        LimitsOutcome::TimedOut => Err("h3 exchange timed out".to_string()),
        LimitsOutcome::H2Reset(error) => Err(format!("h2 reset on an h3 exchange: {error:?}")),
        LimitsOutcome::SetupFailed => Err("h3 exchange could not be set up".to_string()),
    }
}

pub(crate) async fn h3_exchange_outcome(
    port: u16,
    method: &str,
    target: &str,
    body: &[u8],
) -> LimitsOutcome {
    let (_endpoint, connection): (quinn::Endpoint, quinn::Connection) =
        match connect_quic(port, ALPN_HTTP3).await {
            Ok(pair) => pair,
            Err(_) => return LimitsOutcome::SetupFailed,
        };
    let (mut driver, mut client) =
        match h3::client::new(h3_quinn::Connection::new(connection.clone())).await {
            Ok(pair) => pair,
            Err(_) => return LimitsOutcome::SetupFailed,
        };
    let driving: JoinHandle<()> = spawn(async move {
        let _final: h3::error::ConnectionError = driver.wait_idle().await;
    });
    let outcome: LimitsOutcome = h3_drive(&mut client, method, target, body).await;
    drop(client);
    driving.abort();
    connection.close(quinn::VarInt::from_u32(0), b"done");
    outcome
}

async fn h3_drive(
    client: &mut h3::client::SendRequest<h3_quinn::OpenStreams, Bytes>,
    method: &str,
    target: &str,
    body: &[u8],
) -> LimitsOutcome {
    let request: http::Request<()> = match http::Request::builder()
        .method(method)
        .uri(target)
        .header(PROBE_HEADER_KEY, PROBE_HEADER_VALUE)
        .body(())
    {
        Ok(request) => request,
        Err(_) => return LimitsOutcome::SetupFailed,
    };
    let mut stream: h3::client::RequestStream<h3_quinn::BidiStream<Bytes>, Bytes> =
        match client.send_request(request).await {
            Ok(stream) => stream,
            Err(error) => return LimitsOutcome::H3Reset(error),
        };
    if !body.is_empty()
        && let Err(error) = stream.send_data(Bytes::copy_from_slice(body)).await
    {
        return LimitsOutcome::H3Reset(error);
    }
    if let Err(error) = stream.finish().await {
        return LimitsOutcome::H3Reset(error);
    }
    h3_collect_outcome(&mut stream).await
}
