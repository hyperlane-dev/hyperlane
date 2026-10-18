use super::*;

/// Accepts TCP connections and serves each one with the configured protocol.
///
/// HTTP/1.1 and HTTP/2 share this listener. When TLS is configured the
/// connection is configured at the socket level first, then wrapped and
/// handshaken with a bound so a client that opens a connection and never
/// completes the handshake cannot stall the loop. HTTP/2 additionally requires
/// its own preface or upgrade, and an HTTP/1.1 selection serves the connection
/// directly.
///
/// Shutdown is not an abort from outside. The loop watches the shutdown signal
/// itself, and on a signal it drops the listener before it returns, so the port
/// is already released by the time the loop publishes its stopped state and a
/// `wait` that follows is observably after the release.
///
/// # Arguments
///
/// - `Arc<Server>` - The server whose pipeline serves each request.
/// - `RequestConfig` - The parsing limits and read timeouts for connections.
/// - `ServerProtocol` - The protocol to serve on accepted connections.
/// - `Option<Arc<rustls::ServerConfig>>` - The TLS configuration, when present.
/// - `TcpListener` - The bound listener to accept connections from.
/// - `Receiver<()>` - The shutdown signal the loop selects on.
/// - `Sender<bool>` - Publishes whether the loop is still serving.
///
/// # Returns
///
/// - `()` - Resolves once the accept loop stops.
pub(crate) async fn tcp_accept(
    server: Arc<Server>,
    request_config: RequestConfig,
    protocol: ServerProtocol,
    stream_tls: Option<Arc<rustls::ServerConfig>>,
    tcp_listener: TcpListener,
    mut shutdown_receiver: Receiver<()>,
    running_sender: Sender<bool>,
) {
    loop {
        let accepted: Option<(TcpStream, SocketAddr)> = tokio::select! {
            biased;
            _ = shutdown_receiver.changed() => None,
            accepted = tcp_listener.accept() => accepted.ok(),
        };
        let Some((stream, remote)) = accepted else {
            break;
        };
        let server: Arc<Server> = server.clone();
        let tls: Option<Arc<rustls::ServerConfig>> = stream_tls.clone();
        spawn(async move {
            serve_tcp_connection(server, request_config, protocol, tls, stream, remote).await;
        });
    }
    drop(tcp_listener);
    let _: Result<(), tokio::sync::watch::error::SendError<bool>> =
        running_sender.send(SERVER_STOPPED);
}

/// Serves one accepted TCP connection.
///
/// # Arguments
///
/// - `Arc<Server>` - The server whose pipeline serves the connection.
/// - `RequestConfig` - The parsing limits and read timeouts for the connection.
/// - `ServerProtocol` - The protocol to serve on this connection.
/// - `Option<Arc<rustls::ServerConfig>>` - The TLS configuration, when present.
/// - `TcpStream` - The accepted socket, before its socket options are applied.
/// - `SocketAddr` - The remote address of the accepted socket.
///
/// # Returns
///
/// - `()` - Resolves once the connection is finished.
pub(crate) async fn serve_tcp_connection(
    server: Arc<Server>,
    request_config: RequestConfig,
    protocol: ServerProtocol,
    stream_tls: Option<Arc<rustls::ServerConfig>>,
    stream: TcpStream,
    remote: SocketAddr,
) {
    let local: Option<SocketAddr> = stream.local_addr().ok();
    configure_socket(&stream, &server);
    let Some(tls) = stream_tls else {
        serve_plain_tcp(server, request_config, protocol, stream, local, remote).await;
        return;
    };
    let Ok(handshake) =
        bounded_handshake(request_config, TlsAcceptor::from(tls).accept(stream)).await
    else {
        return;
    };
    let Ok(handshake) = handshake else {
        return;
    };
    if !negotiated_protocol_matches(&handshake, &protocol) {
        return;
    }
    serve_tls_stream(
        server,
        request_config,
        protocol,
        TlsTransport::new(handshake),
        local,
        remote,
    )
    .await;
}

