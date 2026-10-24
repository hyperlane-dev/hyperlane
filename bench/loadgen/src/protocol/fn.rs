use super::*;

/// Builds the client configuration the secure protocols negotiate with.
///
/// # Arguments
///
/// - `&str` - The path of the certificate authority.
/// - `&[u8]` - The ALPN identifier to advertise.
///
/// # Returns
///
/// - `Result<ClientConfig, String>` - The configuration, or the reason it could
///   not be built.
pub(crate) fn build_client_config(ca_path: &str, alpn: &[u8]) -> Result<ClientConfig, String> {
    let mut roots: RootCertStore = RootCertStore::empty();
    let certificates: rustls_pki_types::pem::ReadIter<BufReader<File>, CertificateDer<'static>> =
        match CertificateDer::pem_file_iter(ca_path) {
            Ok(certificates) => certificates,
            Err(error) => return Err(format!("{ERROR_CA_UNREADABLE}: {error}")),
        };
    for certificate in certificates {
        let certificate: CertificateDer<'static> = match certificate {
            Ok(certificate) => certificate,
            Err(error) => return Err(format!("{ERROR_CA_UNREADABLE}: {error}")),
        };
        if let Err(error) = roots.add(certificate) {
            return Err(format!("{ERROR_CA_UNREADABLE}: {error}"));
        }
    }
    let versions: &[&'static rustls::SupportedProtocolVersion] =
        &[&rustls::version::TLS13, &rustls::version::TLS12];
    let mut config: ClientConfig = match ClientConfig::builder_with_provider(Arc::new(
        rustls::crypto::ring::default_provider(),
    ))
    .with_protocol_versions(versions)
    {
        Ok(builder) => builder.with_root_certificates(roots).with_no_client_auth(),
        Err(error) => return Err(error.to_string()),
    };
    config.alpn_protocols = vec![alpn.to_vec()];
    Ok(config)
}

/// Builds the QUIC client configuration the HTTP/3 variant negotiates with.
///
/// # Arguments
///
/// - `&str` - The path of the certificate authority.
///
/// # Returns
///
/// - `Result<quinn::ClientConfig, String>` - The configuration, or the reason it
///   could not be built.
pub(crate) fn build_quic_config(ca_path: &str) -> Result<quinn::ClientConfig, String> {
    let crypto: quinn::crypto::rustls::QuicClientConfig =
        match quinn::crypto::rustls::QuicClientConfig::try_from(build_client_config(
            ca_path, ALPN_HTTP3,
        )?) {
            Ok(crypto) => crypto,
            Err(error) => return Err(error.to_string()),
        };
    Ok(quinn::ClientConfig::new(Arc::new(crypto)))
}

/// Opens a plain socket to the server under test.
///
/// # Arguments
///
/// - `&str` - The address of the server.
///
/// # Returns
///
/// - `Result<TcpStream, String>` - The socket, or the reason it could not be
///   opened.
pub(crate) async fn open_plain(address: &str) -> Result<TcpStream, String> {
    let stream: TcpStream = match TcpStream::connect(address).await {
        Ok(stream) => stream,
        Err(error) => return Err(error.to_string()),
    };
    let _: Result<(), Error> = stream.set_nodelay(true);
    Ok(stream)
}

/// Opens a TLS session to the server under test.
///
/// # Arguments
///
/// - `&str` - The address of the server.
/// - `&str` - The path of the certificate authority.
/// - `&[u8]` - The ALPN identifier to advertise.
///
/// # Returns
///
/// - `Result<tokio_rustls::client::TlsStream<TcpStream>, String>` - The session,
///   or the reason the handshake failed.
pub(crate) async fn open_tls(
    address: &str,
    ca_path: &str,
    alpn: &[u8],
) -> Result<tokio_rustls::client::TlsStream<TcpStream>, String> {
    let config: ClientConfig = build_client_config(ca_path, alpn)?;
    let connector: tokio_rustls::TlsConnector = tokio_rustls::TlsConnector::from(Arc::new(config));
    let stream: TcpStream = open_plain(address).await?;
    let server_name: ServerName<'static> = ServerName::try_from(VERIFY_NAME)
        .map_err(|error: rustls::pki_types::InvalidDnsNameError| error.to_string())?;
    let session: tokio_rustls::client::TlsStream<TcpStream> =
        match connector.connect(server_name, stream).await {
            Ok(session) => session,
            Err(error) => return Err(error.to_string()),
        };
    Ok(session)
}

