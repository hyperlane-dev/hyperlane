use super::*;

/// Reads the value of one query parameter.
///
/// # Arguments
///
/// - `&str` - The query string.
/// - `&str` - The name of the parameter.
///
/// # Returns
///
/// - `Option<String>` - The value, when the query carries the parameter.
pub(crate) fn query_value(query: &str, name: &str) -> Option<String> {
    for pair in query.split(QUERY_SEPARATOR) {
        let (key, value) = match pair.split_once(QUERY_ASSIGN) {
            Some(split) => split,
            None => (pair, ""),
        };
        if key.trim() == name {
            return Some(value.to_owned());
        }
    }
    None
}

/// Builds the answer one request gets.
///
/// # Arguments
///
/// - `&str` - The path the request carries.
/// - `&str` - The query the request carries.
/// - `Option<&str>` - The value of the tag header, when the request carries it.
/// - `&[u8]` - The body the request carries.
///
/// # Returns
///
/// - `Answer` - The answer.
pub(crate) fn answer_for(path: &str, query: &str, tag: Option<&str>, body: &[u8]) -> Answer {
    if path.starts_with(PATH_USER) {
        let id: &str = path.rsplit(PATH_SEPARATOR).next().unwrap_or(path);
        let mut answer: Vec<u8> = Vec::with_capacity(BODY_USER_PREFIX.len() + id.len());
        answer.extend_from_slice(BODY_USER_PREFIX.as_bytes());
        answer.extend_from_slice(id.as_bytes());
        return Answer { body: answer };
    }
    if path.starts_with(PATH_SEARCH) {
        let value: String = query_value(query, PARAM_QUERY).unwrap_or_default();
        return Answer {
            body: format!("{BODY_QUERY_PREFIX}{value}").into_bytes(),
        };
    }
    if path.starts_with(PATH_HEADERS) {
        let value: &str = tag.unwrap_or_default();
        return Answer {
            body: format!("{BODY_TAG_PREFIX}{value}").into_bytes(),
        };
    }
    if path.starts_with(PATH_ECHO) {
        return Answer {
            body: body.to_vec(),
        };
    }
    Answer {
        body: BODY_ROOT.as_bytes().to_vec(),
    }
}

/// Keeps the certificates that parsed cleanly.
///
/// # Arguments
///
/// - `Result<CertificateDer<'static>, rustls_pki_types::pem::Error>` - The
///   certificate that was read.
///
/// # Returns
///
/// - `Option<CertificateDer<'static>>` - The certificate, when it parsed.
pub(crate) fn keep_certificate(
    certificate: Result<CertificateDer<'static>, rustls_pki_types::pem::Error>,
) -> Option<CertificateDer<'static>> {
    certificate.ok()
}

/// Builds the server configuration the TLS protocols terminate with.
///
/// # Arguments
///
/// - `&str` - The path of the certificate chain.
/// - `&str` - The path of the private key.
/// - `&[u8]` - The ALPN identifier to advertise.
///
/// # Returns
///
/// - `Result<ServerConfig, String>` - The configuration, or the reason it could
///   not be built.
pub(crate) fn build_server_config(
    certificate_path: &str,
    key_path: &str,
    alpn: &[u8],
) -> Result<ServerConfig, String> {
    let certificates: Vec<CertificateDer<'static>> =
        match CertificateDer::pem_file_iter(certificate_path) {
            Ok(certificates) => certificates.filter_map(keep_certificate).collect(),
            Err(error) => return Err(format!("cannot read {certificate_path}: {error}")),
        };
    let key: PrivateKeyDer<'static> = match PrivateKeyDer::from_pem_file(key_path) {
        Ok(key) => key,
        Err(error) => return Err(format!("cannot read {key_path}: {error}")),
    };
    let versions: &[&'static rustls::SupportedProtocolVersion] =
        &[&rustls::version::TLS13, &rustls::version::TLS12];
    let mut config: ServerConfig = match ServerConfig::builder_with_provider(Arc::new(
        rustls::crypto::ring::default_provider(),
    ))
    .with_protocol_versions(versions)
    {
        Ok(builder) => match builder
            .with_no_client_auth()
            .with_single_cert(certificates, key)
        {
            Ok(config) => config,
            Err(error) => return Err(error.to_string()),
        },
        Err(error) => return Err(error.to_string()),
    };
    config.alpn_protocols = vec![alpn.to_vec()];
    Ok(config)
}

/// Finds the end of the first line of a request.
///
/// # Arguments
///
/// - `&[u8]` - The request bytes.
///
/// # Returns
///
/// - `Option<usize>` - The offset one past the line feed, when the request
///   carries one.
pub(crate) fn find_line_end(request: &[u8]) -> Option<usize> {
    request
        .iter()
        .position(|byte: &u8| *byte == b'\n')
        .map(|index: usize| index + 1)
}

