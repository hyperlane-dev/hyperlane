use super::*;

pub(crate) fn limits_request_config(max_body_size: usize, read_timeout_ms: u64) -> RequestConfig {
    let mut config: RequestConfig = RequestConfig::low_security();
    config.set_max_body_size(max_body_size);
    config.set_read_timeout_ms(read_timeout_ms);
    config
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
    register_limits_hooks(&mut server);
    run_limits_server(server, protocol, request_config).await
}

pub(super) async fn start_default_request_error_server(
    protocol: ServerProtocol,
    max_body_size: usize,
) -> Result<(ServerControlHook, u16), Box<ServerError>> {
    let mut server: Server = Server::default();
    server.route::<BodyEchoRoute>(BODY_ECHO_ROUTE_PATH);
    server.response_middleware::<LimitsResponseSender>();
    run_limits_server(
        server,
        protocol,
        limits_request_config(max_body_size, READ_TIMEOUT_MS),
    )
    .await
}

async fn run_limits_server(
    mut server: Server,
    protocol: ServerProtocol,
    request_config: RequestConfig,
) -> Result<(ServerControlHook, u16), Box<ServerError>> {
    let tls: Option<TlsConfig> = if protocol.is_quic() {
        Some(trusted_tls_config())
    } else {
        None
    };
    server.server_config(protocol_config(PLACEHOLDER_ADDRESS, protocol, tls));
    server.request_config(request_config);
    let (control, port): (ServerControlHook, u16) = run_on_reserved_port(&mut server).await?;
    drop(server);
    sleep(Duration::from_millis(SERVER_BOOT_MILLIS)).await;
    Ok((control, port))
}

pub(super) fn http1_body_request(payload: &[u8]) -> Vec<u8> {
    let mut request: Vec<u8> = format!(
        "{POST} {BODY_ECHO_ROUTE_PATH} {HTTP_VERSION_1_1}{HTTP_BR}Host: {LOOPBACK_HOST}{HTTP_BR}\
         {CONTENT_LENGTH}: {}{HTTP_DOUBLE_BR}",
        payload.len(),
    )
    .into_bytes();
    request.extend_from_slice(payload);
    request
}

pub(super) async fn h2_oversized_body_then_get(
    port: u16,
) -> Result<(u16, HeaderMap, Vec<u8>), String> {
    let (mut sender, connection): (SendRequest<Bytes>, h2::client::Connection<TcpStream, Bytes>) =
        h2_plaintext_session(port).await?;
    tokio::select! {
        biased;
        result = async {
            let request: http::Request<()> =
                h2_build_request(POST, &h2_target(port, BODY_ECHO_ROUTE_PATH), None)?;
            let (response, mut send_stream): (h2::client::ResponseFuture, h2::SendStream<Bytes>) =
                h2_open_stream(&mut sender, request, false)?;
            h2_push_body(&mut send_stream, &oversized_payload(), true)?;
            let reply: (u16, http::HeaderMap, Vec<u8>) = h2_await_response(response).await?;
            let (status, _headers, _body): (u16, http::HeaderMap, Vec<u8>) =
                h2_get(&mut sender, port, BODY_ECHO_ROUTE_PATH).await?;
            if status != WIRE_OK_STATUS {
                return Err(format!("later request on the same h2 connection returned {status}"));
            }
            Ok(reply)
        } => result,
        closed = connection => Err(format!("h2 connection ended during body limit test: {closed:?}")),
    }
}

pub(super) async fn h3_oversized_body_then_get(
    port: u16,
) -> Result<(u16, HeaderMap, Vec<u8>), String> {
    let (_endpoint, connection): (Endpoint, quinn::Connection) =
        connect_quic(port, ALPN_HTTP3).await?;
    let (mut driver, mut client): (
        h3::client::Connection<h3_quinn::Connection, Bytes>,
        client::SendRequest<OpenStreams, Bytes>,
    ) = new(h3_quinn::Connection::new(connection))
        .await
        .map_err(|error: h3::error::ConnectionError| format!("h3 handshake failed: {error}"))?;
    let target: String = h3_target(port, BODY_ECHO_ROUTE_PATH);
    tokio::select! {
        biased;
        result = async {
            let reply: (u16, http::HeaderMap, Vec<u8>) =
                h3_outcome_to_reply(h3_drive(&mut client, POST, &target, &oversized_payload()).await)?;
            let (status, _headers, _body): (u16, http::HeaderMap, Vec<u8>) =
                h3_outcome_to_reply(h3_drive(&mut client, GET, &target, &[]).await)?;
            if status != WIRE_OK_STATUS {
                return Err(format!("later request on the same h3 connection returned {status}"));
            }
            Ok(reply)
        } => result,
        closed = driver.wait_idle() => Err(format!("h3 connection ended during body limit test: {closed}")),
    }
}

