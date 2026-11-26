use super::*;

/// Binds one exclusive UDP socket and constructs one or more CID-steered QUIC endpoints.
///
/// # Arguments
///
/// - `Arc<QuicServerConfig>` - The QUIC TLS configuration.
/// - `SocketAddr` - The UDP bind address.
///
/// # Returns
///
/// - `Result<QuicEndpointSet, Box<ServerError>>` - Exclusive QUIC endpoint set.
pub(crate) fn bind_quic_endpoints(
    quic_config: Arc<QuicServerConfig>,
    bind_addr: SocketAddr,
) -> Result<QuicEndpointSet, Box<ServerError>> {
    create_quic_endpoint_set(quic_config, bind_addr)
}

/// Binds one or more TCP listeners with a deep accept backlog and exclusive port ownership.
///
/// # Arguments
///
/// - `&str` - The configured bind address string.
/// - `bool` - Whether to enable `TCP_NODELAY` on the listener socket.
///
/// # Returns
///
/// - `Result<Vec<TcpListener>, Box<ServerError>>` - Bound Tokio TCP listeners.
pub(crate) async fn bind_tcp_listeners(
    bind_address: &str,
    nodelay: bool,
) -> Result<Vec<TcpListener>, Box<ServerError>> {
    let Ok(bind_addr) = bind_address.parse::<SocketAddr>() else {
        let listener: TcpListener = TcpListener::bind(bind_address)
            .await
            .map_err(|error: io::Error| Box::new(ServerError::from(error)))?;
        SockRef::from(&listener)
            .set_tcp_nodelay(nodelay)
            .map_err(|error: io::Error| Box::new(ServerError::from(error)))?;
        return Ok(vec![listener]);
    };
    let shard_count: usize = 1;
    let mut listeners: Vec<TcpListener> = Vec::with_capacity(shard_count);
    for _ in 0..shard_count {
        let socket: Socket = Socket::new(Domain::for_address(bind_addr), Type::STREAM, None)
            .map_err(|error: io::Error| Box::new(ServerError::from(error)))?;
        #[cfg(unix)]
        if bind_addr.port() != 0 {
            socket
                .set_reuse_address(true)
                .map_err(|error: io::Error| Box::new(ServerError::from(error)))?;
        }
        let _: Result<(), io::Error> = socket.set_tcp_nodelay(nodelay);
        socket
            .set_nonblocking(true)
            .map_err(|error: io::Error| Box::new(ServerError::from(error)))?;
        socket
            .bind(&bind_addr.into())
            .map_err(|error: io::Error| Box::new(ServerError::from(error)))?;
        socket
            .listen(TCP_LISTEN_BACKLOG)
            .map_err(|error: io::Error| Box::new(ServerError::from(error)))?;
        let std_listener: StdTcpListener = socket.into();
        let listener: TcpListener = TcpListener::from_std(std_listener)
            .map_err(|error: io::Error| Box::new(ServerError::from(error)))?;
        listeners.push(listener);
    }
    Ok(listeners)
}

/// Runs one TCP accept loop on a shared exclusive listener until the shutdown signal fires.
///
/// # Arguments
///
/// - `ConnectionEnv` - Shared server state and protocol configuration.
/// - `Arc<TcpListener>` - Exclusively bound TCP listener shared across accept workers.
/// - `Receiver<()>` - Shutdown signal.
async fn tcp_accept_loop(
    env: ConnectionEnv,
    tcp_listener: Arc<TcpListener>,
    mut shutdown_receiver: Receiver<()>,
) {
    let local: Option<SocketAddr> = tcp_listener.local_addr().ok();
    loop {
        let accepted: Option<io::Result<(TcpStream, SocketAddr)>> = tokio::select! {
            biased;
            _ = shutdown_receiver.changed() => None,
            accepted = tcp_listener.accept() => Some(accepted),
        };
        let Some(result) = accepted else {
            break;
        };
        let (stream, remote): (TcpStream, SocketAddr) = match result {
            Ok(pair) => pair,
            Err(error) => {
                if !matches!(
                    error.kind(),
                    ErrorKind::ConnectionAborted
                        | ErrorKind::ConnectionReset
                        | ErrorKind::Interrupted
                        | ErrorKind::WouldBlock
                ) {
                    sleep(Duration::from_millis(TCP_ACCEPT_BACKOFF_MS)).await;
                }
                continue;
            }
        };
        let env: ConnectionEnv = env.clone();
        let conn_shutdown: Receiver<()> = shutdown_receiver.clone();
        spawn(async move {
            serve_tcp_connection(env, stream, local, remote, conn_shutdown).await;
        });
    }
    drop(tcp_listener);
}

