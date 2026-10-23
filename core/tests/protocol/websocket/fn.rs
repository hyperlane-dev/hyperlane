use super::*;

fn websocket_upgrade_request(key: &str) -> Vec<u8> {
    let mut head: String =
        format!("GET {WEBSOCKET_ROUTE_PATH} HTTP/1.1{HTTP_BR}Host: {TLS_SERVER_NAME}{HTTP_BR}");
    head.push_str(&format!("{UPGRADE}: {WEBSOCKET_UPGRADE_VALUE}{HTTP_BR}"));
    head.push_str(&format!(
        "{CONNECTION}: {WEBSOCKET_CONNECTION_VALUE}{HTTP_BR}"
    ));
    head.push_str(&format!("{SEC_WEBSOCKET_VERSION}: "));
    head.push_str(WEBSOCKET_PROTOCOL_VERSION);
    head.push_str(HTTP_BR);
    head.push_str(&format!("{SEC_WEBSOCKET_KEY}: {key}{HTTP_BR}"));
    head.push_str(HTTP_BR);
    head.into_bytes()
}

fn client_masked_frame(opcode: u8, payload: &[u8], mask: [u8; 4]) -> Vec<u8> {
    let mut frame: Vec<u8> = vec![WEBSOCKET_FIN_BIT | opcode];
    if payload.len() < WEBSOCKET_EXTENDED_LENGTH_MARK as usize {
        frame.push(WEBSOCKET_MASK_BIT | payload.len() as u8);
    } else if payload.len() <= WEBSOCKET_MAX_SHORT_FRAME {
        frame.push(WEBSOCKET_MASK_BIT | WEBSOCKET_EXTENDED_LENGTH_MARK);
        frame.extend_from_slice(&(payload.len() as u16).to_be_bytes());
    } else {
        frame.push(WEBSOCKET_MASK_BIT | WEBSOCKET_MAX_LENGTH_MARK);
        frame.extend_from_slice(&(payload.len() as u64).to_be_bytes());
    }
    frame.extend_from_slice(&mask);
    for (index, byte) in payload.iter().enumerate() {
        frame.push(byte ^ mask[index % 4]);
    }
    frame
}

fn decode_unmasked_frame(raw: &[u8]) -> Option<(Vec<u8>, Vec<u8>)> {
    if raw.len() < 2 {
        return None;
    }
    if raw[1] & WEBSOCKET_MASK_BIT != 0 {
        panic!("a server frame must never be masked, got: {raw:?}");
    }
    let short_length: usize = (raw[1] & !WEBSOCKET_MASK_BIT) as usize;
    let mut cursor: usize = 2;
    let payload_len: usize = if short_length == WEBSOCKET_EXTENDED_LENGTH_MARK as usize {
        if raw.len() < cursor + 2 {
            return None;
        }
        let value: usize = u16::from_be_bytes([raw[cursor], raw[cursor + 1]]) as usize;
        cursor += 2;
        value
    } else {
        short_length
    };
    if raw.len() < cursor + payload_len {
        return None;
    }
    Some((
        raw[..cursor].to_vec(),
        raw[cursor..cursor + payload_len].to_vec(),
    ))
}

async fn read_websocket_frame<T>(stream: &mut T) -> io::Result<(Vec<u8>, Vec<u8>)>
where
    T: AsyncRead + Unpin,
{
    let bound: Duration = Duration::from_secs(WEBSOCKET_EXCHANGE_TIMEOUT_SECS);
    let mut collected: Vec<u8> = Vec::new();
    loop {
        if let Some(frame) = decode_unmasked_frame(&collected) {
            return Ok(frame);
        }
        let mut chunk: [u8; WEBSOCKET_READ_CHUNK_LEN] = [0; WEBSOCKET_READ_CHUNK_LEN];
        let read: usize = match timeout(bound, stream.read(&mut chunk)).await {
            Ok(Ok(count)) => count,
            Ok(Err(error)) => return Err(error),
            Err(_) => {
                return Err(io::Error::new(
                    io::ErrorKind::TimedOut,
                    "the echoed websocket frame did not arrive inside its timeout",
                ));
            }
        };
        if read == 0 {
            return Err(io::Error::new(
                io::ErrorKind::UnexpectedEof,
                "the connection ended before the echoed frame arrived",
            ));
        }
        collected.extend_from_slice(&chunk[..read]);
    }
}

