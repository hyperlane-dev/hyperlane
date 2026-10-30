use super::*;

pub(crate) async fn h2_plaintext_session(
    port: u16,
) -> Result<
    (
        h2::client::SendRequest<Bytes>,
        h2::client::Connection<TcpStream, Bytes>,
    ),
    String,
> {
    let socket: TcpStream = connect_tcp(port)
        .await
        .map_err(|error: io::Error| format!("h2 plaintext connect failed: {error}"))?;
    timeout(
        Duration::from_secs(CONNECT_TIMEOUT_SECS),
        h2::client::handshake(socket),
    )
    .await
    .map_err(|_| "h2 plaintext handshake timed out".to_string())?
    .map_err(|error: h2::Error| format!("h2 plaintext handshake failed: {error}"))
}

pub(crate) async fn h2_tls_alpn(port: u16) -> Result<(Option<Vec<u8>>, H2TlsSession), String> {
    let stream: ClientTlsStream = connect_tls(port, ALPN_HTTP2)
        .await
        .map_err(|error: io::Error| format!("h2 tls connect failed: {error}"))?;
    let alpn: Option<Vec<u8>> = negotiated_alpn(&stream);
    let (send, connection) = timeout(
        Duration::from_secs(CONNECT_TIMEOUT_SECS),
        h2::client::handshake(stream),
    )
    .await
    .map_err(|_| "h2 tls handshake timed out".to_string())?
    .map_err(|error: h2::Error| format!("h2 tls handshake failed: {error}"))?;
    Ok((alpn, (send, connection)))
}

pub(crate) async fn h2_request(
    sender: &mut h2::client::SendRequest<Bytes>,
    method: &str,
    target: &str,
    body: &[u8],
) -> Result<H2Reply, String> {
    let request: http::Request<()> = http::Request::builder()
        .method(method)
        .uri(target)
        .header(PROBE_HEADER_KEY, PROBE_HEADER_VALUE)
        .body(())
        .map_err(|error: http::Error| format!("h2 request build failed: {error}"))?;
    let (response, mut send_stream) = sender
        .send_request(request, body.is_empty())
        .map_err(|error: h2::Error| format!("h2 send_request failed: {error}"))?;
    if !body.is_empty() {
        send_stream
            .send_data(Bytes::copy_from_slice(body), true)
            .map_err(|error: h2::Error| format!("h2 send_data failed: {error}"))?;
    }
    let response: http::Response<h2::RecvStream> =
        timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS), response)
            .await
            .map_err(|_| "h2 response timed out".to_string())?
            .map_err(|error: h2::Error| format!("h2 response failed: {error}"))?;
    let status: u16 = response.status().as_u16();
    let version: http::Version = response.version();
    let mut received: h2::RecvStream = response.into_body();
    let mut collected: Vec<u8> = Vec::new();
    let deadline: Instant = Instant::now() + Duration::from_secs(REQUEST_TIMEOUT_SECS);
    loop {
        let remaining: Duration = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            return Err("h2 body timed out".to_string());
        }
        match timeout(remaining, received.data()).await {
            Ok(Some(Ok(chunk))) => {
                let _released: Result<(), h2::Error> =
                    received.flow_control().release_capacity(chunk.len());
                collected.extend_from_slice(&chunk);
            }
            Ok(Some(Err(error))) => return Err(format!("h2 body read failed: {error}")),
            Ok(None) => break,
            Err(_) => return Err("h2 body timed out".to_string()),
        }
    }
    Ok((status, version, collected))
}

