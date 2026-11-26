use super::*;

/// Percent-encodes one query or form component using unreserved RFC 3986 characters.
///
/// # Arguments
///
/// - `&str` - Raw unencoded key or value text.
/// - `&mut String` - Destination string receiving percent-encoded bytes.
pub(crate) fn push_percent_encoded(input: &str, out: &mut String) {
    for &byte in input.as_bytes() {
        let unreserved: bool = byte.is_ascii_alphanumeric() || b"-_.~".contains(&byte);
        if unreserved {
            out.push(char::from(byte));
        } else {
            out.push('%');
            out.push(char::from(HEX_UPPER[(byte >> 4) as usize]));
            out.push(char::from(HEX_UPPER[(byte & 0x0f) as usize]));
        }
    }
}

/// Encodes one or more [`BodyPart`] entries into an RFC 7578 `multipart/form-data` body.
///
/// # Arguments
///
/// - `&[BodyPart]` - Multipart fields and file parts to serialize.
///
/// # Returns
///
/// - `(String, Vec<u8>)` - `(content_type_header_value, serialized_body_bytes)`.
pub(crate) fn encode_multipart_parts(parts: &[BodyPart]) -> (String, Vec<u8>) {
    let mut seed: u64 = 0x9e37_79b9_7f4a_7c15;
    for part in parts {
        let name_len: usize = part.try_get_name().map_or(0, String::len);
        let data_len: usize = part.try_get_data().map_or(0, <[u8]>::len);
        seed = seed.rotate_left(7) ^ (name_len as u64);
        seed = seed.wrapping_mul(0x517c_c1b7_2722_0a95) ^ (data_len as u64);
    }
    let boundary: String = format!("{MULTIPART_BOUNDARY_PREFIX}{seed:016x}");
    let estimated: usize = parts
        .iter()
        .map(|part: &BodyPart| {
            let name_len: usize = part.try_get_name().map_or(0, String::len);
            let data_len: usize = part.try_get_data().map_or(0, <[u8]>::len);
            data_len + name_len + 128
        })
        .sum::<usize>()
        + boundary.len()
        + 16;
    let mut body: Vec<u8> = Vec::with_capacity(estimated);
    for part in parts {
        let name: &str = part.try_get_name().map_or("", String::as_str);
        body.extend_from_slice(b"--");
        body.extend_from_slice(boundary.as_bytes());
        body.extend_from_slice(HTTP_BR_BYTES);
        body.extend_from_slice(b"Content-Disposition: form-data; name=\"");
        for byte in name.bytes() {
            if byte == b'"' || byte == b'\r' || byte == b'\n' {
                body.push(b'_');
            } else {
                body.push(byte);
            }
        }
        body.push(b'"');
        if let Some(filename) = part.try_get_file_name() {
            body.extend_from_slice(b"; filename=\"");
            for byte in filename.bytes() {
                if byte == b'"' || byte == b'\r' || byte == b'\n' {
                    body.push(b'_');
                } else {
                    body.push(byte);
                }
            }
            body.push(b'"');
        }
        body.extend_from_slice(HTTP_BR_BYTES);
        if let Some(content_type) = part.try_get_content_type()
            && is_valid_header_value(content_type.as_bytes())
        {
            body.extend_from_slice(b"Content-Type: ");
            body.extend_from_slice(content_type.as_bytes());
            body.extend_from_slice(HTTP_BR_BYTES);
        }
        body.extend_from_slice(HTTP_BR_BYTES);
        if let Some(data) = part.try_get_data() {
            body.extend_from_slice(data);
        }
        body.extend_from_slice(HTTP_BR_BYTES);
    }
    body.extend_from_slice(b"--");
    body.extend_from_slice(boundary.as_bytes());
    body.extend_from_slice(b"--");
    body.extend_from_slice(HTTP_BR_BYTES);
    let content_type: String = format!("multipart/form-data; boundary={boundary}");
    (content_type, body)
}