async fn open_websocket<T>(stream: &mut T, key: &str) -> io::Result<String>
where
    T: AsyncRead + AsyncWrite + Unpin,
{
    let request: Vec<u8> = websocket_upgrade_request(key);
    write_bounded(stream, &request).await?;
    stream.flush().await?;
    let (head, _leftover): (String, Vec<u8>) = read_http1_head(stream, 0).await?;
    if !head.starts_with(WEBSOCKET_SWITCHING_LINE) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidData,
            format!("the handshake must switch protocols, got: {head}"),
        ));
    }
    Ok(head)
}

async fn echo_one_masked_frame<T>(stream: &mut T, payload: &[u8]) -> io::Result<(Vec<u8>, Vec<u8>)>
where
    T: AsyncRead + AsyncWrite + Unpin,
{
    let frame: Vec<u8> =
        client_masked_frame(WEBSOCKET_BINARY_OPCODE, payload, WEBSOCKET_CLIENT_MASK);
    write_bounded(stream, &frame).await?;
    stream.flush().await?;
    read_websocket_frame(stream).await
}

async fn close_websocket<T>(stream: &mut T) -> io::Result<()>
where
    T: AsyncWrite + Unpin,
{
    let frame: Vec<u8> = client_masked_frame(WEBSOCKET_CLOSE_OPCODE, &[], WEBSOCKET_CLIENT_MASK);
    write_bounded(stream, &frame).await?;
    stream.flush().await?;
    let bound: Duration = Duration::from_secs(WEBSOCKET_EXCHANGE_TIMEOUT_SECS);
    match timeout(bound, stream.shutdown()).await {
        Ok(Ok(())) => Ok(()),
        Ok(Err(error)) => Err(error),
        Err(_) => Err(io::Error::new(
            io::ErrorKind::TimedOut,
            "the client half close did not finish inside its timeout",
        )),
    }
}

async fn start_websocket_server(
    tls: Option<TlsConfig>,
) -> Result<(ServerControlHook, u16), Box<ServerError>> {
    let mut config: ServerConfig = ServerConfig::default();
    config.set_tls(tls);
    let mut server: Server = Server::default();
    server
        .server_config(config)
        .request_middleware::<WebsocketUpgradeMiddleware>()
        .route::<WebsocketEchoRoute>(WEBSOCKET_ROUTE_PATH);
    let (control, port): (ServerControlHook, u16) = run_on_reserved_port(&mut server).await?;
    drop(server);
    let probe: io::Result<TcpStream> = connect_tcp(port).await;
    assert!(
        probe.is_ok(),
        "the loopback websocket listener must accept once run resolved"
    );
    Ok((control, port))
}

fn assert_websocket_head(head: &str) {
    assert!(
        head.starts_with(WEBSOCKET_SWITCHING_LINE),
        "an upgraded request must answer 101, got: {head}"
    );
    assert_eq!(
        response_header_value(head, SEC_WEBSOCKET_ACCEPT),
        RFC6455_SAMPLE_ACCEPT,
        "the accept key must be the rfc6455 value for this key, got: {head}"
    );
    assert_eq!(
        response_header_value(head, UPGRADE),
        WEBSOCKET_UPGRADE_VALUE,
        "the upgraded response must name the websocket protocol, got: {head}"
    );
    assert_eq!(
        response_header_value(head, CONNECTION).to_ascii_lowercase(),
        WEBSOCKET_CONNECTION_VALUE.to_ascii_lowercase(),
        "the upgraded response must switch the connection, got: {head}"
    );
}

fn assert_echoed_binary_frame(head: Vec<u8>, echoed: Vec<u8>, sent: &[u8]) {
    let mut expected_head: Vec<u8> = vec![WEBSOCKET_FIN_BIT | WEBSOCKET_BINARY_OPCODE];
    let payload_len: usize = sent.len();
    if payload_len < WEBSOCKET_EXTENDED_LENGTH_MARK as usize {
        expected_head.push(payload_len as u8);
    } else {
        expected_head.push(WEBSOCKET_EXTENDED_LENGTH_MARK);
        expected_head.extend_from_slice(&(payload_len as u16).to_be_bytes());
    }
    assert_eq!(
        head, expected_head,
        "the echo must be one final unmasked binary frame with the rfc6455 length encoding"
    );
    assert_eq!(
        echoed.len(),
        payload_len,
        "the echoed frame must carry every byte the client sent"
    );
    assert_eq!(
        echoed, sent,
        "hostile binary bytes must survive the upgrade unmasked and byte exact"
    );
}

fn assert_port_released(port: u16) {
    let rebound: io::Result<TcpListener> = TcpListener::bind((LOOPBACK_HOST, port));
    assert!(
        rebound.is_ok(),
        "a stopped websocket server must release tcp port {port}, got: {:?}",
        rebound.err()
    );
}