/// Runs one QUIC accept shard until shutdown, then closes and drains its endpoint.
///
/// # Arguments
///
/// - `ConnectionEnv` - Shared server state and protocol configuration.
/// - `Endpoint` - Bound QUIC endpoint shard.
/// - `Receiver<()>` - Shutdown signal.
///
/// # Returns
///
/// - `Result<(), Box<ServerError>>` - `Ok(())` once drained.
async fn quic_accept_loop(
    env: ConnectionEnv,
    endpoint: Endpoint,
    mut shutdown_receiver: Receiver<()>,
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
        let env: ConnectionEnv = env.clone();
        spawn(async move {
            let Ok(Ok(connection)) =
                bounded_handshake(env.shared.request_config, async move { connecting.await }).await
            else {
                return;
            };
            if !negotiated_quic_protocol_matches(&connection) {
                connection.close(VarInt::from_u32(QUIC_CLOSE_CODE), QUIC_CLOSE_REASON);
                return;
            }
            serve_h3_connection(env, StreamPeers { local, remote }, connection).await;
        });
    }
    let close_code: VarInt = VarInt::from_u32(QUIC_CLOSE_CODE);
    endpoint.close(close_code, QUIC_CLOSE_REASON);
    endpoint.wait_idle().await;
    drop(endpoint);
    Ok(())
}

/// Accepts TCP connections across all bound listener shards and publishes stopped state once every shard exits.
///
/// # Arguments
///
/// - `ConnectionEnv` - Shared server state and protocol configuration.
/// - `Vec<TcpListener>` - All bound listener shards.
/// - `Receiver<()>` - Shutdown signal.
/// - `Sender<bool>` - Running-state publisher.
pub(crate) async fn tcp_accept_all(
    env: ConnectionEnv,
    mut listeners: Vec<TcpListener>,
    shutdown_receiver: Receiver<()>,
    running_sender: Sender<bool>,
) {
    #[cfg(target_os = "linux")]
    {
        match try_serve_uring_tcp(
            env.clone(),
            listeners,
            shutdown_receiver.clone(),
            running_sender.clone(),
        )
        .await
        {
            Ok(()) => return,
            Err(remaining) => listeners = remaining,
        }
    }
    let workers: usize = Handle::current().metrics().num_workers();
    if listeners.len() == 1 && workers <= 1 {
        let listener: Arc<TcpListener> = Arc::new(listeners.remove(0));
        tcp_accept_loop(env, listener, shutdown_receiver).await;
    } else if listeners.len() == 1 {
        let listener: Arc<TcpListener> = Arc::new(listeners.remove(0));
        let task_count: usize = workers.clamp(1, 8);
        let mut handles: Vec<JoinHandle<()>> = Vec::with_capacity(task_count);
        for _ in 0..task_count {
            let shard_env: ConnectionEnv = env.clone();
            let shard_listener: Arc<TcpListener> = Arc::clone(&listener);
            let shard_shutdown: Receiver<()> = shutdown_receiver.clone();
            handles.push(spawn(async move {
                tcp_accept_loop(shard_env, shard_listener, shard_shutdown).await;
            }));
        }
        drop(listener);
        for handle in handles {
            let _: Result<(), JoinError> = handle.await;
        }
    } else {
        let mut handles: Vec<JoinHandle<()>> = Vec::with_capacity(listeners.len());
        for listener in listeners {
            let shard_env: ConnectionEnv = env.clone();
            let shard_listener: Arc<TcpListener> = Arc::new(listener);
            let shard_shutdown: Receiver<()> = shutdown_receiver.clone();
            handles.push(spawn(async move {
                tcp_accept_loop(shard_env, shard_listener, shard_shutdown).await;
            }));
        }
        for handle in handles {
            let _: Result<(), JoinError> = handle.await;
        }
    }
    let _: Result<(), SendError<bool>> = running_sender.send(SERVER_STOPPED);
}

