use super::*;

pub(crate) fn reserve_tcp_port() -> u16 {
    let listener: TcpListener = TcpListener::bind(format!("{LOOPBACK_HOST}:0"))
        .expect("loopback TCP port must be reservable");
    let port: u16 = listener
        .local_addr()
        .expect("reserved listener must report a local address")
        .port();
    drop(listener);
    port
}

pub(crate) fn reserve_udp_port() -> u16 {
    let socket: UdpSocket = UdpSocket::bind(format!("{LOOPBACK_HOST}:0"))
        .expect("loopback UDP port must be reservable");
    let port: u16 = socket
        .local_addr()
        .expect("reserved socket must report a local address")
        .port();
    drop(socket);
    port
}

pub(crate) fn protocol_config(
    address: &str,
    protocol: ServerProtocol,
    tls: Option<TlsConfig>,
) -> ServerConfig {
    let mut config: ServerConfig = ServerConfig::default();
    config
        .set_address(address)
        .set_protocol(protocol)
        .set_tls(tls);
    config
}

pub(crate) async fn start_protocol_server(
    protocol: ServerProtocol,
    tls: Option<TlsConfig>,
) -> Result<(ServerControlHook, u16), Box<ServerError>> {
    let mut server: Server = Server::default();
    let address: String = if protocol.is_quic() {
        format!("{LOOPBACK_HOST}:{}", reserve_udp_port())
    } else {
        format!("{LOOPBACK_HOST}:{}", reserve_tcp_port())
    };
    server.server_config(protocol_config(&address, protocol, tls));
    register_probe_hooks(&mut server);
    let control: ServerControlHook = server.run().await?;
    drop(server);
    sleep(Duration::from_millis(SERVER_BOOT_MILLIS)).await;
    let port: u16 = address
        .rsplit(':')
        .next()
        .and_then(|value: &str| value.parse::<u16>().ok())
        .expect("bind address must carry a numeric port");
    Ok((control, port))
}

pub(crate) async fn start_dropped_caller_server()
-> Result<(ServerControlHook, u16), Box<ServerError>> {
    let address: String = format!("{LOOPBACK_HOST}:{}", reserve_tcp_port());
    let control: ServerControlHook = {
        let mut server: Server = Server::default();
        server.server_config(protocol_config(&address, ServerProtocol::Http1, None));
        register_probe_hooks(&mut server);
        server.run().await?
    };
    sleep(Duration::from_millis(SERVER_BOOT_MILLIS)).await;
    let port: u16 = address
        .rsplit(':')
        .next()
        .and_then(|value: &str| value.parse::<u16>().ok())
        .expect("bind address must carry a numeric port");
    Ok((control, port))
}

pub(crate) async fn assert_wait_pending_then_complete(control: &ServerControlHook) {
    let probe: ServerControlHook = control.clone();
    let pending: JoinHandle<()> = spawn(async move {
        probe.wait().await;
    });
    tokio::time::sleep(Duration::from_millis(SHUTDOWN_PROBE_MILLIS)).await;
    assert!(
        !pending.is_finished(),
        "wait must still be pending before shutdown is requested"
    );
    control.shutdown().await;
    timeout(Duration::from_secs(SHUTDOWN_TIMEOUT_SECS), pending)
        .await
        .expect("wait must complete once shutdown has been requested")
        .expect("the wait task must not panic");
}

pub(crate) async fn connect_tcp(port: u16) -> io::Result<TcpStream> {
    timeout(
        Duration::from_secs(CONNECT_TIMEOUT_SECS),
        TcpStream::connect((LOOPBACK_HOST, port)),
    )
    .await
    .map_err(|_| io::Error::new(io::ErrorKind::TimedOut, "tcp connect timed out"))?
}

pub(crate) async fn raw_http1_exchange(port: u16, request: &[u8]) -> io::Result<Vec<u8>> {
    timeout(Duration::from_secs(REQUEST_TIMEOUT_SECS), async move {
        let mut stream: TcpStream = connect_tcp(port).await?;
        stream.write_all(request).await?;
        stream.flush().await?;
        let mut received: Vec<u8> = Vec::new();
        let mut chunk: [u8; 16 * 1024] = [0; 16 * 1024];
        loop {
            match stream.read(&mut chunk).await {
                Ok(0) => break,
                Ok(read) => received.extend_from_slice(&chunk[..read]),
                Err(error) if error.kind() == io::ErrorKind::UnexpectedEof => break,
                Err(error) => return Err(error),
            }
        }
        Ok(received)
    })
    .await
    .map_err(|_| io::Error::new(io::ErrorKind::TimedOut, "http1 exchange timed out"))?
}