/// Opens the transport one protocol variant writes its bytes over.
///
/// # Arguments
///
/// - `&str` - The address of the server.
/// - `Protocol` - The protocol the transport speaks.
/// - `Option<&str>` - The path of the certificate authority.
///
/// # Returns
///
/// - `Result<Box<dyn Transport>, String>` - The transport, or the reason it could
///   not be opened.
pub(crate) async fn open_transport(
    address: &str,
    protocol: Protocol,
    ca_path: Option<&str>,
) -> Result<Box<dyn Transport>, String> {
    let resolved: String = resolve_address(address).await?.to_string();
    let address: &str = resolved.as_str();
    match protocol {
        Protocol::Http1 => {
            let stream: TcpStream = open_plain(address).await?;
            Ok(Box::new(stream))
        }
        Protocol::Http1Tls => {
            let ca_path: &str = match ca_path {
                Some(ca_path) => ca_path,
                None => return Err(ERROR_TLS_NEEDS_CA.to_owned()),
            };
            let session: tokio_rustls::client::TlsStream<TcpStream> =
                open_tls(address, ca_path, ALPN_HTTP1).await?;
            Ok(Box::new(session))
        }
        Protocol::H2c | Protocol::H2Tls => {
            let transport: Box<dyn Transport> = match protocol {
                Protocol::H2Tls => {
                    let ca_path: &str = match ca_path {
                        Some(ca_path) => ca_path,
                        None => return Err(ERROR_TLS_NEEDS_CA.to_owned()),
                    };
                    let session: tokio_rustls::client::TlsStream<TcpStream> =
                        open_tls(address, ca_path, ALPN_HTTP2).await?;
                    Box::new(session)
                }
                _ => {
                    let stream: TcpStream = open_plain(address).await?;
                    Box::new(stream)
                }
            };
            Ok(transport)
        }
        Protocol::Http3 => Err(ERROR_TLS_NEEDS_CA.to_owned()),
    }
}

/// Resolves an address to the socket address the clients connect to.
///
/// The harness names the server by its container name, and the QUIC client
/// takes a numeric address. The name is resolved here, once, so the client
/// does not have to carry a resolver of its own.
///
/// # Arguments
///
/// - `&str` - The address, either numeric or a host name.
///
/// # Returns
///
/// - `Result<SocketAddr, String>` - The socket address, or the reason it could
///   not be resolved.
pub(crate) async fn resolve_address(address: &str) -> Result<SocketAddr, String> {
    if let Ok(parsed) = address.parse::<SocketAddr>() {
        return Ok(parsed);
    }
    let mut resolved: Vec<SocketAddr> = match tokio::net::lookup_host(address).await {
        Ok(resolved) => resolved.collect(),
        Err(error) => return Err(format!("{ERROR_RESOLVE}: {error}")),
    };
    match resolved.pop() {
        Some(address) => Ok(address),
        None => Err(format!("{ERROR_RESOLVE}: {address} resolves to nothing")),
    }
}