/// Accepts QUIC connections across all bound endpoints and stops the UDP dispatcher before publishing stopped state.
///
/// # Arguments
///
/// - `ConnectionEnv` - Shared server state and protocol configuration.
/// - `QuicEndpointSet` - All bound QUIC endpoints and optional exclusive-socket dispatcher.
/// - `Receiver<()>` - Shutdown signal.
/// - `Sender<bool>` - Running-state publisher.
pub(crate) async fn quic_accept_all(
    env: ConnectionEnv,
    endpoint_set: QuicEndpointSet,
    shutdown_receiver: Receiver<()>,
    running_sender: Sender<bool>,
) {
    let QuicEndpointSet {
        mut endpoints,
        dispatcher,
        dispatcher_stop,
    } = endpoint_set;
    if endpoints.len() == 1 {
        let endpoint: Endpoint = endpoints.remove(0);
        let _: Result<(), Box<ServerError>> =
            quic_accept_loop(env, endpoint, shutdown_receiver).await;
    } else {
        let mut handles: Vec<JoinHandle<()>> = Vec::with_capacity(endpoints.len());
        for endpoint in endpoints {
            let shard_env: ConnectionEnv = env.clone();
            let shard_shutdown: Receiver<()> = shutdown_receiver.clone();
            handles.push(spawn(async move {
                let _: Result<(), Box<ServerError>> =
                    quic_accept_loop(shard_env, endpoint, shard_shutdown).await;
            }));
        }
        for handle in handles {
            let _: Result<(), JoinError> = handle.await;
        }
    }
    if let Some(stop_tx) = dispatcher_stop {
        let _: Result<(), SendError<()>> = stop_tx.send(());
    }
    if let Some(handle) = dispatcher {
        let _: Result<Result<(), Box<dyn Any + Send>>, JoinError> =
            spawn_blocking(move || handle.join()).await;
    }
    let _: Result<(), SendError<bool>> = running_sender.send(SERVER_STOPPED);
}

