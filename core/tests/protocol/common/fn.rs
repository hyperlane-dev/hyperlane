use super::*;

pub(crate) fn fixture_path(name: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join(FIXTURE_DIR_NAME)
        .join(name)
}

pub(crate) fn trusted_tls_config() -> TlsConfig {
    TlsConfig::new(
        fixture_path(LEAF_CHAIN_PEM_NAME)
            .to_string_lossy()
            .to_string(),
        fixture_path(LEAF_KEY_NAME).to_string_lossy().to_string(),
    )
}

pub(crate) fn install_trusted_ca(roots: &mut RootCertStore) {
    let ca: CertificateDer<'static> =
        CertificateDer::from_pem_file(fixture_path(CA_PEM_NAME)).expect("fixture CA must exist");
    roots
        .add(ca)
        .expect("fixture CA must be a valid trust anchor certificate");
}

fn alpn_client_config(alpn: &[u8]) -> Arc<ClientConfig> {
    let mut roots: RootCertStore = RootCertStore::empty();
    install_trusted_ca(&mut roots);
    let mut config: ClientConfig = ClientConfig::builder()
        .with_root_certificates(roots)
        .with_no_client_auth();
    config.alpn_protocols = vec![alpn.to_vec()];
    Arc::new(config)
}

fn no_alpn_client_config() -> Arc<ClientConfig> {
    let mut roots: RootCertStore = RootCertStore::empty();
    install_trusted_ca(&mut roots);
    Arc::new(
        ClientConfig::builder()
            .with_root_certificates(roots)
            .with_no_client_auth(),
    )
}

pub(crate) fn negotiated_alpn(stream: &ClientTlsStream) -> Option<Vec<u8>> {
    stream
        .get_ref()
        .1
        .alpn_protocol()
        .map(|alpn: &[u8]| alpn.to_vec())
}

pub(crate) fn hostile_binary_payload() -> Vec<u8> {
    let mut payload: Vec<u8> = (0..=255).map(|value: u8| value).collect();
    payload.extend_from_slice(b"\r\n\r\n");
    payload.extend_from_slice(b"GET /admin HTTP/1.1\r\nHost: localhost\r\n\r\n");
    payload.extend_from_slice(&[0u8, 1u8, 2u8]);
    while payload.len() < HOSTILE_BYTES_LEN {
        payload.push((payload.len() % 251) as u8);
    }
    payload.truncate(HOSTILE_BYTES_LEN);
    payload
}

pub(crate) fn large_binary_payload() -> Vec<u8> {
    let mut payload: Vec<u8> = Vec::with_capacity(ABOVE_FLOW_CONTROL_WINDOW_LEN);
    let mut seed: u32 = 0x1357_9bdf;
    while payload.len() < ABOVE_FLOW_CONTROL_WINDOW_LEN {
        for _ in 0..1024 {
            seed = seed.wrapping_mul(1_664_525).wrapping_add(1_013_904_223);
            payload.extend_from_slice(&seed.to_be_bytes());
        }
    }
    payload.truncate(ABOVE_FLOW_CONTROL_WINDOW_LEN);
    payload
}

pub(crate) fn http1_request(method: &str, target: &str, body: &[u8]) -> Vec<u8> {
    http1_request_with_headers(method, target, PROBE_REPEATED_HEADERS, body)
}

pub(crate) fn http1_request_with_headers(
    method: &str,
    target: &str,
    header_values: [&str; 2],
    body: &[u8],
) -> Vec<u8> {
    let mut head: String = format!("{method} {target} HTTP/1.1{HTTP_BR}");
    head.push_str(&format!("Host: {TLS_SERVER_NAME}{HTTP_BR}"));
    for value in header_values {
        head.push_str(&format!("{PROBE_HEADER_KEY}: {value}{HTTP_BR}"));
    }
    head.push_str(&format!("Content-Length: {}{HTTP_BR}", body.len()));
    head.push_str(HTTP_BR);
    let mut request: Vec<u8> = head.into_bytes();
    request.extend_from_slice(body);
    request
}

pub(crate) fn http1_close_request(method: &str, target: &str) -> Vec<u8> {
    let mut head: String = format!("{method} {target} HTTP/1.1{HTTP_BR}");
    head.push_str(&format!("Host: {TLS_SERVER_NAME}{HTTP_BR}"));
    head.push_str(&format!("Content-Length: 0{HTTP_BR}"));
    head.push_str(&format!("{CONNECTION}: close{HTTP_BR}"));
    head.push_str(HTTP_BR);
    head.into_bytes()
}

pub(crate) fn response_header_value(head: &str, key: &str) -> String {
    let needle: String = format!("{}:", key.to_ascii_lowercase());
    let mut found: Vec<String> = Vec::new();
    for line in head.lines() {
        let Some((name, rest)) = line.split_once(':') else {
            continue;
        };
        if name.trim().to_ascii_lowercase() == needle.trim_end_matches(':') {
            found.push(rest.trim().to_string());
        }
    }
    found.join(",")
}

pub(crate) async fn read_more(stream: &mut TcpStream, sink: &mut Vec<u8>) {
    let mut chunk: [u8; 8192] = [0; 8192];
    let bound: Duration = Duration::from_secs(2);
    match timeout(bound, stream.read(&mut chunk)).await {
        Ok(Ok(read)) => {
            if read > 0 {
                sink.extend_from_slice(&chunk[..read]);
            }
        }
        Ok(Err(_)) => {}
        Err(_) => {}
    }
}

