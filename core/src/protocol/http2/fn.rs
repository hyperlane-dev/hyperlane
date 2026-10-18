use super::*;

/// Serves one HTTP/2 connection until it closes.
///
/// The connection half stays polled for the whole lifetime of the connection:
/// it owns flow control and the settings exchange for every stream, so parking
/// it while a handler runs would stall the other requests multiplexed onto the
/// same connection. Each request is therefore spawned onto its own task while
/// this loop keeps polling.
///
/// # Arguments
///
/// - `Arc<Server>` - The server whose pipeline serves each request.
/// - `RequestConfig` - The parsing limits and read timeout for the connection.
/// - `SocketAddr` - The `value` argument.
/// - `Result<ConnectionH2<T>, String>` - The negotiated HTTP/2 connection, or
///
pub(crate) async fn serve_h2_connection<T>(
    server: Arc<Server>,
    request_config: RequestConfig,
    local: SocketAddr,
    remote: SocketAddr,
    connection: Result<ConnectionH2<T>, String>,
) where
    T: AsyncRead + AsyncWrite + Unpin + Send + Sync + 'static,
{
    let mut connection: ConnectionH2<T> = match connection {
        Ok(connection) => connection,
        Err(_) => return,
    };
    loop {
        match connection.accept().await {
            Some(Ok((request, send_response))) => {
                spawn(h2_request_task(
                    server.clone(),
                    request_config,
                    local,
                    remote,
                    request,
                    send_response,
                ));
            }
            Some(Err(_)) => return,
            None => return,
        }
    }
}

/// Runs the framework lifecycle over one HTTP/2 request stream.
///
/// The lifecycle and the framing pump run concurrently: the lifecycle writes
/// HTTP/1.1 shaped bytes into a duplex pipe and the pump frames them onto the
/// stream as they arrive. Awaiting both keeps the head and every body chunk on
/// the wire before the handler's completion is observed.
///
/// # Arguments
///
/// - `Arc<Server>` - The server whose pipeline serves the request.
/// - `RequestConfig` - The parsing limits and read timeout for the connection.
/// - `SocketAddr` - The local address of the carrying connection.
/// - `http::Request<h2::RecvStream>` - The decoded protocol request.
/// - `h2::server::SendResponse<Bytes>` - The response half of the stream.
///
pub(crate) async fn h2_request_task(
    server: Arc<Server>,
    request_config: RequestConfig,
    local: SocketAddr,
    remote: SocketAddr,
    request: http::Request<h2::RecvStream>,
    send_response: h2::server::SendResponse<Bytes>,
) {
    let (parts, mut recv_stream): (http::request::Parts, h2::RecvStream) = request.into_parts();
    let uri: &http::Uri = &parts.uri;
    let mut framer: H2Framer = H2Framer::new(send_response);
    let mut body: Vec<u8> = Vec::new();
    if let Err(error) =
        collect_h2_body(&mut recv_stream, &request_config, &parts.headers, &mut body).await
    {
        h2_fail_request(
            &server,
            &mut framer,
            request_config,
            local,
            remote,
            HttpVersion::Http2,
            error,
        )
        .await;
        return;
    }
    let built: Result<Request, RequestError> = build_protocol_request(
        &request_config,
        HttpVersion::Http2,
        parts.method.clone(),
        &request_target(uri.path(), uri.query()),
        uri,
        &parts.headers,
        body,
    );
    let request: Request = match built {
        Ok(request) => request,
        Err(error) => {
            h2_fail_request(
                &server,
                &mut framer,
                request_config,
                local,
                remote,
                HttpVersion::Http2,
                error,
            )
            .await;
            return;
        }
    };
    let is_head: bool = request.get_method().is_head();
    let (stream, peer): (Stream, DuplexStream) =
        build_duplex_transport(request_config, Some(local), Some(remote));
    let lifecycle: ServerLifecycle = ServerLifecycle::new(server, request);
    let joined: Result<((), ()), ResponseError> = futures::try_join!(
        async {
            lifecycle.run(stream).await;
            Ok::<(), ResponseError>(())
        },
        pump_response(&mut framer, peer, RESPONSE_HEAD_MAX, is_head)
    );
    let pump_result: Result<(), ResponseError> = joined.map(|(_, pumped): ((), ())| pumped);
    if pump_result.is_err() {
        framer.reset();
    }
}