/// Reports whether a negotiated TLS session speaks the protocol being served.
///
/// Advertising the ALPN identifier is only half the contract: a client may
/// complete the handshake without offering any protocol at all, and treating
/// that as a successful negotiation would answer an HTTP/2 request under
/// HTTP/1.1 rules. HTTP/2 therefore requires the negotiated identifier to be
/// its own. HTTP/1.1 keeps accepting a session with no ALPN, because clients
/// that predate ALPN negotiate nothing and must still be served.
///
/// # Arguments
///
/// - `&tokio_rustls::server::TlsStream<TcpStream>` - The negotiated session.
/// - `&ServerProtocol` - The protocol this connection is being served as.
///
/// # Returns
///
/// - `bool` - `true` when the session may be served under this protocol.
pub(crate) fn negotiated_protocol_matches(
    session: &tokio_rustls::server::TlsStream<TcpStream>,
    protocol: &ServerProtocol,
) -> bool {
    let (_, connection): (&TcpStream, &rustls::ServerConnection) = session.get_ref();
    match connection.alpn_protocol() {
        Some(negotiated) => negotiated == protocol.get_alpn_protocol(),
        None => protocol != &ServerProtocol::Http2,
    }
}

/// Reports whether a QUIC connection negotiated HTTP/3.
///
/// QUIC requires TLS 1.3 and carries the handshake inside its own transport,
/// so the negotiated identifier is the only statement of intent a client can
/// make. A connection that negotiated nothing, or something other than `h3`,
/// is closed rather than served: it is a client aimed at a different protocol,
/// and answering it under HTTP/3 rules would produce a response it cannot read.
///
/// # Arguments
///
/// - `&quinn::Connection` - The established QUIC connection.
///
/// # Returns
///
/// - `bool` - `true` when the connection negotiated the `h3` identifier.
pub(crate) fn negotiated_quic_protocol_matches(connection: &quinn::Connection) -> bool {
    let Some(handshake) = connection.handshake_data() else {
        return false;
    };
    match handshake.downcast::<quinn::crypto::rustls::HandshakeData>() {
        Ok(data) => data.protocol.as_deref() == Some(ServerProtocol::Http3.get_alpn_protocol()),
        Err(_) => false,
    }
}

/// Bounds a protocol handshake with the configured read timeout.
///
/// The bound covers the whole handshake rather than each byte of it: a client
/// that trickles one byte at a time must not be able to hold a connection open
/// forever by staying under the timeout on every individual read.
///
/// # Arguments
///
/// - `RequestConfig` - The parsing limits and read timeouts for the connection.
/// - `F` - The handshake future to bound.
///
/// # Returns
///
/// - `Result<T, tokio::time::error::Elapsed>` - The handshake result, or
///   the elapsed that stopped it.
pub(crate) async fn bounded_handshake<F, T>(
    request_config: RequestConfig,
    handshake: F,
) -> Result<T, tokio::time::error::Elapsed>
where
    F: Future<Output = T>,
{
    let read_timeout_ms: u64 = request_config.get_read_timeout_ms();
    if read_timeout_ms == NO_TIMEOUT_MS {
        return Ok(handshake.await);
    }
    timeout(Duration::from_millis(read_timeout_ms), handshake).await
}

/// Applies the configured socket options before any TLS handshake starts.
///
/// # Arguments
///
/// - `&TcpStream` - The accepted socket to configure.
/// - `&Arc<Server>` - The server holding the socket options to apply.
///
/// # Returns
///
/// - `()` - Resolves once the options are applied.
pub(crate) fn configure_socket(stream: &TcpStream, server: &Arc<Server>) {
    let server_config: &ServerConfig = server.get_server_config();
    if let Some(nodelay) = server_config.get_nodelay() {
        let _: Result<(), io::Error> = stream.set_nodelay(nodelay);
    }
    if let Some(ttl) = server_config.get_ttl() {
        let _: Result<(), io::Error> = stream.set_ttl(ttl);
    }
}

/// Serves a plaintext connection, selecting the protocol it speaks.
///
/// # Arguments
///
/// - `Arc<Server>` - The server whose pipeline serves the connection.
/// - `RequestConfig` - The parsing limits and read timeouts for the connection.
/// - `ServerProtocol` - The protocol to serve on this connection.
/// - `TcpStream` - The accepted socket.
/// - `Option<SocketAddr>` - The local address of the connection.
/// - `SocketAddr` - The remote address of the connection.
///
/// # Returns
///
/// - `()` - Resolves once the connection is finished.
pub(crate) async fn serve_plain_tcp(
    server: Arc<Server>,
    request_config: RequestConfig,
    protocol: ServerProtocol,
    stream: TcpStream,
    local: Option<SocketAddr>,
    remote: SocketAddr,
) {
    match protocol {
        ServerProtocol::Http2 => {
            serve_h2_connection(
                server,
                request_config,
                local.unwrap_or(remote),
                remote,
                bounded_handshake(request_config, h2::server::handshake(stream))
                    .await
                    .map_err(|_: tokio::time::error::Elapsed| H2_HANDSHAKE_TIMEOUT.to_owned())
                    .and_then(|connection: Result<_, h2::Error>| {
                        connection.map_err(|error: h2::Error| error.to_string())
                    }),
            )
            .await;
        }
        _ => {
            serve_h1_connection(server, request_config, stream).await;
        }
    }
}