/// Serializes an HTTP/1 request directly into one pre-sized wire buffer without redundant cloning.
///
/// # Arguments
///
/// - `&HttpRequest` - Outgoing request with validated headers and body.
/// - `&HttpUrlComponents` - Already-parsed URL components for this hop.
/// - `&str` - Validated HTTP method token.
/// - `bool` - Whether the method is `HEAD`.
/// - `&str` - Validated HTTP version token.
///
/// # Returns
///
/// - `Result<Vec<u8>, RequestError>` - Serialized HTTP/1 request bytes.
pub(crate) fn build_wire_request_direct(
    request: &HttpRequest,
    url_obj: &HttpUrlComponents,
    method_token: &str,
    is_head: bool,
    version: &str,
) -> Result<Vec<u8>, RequestError> {
    let headers: &HashMap<String, String> = request.get_headers();
    let body_slice: &[u8] = if is_head {
        &[]
    } else {
        request.get_body().as_slice()
    };
    let path: String = url_obj.get_path().unwrap_or_default();
    let query: String = url_obj.get_query().unwrap_or_default();
    let host_value: String = url_obj.get_host().unwrap_or_default();
    let mut has_host: bool = false;
    let mut has_content_length: bool = false;
    let mut has_accept: bool = false;
    let mut has_user_agent: bool = false;
    let mut has_connection: bool = false;
    let mut header_cap: usize = 96;
    for (name, value) in headers {
        if name.eq_ignore_ascii_case(HOST) {
            has_host = true;
        } else if name.eq_ignore_ascii_case(CONTENT_LENGTH) {
            has_content_length = true;
        } else if name.eq_ignore_ascii_case(ACCEPT) {
            has_accept = true;
        } else if name.eq_ignore_ascii_case(USER_AGENT) {
            has_user_agent = true;
        } else if name.eq_ignore_ascii_case(CONNECTION) {
            has_connection = true;
        }
        header_cap += name.len() + value.len() + 4;
    }
    let query_extra: usize = if query.is_empty() { 0 } else { 1 + query.len() };
    let total_cap: usize = method_token.len()
        + 1
        + path.len()
        + query_extra
        + 1
        + version.len()
        + 2
        + header_cap
        + host_value.len()
        + 2
        + body_slice.len();
    let mut out: Vec<u8> = Vec::with_capacity(total_cap);
    out.extend_from_slice(method_token.as_bytes());
    out.push(SPACE_U8);
    out.extend_from_slice(path.as_bytes());
    if !query.is_empty() {
        out.push(b'?');
        out.extend_from_slice(query.as_bytes());
    }
    out.push(SPACE_U8);
    out.extend_from_slice(version.as_bytes());
    out.extend_from_slice(HTTP_BR_BYTES);
    for (name, value) in headers {
        out.extend_from_slice(name.as_bytes());
        out.extend_from_slice(HEADER_VALUE_SEPARATOR);
        out.extend_from_slice(value.as_bytes());
        out.extend_from_slice(HTTP_BR_BYTES);
    }
    if !has_host {
        if !is_valid_header_value(host_value.as_bytes()) {
            return Err(RequestError::Request(INVALID_REQUEST_HEADER.to_owned()));
        }
        out.extend_from_slice(HOST.as_bytes());
        out.extend_from_slice(HEADER_VALUE_SEPARATOR);
        out.extend_from_slice(host_value.as_bytes());
        out.extend_from_slice(HTTP_BR_BYTES);
    }
    if !has_content_length {
        out.extend_from_slice(CONTENT_LENGTH.as_bytes());
        out.extend_from_slice(HEADER_VALUE_SEPARATOR);
        let mut digits: [u8; 20] = [0u8; 20];
        let mut n: usize = body_slice.len();
        let mut idx: usize = digits.len();
        if n == 0 {
            idx -= 1;
            digits[idx] = b'0';
        } else {
            while n > 0 {
                idx -= 1;
                digits[idx] = b'0' + (n % 10) as u8;
                n /= 10;
            }
        }
        out.extend_from_slice(&digits[idx..]);
        out.extend_from_slice(HTTP_BR_BYTES);
    }
    if !has_accept {
        out.extend_from_slice(ACCEPT.as_bytes());
        out.extend_from_slice(HEADER_VALUE_SEPARATOR);
        out.extend_from_slice(ACCEPT_ANY.as_bytes());
        out.extend_from_slice(HTTP_BR_BYTES);
    }
    if !has_user_agent {
        out.extend_from_slice(USER_AGENT.as_bytes());
        out.extend_from_slice(HEADER_VALUE_SEPARATOR);
        out.extend_from_slice(APP_NAME.as_bytes());
        out.extend_from_slice(HTTP_BR_BYTES);
    }
    if !has_connection && request.get_config().get_keep_alive() {
        out.extend_from_slice(CONNECTION.as_bytes());
        out.extend_from_slice(HEADER_VALUE_SEPARATOR);
        out.extend_from_slice(KEEP_ALIVE.as_bytes());
        out.extend_from_slice(HTTP_BR_BYTES);
    }
    out.extend_from_slice(HTTP_BR_BYTES);
    if !body_slice.is_empty() {
        out.extend_from_slice(body_slice);
    }
    Ok(out)
}