pub(crate) fn response_head(raw: &[u8]) -> String {
    let text: String = String::from_utf8_lossy(raw).to_string();
    let end: usize = text.find(HTTP_DOUBLE_BR).unwrap_or(text.len());
    text[..end].to_string()
}

pub(crate) fn response_head_and_body(raw: &[u8]) -> (String, Vec<u8>) {
    match raw
        .windows(HTTP_DOUBLE_BR_BYTES.len())
        .position(|window: &[u8]| window == HTTP_DOUBLE_BR_BYTES)
    {
        Some(index) => (
            String::from_utf8_lossy(&raw[..index]).to_string(),
            raw[index + HTTP_DOUBLE_BR_BYTES.len()..].to_vec(),
        ),
        None => (String::from_utf8_lossy(raw).to_string(), Vec::new()),
    }
}

pub(crate) fn count_http1_responses(buffer: &[u8]) -> usize {
    let mut count: usize = 0;
    let mut position: usize = 0;
    while let Some(index) = buffer[position.min(buffer.len())..]
        .windows(HTTP_DOUBLE_BR_BYTES.len())
        .position(|window: &[u8]| window == HTTP_DOUBLE_BR_BYTES)
    {
        let head_end: usize = position + index;
        let head: String = String::from_utf8_lossy(&buffer[position..head_end]).to_string();
        let mut content_length: Option<usize> = None;
        for line in head.split(HTTP_BR) {
            if let Some((key, value)) = line.split_once(':')
                && key.trim().eq_ignore_ascii_case(CONTENT_LENGTH)
            {
                content_length = value.trim().parse::<usize>().ok();
            }
        }
        let body_start: usize = head_end + HTTP_DOUBLE_BR_BYTES.len();
        match content_length {
            Some(length) if buffer.len() >= body_start + length => {
                count += 1;
                position = body_start + length;
            }
            _ => break,
        }
    }
    count
}

pub(crate) async fn read_http1_head<T>(
    stream: &mut T,
    expected_body: usize,
) -> io::Result<(String, Vec<u8>)>
where
    T: tokio::io::AsyncRead + Unpin,
{
    let mut collected: Vec<u8> = Vec::new();
    let mut chunk: [u8; 16 * 1024] = [0; 16 * 1024];
    let deadline: Instant = Instant::now() + Duration::from_secs(REQUEST_TIMEOUT_SECS);
    while Instant::now() < deadline {
        if let Some(index) = collected
            .windows(HTTP_DOUBLE_BR_BYTES.len())
            .position(|window: &[u8]| window == HTTP_DOUBLE_BR_BYTES)
        {
            let body_start: usize = index + HTTP_DOUBLE_BR_BYTES.len();
            if collected.len() >= body_start + expected_body {
                break;
            }
        }
        let bound: Duration = Duration::from_secs(REQUEST_TIMEOUT_SECS);
        let read: usize = match timeout(bound, stream.read(&mut chunk)).await {
            Ok(Ok(count)) => count,
            Ok(Err(error)) => return Err(error),
            Err(_) => break,
        };
        if read == 0 {
            break;
        }
        collected.extend_from_slice(&chunk[..read]);
    }
    Ok(response_head_and_body(&collected))
}

pub(crate) async fn write_bounded<T>(stream: &mut T, data: &[u8]) -> io::Result<()>
where
    T: tokio::io::AsyncWrite + Unpin,
{
    timeout(
        Duration::from_secs(REQUEST_TIMEOUT_SECS),
        stream.write_all(data),
    )
    .await
    .map_err(|_: tokio::time::error::Elapsed| {
        io::Error::new(io::ErrorKind::TimedOut, "socket write timed out")
    })?
}

pub(crate) async fn exchange_on(stream: &mut TcpStream, request: &[u8]) -> Vec<u8> {
    write_bounded(stream, request)
        .await
        .expect("request write must not time out");
    stream.flush().await.expect("request flush must succeed");
    let mut received: Vec<u8> = Vec::new();
    let deadline: Instant = Instant::now() + Duration::from_secs(REQUEST_TIMEOUT_SECS);
    let mut chunk: [u8; 16 * 1024] = [0; 16 * 1024];
    while Instant::now() < deadline {
        match stream.read(&mut chunk).await {
            Ok(0) => break,
            Ok(read) => received.extend_from_slice(&chunk[..read]),
            Err(_) => break,
        }
    }
    received
}