/// Serves a connection whose transport is already negotiated.
///
/// HTTP/2 is served by handing the session to its own connection loop, which
/// keeps its own control flow; HTTP/1.1 is served by the existing keep alive
/// pipeline over a `Stream` wrapping the session.
///
/// # Arguments
///
/// - `Arc<Server>` - The server whose pipeline serves the connection.
/// - `RequestConfig` - The parsing limits and read timeouts for the connection.
/// - `ServerProtocol` - The protocol to serve on this connection.
/// - `TlsTransport` - The negotiated session carrying the connection.
/// - `Option<SocketAddr>` - The local address of the connection.
/// - `SocketAddr` - The remote address of the connection.
///
/// # Returns
///
/// - `()` - Resolves once the connection is finished.
pub(crate) async fn serve_tls_stream(
    server: Arc<Server>,
    request_config: RequestConfig,
    protocol: ServerProtocol,
    transport: TlsTransport,
    local: Option<SocketAddr>,
    remote: SocketAddr,
) {
    match protocol {
        ServerProtocol::Http2 => {
            let session: TlsTransport = transport;
            serve_h2_connection(
                server,
                request_config,
                local.unwrap_or(remote),
                remote,
                bounded_handshake(request_config, h2::server::handshake(session))
                    .await
                    .map_err(|_: tokio::time::error::Elapsed| H2_HANDSHAKE_TIMEOUT.to_owned())
                    .and_then(|connection: Result<_, h2::Error>| {
                        connection.map_err(|error: h2::Error| error.to_string())
                    }),
            )
            .await;
        }
        _ => {
            serve_h1_transport(server, request_config, transport).await;
        }
    }
}

/// Accepts QUIC connections and serves each one with HTTP/3.
///
/// Shutdown here is not an abort either, and it cannot be one alone: dropping
/// the loop would leave the UDP socket bound and every established connection
/// running. The loop therefore closes the endpoint first, which tells peers the
/// server is going away and makes the accept future resolve, then waits for the
/// connections to drain before publishing the stopped state and dropping the
/// socket.
///
/// # Arguments
///
/// - `Arc<Server>` - The server whose pipeline serves each request.
/// - `RequestConfig` - The parsing limits and read timeouts for connections.
/// - `Endpoint` - The bound QUIC endpoint to accept connections from.
/// - `Receiver<()>` - The shutdown signal the loop selects on.
/// - `Sender<bool>` - Publishes whether the loop is still serving.
///
/// # Returns
///
/// - `Result<(), Box<ServerError>>` - `Ok(())` once the endpoint has been closed and
///   drained, or the failure that stopped the loop early.
pub(crate) async fn quic_accept(
    server: Arc<Server>,
    request_config: RequestConfig,
    endpoint: Endpoint,
    mut shutdown_receiver: Receiver<()>,
    running_sender: Sender<bool>,
) -> Result<(), Box<ServerError>> {
    let local: SocketAddr = endpoint
        .local_addr()
        .map_err(|error: io::Error| ServerError::from(error))?;
    loop {
        let connecting: Option<Incoming> = tokio::select! {
            biased;
            _ = shutdown_receiver.changed() => None,
            accepted = endpoint.accept() => accepted,
        };
        let Some(connecting) = connecting else {
            break;
        };
        let remote: SocketAddr = connecting.remote_address();
        let server: Arc<Server> = server.clone();
        spawn(async move {
            let Ok(connection) = connecting.await else {
                return;
            };
            if !negotiated_quic_protocol_matches(&connection) {
                connection.close(VarInt::from_u32(QUIC_CLOSE_CODE), QUIC_CLOSE_REASON);
                return;
            }
            let Ok(h3_connection) = h3::server::Connection::<h3_quinn::Connection, Bytes>::new(
                h3_quinn::Connection::new(connection),
            )
            .await
            else {
                return;
            };
            serve_h3_connection(server, request_config, local, remote, h3_connection).await;
        });
    }
    let close_code: VarInt = VarInt::from_u32(QUIC_CLOSE_CODE);
    endpoint.close(close_code, QUIC_CLOSE_REASON);
    endpoint.wait_idle().await;
    drop(endpoint);
    let _: Result<(), tokio::sync::watch::error::SendError<bool>> =
        running_sender.send(SERVER_STOPPED);
    Ok(())
}