pub(crate) async fn http1_head_until<T>(stream: &mut T, delimiter: &str) -> String
where
    T: tokio::io::AsyncRead + Unpin,
{
    let mut collected: Vec<u8> = Vec::new();
    let mut chunk: [u8; 8192] = [0; 8192];
    let deadline: Instant = Instant::now() + Duration::from_secs(REQUEST_TIMEOUT_SECS);
    while Instant::now() < deadline {
        let text: String = String::from_utf8_lossy(&collected).to_string();
        if text.contains(delimiter) {
            return text;
        }
        let remaining: Duration = deadline.saturating_duration_since(Instant::now());
        let read: usize = match timeout(remaining, stream.read(&mut chunk)).await {
            Ok(Ok(count)) => count,
            Ok(Err(error)) => {
                panic!("the response head read failed before the delimiter arrived: {error}")
            }
            Err(_) => panic!("the response head must arrive inside its timeout"),
        };
        if read == 0 {
            return String::from_utf8_lossy(&collected).to_string();
        }
        collected.extend_from_slice(&chunk[..read]);
    }
    panic!("the response head never carried its {delimiter} delimiter")
}

pub(crate) async fn tls_exchange(
    mut stream: ClientTlsStream,
    request: Vec<u8>,
) -> io::Result<Vec<u8>> {
    write_bounded(&mut stream, &request).await?;
    stream.flush().await?;
    let deadline: Instant = Instant::now() + Duration::from_secs(REQUEST_TIMEOUT_SECS);
    let mut received: Vec<u8> = Vec::new();
    let mut chunk: [u8; 16 * 1024] = [0; 16 * 1024];
    loop {
        let remaining: Duration = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            return Err(io::Error::new(
                io::ErrorKind::TimedOut,
                "the tls exchange did not reach end of stream inside its timeout",
            ));
        }
        match timeout(remaining, stream.read(&mut chunk)).await {
            Ok(Ok(0)) => break,
            Ok(Ok(read)) => received.extend_from_slice(&chunk[..read]),
            Ok(Err(error)) => return Err(error),
            Err(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::TimedOut,
                    "the tls exchange did not reach end of stream inside its timeout",
                ));
            }
        }
    }
    Ok(received)
}

pub(crate) async fn connect_tls(port: u16, alpn: &[u8]) -> io::Result<ClientTlsStream> {
    let name: ServerName<'static> = ServerName::try_from(TLS_SERVER_NAME).map_err(
        |error: rustls::pki_types::InvalidDnsNameError| {
            io::Error::new(io::ErrorKind::InvalidInput, error.to_string())
        },
    )?;
    let connector: tokio_rustls::TlsConnector =
        tokio_rustls::TlsConnector::from(alpn_client_config(alpn));
    let socket: TcpStream = connect_tcp(port).await?;
    let handshake: tokio_rustls::Connect<TcpStream> = connector.connect(name, socket);
    let bound: Duration = Duration::from_secs(CONNECT_TIMEOUT_SECS);
    match timeout(bound, handshake).await {
        Ok(outcome) => outcome,
        Err(_) => Err(io::Error::new(
            io::ErrorKind::TimedOut,
            "tls handshake timed out",
        )),
    }
}

pub(crate) async fn connect_tls_untrusted(port: u16) -> Result<(), String> {
    let name: ServerName<'static> = ServerName::try_from(TLS_SERVER_NAME)
        .map_err(|error: rustls::pki_types::InvalidDnsNameError| error.to_string())?;
    let connector: tokio_rustls::TlsConnector =
        tokio_rustls::TlsConnector::from(untrusted_client_config(ALPN_HTTP2));
    let socket: TcpStream = connect_tcp(port)
        .await
        .map_err(|error: io::Error| error.to_string())?;
    let bound: Duration = Duration::from_secs(CONNECT_TIMEOUT_SECS);
    let outcome: io::Result<ClientTlsStream> = timeout(bound, connector.connect(name, socket))
        .await
        .map_err(|_| "handshake timed out".to_string())?;
    match outcome {
        Ok(_) => Ok(()),
        Err(_) => Err("an untrusted chain must fail verification".to_string()),
    }
}

fn untrusted_client_config(alpn: &[u8]) -> Arc<ClientConfig> {
    let roots: RootCertStore = RootCertStore::empty();
    let mut config: ClientConfig = ClientConfig::builder()
        .with_root_certificates(roots)
        .with_no_client_auth();
    config.alpn_protocols = vec![alpn.to_vec()];
    Arc::new(config)
}

pub(crate) async fn connect_tls_without_alpn(port: u16) -> io::Result<ClientTlsStream> {
    let name: ServerName<'static> = ServerName::try_from(TLS_SERVER_NAME).map_err(
        |error: rustls::pki_types::InvalidDnsNameError| {
            io::Error::new(io::ErrorKind::InvalidInput, error.to_string())
        },
    )?;
    let connector: tokio_rustls::TlsConnector =
        tokio_rustls::TlsConnector::from(no_alpn_client_config());
    let socket: TcpStream = connect_tcp(port).await?;
    let handshake: tokio_rustls::Connect<TcpStream> = connector.connect(name, socket);
    let bound: Duration = Duration::from_secs(CONNECT_TIMEOUT_SECS);
    match timeout(bound, handshake).await {
        Ok(outcome) => outcome,
        Err(_) => Err(io::Error::new(
            io::ErrorKind::TimedOut,
            "tls handshake timed out",
        )),
    }
}
