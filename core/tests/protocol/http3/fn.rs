use super::*;

fn quic_client_config(alpn: &[u8]) -> quinn::ClientConfig {
    let mut roots: RootCertStore = RootCertStore::empty();
    install_trusted_ca(&mut roots);
    let mut config: ClientConfig = ClientConfig::builder()
        .with_root_certificates(roots)
        .with_no_client_auth();
    config.alpn_protocols = vec![alpn.to_vec()];
    quinn::ClientConfig::new(Arc::new(
        quinn::crypto::rustls::QuicClientConfig::try_from(config)
            .expect("the ring provider supports the QUIC initial cipher suite"),
    ))
}

pub(crate) async fn connect_quic(
    port: u16,
    alpn: &[u8],
) -> Result<(quinn::Endpoint, quinn::Connection), String> {
    let bind: SocketAddr = format!("{LOOPBACK_HOST}:0")
        .parse()
        .map_err(|error: AddrParseError| format!("loopback bind address is invalid: {error}"))?;
    let mut endpoint: quinn::Endpoint = quinn::Endpoint::client(bind)
        .map_err(|error: io::Error| format!("quic endpoint bind failed: {error}"))?;
    endpoint.set_default_client_config(quic_client_config(alpn));
    let address: SocketAddr = format!("{LOOPBACK_HOST}:{port}")
        .parse()
        .map_err(|error: AddrParseError| format!("loopback address is invalid: {error}"))?;
    let connecting: quinn::Connecting = endpoint
        .connect(address, TLS_SERVER_NAME)
        .map_err(|error: quinn::ConnectError| format!("quic connect failed: {error}"))?;
    let connection: quinn::Connection =
        timeout(Duration::from_secs(CONNECT_TIMEOUT_SECS), connecting)
            .await
            .map_err(|_: tokio::time::error::Elapsed| "quic connect timed out".to_string())?
            .map_err(|error: quinn::ConnectionError| format!("quic connect failed: {error}"))?;
    Ok((endpoint, connection))
}

fn negotiated_quic_alpn(connection: &quinn::Connection) -> Option<Vec<u8>> {
    let data: Option<Box<dyn Any>> = connection.handshake_data();
    match data {
        Some(data) => match data.downcast::<quinn::crypto::rustls::HandshakeData>() {
            Ok(handshake) => handshake.protocol,
            Err(_) => None,
        },
        None => None,
    }
}

pub(crate) async fn h3_request_with_headers(
    port: u16,
    method: &str,
    target: &str,
    body: &[u8],
) -> Result<H3HeadReply, String> {
    let (_endpoint, connection): (quinn::Endpoint, quinn::Connection) =
        connect_quic(port, ALPN_HTTP3).await?;
    let alpn: Option<Vec<u8>> = negotiated_quic_alpn(&connection);
    h3_exchange_on(H3Exchange {
        connection,
        alpn,
        method,
        target,
        body,
    })
    .await
}

pub(crate) async fn h3_exchange_on(exchange: H3Exchange<'_>) -> Result<H3HeadReply, String> {
    let H3Exchange {
        connection,
        alpn,
        method,
        target,
        body,
    } = exchange;
    let (mut driver, mut client) = h3::client::new(h3_quinn::Connection::new(connection.clone()))
        .await
        .map_err(|error: h3::error::ConnectionError| format!("h3 handshake failed: {error}"))?;
    let driving: JoinHandle<()> = spawn(async move {
        let _final: h3::error::ConnectionError = driver.wait_idle().await;
    });
    let request: http::Request<()> = http::Request::builder()
        .method(method)
        .uri(target)
        .header(PROBE_HEADER_KEY, PROBE_HEADER_VALUE)
        .body(())
        .map_err(|error: http::Error| format!("request build failed: {error}"))?;
    let mut stream: h3::client::RequestStream<h3_quinn::BidiStream<Bytes>, Bytes> = client
        .send_request(request)
        .await
        .map_err(|error: h3::error::StreamError| format!("send_request failed: {error}"))?;
    if !body.is_empty() {
        stream
            .send_data(Bytes::copy_from_slice(body))
            .await
            .map_err(|error: h3::error::StreamError| format!("send_data failed: {error}"))?;
    }
    stream
        .finish()
        .await
        .map_err(|error: h3::error::StreamError| format!("finish failed: {error}"))?;
    let response: http::Response<()> = timeout(
        Duration::from_secs(REQUEST_TIMEOUT_SECS),
        stream.recv_response(),
    )
    .await
    .map_err(|_: tokio::time::error::Elapsed| "h3 response timed out".to_string())?
    .map_err(|error: h3::error::StreamError| format!("recv_response failed: {error}"))?;
    let status: u16 = response.status().as_u16();
    let headers: http::HeaderMap = response.headers().clone();
    let mut collected: Vec<u8> = Vec::new();
    let deadline: Instant = Instant::now() + Duration::from_secs(REQUEST_TIMEOUT_SECS);
    loop {
        let remaining: Duration = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            return Err("h3 body timed out".to_string());
        }
        match timeout(remaining, stream.recv_data()).await {
            Ok(Ok(Some(chunk))) => collected.extend_from_slice(chunk.chunk()),
            Ok(Ok(None)) => break,
            Ok(Err(error)) => return Err(format!("h3 body read failed: {error}")),
            Err(_) => return Err("h3 body timed out".to_string()),
        }
    }
    drop(client);
    driving.abort();
    connection.close(quinn::VarInt::from_u32(0), b"done");
    Ok((status, headers, collected, alpn))
}

