use super::*;

fn test_server() -> Server {
    let mut server: Server = Server::default();
    server
        .route::<LargeRoute>(LARGE_PATH)
        .route::<SmallRoute>(SMALL_PATH)
        .route::<CodedRoute>(CODED_PATH)
        .route::<StreamRoute>(STREAM_PATH)
        .route::<NoContentRoute>(NO_CONTENT_PATH)
        .route::<VaryRoute>(VARY_PATH)
        .route::<ValidatedRoute>(VALIDATED_PATH)
        .response_middleware::<CompressMiddleware>()
        .response_middleware::<ConditionalMiddleware>()
        .response_middleware::<SenderMiddleware>();
    server
}

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
        .map_err(|_: Elapsed| "server startup must finish".to_owned())??;
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
    .map_err(|_: Elapsed| "server shutdown must finish".to_owned())?;
    result
        .map_err(|_: Elapsed| "exchange must reach EOF".to_owned())?
        .map_err(|error: io::Error| format!("exchange must succeed: {error}"))
}

fn get_request(path: &str, fields: &str) -> String {
    format!("GET {path} {HTTP_ONE}\r\nHost: {LOOPBACK}\r\n{fields}Connection: close\r\n\r\n")
}

fn method_request(method: &str, path: &str, fields: &str) -> String {
    format!("{method} {path} {HTTP_ONE}\r\nHost: {LOOPBACK}\r\n{fields}Connection: close\r\n\r\n")
}

fn response_status(wire: &[u8]) -> Result<usize, String> {
    let (head, _): (&[u8], &[u8]) = split_wire(wire)?;
    let text: &str =
        from_utf8(head).map_err(|error: Utf8Error| format!("head must be text: {error}"))?;
    let line: &str = text.split(HTTP_BR).next().unwrap_or_default();
    let code: &str = line.split(' ').nth(1).unwrap_or_default();
    code.parse::<usize>()
        .map_err(|error: ParseIntError| format!("status must be a number: {error}"))
}

fn split_wire(wire: &[u8]) -> Result<(&[u8], &[u8]), String> {
    let separator: &[u8] = HTTP_DOUBLE_BR.as_bytes();
    let index: usize = wire
        .windows(separator.len())
        .position(|window: &[u8]| window == separator)
        .ok_or_else(|| "response must carry a header block".to_owned())?;
    Ok((&wire[..index], &wire[index + separator.len()..]))
}

fn header_lines(head: &[u8]) -> Result<Vec<(&str, &str)>, String> {
    let text: &str =
        from_utf8(head).map_err(|error: Utf8Error| format!("head must be text: {error}"))?;
    let mut lines: Vec<(&str, &str)> = Vec::new();
    for line in text.split(HTTP_BR).skip(1) {
        if let Some((name, value)) = line.split_once(COLON) {
            lines.push((name.trim(), value.trim()));
        }
    }
    Ok(lines)
}

fn header_value<'a>(lines: &[(&'a str, &'a str)], name: &str) -> Option<&'a str> {
    lines
        .iter()
        .find(|(key, _): &&(&str, &str)| key.eq_ignore_ascii_case(name))
        .map(|(_, value): &(&str, &str)| *value)
}

fn header_count(lines: &[(&str, &str)], name: &str) -> usize {
    lines
        .iter()
        .filter(|(key, _): &&(&str, &str)| key.eq_ignore_ascii_case(name))
        .count()
}

fn assert_coding(wire: &[u8], token: &str) -> Result<Vec<u8>, String> {
    let (head, body): (&[u8], &[u8]) = split_wire(wire)?;
    let lines: Vec<(&str, &str)> = header_lines(head)?;
    let coding: Option<&str> = header_value(&lines, CONTENT_ENCODING);
    if coding != Some(token) {
        return Err(format!("response must declare {token}, got {coding:?}"));
    }
    let vary: Option<&str> = header_value(&lines, VARY);
    if vary != Some(VARY_ACCEPT_ENCODING) {
        return Err(format!("response must vary on the coding, got {vary:?}"));
    }
    let length: Option<&str> = header_value(&lines, CONTENT_LENGTH);
    if length != Some(body.len().to_string().as_str()) {
        return Err(format!(
            "response must advertise the coded length, got {length:?} for {}",
            body.len()
        ));
    }
    Ok(body.to_vec())
}