/// Drives one HTTP/2 connection until the deadline.
///
/// # Arguments
///
/// - `Run` - The parameters of the run.
///
/// # Returns
///
/// - `ConnStats` - The counters of the connection.
pub(crate) async fn run_http2(run: Run) -> ConnStats {
    let mut stats: ConnStats = ConnStats::default();
    let transport: Box<dyn Transport> =
        match open_transport(&run.address, run.protocol, run.ca_path.as_deref()).await {
            Ok(transport) => transport,
            Err(_) => return stats,
        };
    let (mut sender, connection) = match h2::client::handshake(transport).await {
        Ok(handshake) => handshake,
        Err(_) => return stats,
    };
    let driver: JoinHandle<()> = tokio::spawn(async move {
        let _: Result<(), h2::Error> = connection.await;
    });
    let scheme: &str = run.protocol.scheme();
    let _: BarrierWaitResult = run.barrier.wait().await;
    let mut index: usize = 0;
    loop {
        let now: Instant = Instant::now();
        if now >= run.deadline {
            break;
        }
        let Some(case) = run.cases.get(index % run.cases.len()) else {
            break;
        };
        index = index.wrapping_add(1);
        let request: Request<()> = build_request(case, scheme, AUTHORITY);
        let (response, mut send_body) = match sender.send_request(request, case.body.is_empty()) {
            Ok(pair) => pair,
            Err(_) => {
                stats.errors += 1;
                break;
            }
        };
        if !case.body.is_empty() {
            let body: Bytes = Bytes::copy_from_slice(case.body.as_slice());
            if send_body.send_data(body, true).is_err() {
                stats.errors += 1;
                break;
            }
        }
        let received: http::Response<h2::RecvStream> = match response.await {
            Ok(response) => response,
            Err(_) => {
                stats.errors += 1;
                break;
            }
        };
        let length: usize = match read_h2_body(received.into_body()).await {
            Ok(length) => length,
            Err(_) => {
                stats.errors += 1;
                break;
            }
        };
        if length == case.expect_len {
            stats.completed += 1;
        } else {
            stats.errors += 1;
        }
        let micros: u128 = now.elapsed().as_micros();
        if now >= run.warmup_until {
            stats
                .latencies_us
                .push(u64::try_from(micros).unwrap_or(u64::MAX));
        }
    }
    driver.abort();
    stats
}

/// Sends one HTTP/2 request on a connection that is opened for it alone.
///
/// # Arguments
///
/// - `&str` - The address of the server under test.
/// - `&ProtocolCase` - The request to send.
/// - `Protocol` - The protocol the connection speaks.
/// - `Option<&str>` - The path of the certificate authority.
///
/// # Returns
///
/// - `Result<usize, String>` - The body length of the answer, or the reason the
///   exchange failed.
pub(crate) async fn send_one_http2(
    address: &str,
    case: &ProtocolCase,
    protocol: Protocol,
    ca_path: Option<&str>,
) -> Result<usize, String> {
    let transport: Box<dyn Transport> = open_transport(address, protocol, ca_path).await?;
    let (mut sender, connection) = h2::client::handshake(transport)
        .await
        .map_err(|error: h2::Error| error.to_string())?;
    let driver: JoinHandle<()> = tokio::spawn(async move {
        let _: Result<(), h2::Error> = connection.await;
    });
    let scheme: &str = protocol.scheme();
    let request: Request<()> = build_request(case, scheme, AUTHORITY);
    let (response, mut send_body) = sender
        .send_request(request, case.body.is_empty())
        .map_err(|error: h2::Error| error.to_string())?;
    if !case.body.is_empty() {
        let body: Bytes = Bytes::copy_from_slice(case.body.as_slice());
        send_body
            .send_data(body, true)
            .map_err(|error: h2::Error| error.to_string())?;
    }
    let received: http::Response<h2::RecvStream> = response
        .await
        .map_err(|error: h2::Error| error.to_string())?;
    let length: usize = read_h2_body(received.into_body())
        .await
        .map_err(|error: String| error)?;
    driver.abort();
    Ok(length)
}