/// Serves one accepted TCP connection.
///
/// # Arguments
///
/// - `ConnectionEnv` - The shared server, compiled pipeline, TLS acceptor, limits and protocol.
/// - `TcpStream` - The accepted socket, before its socket options are applied.
/// - `Option<SocketAddr>` - The cached local listener address, if known.
/// - `SocketAddr` - The remote address of the accepted socket.
/// - `Receiver<()>` - Graceful shutdown signal for multiplexed connections.
///
/// # Returns
///
/// - `()` - Resolves once the connection is finished.
pub(crate) async fn serve_tcp_connection(
    env: ConnectionEnv,
    stream: TcpStream,
    cached_local: Option<SocketAddr>,
    remote: SocketAddr,
    shutdown_receiver: Receiver<()>,
) {
    let shared: &ConnectionShared = &env.shared;
    configure_socket(&stream, &shared.server);
    let Some(tls_config) = shared.tls_config.as_ref() else {
        if shared.protocol == ServerProtocol::Http2 {
            let local: SocketAddr = cached_local
                .or_else(|| stream.local_addr().ok())
                .unwrap_or(remote);
            serve_plain_tcp(
                env,
                StreamPeers { local, remote },
                stream,
                shutdown_receiver,
            )
            .await;
        } else {
            serve_h1_connection(env, stream).await;
        }
        return;
    };
    let local: Option<SocketAddr> = if shared.protocol == ServerProtocol::Http2 {
        cached_local.or_else(|| stream.local_addr().ok())
    } else {
        None
    };
    let Ok(handshake) = bounded_handshake(
        shared.request_config,
        accept_unbuffered_tls(Arc::clone(tls_config), stream),
    )
    .await
    else {
        return;
    };
    let Ok(transport) = handshake else {
        return;
    };
    if !negotiated_protocol_matches(&transport, &shared.protocol) {
        return;
    }
    if shared.protocol == ServerProtocol::Http2 {
        let peers: StreamPeers = StreamPeers {
            local: local.unwrap_or(remote),
            remote,
        };
        serve_tls_stream(env, peers, transport, shutdown_receiver).await;
    } else {
        serve_h1_transport(env, transport).await;
    }
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
/// - `&TlsTransport<T>` - The negotiated TLS transport.
/// - `&ServerProtocol` - The protocol this connection is being served as.
///
/// # Returns
///
/// - `bool` - `true` when the session may be served under this protocol.
pub(crate) fn negotiated_protocol_matches<T>(
    transport: &TlsTransport<T>,
    protocol: &ServerProtocol,
) -> bool
where
    T: AsyncRead + AsyncWrite + Unpin,
{
    match transport.alpn_protocol() {
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
    match handshake.downcast::<HandshakeData>() {
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
/// - `Result<T, Elapsed>` - The handshake result, or
///   the elapsed that stopped it.
pub(crate) async fn bounded_handshake<F, T>(
    request_config: RequestConfig,
    handshake: F,
) -> Result<T, Elapsed>
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
/// - `&Server` - The server holding the socket options to apply.
///
/// # Returns
///
/// - `()` - Resolves once the options are applied.
pub(crate) fn configure_socket(stream: &TcpStream, server: &Server) {
    let server_config: &ServerConfig = server.get_server_config();
    #[cfg(not(unix))]
    {
        let nodelay: bool = server_config.get_nodelay().unwrap_or(true);
        let _: Result<(), io::Error> = stream.set_nodelay(nodelay);
    }
    if let Some(ttl) = server_config.get_ttl() {
        let _: Result<(), io::Error> = stream.set_ttl(ttl);
    }
}

/// Builds the HTTP/2 server handshake builder with tuned flow-control windows and DoS bounds.
///
/// # Arguments
///
/// - `&RequestConfig` - Request limits used to derive `SETTINGS_MAX_HEADER_LIST_SIZE`.
///
/// # Returns
///
/// - `H2ServerBuilder` - The configured HTTP/2 server handshake builder.
fn h2_server_builder(request_config: &RequestConfig) -> H2ServerBuilder {
    let max_header_count: usize = request_config.get_max_header_count();
    let max_header_key_size: usize = request_config.get_max_header_key_size();
    let max_header_value_size: usize = request_config.get_max_header_value_size();
    let is_low_security: bool = max_header_count == DEFAULT_LOW_SECURITY_MAX_HEADER_COUNT
        || max_header_key_size == DEFAULT_LOW_SECURITY_MAX_HEADER_KEY_SIZE
        || max_header_value_size == DEFAULT_LOW_SECURITY_MAX_HEADER_VALUE_SIZE;
    let max_header_list_size: u32 = if is_low_security {
        H2_MAX_HEADER_LIST_SIZE_CAP
    } else {
        let per_header: usize = max_header_key_size
            .saturating_add(max_header_value_size)
            .saturating_add(32);
        let budget: usize = max_header_count
            .saturating_mul(per_header)
            .saturating_add(request_config.get_max_path_size());
        u32::try_from(budget)
            .unwrap_or(H2_MAX_HEADER_LIST_SIZE_CAP)
            .clamp(H2_MIN_MAX_HEADER_LIST_SIZE, H2_MAX_HEADER_LIST_SIZE_CAP)
    };
    let mut builder: H2ServerBuilder = H2ServerBuilder::new();
    builder
        .initial_window_size(H2_INITIAL_STREAM_WINDOW_SIZE)
        .initial_connection_window_size(H2_INITIAL_CONNECTION_WINDOW_SIZE)
        .max_concurrent_streams(H2_MAX_CONCURRENT_STREAMS)
        .max_header_list_size(max_header_list_size)
        .max_concurrent_reset_streams(H2_MAX_CONCURRENT_RESET_STREAMS)
        .max_pending_accept_reset_streams(H2_MAX_PENDING_ACCEPT_RESET_STREAMS);
    builder
}

/// Serves a plaintext connection, selecting the protocol it speaks.
///
/// # Arguments
///
/// - `ConnectionEnv` - The server, limits and protocol for the connection.
/// - `StreamPeers` - The addresses of the connection.
/// - `TcpStream` - The accepted socket.
/// - `Receiver<()>` - Graceful shutdown signal for HTTP/2 connections.
///
/// # Returns
///
/// - `()` - Resolves once the connection is finished.
pub(crate) async fn serve_plain_tcp(
    env: ConnectionEnv,
    peers: StreamPeers,
    stream: TcpStream,
    shutdown_receiver: Receiver<()>,
) {
    match env.shared.protocol {
        ServerProtocol::Http2 => {
            let handshake: Result<ConnectionH2<TcpStream>, String> = bounded_handshake(
                env.shared.request_config,
                h2_server_builder(&env.shared.request_config).handshake(stream),
            )
            .await
            .map_err(|_: Elapsed| H2_HANDSHAKE_TIMEOUT.to_owned())
            .and_then(|connection: Result<_, H2Error>| {
                connection.map_err(|error: H2Error| error.to_string())
            });
            serve_h2_connection(env, peers, handshake, shutdown_receiver).await;
        }
        _ => {
            serve_h1_connection(env, stream).await;
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
/// - `ConnectionEnv` - The server, limits and protocol for the connection.
/// - `StreamPeers` - The addresses of the connection.
/// - `TlsTransport` - The negotiated session carrying the connection.
/// - `Receiver<()>` - Graceful shutdown signal for HTTP/2 connections.
///
/// # Returns
///
/// - `()` - Resolves once the connection is finished.
pub(crate) async fn serve_tls_stream(
    env: ConnectionEnv,
    peers: StreamPeers,
    transport: TlsTransport,
    shutdown_receiver: Receiver<()>,
) {
    match env.shared.protocol {
        ServerProtocol::Http2 => {
            let session: TlsTransport = transport;
            let handshake: Result<ConnectionH2<TlsTransport>, String> = bounded_handshake(
                env.shared.request_config,
                h2_server_builder(&env.shared.request_config).handshake(session),
            )
            .await
            .map_err(|_: Elapsed| H2_HANDSHAKE_TIMEOUT.to_owned())
            .and_then(|connection: Result<_, H2Error>| {
                connection.map_err(|error: H2Error| error.to_string())
            });
            serve_h2_connection(env, peers, handshake, shutdown_receiver).await;
        }
        _ => {
            serve_h1_transport(env, transport).await;
        }
    }
}

/// Serves one plaintext HTTP/1.1 connection through the existing pipeline.
///
/// # Arguments
///
/// - `ConnectionEnv` - The shared server, compiled pipeline and parsing limits.
/// - `TcpStream` - The accepted socket.
///
/// # Returns
///
/// - `()` - Resolves once the connection is finished.
pub(crate) async fn serve_h1_connection(env: ConnectionEnv, stream: TcpStream) {
    let mut stream: Stream = Stream::new(stream, env.shared.request_config, false);
    serve_h1_stream(env, &mut stream).await;
}

/// Serves one HTTP/1.1 connection whose transport is already negotiated.
///
/// # Arguments
///
/// - `ConnectionEnv` - The shared server, compiled pipeline and parsing limits.
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
pub(crate) async fn serve_h1_transport(env: ConnectionEnv, transport: TlsTransport) {
    let request_config: RequestConfig = env.shared.request_config;
    let mut stream: Stream = Stream::from_transport(transport, request_config, false);
    serve_h1_stream(env, &mut stream).await;
    let closed: Result<Result<(), io::Error>, Elapsed> =
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
/// - `ConnectionEnv` - The shared server and compiled inline pipeline.
/// - `&mut Stream` - The stream carrying the connection.
///
/// # Returns
///
/// - `()` - Resolves once the connection is finished.
pub(crate) async fn serve_h1_stream(env: ConnectionEnv, stream: &mut Stream) {
    env.shared
        .server
        .serve_h1_connection_stream(&env.shared.pipeline, stream)
        .await;
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