pub(super) async fn expect_exchange(
    protocol: ServerProtocol,
    content_length: usize,
    upload: Option<&[u8]>,
) -> (u16, Vec<u8>) {
    let _guard: MutexGuard<'_, ()> = limits_test_lock().lock().await;
    let (control, port): (ServerControlHook, u16) = start_limits_server(
        protocol,
        limits_request_config(MAX_BODY_SIZE, DEFAULT_LOW_SECURITY_READ_TIMEOUT_MS),
    )
    .await
    .expect("Expect test server must start");
    let result: Result<Result<(u16, Vec<u8>), String>, Elapsed> =
        timeout(Duration::from_secs(EXPECT_EXCHANGE_TIMEOUT_SECS), async {
            match protocol {
                ServerProtocol::Http2 => h2_expect_exchange(port, content_length, upload).await,
                ServerProtocol::Http3 => h3_expect_exchange(port, content_length, upload).await,
                _ => unreachable!("unsupported Expect test protocol"),
            }
        })
        .await;
    control.shutdown().await;
    result
        .expect("Expect exchange timed out while withholding or echoing the body")
        .unwrap_or_else(|error: String| panic!("Expect exchange failed: {error}"))
}

fn expect_request(target: &str, content_length: usize) -> Result<http::Request<()>, String> {
    http::Request::builder()
        .method(POST)
        .uri(target)
        .header(EXPECT, CONTINUE_EXPECT)
        .header(CONTENT_LENGTH, content_length.to_string())
        .body(())
        .map_err(|error: http::Error| format!("Expect request build failed: {error}"))
}

fn check_continue_head(head: &http::Response<()>) -> Result<(), String> {
    if head.status() != http::StatusCode::CONTINUE || !head.headers().is_empty() {
        return Err(format!("expected an empty 100 Continue head, got {head:?}"));
    }
    Ok(())
}

async fn h2_expect_exchange(
    port: u16,
    content_length: usize,
    upload: Option<&[u8]>,
) -> Result<(u16, Vec<u8>), String> {
    let (mut sender, connection): (SendRequest<Bytes>, h2::client::Connection<TcpStream, Bytes>) =
        h2_plaintext_session(port).await?;
    tokio::select! {
        biased;
        result = async {
            let request: http::Request<()> =
                expect_request(&h2_target(port, BODY_ECHO_ROUTE_PATH), content_length)?;
            let (mut response, mut send_stream): (h2::client::ResponseFuture, h2::SendStream<Bytes>) =
                h2_open_stream(&mut sender, request, false)?;
            let interim: Option<http::Response<()>> = h2_next_informational(&mut response).await?;
            if let Some(body) = upload {
                let head: http::Response<()> = interim.ok_or_else(|| "expected 100 Continue before sending the body".to_owned())?;
                check_continue_head(&head)?;
                h2_push_body(&mut send_stream, body, true)?;
                if let Some(head) = h2_next_informational(&mut response).await? {
                    return Err(format!("expected the final response after 100 Continue, got {head:?}"));
                }
                let (status, _headers, echoed): (u16, http::HeaderMap, Vec<u8>) = h2_await_response(response).await?;
                return Ok((status, echoed));
            }
            if let Some(head) = interim {
                return Err(format!("oversized declaration received an informational response: {head:?}"));
            }
            let final_head: http::Response<h2::RecvStream> = response.await
                .map_err(|error: h2::Error| format!("h2 rejection response failed: {error}"))?;
            Ok((final_head.status().as_u16(), Vec::new()))
        } => result,
        closed = connection => Err(format!("h2 connection ended during Expect exchange: {closed:?}")),
    }
}