/// Serves one plaintext HTTP/1.1 connection through the existing pipeline.
///
/// # Arguments
///
/// - `Arc<Server>` - The server whose pipeline serves the connection.
/// - `RequestConfig` - The parsing limits and read timeouts for the connection.
/// - `TcpStream` - The accepted socket.
///
/// # Returns
///
/// - `()` - Resolves once the connection is finished.
pub(crate) async fn serve_h1_connection(
    server: Arc<Server>,
    request_config: RequestConfig,
    stream: TcpStream,
) {
    let mut stream: Stream = Stream::new(stream, request_config, false);
    serve_h1_stream(server, &mut stream).await;
}

/// Serves one HTTP/1.1 connection whose transport is already negotiated.
///
/// # Arguments
///
/// - `Arc<Server>` - The server whose pipeline serves the connection.
/// - `RequestConfig` - The parsing limits and read timeouts for the connection.
/// - `TlsTransport` - The negotiated session carrying the connection.
///
/// # Returns
///
/// - `()` - Resolves once the connection is finished.
///
/// A TLS session is not closed by dropping it: rustls treats an abrupt drop as
/// `UnexpectedEof`, which a well behaved peer reports as a truncated
/// connection rather than an orderly shutdown. The write half is therefore shut
/// down explicitly so the close notify alert reaches the peer. A finite read
/// timeout bounds shutdown so a peer that stops reading cannot hold the
/// connection open forever; the no-timeout sentinel disables that bound.
pub(crate) async fn serve_h1_transport(
    server: Arc<Server>,
    request_config: RequestConfig,
    transport: TlsTransport,
) {
    let mut stream: Stream = Stream::from_transport(transport, request_config, false);
    serve_h1_stream(server, &mut stream).await;
    let closed: Result<Result<(), io::Error>, tokio::time::error::Elapsed> =
        bounded_handshake(request_config, stream.get_mut_stream().shutdown()).await;
    match closed {
        Ok(Ok(())) => {}
        Ok(Err(_)) => {}
        Err(_) => {}
    }
}

/// Runs the keep alive request loop of one HTTP/1.1 connection.
///
/// # Arguments
///
/// - `Arc<Server>` - The server whose pipeline serves the connection.
/// - `&mut Stream` - The stream carrying the connection.
///
/// # Returns
///
/// - `()` - Resolves once the connection is finished.
pub(crate) async fn serve_h1_stream(server: Arc<Server>, stream: &mut Stream) {
    server.serve_h1_connection_stream(stream).await;
}

/// Runs one connection future and reports the panic payload if it unwinds.
///
/// The accept loop must survive a handler that panics, and the panic hooks must
/// see the same data a join error would have carried, so the connection future
/// is unwound here rather than being spawned and forgotten.
///
/// # Arguments
///
/// - `F` - The connection future to run.
///
/// # Returns
///
/// - `Option<PanicData>` - `None` when the connection finished normally.
pub(crate) async fn catch_connection_panic<F>(connection: F) -> Option<PanicData>
where
    F: Future<Output = ()>,
{
    match AssertUnwindSafe(connection).catch_unwind().await {
        Ok(()) => None,
        Err(payload) => Some(panic_data_from_payload(payload)),
    }
}

/// Converts an unwound panic payload into the framework panic representation.
///
/// # Arguments
///
/// - `Box<dyn Any + Send>` - The payload the unwind produced.
///
/// # Returns
///
/// - `PanicData` - The panic message and payload, if either could be read.
pub(crate) fn panic_data_from_payload(payload: Box<dyn Any + Send>) -> PanicData {
    match payload.downcast::<String>() {
        Ok(message) => PanicData::new(Some(*message), None, None),
        Err(payload) => match payload.downcast::<&'static str>() {
            Ok(message) => PanicData::new(Some((*message).to_owned()), None, None),
            Err(_) => PanicData::new(None, None, None),
        },
    }
}