/// Finds the end of the head of an HTTP/1.1 request.
///
/// # Arguments
///
/// - `&[u8]` - The request bytes.
///
/// # Returns
///
/// - `Option<usize>` - The offset one past the blank line, when the request
///   carries one.
pub(crate) fn find_head_end(request: &[u8]) -> Option<usize> {
    for index in 0..request.len().saturating_sub(3) {
        if request[index..index + HEAD_TERMINATOR.len()] == *HEAD_TERMINATOR {
            return Some(index + HEAD_TERMINATOR.len());
        }
    }
    None
}

/// Reads the path and the query out of an HTTP/1.1 request line.
///
/// # Arguments
///
/// - `&[u8]` - The request bytes.
///
/// # Returns
///
/// - `(String, String)` - The path and the query.
pub(crate) fn parse_request_line(request: &[u8]) -> (String, String) {
    let line: String = match find_line_end(request) {
        Some(end) => String::from_utf8_lossy(&request[..end]).to_string(),
        None => String::new(),
    };
    let target: &str = line.split(' ').nth(1).unwrap_or_default();
    match target.split_once(PATH_QUERY_SEPARATOR) {
        Some((path, query)) => (path.to_owned(), query.to_owned()),
        None => (target.to_owned(), String::new()),
    }
}

/// Reads the body length an HTTP/1.1 request announces.
///
/// # Arguments
///
/// - `&[u8]` - The head of the request.
///
/// # Returns
///
/// - `usize` - The announced length, or zero when the head announces none.
pub(crate) fn announced_length(head: &[u8]) -> usize {
    let text: String = String::from_utf8_lossy(head).to_string();
    for line in text.lines() {
        let Some((key, value)) = line.split_once(':') else {
            continue;
        };
        if key.trim().eq_ignore_ascii_case(HEADER_CONTENT_LENGTH) {
            return value.trim().parse().unwrap_or(0);
        }
    }
    0
}

/// Reads the value of one request header.
///
/// # Arguments
///
/// - `&[u8]` - The request bytes.
/// - `&str` - The name of the header.
///
/// # Returns
///
/// - `Option<String>` - The value, when the request carries the header.
pub(crate) fn parse_header(request: &[u8], name: &str) -> Option<String> {
    let text: String = String::from_utf8_lossy(request).to_string();
    for line in text.lines() {
        if line.is_empty() {
            break;
        }
        let (key, value) = match line.split_once(':') {
            Some(split) => split,
            None => continue,
        };
        if key.trim().eq_ignore_ascii_case(name) {
            return Some(value.trim().to_owned());
        }
    }
    None
}

/// Builds the HTTP/1.1 answer one body gets.
///
/// # Arguments
///
/// - `&[u8]` - The body the answer carries.
///
/// # Returns
///
/// - `Vec<u8>` - The answer.
pub(crate) fn build_http1_response(body: &[u8]) -> Vec<u8> {
    let mut response: Vec<u8> = Vec::with_capacity(body.len() + 64);
    response.extend_from_slice(RESPONSE_STATUS_LINE);
    response.extend_from_slice(CONTENT_LENGTH_PREFIX);
    response.extend_from_slice(body.len().to_string().as_bytes());
    response.extend_from_slice(HEAD_TERMINATOR);
    response.extend_from_slice(body);
    response
}

/// Serves one HTTP/1.1 connection until the peer closes it.
///
/// # Arguments
///
/// - `Box<dyn Transport>` - The accepted connection.
///
/// # Returns
///
/// - `Result<(), String>` - Nothing, or the reason the connection failed.
pub(crate) async fn serve_http1(mut stream: Box<dyn Transport>) -> Result<(), String> {
    let mut buffer: Vec<u8> = vec![0; READ_BUFFER_SIZE];
    // A read can carry several requests, and a request can be split across
    // two reads. The buffer therefore holds the bytes no request has claimed
    // yet, and each pass answers every whole request it finds in it.
    let mut pending: Vec<u8> = Vec::new();
    loop {
        let read: usize = match stream.read(&mut buffer).await {
            Ok(read) => read,
            Err(error) => return Err(error.to_string()),
        };
        if read == 0 {
            return Ok(());
        }
        pending.extend_from_slice(&buffer[..read]);
        let mut answered: usize = 0;
        while let Some(head_end) = find_head_end(&pending[answered..]) {
            let request: &[u8] = &pending[answered..head_end];
            let length: usize = announced_length(request);
            let available: usize = pending.len().saturating_sub(head_end);
            if available < length {
                break;
            }
            let (path, query) = parse_request_line(request);
            let tag: Option<String> = parse_header(request, HEADER_TAG);
            let body: Vec<u8> = pending[head_end..head_end + length].to_vec();
            let answer: Answer = answer_for(&path, &query, tag.as_deref(), &body);
            let response: Vec<u8> = build_http1_response(&answer.body);
            if stream.write_all(&response).await.is_err() {
                return Ok(());
            }
            answered = head_end + length;
        }
        if answered > 0 {
            pending.drain(..answered);
        }
    }
}