async fn h2_next_informational(
    response: &mut ResponseFuture,
) -> Result<Option<http::Response<()>>, String> {
    poll_fn(|context: &mut task::Context<'_>| response.poll_informational(context))
        .await
        .transpose()
        .map_err(|error: Error| format!("h2 informational response failed: {error}"))
}

async fn h3_expect_exchange(
    port: u16,
    content_length: usize,
    upload: Option<&[u8]>,
) -> Result<(u16, Vec<u8>), String> {
    let (_endpoint, connection): (Endpoint, quinn::Connection) =
        connect_quic(port, ALPN_HTTP3).await?;
    let (mut driver, mut client): (
        h3::client::Connection<h3_quinn::Connection, Bytes>,
        client::SendRequest<OpenStreams, Bytes>,
    ) = new(h3_quinn::Connection::new(connection))
        .await
        .map_err(|error: h3::error::ConnectionError| format!("h3 handshake failed: {error}"))?;
    tokio::select! {
        biased;
        result = async {
            let request: http::Request<()> =
                expect_request(&h3_target(port, BODY_ECHO_ROUTE_PATH), content_length)?;
            let mut stream: h3::client::RequestStream<h3_quinn::BidiStream<Bytes>, Bytes> =
                client.send_request(request).await
                    .map_err(|error: h3::error::StreamError| format!("h3 send_request failed: {error}"))?;
            let head: http::Response<()> = stream.recv_response().await
                .map_err(|error: h3::error::StreamError| format!("h3 first response failed: {error}"))?;
            let Some(body) = upload else {
                return Ok((head.status().as_u16(), Vec::new()));
            };
            check_continue_head(&head)?;
            stream.send_data(Bytes::copy_from_slice(body)).await
                .map_err(|error: h3::error::StreamError| format!("h3 send_data failed: {error}"))?;
            stream.finish().await
                .map_err(|error: h3::error::StreamError| format!("h3 finish failed: {error}"))?;
            let (status, _headers, echoed): (u16, http::HeaderMap, Vec<u8>) = h3_collect(&mut stream).await?;
            Ok((status, echoed))
        } => result,
        closed = driver.wait_idle() => Err(format!("h3 connection ended during Expect exchange: {closed}")),
    }
}

pub(crate) fn h2_build_request(
    method: &str,
    target: &str,
    content_length: Option<usize>,
) -> Result<http::Request<()>, String> {
    let mut builder: Builder = http::Request::builder()
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
    sender: &mut SendRequest<Bytes>,
    request: http::Request<()>,
    end_of_stream: bool,
) -> Result<(ResponseFuture, SendStream<Bytes>), String> {
    sender
        .send_request(request, end_of_stream)
        .map_err(|error: Error| format!("h2 send_request failed: {error}"))
}

pub(crate) fn h2_push_body(
    send_stream: &mut SendStream<Bytes>,
    body: &[u8],
    end_of_stream: bool,
) -> Result<(), String> {
    send_stream
        .send_data(Bytes::copy_from_slice(body), end_of_stream)
        .map_err(|error: Error| format!("h2 send_data failed: {error}"))
}

pub(crate) fn h2_push_body_typed(
    send_stream: &mut SendStream<Bytes>,
    body: &[u8],
    end_of_stream: bool,
) -> Result<(), Error> {
    send_stream.send_data(Bytes::copy_from_slice(body), end_of_stream)
}

pub(crate) async fn h2_await_response(
    response: ResponseFuture,
) -> Result<(u16, HeaderMap, Vec<u8>), String> {
    match h2_await_outcome(response).await {
        LimitsOutcome::Replied(status, headers, body) => Ok((status, headers, body)),
        LimitsOutcome::H2Reset(error) => Err(format!("h2 stream reset: {error:?}")),
        LimitsOutcome::H3Reset(error) => Err(format!("h3 stream reset: {error:?}")),
        LimitsOutcome::TimedOut => Err("h2 exchange timed out".to_string()),
        LimitsOutcome::SetupFailed => Err("h2 response future must resolve".to_string()),
    }
}

pub(crate) async fn h2_await_outcome(response: ResponseFuture) -> LimitsOutcome {
    let received: Result<http::Response<RecvStream>, Error> =
        match timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS), response).await {
            Ok(received) => received,
            Err(_) => return LimitsOutcome::TimedOut,
        };
    let received: http::Response<RecvStream> = match received {
        Ok(received) => received,
        Err(error) => return LimitsOutcome::H2Reset(error),
    };
    let status: u16 = received.status().as_u16();
    let headers: HeaderMap = received.headers().clone();
    let mut body_stream: RecvStream = received.into_body();
    let mut collected: Vec<u8> = Vec::new();
    let deadline: Instant = Instant::now() + Duration::from_secs(REQUEST_TIMEOUT_SECS);
    loop {
        let remaining: Duration = deadline.saturating_duration_since(Instant::now());
        let chunk: Result<Option<Result<Bytes, Error>>, Elapsed> =
            timeout(remaining, body_stream.data()).await;
        match chunk {
            Err(_) => return LimitsOutcome::TimedOut,
            Ok(None) => break,
            Ok(Some(Err(error))) => return LimitsOutcome::H2Reset(error),
            Ok(Some(Ok(chunk))) => {
                let released: Result<(), Error> =
                    body_stream.flow_control().release_capacity(chunk.len());
                collected.extend_from_slice(&chunk);
                let _: Result<(), Error> = released;
            }
        }
    }
    LimitsOutcome::Replied(status, headers, collected)
}