/// Answers a request that failed before a handler could run.
///
/// The error reaches the existing request error hooks over the same duplex pipe
/// and the same framing pump a real response takes, so the error handler writes
/// an HTTP/1.1 shaped head that is framed as HTTP/2 on the wire instead of the
/// stream ending silently.
///
/// # Arguments
///
/// - `&Arc<Server>` - The server whose error hooks answer the request.
/// - `&mut H2Framer` - The framer owning the stream's frames.
/// - `RequestConfig` - The parsing limits and read timeout for the connection.
/// - `SocketAddr` - The local address of the carrying connection.
/// - `HttpVersion` - The real protocol version to stamp on the error context.
/// - `RequestError` - The failure that stopped the request.
///
pub(crate) async fn h2_fail_request(
    server: &Arc<Server>,
    framer: &mut H2Framer,
    request_config: RequestConfig,
    local: SocketAddr,
    remote: SocketAddr,
    version: HttpVersion,
    error: RequestError,
) {
    let mut ctx: Context = Context::default();
    let mut request: Request = Request::default();
    request.set_version(version.clone());
    ctx.set_request(request);
    ctx.get_mut_response().reset().set_version(version);
    let (mut stream, peer): (Stream, DuplexStream) =
        build_duplex_transport(request_config, Some(local), Some(remote));
    let is_head: bool = false;
    let server: Arc<Server> = server.clone();
    let (_, pump_result): ((), Result<(), ResponseError>) = join!(
        async move {
            server
                .handle_request_error(&mut stream, &mut ctx, &error)
                .await;
            drop(stream);
        },
        pump_response(framer, peer, RESPONSE_HEAD_MAX, is_head)
    );
    if pump_result.is_err() {
        framer.reset();
    }
}

/// Collects an HTTP/2 request body into a bounded buffer.
///
/// `Ok(None)` from the protocol is end of body and completes collection; only
/// a read that never returns is a timeout. The bound is one deadline for the
/// whole request read rather than a fresh one per chunk, so a client that
/// dribbles one byte at a time cannot hold a stream open forever. A config that
/// asks for the permissive low security timeout applies no deadline at all,
/// matching the HTTP/1.1 reader. Receive capacity is released as each chunk is
/// drained, so a handler that reads slowly cannot stall the window.
///
/// # Arguments
///
/// - `&mut h2::RecvStream` - The receive half of the request stream.
/// - `&RequestConfig` - The parsing limits to enforce.
/// - `&ProtocolHeaders` - The protocol request headers carrying the declared length.
/// - `&mut Vec<u8>` - The buffer to collect the body into.
///
/// # Returns
///
/// - `Result<(), RequestError>` - `Ok(())` once the body is complete, or the
pub(crate) async fn collect_h2_body(
    recv_stream: &mut h2::RecvStream,
    request_config: &RequestConfig,
    headers: &ProtocolHeaders,
    body: &mut Vec<u8>,
) -> Result<(), RequestError> {
    let max_body_size: usize = request_config.get_max_body_size();
    let declared: Option<usize> = declared_content_length(headers, max_body_size)?;
    let read_timeout_ms: u64 = request_config.get_read_timeout_ms();
    if read_timeout_ms == DEFAULT_LOW_SECURITY_READ_TIMEOUT_MS {
        collect_h2_body_until_end(recv_stream, max_body_size, body).await?;
    } else {
        let bound: Duration = Duration::from_millis(read_timeout_ms);
        match timeout(
            bound,
            collect_h2_body_until_end(recv_stream, max_body_size, body),
        )
        .await
        {
            Ok(collected) => collected?,
            Err(_) => return Err(RequestError::ReadTimeout(HttpStatus::RequestTimeout)),
        }
    }
    match declared {
        None => Ok(()),
        Some(declared) => match body.len().cmp(&declared) {
            Ordering::Equal => Ok(()),
            _ => Err(RequestError::Request(CONTENT_LENGTH_MISMATCH.to_owned())),
        },
    }
}

/// Drains an HTTP/2 request stream until end of body or a limit is violated.
///
/// # Arguments
///
/// - `&mut h2::RecvStream` - The receive half of the request stream.
/// - `usize` - The maximum number of body bytes to accept.
/// - `&mut Vec<u8>` - The buffer to collect the body into.
///
/// # Returns
///
/// - `Result<(), RequestError>` - `Ok(())` once the body is complete, or the
async fn collect_h2_body_until_end(
    recv_stream: &mut h2::RecvStream,
    max_body_size: usize,
    body: &mut Vec<u8>,
) -> Result<(), RequestError> {
    loop {
        let received: Option<Result<Bytes, h2::Error>> = recv_stream.data().await;
        let Some(received) = received else {
            return Ok(());
        };
        let data: Bytes =
            received.map_err(|error: h2::Error| RequestError::Request(error.to_string()))?;
        let data_len: usize = data.len();
        let collected: Result<(), RequestError> = push_bounded_body(max_body_size, body, data);
        let _: Result<(), h2::Error> = recv_stream.flow_control().release_capacity(data_len);
        collected?;
    }
}