pub(crate) async fn h2_request_with_headers(
    sender: &mut h2::client::SendRequest<Bytes>,
    method: &str,
    target: &str,
    body: &[u8],
) -> Result<H2HeadReply, String> {
    let request: http::Request<()> = http::Request::builder()
        .method(method)
        .uri(target)
        .header(PROBE_HEADER_KEY, PROBE_HEADER_VALUE)
        .body(())
        .map_err(|error: http::Error| format!("h2 request build failed: {error}"))?;
    let (response, mut send_stream) = sender
        .send_request(request, body.is_empty())
        .map_err(|error: h2::Error| format!("h2 send_request failed: {error}"))?;
    if !body.is_empty() {
        send_stream
            .send_data(Bytes::copy_from_slice(body), true)
            .map_err(|error: h2::Error| format!("h2 send_data failed: {error}"))?;
    }
    let response: http::Response<h2::RecvStream> =
        timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS), response)
            .await
            .map_err(|_| "h2 response timed out".to_string())?
            .map_err(|error: h2::Error| format!("h2 response failed: {error}"))?;
    let status: u16 = response.status().as_u16();
    let headers: http::HeaderMap = response.headers().clone();
    let mut received: h2::RecvStream = response.into_body();
    let mut collected: Vec<u8> = Vec::new();
    let deadline: Instant = Instant::now() + Duration::from_secs(REQUEST_TIMEOUT_SECS);
    loop {
        let remaining: Duration = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            return Err("h2 body timed out".to_string());
        }
        match timeout(remaining, received.data()).await {
            Ok(Some(Ok(chunk))) => {
                let _released: Result<(), h2::Error> =
                    received.flow_control().release_capacity(chunk.len());
                collected.extend_from_slice(&chunk);
            }
            Ok(Some(Err(error))) => return Err(format!("h2 body read failed: {error}")),
            Ok(None) => break,
            Err(_) => return Err("h2 body timed out".to_string()),
        }
    }
    Ok((status, headers, collected))
}

pub(crate) async fn h2_stream_until(port: u16, target: &str, needle: &str) -> (Vec<u8>, bool) {
    let (mut sender, connection) = h2_plaintext_session(port)
        .await
        .expect("h2 handshake must succeed");
    let driving: JoinHandle<()> = spawn(async move {
        let _result: Result<(), h2::Error> = connection.await;
    });
    let request: http::Request<()> = http::Request::builder()
        .method("GET")
        .uri(target)
        .body(())
        .expect("request must build");
    let (response, _send_stream) = sender
        .send_request(request, true)
        .expect("h2 request must be accepted");
    let response: http::Response<h2::RecvStream> =
        timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS), response)
            .await
            .expect("h2 response head must arrive")
            .expect("h2 response must not fail");
    let mut body: h2::RecvStream = response.into_body();
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
        match timeout(remaining, body.data()).await {
            Ok(Some(Ok(chunk))) => {
                let _released: Result<(), h2::Error> =
                    body.flow_control().release_capacity(chunk.len());
                collected.extend_from_slice(&chunk);
            }
            Ok(Some(Err(_))) => break,
            Ok(None) => break,
            Err(_) => break,
        }
    }
    drop(sender);
    driving.abort();
    (collected, matched)
}

pub(crate) async fn h2_request_with_host_header(
    sender: &mut h2::client::SendRequest<Bytes>,
    method: &str,
    target: &str,
    host: &str,
) -> Result<Option<(u16, Vec<u8>)>, String> {
    let request: http::Request<()> = http::Request::builder()
        .method(method)
        .uri(target)
        .header(HOST, host)
        .body(())
        .map_err(|error: http::Error| format!("h2 request build failed: {error}"))?;
    let (response, _send_stream) = sender
        .send_request(request, true)
        .map_err(|error: h2::Error| format!("h2 send_request failed: {error}"))?;
    let response: http::Response<h2::RecvStream> =
        match timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS), response).await {
            Ok(Ok(response)) => response,
            Ok(Err(_error)) => return Ok(None),
            Err(_) => return Err("h2 response timed out".to_string()),
        };
    let status: u16 = response.status().as_u16();
    let mut received: h2::RecvStream = response.into_body();
    let mut collected: Vec<u8> = Vec::new();
    let deadline: Instant = Instant::now() + Duration::from_secs(REQUEST_TIMEOUT_SECS);
    loop {
        let remaining: Duration = deadline.saturating_duration_since(Instant::now());
        if remaining.is_zero() {
            return Err("h2 body timed out".to_string());
        }
        match timeout(remaining, received.data()).await {
            Ok(Some(Ok(chunk))) => {
                let _released: Result<(), h2::Error> =
                    received.flow_control().release_capacity(chunk.len());
                collected.extend_from_slice(&chunk);
            }
            Ok(Some(Err(_))) => return Ok(None),
            Ok(None) => break,
            Err(_) => return Err("h2 body timed out".to_string()),
        }
    }
    Ok(Some((status, collected)))
}