pub(crate) async fn h2_still_pending(response: ResponseFuture, budget_ms: u64) -> bool {
    let outcome: Result<Result<http::Response<RecvStream>, Error>, Elapsed> =
        timeout(Duration::from_millis(budget_ms), response).await;
    outcome.is_err()
}

pub(crate) async fn h2_get(
    sender: &mut SendRequest<Bytes>,
    port: u16,
    path: &str,
) -> Result<(u16, HeaderMap, Vec<u8>), String> {
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

pub(crate) fn h3_reset_is_internal(error: &StreamError) -> bool {
    match error {
        h3::error::StreamError::StreamError { code, .. } => *code == Code::H3_INTERNAL_ERROR,
        h3::error::StreamError::RemoteTerminate { code, .. } => *code == Code::H3_INTERNAL_ERROR,
        _ => false,
    }
}

pub(crate) fn header_value(headers: &HeaderMap, key: &str) -> Option<String> {
    headers
        .get(key)
        .map(|value: &HeaderValue| value.to_str().unwrap_or_default().to_owned())
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
    stream: &mut RequestStream<BidiStream<Bytes>, Bytes>,
) -> Result<(u16, HeaderMap, Vec<u8>), String> {
    match h3_collect_outcome(stream).await {
        LimitsOutcome::Replied(status, headers, body) => Ok((status, headers, body)),
        LimitsOutcome::H3Reset(error) => Err(format!("h3 stream reset: {error:?}")),
        LimitsOutcome::TimedOut => Err("h3 exchange timed out".to_string()),
        LimitsOutcome::H2Reset(error) => Err(format!("h2 reset on an h3 exchange: {error:?}")),
        LimitsOutcome::SetupFailed => Err("h3 response must resolve".to_string()),
    }
}

async fn h3_collect_outcome(stream: &mut RequestStream<BidiStream<Bytes>, Bytes>) -> LimitsOutcome {
    let received: Result<http::Response<()>, StreamError> = match timeout(
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
    let headers: HeaderMap = received.headers().clone();
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
) -> Result<(u16, HeaderMap, Vec<u8>), String> {
    h3_outcome_to_reply(h3_exchange_outcome(port, method, target, body).await)
}

fn h3_outcome_to_reply(outcome: LimitsOutcome) -> Result<(u16, HeaderMap, Vec<u8>), String> {
    match outcome {
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
    let (_endpoint, connection): (Endpoint, quinn::Connection) =
        match connect_quic(port, ALPN_HTTP3).await {
            Ok(pair) => pair,
            Err(_) => return LimitsOutcome::SetupFailed,
        };
    let (mut driver, mut client) = match new(h3_quinn::Connection::new(connection.clone())).await {
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
    client: &mut client::SendRequest<OpenStreams, Bytes>,
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
    let mut stream: RequestStream<BidiStream<Bytes>, Bytes> =
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