/// Reports whether an HTTP/1 connection is clean and eligible for keep-alive pool reuse.
///
/// # Arguments
///
/// - `&[u8]` - Complete raw response head bytes.
/// - `ResponseFraming` - Framing resolved for the response body.
/// - `bool` - Whether unread trailing bytes remained after the framed response body.
/// - `bool` - Whether the caller disabled keep-alive on this request.
///
/// # Returns
///
/// - `bool` - `true` only when the connection can safely serve a subsequent HTTP/1 request.
pub(crate) fn response_allows_pool_reuse(
    head_bytes: &[u8],
    framing: ResponseFraming,
    has_trailing_bytes: bool,
    disable_keep_alive: bool,
) -> bool {
    if disable_keep_alive || has_trailing_bytes {
        return false;
    }
    if framing == ResponseFraming::Close {
        return false;
    }
    let is_http_1_0: bool = head_bytes.starts_with(HTTP_1_0_PREFIX);
    let mut saw_keep_alive: bool = false;
    let mut pos: usize = 0;
    while pos < head_bytes.len() {
        let end: usize = find_crlf(head_bytes, pos).unwrap_or(head_bytes.len());
        let line: &[u8] = &head_bytes[pos..end];
        if line.is_empty() {
            break;
        }
        if pos > 0
            && let Some(sep) = line.iter().position(|b: &u8| *b == COLON_U8)
        {
            let name: &[u8] = &line[..sep];
            if name.eq_ignore_ascii_case(CONNECTION.as_bytes()) {
                let raw_val: &[u8] = &line[sep + 1..];
                for token in raw_val.split(|b: &u8| *b == b',') {
                    let trimmed: &[u8] = token.trim_ascii();
                    if trimmed.eq_ignore_ascii_case(CLOSE.as_bytes()) {
                        return false;
                    }
                    if trimmed.eq_ignore_ascii_case(KEEP_ALIVE.as_bytes()) {
                        saw_keep_alive = true;
                    }
                }
            }
        }
        if end == head_bytes.len() {
            break;
        }
        pos = end + HTTP_BR_BYTES.len();
    }
    if is_http_1_0 { saw_keep_alive } else { true }
}

/// Reports whether the outgoing request opts out of HTTP keep-alive connection pooling.
///
/// # Arguments
///
/// - `&HttpRequest` - Request configuration and headers.
///
/// # Returns
///
/// - `bool` - `true` when `keep_alive` is disabled or a `Connection: close` header is present.
pub(crate) fn request_disables_keep_alive(request: &HttpRequest) -> bool {
    if !request.get_config().get_keep_alive() {
        return true;
    }
    for (name, value) in request.get_headers() {
        if name.eq_ignore_ascii_case(CONNECTION)
            && value
                .split(',')
                .any(|tok: &str| tok.trim().eq_ignore_ascii_case(CLOSE))
        {
            return true;
        }
    }
    false
}

