use super::*;

async fn start_server(server: &mut Server) -> Result<(ServerControlHook, u16), String> {
    let mut attempt: usize = 1;
    loop {
        let listener: TcpListener = TcpListener::bind((LOOPBACK, 0))
            .map_err(|error: io::Error| format!("loopback port must be reservable: {error}"))?;
        let port: u16 = listener
            .local_addr()
            .map_err(|error: io::Error| format!("listener must have an address: {error}"))?
            .port();
        let mut config: ServerConfig = ServerConfig::default();
        config
            .set_address(format!("{LOOPBACK}:{port}"))
            .set_protocol(ServerProtocol::Http1);
        server.server_config(config);
        drop(listener);
        match server.run().await {
            Ok(control) => return Ok((control, port)),
            Err(error)
                if attempt < BIND_ATTEMPTS && matches!(error.as_ref(), ServerError::TcpBind(_)) =>
            {
                attempt += 1;
            }
            Err(error) => return Err(format!("HTTP/1.1 server must start: {error}")),
        }
    }
}

async fn exchange(server: &mut Server, request: &str) -> Result<Vec<u8>, String> {
    let (control, port): (ServerControlHook, u16) = timeout(WIRE_TIMEOUT, start_server(server))
        .await
        .map_err(|_: Elapsed| "HTTP/1.1 server startup must finish".to_owned())??;
    let result: Result<io::Result<Vec<u8>>, Elapsed> = timeout(WIRE_TIMEOUT, async {
        let mut stream: TcpStream = TcpStream::connect((LOOPBACK, port)).await?;
        stream.write_all(request.as_bytes()).await?;
        let mut response: Vec<u8> = Vec::new();
        stream.read_to_end(&mut response).await?;
        Ok(response)
    })
    .await;
    timeout(WIRE_TIMEOUT, async {
        control.shutdown().await;
        control.wait().await;
    })
    .await
    .map_err(|_: Elapsed| "HTTP/1.1 server shutdown must finish".to_owned())?;
    result
        .map_err(|_: Elapsed| "pipelined exchange must reach EOF".to_owned())?
        .map_err(|error: io::Error| format!("pipelined exchange must succeed: {error}"))
}

fn response_body<'a>(
    response: &'a str,
    request: &str,
    expected_length: usize,
) -> Result<&'a str, String> {
    let (headers, body): (&str, &str) = response
        .split_once(HTTP_DOUBLE_BR)
        .ok_or_else(|| format!("{request} must have a complete header block: {response:?}"))?;
    let lengths: Vec<&str> = headers
        .split(HTTP_BR)
        .filter_map(|line: &str| line.split_once(':'))
        .filter_map(|(name, value): (&str, &str)| {
            name.eq_ignore_ascii_case(CONTENT_LENGTH)
                .then_some(value.trim())
        })
        .collect();
    if lengths != [expected_length.to_string().as_str()] {
        return Err(format!(
            "{request} must advertise the GET representation length exactly once, got {lengths:?}"
        ));
    }
    Ok(body)
}

#[tokio::test]
async fn http1_head_no_arg_send_attributes_preserve_length_without_body() -> Result<(), String> {
    let mut server: Server = Server::default();
    server
        .route::<SendRoute>(SEND_PATH)
        .route::<TrySendRoute>(TRY_SEND_PATH);
    let request: String = format!(
        "HEAD {SEND_PATH} HTTP/1.1\r\nHost: {LOCALHOST}\r\n\r\n\
         HEAD {TRY_SEND_PATH} HTTP/1.1\r\nHost: {LOCALHOST}\r\n\r\n\
         GET {SEND_PATH} HTTP/1.1\r\nHost: {LOCALHOST}\r\n\r\n\
         GET {TRY_SEND_PATH} HTTP/1.1\r\nHost: {LOCALHOST}\r\nConnection: close\r\n\r\n"
    );
    let raw: Vec<u8> = exchange(&mut server, &request).await?;
    let wire: &str = from_utf8(&raw).map_err(|error: Utf8Error| error.to_string())?;
    let responses: Vec<&str> = wire.split(OK_STATUS_LINE).collect();
    let ["", send_head, try_send_head, send_get, try_send_get]: &[&str] = responses.as_slice()
    else {
        return Err(format!(
            "expected four HTTP/1.1 200 OK responses in order, got {wire:?}"
        ));
    };
    let bodies: [&str; 4] = [
        response_body(send_head, "HEAD via #[send]", SEND_BODY.len())?,
        response_body(try_send_head, "HEAD via #[try_send]", TRY_SEND_BODY.len())?,
        response_body(send_get, "GET via #[send]", SEND_BODY.len())?,
        response_body(try_send_get, "GET via #[try_send]", TRY_SEND_BODY.len())?,
    ];
    if bodies != ["", "", SEND_BODY, TRY_SEND_BODY] {
        return Err(format!(
            "HEAD must send no bytes after its headers; the pipelined GETs must still send their bodies, got {bodies:?}"
        ));
    }
    Ok(())
}

#[tokio::test]
async fn http1_head_explicit_send_attributes_write_bytes_verbatim() -> Result<(), String> {
    let mut server: Server = Server::default();
    server
        .route::<RawSendRoute>(RAW_SEND_PATH)
        .route::<RawTrySendRoute>(RAW_TRY_SEND_PATH);
    let request: String = format!(
        "HEAD {RAW_SEND_PATH} HTTP/1.1\r\nHost: {LOCALHOST}\r\n\r\n\
         HEAD {RAW_TRY_SEND_PATH} HTTP/1.1\r\nHost: {LOCALHOST}\r\nConnection: close\r\n\r\n"
    );
    let raw: Vec<u8> = exchange(&mut server, &request).await?;
    let expected: Vec<u8> = format!("{RAW_SEND_RESPONSE}{RAW_TRY_SEND_RESPONSE}").into_bytes();
    if raw != expected {
        return Err(format!(
            "explicit send attributes must write their bytes verbatim, got {raw:?}"
        ));
    }
    Ok(())
}