fn assert_header(wire: &[u8], name: &str, value: &str) -> Result<(), String> {
    let (head, _): (&[u8], &[u8]) = split_wire(wire)?;
    let lines: Vec<(&str, &str)> = header_lines(head)?;
    let found: Option<&str> = header_value(&lines, name);
    if found != Some(value) {
        return Err(format!(
            "response must declare {name} {value}, got {found:?}"
        ));
    }
    Ok(())
}

fn assert_plain(wire: &[u8]) -> Result<Vec<u8>, String> {
    let (head, body): (&[u8], &[u8]) = split_wire(wire)?;
    let lines: Vec<(&str, &str)> = header_lines(head)?;
    if header_value(&lines, CONTENT_ENCODING).is_some() {
        return Err("response must declare no content coding".to_owned());
    }
    if header_value(&lines, VARY).is_some() {
        return Err("response must declare no variance".to_owned());
    }
    Ok(body.to_vec())
}

#[tokio::test]
async fn answers_gzip_when_the_client_accepts_it() -> Result<(), String> {
    let mut server: Server = test_server();
    let request: String = get_request(LARGE_PATH, &format!("Accept-Encoding: {GZIP_TOKEN}\r\n"));
    let wire: Vec<u8> = exchange(&mut server, &request).await?;
    let body: Vec<u8> = assert_coding(&wire, GZIP_TOKEN)?;
    if body.len() < 2 || body[..2] != GZIP_MAGIC {
        return Err(format!("body must be a gzip stream, got {body:?}"));
    }
    Ok(())
}

#[tokio::test]
async fn answers_brotli_when_the_client_accepts_it() -> Result<(), String> {
    let mut server: Server = test_server();
    let request: String = get_request(LARGE_PATH, &format!("Accept-Encoding: {BROTLI_TOKEN}\r\n"));
    let wire: Vec<u8> = exchange(&mut server, &request).await?;
    let _: Vec<u8> = assert_coding(&wire, BROTLI_TOKEN)?;
    Ok(())
}

#[tokio::test]
async fn answers_deflate_when_only_deflate_is_accepted() -> Result<(), String> {
    let mut server: Server = test_server();
    let request: String = get_request(LARGE_PATH, &format!("Accept-Encoding: {DEFLATE_TOKEN}\r\n"));
    let wire: Vec<u8> = exchange(&mut server, &request).await?;
    let _: Vec<u8> = assert_coding(&wire, DEFLATE_TOKEN)?;
    Ok(())
}

#[tokio::test]
async fn answers_the_coding_the_client_prefers() -> Result<(), String> {
    let mut server: Server = test_server();
    let request: String = get_request(LARGE_PATH, &format!("Accept-Encoding: {PREFER_BROTLI}\r\n"));
    let wire: Vec<u8> = exchange(&mut server, &request).await?;
    let _: Vec<u8> = assert_coding(&wire, BROTLI_TOKEN)?;
    let mut server: Server = test_server();
    let request: String = get_request(LARGE_PATH, &format!("Accept-Encoding: {PREFER_GZIP}\r\n"));
    let wire: Vec<u8> = exchange(&mut server, &request).await?;
    let _: Vec<u8> = assert_coding(&wire, GZIP_TOKEN)?;
    Ok(())
}

#[tokio::test]
async fn skips_a_body_below_the_threshold() -> Result<(), String> {
    let mut server: Server = test_server();
    let request: String = get_request(SMALL_PATH, &format!("Accept-Encoding: {GZIP_TOKEN}\r\n"));
    let wire: Vec<u8> = exchange(&mut server, &request).await?;
    let body: Vec<u8> = assert_plain(&wire)?;
    if body != SMALL_BODY.as_bytes() {
        return Err(format!("small body must be sent as it is, got {body:?}"));
    }
    Ok(())
}

