use super::*;

/// Serves one HTTP/2 connection until it closes.
///
/// The connection half stays polled for the whole lifetime of the connection:
/// it owns flow control and the settings exchange for every stream, so parking
/// it while a handler runs would stall the other requests multiplexed onto the
/// same connection. Independent request futures are polled alongside the
/// connection driver without allocating a separate Tokio task for each stream.
///
/// # Arguments
///
/// - `ConnectionEnv` - The server, limits and protocol of the connection.
/// - `StreamPeers` - The addresses of the connection.
/// - `Result<ConnectionH2<T>, String>` - The negotiated HTTP/2 connection, or
///
/// `StreamPeers` travels separately from `ConnectionEnv` because the connection
/// half itself does not read it; only the per-request tasks it spawns do.
pub(crate) async fn serve_h2_connection<T>(
    env: ConnectionEnv,
    peers: StreamPeers,
    connection: Result<ConnectionH2<T>, String>,
) where
    T: AsyncRead + AsyncWrite + Unpin + Send + Sync + 'static,
{
    let mut connection: ConnectionH2<T> = match connection {
        Ok(connection) => connection,
        Err(_) => return,
    };
    let mut requests: FuturesUnordered<_> = FuturesUnordered::new();
    poll_fn(|cx: &mut task::Context<'_>| -> Poll<()> {
        loop {
            let mut progressed: bool = false;
            let mut accepted_batch: usize = 0;
            while accepted_batch < 64 {
                match connection.poll_accept(cx) {
                    Poll::Ready(Some(Ok((request, send_response)))) => {
                        requests.push(
                            AssertUnwindSafe(h2_request_task(
                                H2RequestEnv {
                                    shared: Arc::clone(&env.shared),
                                    peers,
                                },
                                request,
                                send_response,
                            ))
                            .catch_unwind(),
                        );
                        accepted_batch += 1;
                        progressed = true;
                    }
                    Poll::Ready(Some(Err(_))) | Poll::Ready(None) => return Poll::Ready(()),
                    Poll::Pending => break,
                }
            }
            let mut completed_batch: usize = 0;
            while !requests.is_empty() && completed_batch < 64 {
                match requests.poll_next_unpin(cx) {
                    Poll::Ready(Some(_)) => {
                        completed_batch += 1;
                        progressed = true;
                    }
                    Poll::Ready(None) | Poll::Pending => break,
                }
            }
            if !progressed {
                return Poll::Pending;
            }
            if accepted_batch == 64 || completed_batch == 64 {
                cx.waker().wake_by_ref();
                return Poll::Pending;
            }
        }
    })
    .await;
}

/// Runs the framework lifecycle over one HTTP/2 request stream.
///
/// Writes from the handler and middleware are framed directly onto the HTTP/2
/// stream through [`H2DirectTransport`], avoiding duplex pipe allocation,
/// cross-task scheduling, and intermediate response head cloning.
///
/// # Arguments
///
/// - `H2RequestEnv` - The server, limits and connection addresses.
/// - `ProtocolRequest<H2RecvStream>` - The decoded protocol request.
/// - `H2SendResponse` - The response half of the stream.
pub(crate) async fn h2_request_task(
    env: H2RequestEnv,
    request: ProtocolRequest<H2RecvStream>,
    mut send_response: H2SendResponse,
) {
    let H2RequestEnv { shared, peers } = env;
    let server: &Server = &shared.server;
    let request_config: RequestConfig = shared.request_config;
    let (parts, mut recv_stream): (Parts, H2RecvStream) = request.into_parts();
    let mut ctx: Context = take_pooled_context();
    let mut body: Vec<u8> = mem::take(ctx.get_mut_request().get_mut_body());
    body.clear();
    if let Err(error) = collect_h2_body(
        &mut send_response,
        &mut recv_stream,
        &request_config,
        &parts.headers,
        &mut body,
    )
    .await
    {
        *ctx.get_mut_request().get_mut_body() = body;
        h2_fail_request(
            server,
            request_config,
            peers,
            send_response,
            &mut ctx,
            HttpVersion::Http2,
            error,
        )
        .await;
        return_pooled_context(ctx);
        return;
    }
    let mut target: String = take_target_buffer();
    let method: ProtocolMethod = parts.method.clone();
    let built: Result<(), RequestError> = fill_protocol_request(
        &request_config,
        ProtocolRequestHead {
            version: HttpVersion::Http2,
            method,
            parts,
            body,
        },
        ctx.get_mut_request(),
        &mut target,
    );
    if let Err(error) = built {
        return_target_buffer(target);
        h2_fail_request(
            server,
            request_config,
            peers,
            send_response,
            &mut ctx,
            HttpVersion::Http2,
            error,
        )
        .await;
        return_pooled_context(ctx);
        return;
    }
    let is_head: bool = ctx.get_request().get_method().is_head();
    let mut stream: Stream = Stream::from_transport(
        H2DirectTransport::new(send_response, peers, is_head),
        request_config,
        false,
    );
    server
        .run_protocol_lifecycle_in_place(&shared.pipeline, &mut stream, &mut ctx, target)
        .await;
    let _: Result<(), io::Error> = stream.get_mut_stream().shutdown().await;
    return_pooled_context(ctx);
}

