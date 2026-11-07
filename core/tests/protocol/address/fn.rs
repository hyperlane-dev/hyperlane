use super::*;

pub(crate) async fn start_target_server(
    protocol: ServerProtocol,
    tls: Option<TlsConfig>,
) -> Result<(ServerControlHook, u16), Box<ServerError>> {
    let mut server: Server = Server::default();
    server.server_config(protocol_config(PLACEHOLDER_ADDRESS, protocol, tls));
    register_probe_hooks(&mut server);
    server.route::<TargetEchoRoute>(TARGET_ROUTE_PATH);
    let (control, port): (ServerControlHook, u16) = run_on_reserved_port(&mut server).await?;
    drop(server);
    sleep(Duration::from_millis(SERVER_BOOT_MILLIS)).await;
    Ok((control, port))
}

fn http1_target_request(target: &str) -> Vec<u8> {
    let mut head: String = format!("GET {target} HTTP/1.1{HTTP_BR}");
    head.push_str(&format!("Host: {TLS_SERVER_NAME}{HTTP_BR}"));
    head.push_str(&format!(
        "{}: {}{}",
        TARGET_SPOOF_HEADER, TARGET_SPOOF_VALUE, HTTP_BR
    ));
    head.push_str(&format!("Content-Length: 0{HTTP_BR}"));
    head.push_str(HTTP_BR);
    head.into_bytes()
}

pub(crate) async fn http1_target_reply(stream: &mut TcpStream, target: &str) -> (String, Vec<u8>) {
    write_bounded(stream, &http1_target_request(target))
        .await
        .expect("the http1 target request must be written");
    read_http1_head(stream, target.len())
        .await
        .expect("the http1 target response must arrive")
}

pub(crate) async fn h3_targets_on_one_connection(
    port: u16,
    targets: [&str; 2],
) -> Result<Vec<(u16, http::Version, Vec<u8>)>, String> {
    let (_endpoint, connection): (quinn::Endpoint, quinn::Connection) =
        connect_quic(port, ALPN_HTTP3).await?;
    let (mut driver, mut client) = h3::client::new(h3_quinn::Connection::new(connection.clone()))
        .await
        .map_err(|error: h3::error::ConnectionError| format!("h3 handshake failed: {error}"))?;
    let driving: JoinHandle<()> = spawn(async move {
        let _final: h3::error::ConnectionError = driver.wait_idle().await;
    });
    let mut replies: Vec<(u16, http::Version, Vec<u8>)> = Vec::new();
    for target in targets {
        let uri: String = format!("https://{TLS_SERVER_NAME}:{port}{target}");
        let request: http::Request<()> = http::Request::builder()
            .method("GET")
            .uri(uri.as_str())
            .header(PROBE_HEADER_KEY, PROBE_HEADER_VALUE)
            .body(())
            .map_err(|error: http::Error| format!("request build failed: {error}"))?;
        let mut stream: h3::client::RequestStream<h3_quinn::BidiStream<Bytes>, Bytes> = client
            .send_request(request)
            .await
            .map_err(|error: h3::error::StreamError| format!("send_request failed: {error}"))?;
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
        replies.push((status, version, collected));
    }
    drop(client);
    driving.abort();
    connection.close(quinn::VarInt::from_u32(0), b"done");
    Ok(replies)
}
