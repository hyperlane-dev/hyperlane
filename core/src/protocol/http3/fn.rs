use super::*;

/// Serves one HTTP/3 connection until it closes.
///
/// As with HTTP/2, the connection half owns the settings and control streams for
/// every request on it, so it stays polled while each request runs on its own
/// task.
///
/// # Arguments
///
/// - `ConnectionEnv` - The server, limits and protocol of the connection.
/// - `StreamPeers` - The addresses of the connection.
/// - `h3::server::Connection<h3_quinn::Connection, Bytes>` - The HTTP/3 connection.
///
/// `StreamPeers` travels separately from `ConnectionEnv` because the connection
/// half itself does not read it; only the per-request tasks it spawns do.
pub(crate) async fn serve_h3_connection(
    env: ConnectionEnv,
    peers: StreamPeers,
    mut connection: h3::server::Connection<h3_quinn::Connection, Bytes>,
) {
    loop {
        let resolver: Option<h3::server::RequestResolver<h3_quinn::Connection, Bytes>> =
            match connection.accept().await {
                Ok(resolver) => resolver,
                Err(_) => return,
            };
        let Some(resolver) = resolver else {
            return;
        };
        spawn(h3_request_task(
            H3RequestEnv {
                server: env.server.clone(),
                request_config: env.request_config,
                peers,
            },
            resolver,
        ));
    }
}

/// Runs the framework lifecycle over one HTTP/3 request stream.
///
/// # Arguments
///
/// - `H3RequestEnv` - The server, limits and connection addresses.
/// - `RequestResolverH3` - The resolver for the request stream.
///
pub(crate) async fn h3_request_task(env: H3RequestEnv, resolver: RequestResolverH3) {
    let H3RequestEnv {
        server,
        request_config,
        peers,
    } = env;
    let (head, request_stream): (http::Request<()>, RequestH3Stream) =
        match resolver.resolve_request().await {
            Ok(resolved) => resolved,
            Err(_) => return,
        };
    let (parts, ()): (http::request::Parts, ()) = head.into_parts();
    let mut framer: H3Framer = H3Framer::new(request_stream);
    let mut body: Vec<u8> = Vec::new();
    match collect_h3_body(&mut framer, &request_config, &parts.headers, &mut body).await {
        Ok(()) => {}
        Err(error) => {
            h3_fail_request(
                &H3RequestEnv {
                    server,
                    request_config,
                    peers,
                },
                &mut framer,
                HttpVersion::Http3,
                error,
            )
            .await;
            return;
        }
    }
    let built: Result<Request, RequestError> = build_protocol_request(
        &request_config,
        ProtocolRequestHead {
            version: HttpVersion::Http3,
            method: parts.method.clone(),
            parts,
            body,
        },
    );
    let request: Request = match built {
        Ok(request) => request,
        Err(error) => {
            h3_fail_request(
                &H3RequestEnv {
                    server,
                    request_config,
                    peers,
                },
                &mut framer,
                HttpVersion::Http3,
                error,
            )
            .await;
            return;
        }
    };
    let is_head: bool = request.get_method().is_head();
    let (stream, peer): (Stream, DuplexStream) = build_duplex_transport(request_config, peers);
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

/// Collects an HTTP/3 request body into a bounded buffer.
///
/// One deadline covers the whole request read rather than a fresh one per
/// chunk, so a client that dribbles bytes cannot hold the stream open forever,
/// and a config asking for the permissive low security timeout applies no
/// deadline at all, matching the HTTP/1.1 and HTTP/2 readers.
///
/// # Arguments
///
/// - `&mut H3Framer` - The framer holding the request stream's receive half.
/// - `&RequestConfig` - The parsing limits to enforce.
/// - `&ProtocolHeaders` - The protocol request headers carrying the declared length.
/// - `&mut Vec<u8>` - The buffer to collect the body into.
///
/// # Returns
///
/// - `Result<(), RequestError>` - `Ok(())` once the body is complete and matches
pub(crate) async fn collect_h3_body(
    framer: &mut H3Framer,
    request_config: &RequestConfig,
    headers: &ProtocolHeaders,
    body: &mut Vec<u8>,
) -> Result<(), RequestError> {
    let max_body_size: usize = request_config.get_max_body_size();
    let declared: Option<usize> = declared_content_length(headers, max_body_size)?;
    let read_timeout_ms: u64 = request_config.get_read_timeout_ms();
    if read_timeout_ms == DEFAULT_LOW_SECURITY_READ_TIMEOUT_MS {
        collect_h3_body_until_end(framer, max_body_size, body).await?;
    } else {
        let bound: Duration = Duration::from_millis(read_timeout_ms);
        match timeout(
            bound,
            collect_h3_body_until_end(framer, max_body_size, body),
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

/// Drains an HTTP/3 request stream until end of body or a limit is violated.
///
/// The stream stays owned by the framer and is borrowed across each await, so a
/// deadline that cancels this future cancels only the read: the framer keeps
/// the real QUIC stream and the error hooks can still answer on it.
///
/// # Arguments
///
/// - `&mut H3Framer` - The framer holding the request stream's receive half.
/// - `usize` - The maximum number of body bytes to accept.
/// - `&mut Vec<u8>` - The buffer to collect the body into.
///
/// # Returns
///
/// - `Result<(), RequestError>` - `Ok(())` once the body is complete, or the
async fn collect_h3_body_until_end(
    framer: &mut H3Framer,
    max_body_size: usize,
    body: &mut Vec<u8>,
) -> Result<(), RequestError> {
    loop {
        let Some(request_stream) = framer.get_mut_request_stream().as_mut() else {
            return Err(RequestError::Request(CONTENT_LENGTH_MISMATCH.to_owned()));
        };
        let received: Result<Option<_>, h3::error::StreamError> = request_stream.recv_data().await;
        match received {
            Ok(Some(mut data)) => {
                let remaining: usize = bytes::Buf::remaining(&data);
                let chunk: Bytes = bytes::Buf::copy_to_bytes(&mut data, remaining);
                let collected: Result<(), RequestError> =
                    push_bounded_body(max_body_size, body, chunk);
                collected?;
            }
            Ok(None) => return Ok(()),
            Err(error) => return Err(RequestError::Request(error.to_string())),
        }
    }
}

/// Answers a request that failed before a handler could run.
///
/// The error reaches the existing request error hooks over the same duplex pipe
/// and the same framing pump a real response takes, so the error handler writes
/// an HTTP/1.1 shaped head that is framed as HTTP/3 on the wire instead of the
/// stream ending silently.
///
/// Borrows the request environment while framing the error response.
///
/// # Arguments
///
/// - `&H3RequestEnv` - The server, limits and connection addresses.
/// - `&mut H3Framer` - The framer owning the stream's frames.
/// - `HttpVersion` - The real protocol version to stamp on the error context.
/// - `RequestError` - The failure that stopped the request.
///
pub(crate) async fn h3_fail_request(
    env: &H3RequestEnv,
    framer: &mut H3Framer,
    version: HttpVersion,
    error: RequestError,
) {
    let mut ctx: Context = Context::default();
    let mut request: Request = Request::default();
    request.set_version(version.clone());
    ctx.set_request(request);
    ctx.get_mut_response().reset().set_version(version);
    let (mut stream, peer): (Stream, DuplexStream) =
        build_duplex_transport(env.request_config, env.peers);
    let is_head: bool = false;
    let server: Arc<Server> = env.server.clone();
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