#[tokio::test]
async fn skips_a_response_that_already_declares_a_coding() -> Result<(), String> {
    let mut server: Server = test_server();
    let request: String = get_request(CODED_PATH, &format!("Accept-Encoding: {GZIP_TOKEN}\r\n"));
    let wire: Vec<u8> = exchange(&mut server, &request).await?;
    assert_header(&wire, CONTENT_ENCODING, BROTLI_TOKEN)?;
    Ok(())
}

#[tokio::test]
async fn skips_an_event_stream() -> Result<(), String> {
    let mut server: Server = test_server();
    let request: String = get_request(STREAM_PATH, &format!("Accept-Encoding: {GZIP_TOKEN}\r\n"));
    let wire: Vec<u8> = exchange(&mut server, &request).await?;
    let body: Vec<u8> = assert_plain(&wire)?;
    if body != BODY_UNIT.repeat(BODY_REPEATS).as_bytes() {
        return Err("event stream must be sent as it is".to_owned());
    }
    Ok(())
}

#[tokio::test]
async fn skips_a_bodyless_status() -> Result<(), String> {
    let mut server: Server = test_server();
    let request: String = get_request(
        NO_CONTENT_PATH,
        &format!("Accept-Encoding: {GZIP_TOKEN}\r\n"),
    );
    let wire: Vec<u8> = exchange(&mut server, &request).await?;
    let _: Vec<u8> = assert_plain(&wire)?;
    Ok(())
}

#[tokio::test]
async fn ignores_an_absent_field() -> Result<(), String> {
    let mut server: Server = test_server();
    let request: String = get_request(LARGE_PATH, "");
    let wire: Vec<u8> = exchange(&mut server, &request).await?;
    let body: Vec<u8> = assert_plain(&wire)?;
    if body != BODY_UNIT.repeat(BODY_REPEATS).as_bytes() {
        return Err("body must be sent as it is".to_owned());
    }
    Ok(())
}

#[tokio::test]
async fn ignores_a_coding_the_crate_does_not_implement() -> Result<(), String> {
    let mut server: Server = test_server();
    let request: String = get_request(
        LARGE_PATH,
        &format!("Accept-Encoding: {UNSUPPORTED_TOKEN}\r\n"),
    );
    let wire: Vec<u8> = exchange(&mut server, &request).await?;
    let _: Vec<u8> = assert_plain(&wire)?;
    Ok(())
}

#[tokio::test]
async fn ignores_a_coding_the_client_refuses() -> Result<(), String> {
    let mut server: Server = test_server();
    let request: String = get_request(LARGE_PATH, &format!("Accept-Encoding: {REFUSED_TOKEN}\r\n"));
    let wire: Vec<u8> = exchange(&mut server, &request).await?;
    let _: Vec<u8> = assert_plain(&wire)?;
    Ok(())
}

#[tokio::test]
async fn answers_a_head_request_without_a_length() -> Result<(), String> {
    let mut server: Server = test_server();
    let request: String = format!(
        "HEAD {LARGE_PATH} {HTTP_ONE}\r\nHost: {LOOPBACK}\r\nAccept-Encoding: {GZIP_TOKEN}\r\nConnection: close\r\n\r\n"
    );
    let wire: Vec<u8> = exchange(&mut server, &request).await?;
    let (head, body): (&[u8], &[u8]) = split_wire(&wire)?;
    let lines: Vec<(&str, &str)> = header_lines(head)?;
    let coding: Option<&str> = header_value(&lines, CONTENT_ENCODING);
    if coding != Some(GZIP_TOKEN) {
        return Err(format!("head must declare the coding, got {coding:?}"));
    }
    if header_value(&lines, CONTENT_LENGTH).is_some() {
        return Err("head of a coded body must advertise no length".to_owned());
    }
    if !body.is_empty() {
        return Err("head must send no body".to_owned());
    }
    Ok(())
}