/// Reports whether a [`RequestError`] is a transient transport or timeout error eligible for retry.
///
/// # Arguments
///
/// - `&RequestError` - The error returned by one request attempt.
///
/// # Returns
///
/// - `bool` - `true` for I/O resets, truncated EOFs, and timeouts; `false` for deterministic validation/framing errors.
pub(crate) fn is_retryable_error(error: &RequestError) -> bool {
    match error {
        RequestError::ReadTimeout(_) | RequestError::HttpRequestPartsInsufficient(_) => true,
        RequestError::Request(msg) => !matches!(
            msg.as_str(),
            INVALID_REQUEST_HEADER
                | INVALID_HTTP_METHOD
                | INVALID_HTTP_VERSION
                | INVALID_RESPONSE_FRAMING
                | RESPONSE_TOO_LARGE
                | TOO_MANY_INTERIM_RESPONSES
                | PROXY_AUTH_ERROR
                | H2_ALPN_MISMATCH
                | MISSING_REDIRECT_URL
                | REDIRECT_URL_DEAD_LOOP
                | MAX_REDIRECT_TIMES_EXCEEDED
        ),
        _ => false,
    }
}

/// Computes exponential backoff duration in milliseconds for retry attempt `attempt`.
///
/// # Arguments
///
/// - `usize` - Zero-based retry index.
///
/// # Returns
///
/// - `Duration` - Bounded exponential backoff duration.
pub(crate) fn retry_backoff_duration(attempt: usize) -> Duration {
    let shift: u32 = (attempt.min(5)) as u32;
    let ms: u64 = BASE_RETRY_BACKOFF_MS
        .saturating_mul(1u64 << shift)
        .min(MAX_RETRY_BACKOFF_MS);
    Duration::from_millis(ms)
}

/// Reports whether an HTTP request method is safe to retry automatically on transport or 5xx errors.
///
/// # Arguments
///
/// - `&Method` - The HTTP request method.
///
/// # Returns
///
/// - `bool` - `true` for RFC 9110 idempotent methods (`GET`, `HEAD`, `PUT`, `DELETE`, `OPTIONS`).
pub(crate) fn is_idempotent_method(method: &Method) -> bool {
    matches!(
        method,
        Method::Get | Method::Head | Method::Put | Method::Delete | Method::Options
    )
}

/// Reports whether an HTTP status code is transient and eligible for retry.
///
/// # Arguments
///
/// - `usize` - Response status code.
///
/// # Returns
///
/// - `bool` - `true` for `429`, `502`, `503`, `504`.
pub(crate) fn is_retryable_status(status: usize) -> bool {
    [429, 502, 503, 504].contains(&status)
}