#[tokio::test]
async fn websocket_plaintext_upgrade_echoes_hostile_binary_frames() {
    let (control, port): (ServerControlHook, u16) = start_websocket_server(None)
        .await
        .expect("the default plaintext http1 websocket server must start");
    let bound: Duration = Duration::from_secs(WEBSOCKET_EXCHANGE_TIMEOUT_SECS);
    let outcome: io::Result<()> = match timeout(bound, async {
        let mut stream: TcpStream = connect_tcp(port).await?;
        let head: String = open_websocket(&mut stream, RFC6455_SAMPLE_KEY).await?;
        assert_websocket_head(&head);
        let sent: Vec<u8> = hostile_binary_payload();
        let (frame_head, echoed): (Vec<u8>, Vec<u8>) =
            echo_one_masked_frame(&mut stream, &sent).await?;
        assert_echoed_binary_frame(frame_head, echoed, &sent);
        close_websocket(&mut stream).await?;
        Ok(())
    })
    .await
    {
        Ok(value) => value,
        Err(_) => panic!("the plaintext websocket exchange must finish inside its timeout"),
    };
    assert!(
        outcome.is_ok(),
        "the plaintext websocket exchange failed: {outcome:?}"
    );
    assert_wait_pending_then_complete(&control).await;
    assert_port_released(port);
}

#[tokio::test]
async fn websocket_tls_upgrade_echoes_hostile_binary_frames() {
    let (control, port): (ServerControlHook, u16) =
        start_websocket_server(Some(trusted_tls_config()))
            .await
            .expect("the default http1 websocket server must start over tls");
    let bound: Duration = Duration::from_secs(WEBSOCKET_EXCHANGE_TIMEOUT_SECS);
    let outcome: io::Result<()> = match timeout(bound, async {
        let mut stream: ClientTlsStream = connect_tls(port, ALPN_HTTP1).await?;
        let alpn: Option<Vec<u8>> = negotiated_alpn(&stream);
        assert_eq!(
            alpn,
            Some(ALPN_HTTP1.to_vec()),
            "the tls websocket server must negotiate http/1.1"
        );
        let head: String = open_websocket(&mut stream, RFC6455_SAMPLE_KEY).await?;
        assert_websocket_head(&head);
        let sent: Vec<u8> = hostile_binary_payload();
        let (frame_head, echoed): (Vec<u8>, Vec<u8>) =
            echo_one_masked_frame(&mut stream, &sent).await?;
        assert_echoed_binary_frame(frame_head, echoed, &sent);
        close_websocket(&mut stream).await?;
        Ok(())
    })
    .await
    {
        Ok(value) => value,
        Err(_) => panic!("the tls websocket exchange must finish inside its timeout"),
    };
    assert!(
        outcome.is_ok(),
        "the tls websocket exchange failed: {outcome:?}"
    );
    assert_wait_pending_then_complete(&control).await;
    assert_port_released(port);
}
#[test]
fn the_client_encoder_reproduces_the_rfc6455_masked_hello_frame() {
    let frame: Vec<u8> = client_masked_frame(
        WEBSOCKET_TEXT_OPCODE,
        RFC6455_HELLO_PAYLOAD.as_bytes(),
        RFC6455_HELLO_MASK,
    );
    assert_eq!(
        frame,
        RFC6455_HELLO_FRAME.to_vec(),
        "the client encoder must emit the exact masked frame bytes rfc6455 publishes"
    );
}

#[test]
fn a_client_frame_never_carries_a_length_byte_of_its_own() {
    let short_frame: Vec<u8> =
        client_masked_frame(WEBSOCKET_BINARY_OPCODE, b"abc", RFC6455_HELLO_MASK);
    assert_eq!(
        short_frame[1],
        WEBSOCKET_MASK_BIT | 3,
        "the mask bit and the seven bit length share the second header byte"
    );
    assert_eq!(
        short_frame.len(),
        9,
        "2 header bytes, 4 mask bytes, 3 payload bytes"
    );
    let long_frame: Vec<u8> = client_masked_frame(
        WEBSOCKET_BINARY_OPCODE,
        &hostile_binary_payload(),
        WEBSOCKET_CLIENT_MASK,
    );
    assert_eq!(
        long_frame[1],
        WEBSOCKET_MASK_BIT | WEBSOCKET_EXTENDED_LENGTH_MARK,
        "a payload past 125 bytes announces its length with the 126 marker"
    );
    assert_eq!(
        &long_frame[2..4],
        &(hostile_binary_payload().len() as u16).to_be_bytes(),
        "the extended length must be the big endian payload size"
    );
}