#[tokio::test]
async fn extends_a_variance_the_response_already_declares() -> Result<(), String> {
    let mut server: Server = test_server();
    let request: String = get_request(VARY_PATH, &format!("Accept-Encoding: {GZIP_TOKEN}\r\n"));
    let wire: Vec<u8> = exchange(&mut server, &request).await?;
    let (head, _): (&[u8], &[u8]) = split_wire(&wire)?;
    let lines: Vec<(&str, &str)> = header_lines(head)?;
    if header_count(&lines, VARY) != 2 {
        return Err(format!(
            "response must carry both variances, got {:?}",
            lines
                .iter()
                .filter(|(key, _): &&(&str, &str)| key.eq_ignore_ascii_case(VARY))
                .collect::<Vec<_>>()
        ));
    }
    let values: Vec<&str> = lines
        .iter()
        .filter(|(key, _): &&(&str, &str)| key.eq_ignore_ascii_case(VARY))
        .map(|(_, value): &(&str, &str)| *value)
        .collect();
    if !values.contains(&EXISTING_VARY) || !values.contains(&VARY_ACCEPT_ENCODING) {
        return Err(format!("response must keep both variances, got {values:?}"));
    }
    Ok(())
}

#[tokio::test]
async fn installs_a_configuration_once() -> Result<(), String> {
    let config: CompressConfig = CompressConfig::new(DEFAULT_MIN_LENGTH);
    let first: Result<(), String> = CompressMiddleware::init(config);
    if first.is_err() {
        return Err("the first configuration must be accepted".to_owned());
    }
    let second: Result<(), String> = CompressMiddleware::init(config);
    if second.is_ok() {
        return Err("a second configuration must be refused".to_owned());
    }
    Ok(())
}

#[tokio::test]
async fn configures_the_body_length_threshold() {
    let default: CompressConfig = CompressConfig::default();
    assert_eq!(default.get_min_length(), DEFAULT_MIN_LENGTH);
    let custom: CompressConfig = CompressConfig::new(4_096);
    assert_eq!(custom.get_min_length(), 4_096);
}

#[tokio::test]
async fn answers_304_when_the_validator_matches() -> Result<(), String> {
    let mut server: Server = test_server();
    let first: String = get_request(LARGE_PATH, "");
    let wire: Vec<u8> = exchange(&mut server, &first).await?;
    let (head, _): (&[u8], &[u8]) = split_wire(&wire)?;
    let lines: Vec<(&str, &str)> = header_lines(head)?;
    let etag: &str = match header_value(&lines, ETAG) {
        Some(etag) => etag,
        None => return Err("response must declare a validator".to_owned()),
    };
    let second: String = get_request(LARGE_PATH, &format!("If-None-Match: {etag}\r\n"));
    let wire: Vec<u8> = exchange(&mut server, &second).await?;
    if response_status(&wire)? != HttpStatus::NotModified.code() {
        return Err("a matching validator must answer 304".to_owned());
    }
    let (head, body): (&[u8], &[u8]) = split_wire(&wire)?;
    if !body.is_empty() {
        return Err("a 304 must carry no body".to_owned());
    }
    let lines: Vec<(&str, &str)> = header_lines(head)?;
    if header_value(&lines, ETAG) != Some(etag) {
        return Err("a 304 must keep the validator".to_owned());
    }
    Ok(())
}

#[tokio::test]
async fn answers_200_when_the_validator_does_not_match() -> Result<(), String> {
    let mut server: Server = test_server();
    let request: String = get_request(LARGE_PATH, &format!("If-None-Match: {APP_VALIDATOR}\r\n"));
    let wire: Vec<u8> = exchange(&mut server, &request).await?;
    if response_status(&wire)? != HttpStatus::Ok.code() {
        return Err("a validator that does not match must answer 200".to_owned());
    }
    let body: Vec<u8> = assert_plain(&wire)?;
    if body != BODY_UNIT.repeat(BODY_REPEATS).as_bytes() {
        return Err("the body must be sent as it is".to_owned());
    }
    Ok(())
}