/// Executes one asynchronous HTTP/2 (`h2` ALPN over TLS or cleartext `h2c`) exchange with connection pooling.
///
/// # Arguments
///
/// - `&HttpRequest` - Outgoing request configuration, headers, and body.
/// - `H2ExchangeTarget<'_>` - Target metadata, parsed URL, and connection pool key.
///
/// # Returns
///
/// - `Result<(HttpResponse, Option<Vec<u8>>), RequestError>` - Parsed HTTP/2 response and optional redirect target.
pub(crate) async fn send_h2_exchange_async(
    request: &HttpRequest,
    target: H2ExchangeTarget<'_>,
) -> Result<(HttpResponse, Option<Vec<u8>>), RequestError> {
    let mut attempted_pooled: bool = false;
    let pool_enabled: bool = !request_disables_keep_alive(request);
    loop {
        let sender_opt: Option<H2SendRequest> = if pool_enabled && !attempted_pooled {
            get_h2_sender(&target.pool_key)
        } else {
            None
        };
        let (sender, from_pool): (H2SendRequest, bool) = match sender_opt {
            Some(existing) => (existing, true),
            None => {
                let fresh: H2SendRequest =
                    establish_h2_sender(request, target.host, target.port, target.is_https).await?;
                if pool_enabled {
                    put_h2_sender(target.pool_key.clone(), fresh.clone());
                }
                (fresh, false)
            }
        };
        let ready_res: Result<H2SendRequest, h2::Error> = sender.ready().await;
        match ready_res {
            Ok(mut ready_sender) => {
                let exchange: Result<(HttpResponse, Option<Vec<u8>>), RequestError> =
                    run_h2_request_on_sender(&mut ready_sender, request, &target).await;
                if exchange.is_err() && from_pool {
                    remove_h2_sender(&target.pool_key);
                    attempted_pooled = true;
                    continue;
                }
                return exchange;
            }
            Err(error) => {
                if from_pool {
                    remove_h2_sender(&target.pool_key);
                    attempted_pooled = true;
                    continue;
                }
                return Err(RequestError::Request(error.to_string()));
            }
        }
    }
}

/// Establishes a new multiplexed HTTP/2 client connection and spawns its background driver.
///
/// # Arguments
///
/// - `&HttpRequest` - Request carrying proxy, TLS roots, and timeout settings.
/// - `&str` - Target host.
/// - `u16` - Target TCP port.
/// - `bool` - Whether TLS + `h2` ALPN should be negotiated.
///
/// # Returns
///
/// - `Result<H2SendRequest, RequestError>` - Ready multiplexed HTTP/2 request sender.
async fn establish_h2_sender(
    request: &HttpRequest,
    host: &str,
    port: u16,
    is_https: bool,
) -> Result<H2SendRequest, RequestError> {
    let stream: BoxAsyncReadWrite = request.open_async_stream_h2(host, port, is_https).await?;
    let (sender, conn): (H2SendRequest, _) = h2::client::handshake(stream)
        .await
        .map_err(|error: h2::Error| RequestError::Request(error.to_string()))?;
    tokio::spawn(async move {
        let _: Result<(), h2::Error> = conn.await;
    });
    Ok(sender)
}