pub(crate) async fn h3_stream_until(port: u16, target: &str, needle: &str) -> (Vec<u8>, bool) {
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
    let request: http::Request<()> = http::Request::builder()
        .method("GET")
        .uri(target)
        .body(())
        .expect("request must build");
    let mut stream: h3::client::RequestStream<h3_quinn::BidiStream<Bytes>, Bytes> = client
        .send_request(request)
        .await
        .expect("h3 request must be accepted");
    stream.finish().await.expect("h3 request must finish");
    let _response: http::Response<()> = timeout(
        Duration::from_secs(REQUEST_TIMEOUT_SECS),
        stream.recv_response(),
    )
    .await
    .expect("the h3 response head must arrive inside its timeout")
    .expect("the h3 response head must not fail");
    let mut body: h3::client::RequestStream<h3_quinn::BidiStream<Bytes>, Bytes> = stream;
    let mut collected: Vec<u8> = Vec::new();
    let mut matched: bool = false;
    let deadline: Instant = Instant::now() + Duration::from_secs(REQUEST_TIMEOUT_SECS);
    loop {
        if String::from_utf8_lossy(&collected).contains(needle) {
            matched = true;
            break;
        }
        let remaining: Duration = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            break;
        }
        match timeout(remaining, body.recv_data()).await {
            Ok(Ok(Some(chunk))) => collected.extend_from_slice(chunk.chunk()),
            Ok(Ok(None)) => break,
            Ok(Err(error)) => {
                panic!("the h3 body read failed before the needle arrived: {error}")
            }
            Err(_) => break,
        }
    }
    drop(client);
    driving.abort();
    connection.close(quinn::VarInt::from_u32(0), b"done");
    (collected, matched)
}

pub(crate) async fn h3_request(
    port: u16,
    method: &str,
    target: &str,
    body: &[u8],
) -> Result<H3Reply, String> {
    let (_endpoint, connection): (quinn::Endpoint, quinn::Connection) =
        connect_quic(port, ALPN_HTTP3).await?;
    let alpn: Option<Vec<u8>> = negotiated_quic_alpn(&connection);
    let (mut driver, mut client) = h3::client::new(h3_quinn::Connection::new(connection.clone()))
        .await
        .map_err(|error: h3::error::ConnectionError| format!("h3 handshake failed: {error}"))?;
    let driving: JoinHandle<()> = spawn(async move {
        let _final: h3::error::ConnectionError = driver.wait_idle().await;
    });
    let request: http::Request<()> = http::Request::builder()
        .method(method)
        .uri(target)
        .header(PROBE_HEADER_KEY, PROBE_HEADER_VALUE)
        .body(())
        .map_err(|error: http::Error| format!("request build failed: {error}"))?;
    let mut stream: h3::client::RequestStream<h3_quinn::BidiStream<Bytes>, Bytes> = client
        .send_request(request)
        .await
        .map_err(|error: h3::error::StreamError| format!("send_request failed: {error}"))?;
    if !body.is_empty() {
        stream
            .send_data(Bytes::copy_from_slice(body))
            .await
            .map_err(|error: h3::error::StreamError| format!("send_data failed: {error}"))?;
    }
    stream
        .finish()
        .await
        .map_err(|error: h3::error::StreamError| format!("finish failed: {error}"))?;
    let response: http::Response<()> = timeout(
        Duration::from_secs(REQUEST_TIMEOUT_SECS),
        stream.recv_response(),
    )
    .await
    .map_err(|_: tokio::time::error::Elapsed| "h3 response timed out".to_string())?
    .map_err(|error: h3::error::StreamError| format!("recv_response failed: {error}"))?;
    let status: u16 = response.status().as_u16();
    let version: http::Version = response.version();
    let mut collected: Vec<u8> = Vec::new();
    let deadline: Instant = Instant::now() + Duration::from_secs(REQUEST_TIMEOUT_SECS);
    loop {
        let remaining: Duration = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            return Err("h3 body timed out".to_string());
        }
        match timeout(remaining, stream.recv_data()).await {
            Ok(Ok(Some(chunk))) => collected.extend_from_slice(chunk.chunk()),
            Ok(Ok(None)) => break,
            Ok(Err(error)) => return Err(format!("h3 body read failed: {error}")),
            Err(_) => return Err("h3 body timed out".to_string()),
        }
    }
    drop(client);
    driving.abort();
    connection.close(quinn::VarInt::from_u32(0), b"done");
    Ok((status, version, collected, alpn))
}