/// Serves one HTTP/2 connection until it closes.
///
/// # Arguments
///
/// - `Box<dyn Transport>` - The accepted connection.
///
/// # Returns
///
/// - `Result<(), String>` - Nothing, or the reason the connection failed.
pub(crate) async fn serve_http2(stream: Box<dyn Transport>) -> Result<(), String> {
    let mut connection: h2::server::Connection<Box<dyn Transport>, Bytes> =
        match h2::server::handshake(stream).await {
            Ok(connection) => connection,
            Err(error) => return Err(error.to_string()),
        };
    while let Some(result) = connection.accept().await {
        let (request, mut respond) = match result {
            Ok(pair) => pair,
            Err(_) => return Ok(()),
        };
        let path: String = request.uri().path().to_owned();
        let query: String = request.uri().query().unwrap_or_default().to_owned();
        let tag: Option<String> = request
            .headers()
            .get(HEADER_TAG)
            .and_then(|value: &http::HeaderValue| value.to_str().ok())
            .map(|value: &str| value.to_owned());
        let body: Vec<u8> = read_h2_body(request.into_body()).await?;
        let answer: Answer = answer_for(&path, &query, tag.as_deref(), &body);
        let response: http::Response<()> = http::Response::builder()
            .status(STATUS_OK)
            .header(http::header::CONTENT_LENGTH, answer.body.len())
            .body(())
            .map_err(|error: http::Error| error.to_string())?;
        let mut send_body: h2::SendStream<Bytes> = match respond.send_response(response, false) {
            Ok(stream) => stream,
            Err(_) => continue,
        };
        let _: Result<(), h2::Error> = send_body.send_data(Bytes::from(answer.body), true);
    }
    Ok(())
}

/// Reads the body of one HTTP/2 request.
///
/// # Arguments
///
/// - `h2::RecvStream` - The receive half of the request stream.
///
/// # Returns
///
/// - `Result<Vec<u8>, String>` - The body, or the reason it could not be read.
pub(crate) async fn read_h2_body(mut body: h2::RecvStream) -> Result<Vec<u8>, String> {
    let mut collected: Vec<u8> = Vec::new();
    while let Some(chunk) = body.data().await {
        let chunk: Bytes = match chunk {
            Ok(chunk) => chunk,
            Err(error) => return Err(error.to_string()),
        };
        collected.extend_from_slice(&chunk);
    }
    Ok(collected)
}

/// Serves one HTTP/3 connection until it closes.
///
/// # Arguments
///
/// - `quinn::Connection` - The accepted connection.
///
/// # Returns
///
/// - `Result<(), String>` - Nothing, or the reason the connection failed.
pub(crate) async fn serve_http3_connection(connection: quinn::Connection) -> Result<(), String> {
    let mut server: h3::server::Connection<h3_quinn::Connection, Bytes> =
        match h3::server::builder()
            .build(h3_quinn::Connection::new(connection))
            .await
        {
            Ok(server) => server,
            Err(error) => return Err(error.to_string()),
        };
    loop {
        let resolver: h3::server::RequestResolver<h3_quinn::Connection, Bytes> =
            match server.accept().await {
                Ok(Some(resolver)) => resolver,
                Ok(None) => return Ok(()),
                Err(_) => return Ok(()),
            };
        let (request, mut stream) = match resolver.resolve_request().await {
            Ok(resolved) => resolved,
            Err(_) => return Ok(()),
        };
        let path: String = request.uri().path().to_owned();
        let query: String = request.uri().query().unwrap_or_default().to_owned();
        let tag: Option<String> = request
            .headers()
            .get(HEADER_TAG)
            .and_then(|value: &http::HeaderValue| value.to_str().ok())
            .map(|value: &str| value.to_owned());
        let body: Vec<u8> = read_h3_body(&mut stream).await?;
        let answer: Answer = answer_for(&path, &query, tag.as_deref(), &body);
        let response: http::Response<()> = http::Response::builder()
            .status(STATUS_OK)
            .header(http::header::CONTENT_LENGTH, answer.body.len())
            .body(())
            .map_err(|error: http::Error| error.to_string())?;
        let _: Result<(), h3::error::StreamError> = stream.send_response(response).await;
        let _: Result<(), h3::error::StreamError> =
            stream.send_data(Bytes::from(answer.body)).await;
        let _: Result<(), h3::error::StreamError> = stream.finish().await;
    }
}

/// Reads the body of one HTTP/3 request.
///
/// # Arguments
///
/// - `&mut h3::server::RequestStream<h3_quinn::BidiStream<Bytes>, Bytes>` - The
///   request stream.
///
/// # Returns
///
/// - `Result<Vec<u8>, String>` - The body, or the reason it could not be read.
pub(crate) async fn read_h3_body(
    stream: &mut h3::server::RequestStream<h3_quinn::BidiStream<Bytes>, Bytes>,
) -> Result<Vec<u8>, String> {
    let mut collected: Vec<u8> = Vec::new();
    loop {
        match stream.recv_data().await {
            Ok(Some(chunk)) => collected.extend_from_slice(chunk.chunk()),
            Ok(None) => break,
            Err(error) => return Err(error.to_string()),
        }
    }
    Ok(collected)
}