/// Sends one HTTP/2 request over a ready [`H2SendRequest`] and collects the bounded response.
///
/// # Arguments
///
/// - `&mut H2SendRequest` - Ready HTTP/2 client sender.
/// - `&HttpRequest` - Outgoing request.
/// - `&H2ExchangeTarget<'_>` - Target metadata and parsed URL components.
///
/// # Returns
///
/// - `Result<(HttpResponse, Option<Vec<u8>>), RequestError>` - Parsed response and optional redirect target.
async fn run_h2_request_on_sender(
    sender: &mut H2SendRequest,
    request: &HttpRequest,
    target: &H2ExchangeTarget<'_>,
) -> Result<(HttpResponse, Option<Vec<u8>>), RequestError> {
    let method: http::Method = http::Method::from_bytes(target.method_token.as_bytes())
        .map_err(|_| RequestError::Request(INVALID_HTTP_METHOD.to_owned()))?;
    let scheme: &str = if target.is_https {
        HTTPS_LOWERCASE
    } else {
        HTTP_SCHEME
    };
    let path: String = target.url_obj.get_path().unwrap_or_default();
    let query: String = target.url_obj.get_query().unwrap_or_default();
    let authority: String = if (target.is_https && target.port == HTTPS_DEFAULT_PORT)
        || (!target.is_https && target.port == HTTP_DEFAULT_PORT)
    {
        target.host.to_owned()
    } else {
        format!("{}:{}", target.host, target.port)
    };
    let uri_str: String = if query.is_empty() {
        format!("{scheme}://{authority}{path}")
    } else {
        format!("{scheme}://{authority}{path}?{query}")
    };
    let uri: Uri = uri_str
        .parse::<Uri>()
        .map_err(|_| RequestError::InvalidUrl(HttpStatus::BadRequest))?;
    let mut builder: Builder = Request::builder().method(method).uri(uri);
    let mut has_accept: bool = false;
    let mut has_user_agent: bool = false;
    for (name, value) in request.get_headers() {
        if name.eq_ignore_ascii_case(HOST)
            || name.eq_ignore_ascii_case(CONNECTION)
            || name.eq_ignore_ascii_case("keep-alive")
            || name.eq_ignore_ascii_case("transfer-encoding")
            || name.eq_ignore_ascii_case("upgrade")
        {
            continue;
        }
        if name.eq_ignore_ascii_case(ACCEPT) {
            has_accept = true;
        } else if name.eq_ignore_ascii_case(USER_AGENT) {
            has_user_agent = true;
        }
        let header_name: HeaderName = HeaderName::from_bytes(name.as_bytes())
            .map_err(|_| RequestError::Request(INVALID_REQUEST_HEADER.to_owned()))?;
        let header_value: HeaderValue = HeaderValue::from_bytes(value.as_bytes())
            .map_err(|_| RequestError::Request(INVALID_REQUEST_HEADER.to_owned()))?;
        builder = builder.header(header_name, header_value);
    }
    if !has_accept {
        builder = builder.header(ACCEPT, ACCEPT_ANY);
    }
    if !has_user_agent {
        builder = builder.header(USER_AGENT, APP_NAME);
    }
    let body_bytes: &[u8] = if target.is_head {
        &[]
    } else {
        request.get_body().as_slice()
    };
    let end_of_stream: bool = body_bytes.is_empty();
    let proto_req: Request<()> = builder
        .body(())
        .map_err(|error: http::Error| RequestError::Request(error.to_string()))?;
    let (response_fut, mut send_stream) = sender
        .send_request(proto_req, end_of_stream)
        .map_err(|error: h2::Error| RequestError::Request(error.to_string()))?;
    if !end_of_stream {
        send_stream
            .send_data(Bytes::copy_from_slice(body_bytes), true)
            .map_err(|error: h2::Error| RequestError::Request(error.to_string()))?;
    }
    let proto_resp: Response<RecvStream> = response_fut
        .await
        .map_err(|error: h2::Error| RequestError::Request(error.to_string()))?;
    let status: StatusCode = proto_resp.status();
    let status_code: usize = usize::from(status.as_u16());
    let reason_phrase: String = status
        .canonical_reason()
        .map(str::to_owned)
        .unwrap_or_else(|| HttpStatus::phrase_ref(status_code).to_owned());
    let mut headers: HttpResponseHeaders = new_response_headers();
    let mut redirect_url: Option<Vec<u8>> = None;
    for (name, value) in proto_resp.headers() {
        let key: String = name.as_str().to_owned();
        let val_str: String = String::from_utf8_lossy(value.as_bytes()).into_owned();
        if [301, 302, 303, 307, 308].contains(&status_code)
            && key == LOCATION
            && redirect_url.is_none()
        {
            redirect_url = Some(value.as_bytes().to_vec());
        }
        headers.insert(key, val_str);
    }
    let mut body: ResponseBody = ResponseBody::new();
    if !target.is_head {
        let mut recv_stream: RecvStream = proto_resp.into_body();
        while let Some(chunk_res) = recv_stream.data().await {
            let chunk: Bytes =
                chunk_res.map_err(|error: h2::Error| RequestError::Request(error.to_string()))?;
            if body.len().saturating_add(chunk.len()) > MAX_RESPONSE_BODY_BYTES {
                return Err(RequestError::ContentLengthTooLarge(HttpStatus::BadRequest));
            }
            body.extend_from_slice(&chunk);
            let _: Result<(), h2::Error> = recv_stream.flow_control().release_capacity(chunk.len());
        }
    }
    let response: HttpResponse = HttpResponse {
        version: HttpVersion::Http2,
        status_code,
        reason_phrase,
        headers,
        body,
    };
    Ok((response, redirect_url))
}