/// Drives one short HTTP/2 connection: one connection, one request, torn down.
///
/// # Arguments
///
/// - `Run` - The parameters of the run.
///
/// # Returns
///
/// - `ConnStats` - The counters of the connection.
pub(crate) async fn run_http2_short(run: Run) -> ConnStats {
    let mut stats: ConnStats = ConnStats::default();
    let _: BarrierWaitResult = run.barrier.wait().await;
    let interval: Duration = Duration::from_nanos(
        1_000_000_000 * u64::try_from(run.connections).unwrap_or(1) / run.rate_per_sec.max(1),
    );
    let mut next_open: Instant = Instant::now();
    let mut index: usize = 0;
    loop {
        let now: Instant = Instant::now();
        if now >= run.deadline {
            break;
        }
        let Some(case) = run.cases.get(index % run.cases.len()) else {
            break;
        };
        index = index.wrapping_add(1);
        if now < next_open {
            tokio::time::sleep(next_open - now).await;
        }
        next_open += interval;
        match send_one_http2(&run.address, case, run.protocol, run.ca_path.as_deref()).await {
            Ok(length) => {
                if length == case.expect_len {
                    stats.completed += 1;
                } else {
                    stats.errors += 1;
                }
            }
            Err(_) => {
                stats.errors += 1;
            }
        }
        let micros: u128 = now.elapsed().as_micros();
        if now >= run.warmup_until {
            stats
                .latencies_us
                .push(u64::try_from(micros).unwrap_or(u64::MAX));
        }
    }
    stats
}