/// Answers a request that failed before a handler could run.
///
/// # Arguments
///
/// - `&Server` - The server owning the request error hooks.
/// - `RequestConfig` - The parsing limits and read timeout.
/// - `StreamPeers` - The connection's socket addresses.
/// - `H2SendResponse` - The response half of the stream.
/// - `&mut Context` - The context passed to error hooks.
/// - `HttpVersion` - The real protocol version to stamp on the error context.
/// - `RequestError` - The failure that stopped the request.
#[cold]
pub(crate) async fn h2_fail_request(
    server: &Server,
    request_config: RequestConfig,
    peers: StreamPeers,
    send_response: H2SendResponse,
    ctx: &mut Context,
    version: HttpVersion,
    error: RequestError,
) {
    ctx.get_mut_request().reset().set_version(version.clone());
    ctx.get_mut_response().reset().set_version(version.clone());
    ctx.clear_route_params();
    ctx.reset_request_attributes();
    let mut stream: Stream = Stream::from_transport(
        H2DirectTransport::new(send_response, peers, false),
        request_config,
        false,
    );
    reply_to_request_error(server, &mut stream, ctx, &error, Some(version)).await;
    let _: Result<(), io::Error> = stream.get_mut_stream().shutdown().await;
}

/// Collects an HTTP/2 request body into a bounded buffer.
///
/// # Arguments
///
/// - `&mut H2SendResponse` - The response sender for an optional interim response.
/// - `&mut H2RecvStream` - The receive half of the request stream.
/// - `&RequestConfig` - The parsing limits to enforce.
/// - `&ProtocolHeaders` - The protocol request headers carrying the declared length.
/// - `&mut Vec<u8>` - The buffer to collect the body into.
///
/// # Returns
///
/// - `Result<(), RequestError>` - `Ok(())` once the body is complete, or the failure.
async fn collect_h2_body(
    send_response: &mut H2SendResponse,
    recv_stream: &mut H2RecvStream,
    request_config: &RequestConfig,
    headers: &ProtocolHeaders,
    body: &mut Vec<u8>,
) -> Result<(), RequestError> {
    let max_body_size: usize = request_config.get_max_body_size();
    let declared: Option<usize> = declared_content_length(headers, max_body_size)?;
    if recv_stream.is_end_stream() {
        return match declared {
            None | Some(0) => Ok(()),
            Some(_) => Err(RequestError::Request(CONTENT_LENGTH_MISMATCH.to_owned())),
        };
    }
    if let Some(expected) = declared
        && expected > 0
    {
        body.reserve(expected.min(MAX_POOLED_BUFFER_CAPACITY));
    }
    if declared != Some(0) && headers.contains_key(EXPECT) && has_continue_expectation(headers) {
        send_response
            .send_informational(continue_response())
            .map_err(|error: H2Error| RequestError::Request(error.to_string()))?;
    }
    let read_timeout_ms: u64 = request_config.get_read_timeout_ms();
    if read_timeout_ms == DEFAULT_LOW_SECURITY_READ_TIMEOUT_MS {
        collect_h2_body_until_end(recv_stream, max_body_size, body).await?;
    } else {
        let mut reading: Pin<&mut _> =
            pin!(collect_h2_body_until_end(recv_stream, max_body_size, body));
        if let Some(collected) = poll_once_protocol(reading.as_mut()).await {
            collected?;
        } else {
            let bound: Duration = Duration::from_millis(read_timeout_ms);
            match timeout(bound, reading).await {
                Ok(collected) => collected?,
                Err(_) => return Err(RequestError::ReadTimeout(HttpStatus::RequestTimeout)),
            }
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
/// - `&mut H2RecvStream` - The receive half of the request stream.
/// - `usize` - The maximum number of body bytes to accept.
/// - `&mut Vec<u8>` - The buffer to collect the body into.
///
/// # Returns
///
/// - `Result<(), RequestError>` - `Ok(())` once the body is complete, or the failure.
async fn collect_h2_body_until_end(
    recv_stream: &mut H2RecvStream,
    max_body_size: usize,
    body: &mut Vec<u8>,
) -> Result<(), RequestError> {
    loop {
        let received: Option<Result<Bytes, H2Error>> = recv_stream.data().await;
        let Some(received) = received else {
            return Ok(());
        };
        let data: Bytes =
            received.map_err(|error: H2Error| RequestError::Request(error.to_string()))?;
        let data_len: usize = data.len();
        let collected: Result<(), RequestError> = push_bounded_body(max_body_size, body, data);
        let _: Result<(), H2Error> = recv_stream.flow_control().release_capacity(data_len);
        collected?;
    }
}