#[tokio::test]
async fn computes_the_same_validator_for_the_same_body() -> Result<(), String> {
    let mut server: Server = test_server();
    let request: String = get_request(LARGE_PATH, "");
    let first: Vec<u8> = exchange(&mut server, &request).await?;
    let second: Vec<u8> = exchange(&mut server, &request).await?;
    let (first_head, _): (&[u8], &[u8]) = split_wire(&first)?;
    let (second_head, _): (&[u8], &[u8]) = split_wire(&second)?;
    let first_lines: Vec<(&str, &str)> = header_lines(first_head)?;
    let second_lines: Vec<(&str, &str)> = header_lines(second_head)?;
    let first_tag: Option<&str> = header_value(&first_lines, ETAG);
    let second_tag: Option<&str> = header_value(&second_lines, ETAG);
    if first_tag.is_none() || first_tag != second_tag {
        return Err(format!(
            "the same body must carry the same validator, got {first_tag:?} and {second_tag:?}"
        ));
    }
    Ok(())
}

#[tokio::test]
async fn keeps_a_validator_the_application_declared() -> Result<(), String> {
    let mut server: Server = test_server();
    let request: String = get_request(VALIDATED_PATH, "");
    let wire: Vec<u8> = exchange(&mut server, &request).await?;
    assert_header(&wire, ETAG, APP_VALIDATOR)?;
    let request: String = get_request(
        VALIDATED_PATH,
        &format!("If-None-Match: {APP_VALIDATOR}\r\n"),
    );
    let wire: Vec<u8> = exchange(&mut server, &request).await?;
    if response_status(&wire)? != HttpStatus::NotModified.code() {
        return Err("a matching application validator must answer 304".to_owned());
    }
    Ok(())
}

#[tokio::test]
async fn ignores_an_unsafe_method() -> Result<(), String> {
    let mut server: Server = test_server();
    let request: String = method_request(
        "POST",
        LARGE_PATH,
        &format!("If-None-Match: {APP_VALIDATOR}\r\n"),
    );
    let wire: Vec<u8> = exchange(&mut server, &request).await?;
    if response_status(&wire)? != HttpStatus::Ok.code() {
        return Err("an unsafe method must reach the handler".to_owned());
    }
    Ok(())
}

#[tokio::test]
async fn skips_a_validator_for_a_bodyless_status() -> Result<(), String> {
    let mut server: Server = test_server();
    let request: String = get_request(NO_CONTENT_PATH, "");
    let wire: Vec<u8> = exchange(&mut server, &request).await?;
    let (head, _): (&[u8], &[u8]) = split_wire(&wire)?;
    let lines: Vec<(&str, &str)> = header_lines(head)?;
    if header_value(&lines, ETAG).is_some() {
        return Err("a bodyless status must carry no validator".to_owned());
    }
    Ok(())
}

#[tokio::test]
async fn keeps_the_variance_when_the_validator_matches() -> Result<(), String> {
    let mut server: Server = test_server();
    let first: String = get_request(LARGE_PATH, &format!("Accept-Encoding: {GZIP_TOKEN}\r\n"));
    let wire: Vec<u8> = exchange(&mut server, &first).await?;
    let (head, _): (&[u8], &[u8]) = split_wire(&wire)?;
    let lines: Vec<(&str, &str)> = header_lines(head)?;
    let etag: &str = match header_value(&lines, ETAG) {
        Some(etag) => etag,
        None => return Err("response must declare a validator".to_owned()),
    };
    let second: String = get_request(
        LARGE_PATH,
        &format!("Accept-Encoding: {GZIP_TOKEN}\r\nIf-None-Match: {etag}\r\n"),
    );
    let wire: Vec<u8> = exchange(&mut server, &second).await?;
    if response_status(&wire)? != HttpStatus::NotModified.code() {
        return Err("a matching validator must answer 304".to_owned());
    }
    assert_header(&wire, VARY, VARY_ACCEPT_ENCODING)?;
    Ok(())
}