/// Drives one HTTP/3 connection until the deadline.
///
/// # Arguments
///
/// - `Run` - The parameters of the run.
///
/// # Returns
///
/// - `ConnStats` - The counters of the connection.
pub(crate) async fn run_http3(run: Run) -> ConnStats {
    let mut stats: ConnStats = ConnStats::default();
    let Some(ca_path) = run.ca_path.clone() else {
        return stats;
    };
    let config: quinn::ClientConfig = match build_quic_config(&ca_path) {
        Ok(config) => config,
        Err(_) => return stats,
    };
    let bind: SocketAddr = match QUIC_BIND_ADDRESS.parse() {
        Ok(bind) => bind,
        Err(_) => return stats,
    };
    let endpoint: quinn::Endpoint = match quinn::Endpoint::client(bind) {
        Ok(endpoint) => endpoint,
        Err(_) => return stats,
    };
    let server: SocketAddr = match resolve_address(&run.address).await {
        Ok(server) => server,
        Err(_) => return stats,
    };
    let connecting: quinn::Connecting = match endpoint.connect_with(config, server, VERIFY_NAME) {
        Ok(connecting) => connecting,
        Err(_) => return stats,
    };
    let connection: quinn::Connection = match connecting.await {
        Ok(connection) => connection,
        Err(_) => return stats,
    };
    let (client, mut sender) = match h3::client::builder()
        .build(h3_quinn::Connection::new(connection))
        .await
    {
        Ok(pair) => pair,
        Err(_) => return stats,
    };
    let driver: JoinHandle<()> = tokio::spawn(async move {
        let mut client: h3::client::Connection<h3_quinn::Connection, Bytes> = client;
        let _: h3::error::ConnectionError =
            futures::future::poll_fn(|context: &mut Context<'_>| client.poll_close(context)).await;
    });
    let _: BarrierWaitResult = run.barrier.wait().await;
    let mut index: usize = 0;
    loop {
        let now: Instant = Instant::now();
        if now >= run.deadline {
            break;
        }
        let Some(case) = run.cases.get(index % run.cases.len()) else {
            break;
        };
        index = index.wrapping_add(1);
        let request: Request<()> = build_request(case, SCHEME_SECURE, AUTHORITY);
        let mut stream: h3::client::RequestStream<h3_quinn::BidiStream<Bytes>, Bytes> =
            match sender.send_request(request).await {
                Ok(stream) => stream,
                Err(_) => {
                    stats.errors += 1;
                    break;
                }
            };
        if !case.body.is_empty() {
            let body: Bytes = Bytes::copy_from_slice(case.body.as_slice());
            if stream.send_data(body).await.is_err() {
                stats.errors += 1;
                break;
            }
        }
        if stream.finish().await.is_err() {
            stats.errors += 1;
            break;
        }
        if stream.recv_response().await.is_err() {
            stats.errors += 1;
            break;
        }
        let mut length: usize = 0;
        loop {
            match stream.recv_data().await {
                Ok(Some(chunk)) => length += chunk.chunk().len(),
                Ok(None) => break,
                Err(_) => {
                    stats.errors += 1;
                    break;
                }
            }
        }
        if length == case.expect_len {
            stats.completed += 1;
        } else {
            stats.errors += 1;
        }
        let micros: u128 = now.elapsed().as_micros();
        if now >= run.warmup_until {
            stats
                .latencies_us
                .push(u64::try_from(micros).unwrap_or(u64::MAX));
        }
    }
    driver.abort();
    endpoint.close(0u8.into(), QUIC_CLOSE_REASON);
    stats
}

/// Sends one HTTP/3 request on a connection that is opened for it alone.
///
/// # Arguments
///
/// - `&str` - The address of the server under test.
/// - `&ProtocolCase` - The request to send.
/// - `Option<String>` - The path of the certificate authority.
///
/// # Returns
///
/// - `Result<usize, String>` - The body length of the answer, or the reason the
///   exchange failed.
pub(crate) async fn send_one_http3(
    address: &str,
    case: &ProtocolCase,
    ca_path: Option<String>,
) -> Result<usize, String> {
    let ca_path: String = match ca_path {
        Some(ca_path) => ca_path,
        None => return Err(ERROR_TLS_NEEDS_CA.to_owned()),
    };
    let config: quinn::ClientConfig = build_quic_config(&ca_path)?;
    let bind: SocketAddr = QUIC_BIND_ADDRESS
        .parse()
        .map_err(|error: AddrParseError| error.to_string())?;
    let endpoint: quinn::Endpoint =
        quinn::Endpoint::client(bind).map_err(|error: Error| error.to_string())?;
    let server: SocketAddr = resolve_address(address).await?;
    let connecting: quinn::Connecting = endpoint
        .connect_with(config, server, VERIFY_NAME)
        .map_err(|error: quinn::ConnectError| error.to_string())?;
    let connection: quinn::Connection = connecting
        .await
        .map_err(|error: quinn::ConnectionError| error.to_string())?;
    let (client, mut sender) = h3::client::builder()
        .build(h3_quinn::Connection::new(connection))
        .await
        .map_err(|error: h3::error::ConnectionError| error.to_string())?;
    let driver: JoinHandle<()> = tokio::spawn(async move {
        let mut client: h3::client::Connection<h3_quinn::Connection, Bytes> = client;
        let _: h3::error::ConnectionError =
            futures::future::poll_fn(|context: &mut Context<'_>| client.poll_close(context)).await;
    });
    let request: Request<()> = build_request(case, SCHEME_SECURE, AUTHORITY);
    let mut stream: h3::client::RequestStream<h3_quinn::BidiStream<Bytes>, Bytes> = sender
        .send_request(request)
        .await
        .map_err(|error: h3::error::StreamError| error.to_string())?;
    if !case.body.is_empty() {
        let body: Bytes = Bytes::copy_from_slice(case.body.as_slice());
        stream
            .send_data(body)
            .await
            .map_err(|error: h3::error::StreamError| error.to_string())?;
    }
    stream
        .finish()
        .await
        .map_err(|error: h3::error::StreamError| error.to_string())?;
    stream
        .recv_response()
        .await
        .map_err(|error: h3::error::StreamError| error.to_string())?;
    let mut length: usize = 0;
    loop {
        match stream.recv_data().await {
            Ok(Some(chunk)) => length += chunk.chunk().len(),
            Ok(None) => break,
            Err(error) => {
                driver.abort();
                return Err(error.to_string());
            }
        }
    }
    driver.abort();
    endpoint.close(0u8.into(), QUIC_CLOSE_REASON);
    Ok(length)
}

/// Drives one short HTTP/3 connection: one connection, one request, torn down.
///
/// # Arguments
///
/// - `Run` - The parameters of the run.
///
/// # Returns
///
/// - `ConnStats` - The counters of the connection.
pub(crate) async fn run_http3_short(run: Run) -> ConnStats {
    let mut stats: ConnStats = ConnStats::default();
    let _: BarrierWaitResult = run.barrier.wait().await;
    let interval: Duration = Duration::from_nanos(
        1_000_000_000 * u64::try_from(run.connections).unwrap_or(1) / run.rate_per_sec.max(1),
    );
    let mut next_open: Instant = Instant::now();
    let mut index: usize = 0;
    loop {
        let now: Instant = Instant::now();
        if now >= run.deadline {
            break;
        }
        let Some(case) = run.cases.get(index % run.cases.len()) else {
            break;
        };
        index = index.wrapping_add(1);
        if now < next_open {
            tokio::time::sleep(next_open - now).await;
        }
        next_open += interval;
        match send_one_http3(&run.address, case, run.ca_path.clone()).await {
            Ok(length) => {
                if length == case.expect_len {
                    stats.completed += 1;
                } else {
                    stats.errors += 1;
                }
            }
            Err(_) => {
                stats.errors += 1;
            }
        }
        let micros: u128 = now.elapsed().as_micros();
        if now >= run.warmup_until {
            stats
                .latencies_us
                .push(u64::try_from(micros).unwrap_or(u64::MAX));
        }
    }
    stats
}

/// Builds the request one case of the mix carries.
///
/// # Arguments
///
/// - `&ProtocolCase` - The case.
/// - `&str` - The scheme the request carries.
/// - `&str` - The authority the request addresses.
///
/// # Returns
///
/// - `Request<()>` - The request.
pub(crate) fn build_request(case: &ProtocolCase, scheme: &str, authority: &str) -> Request<()> {
    let text: String = String::from_utf8_lossy(&case.bytes).to_string();
    let mut lines: Vec<&str> = text.split("\r\n").collect();
    let target: &str = lines.first().copied().unwrap_or_default();
    let mut fields: Vec<&str> = target.split(' ').collect();
    let method: &str = fields.first().copied().unwrap_or(METHOD_GET);
    let path: &str = fields.get(1).copied().unwrap_or(PATH_ROOT);
    fields.clear();
    let mut request: Request<()> = Request::new(());
    *request.method_mut() = match Method::from_bytes(method.as_bytes()) {
        Ok(method) => method,
        Err(_) => Method::GET,
    };
    let uri: String = format!("{scheme}://{authority}{path}");
    match uri.parse::<Uri>() {
        Ok(parsed) => *request.uri_mut() = parsed,
        Err(_) => *request.uri_mut() = Uri::from_static("/"),
    }
    let mut headers: HeaderMap = HeaderMap::new();
    if let Ok(value) = HeaderValue::from_str(authority) {
        headers.insert(http::header::HOST, value);
    }
    headers.insert(
        http::header::ACCEPT,
        HeaderValue::from_static(HEADER_ACCEPT),
    );
    headers.insert(
        http::header::USER_AGENT,
        HeaderValue::from_static(HEADER_USER_AGENT),
    );
    if let Ok(value) = HeaderValue::from_str(HEADER_TAG_VALUE)
        && let Ok(name) = http::header::HeaderName::from_bytes(HEADER_TAG.as_bytes())
    {
        headers.insert(name, value);
    }
    *request.headers_mut() = headers;
    lines.clear();
    request
}

/// Reads the body of one HTTP/2 response.
///
/// # Arguments
///
/// - `h2::RecvStream` - The receive half of the response stream.
///
/// # Returns
///
/// - `Result<usize, String>` - The body length, or the reason the body could
///   not be read.
pub(crate) async fn read_h2_body(mut body: h2::RecvStream) -> Result<usize, String> {
    let mut length: usize = 0;
    while let Some(chunk) = body.data().await {
        let chunk: Bytes = match chunk {
            Ok(chunk) => chunk,
            Err(error) => return Err(error.to_string()),
        };
        length += chunk.len();
    }
    Ok(length)
}
