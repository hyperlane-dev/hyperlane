use super::*;

/// Chooses an HTTP/1 reply version when the parser saw a request rather than an idle EOF.
///
/// # Arguments
///
/// - `&Stream` - The stream with its latest request line buffer.
/// - `&RequestError` - The failure that ended parsing.
/// # Returns
///
/// - `Option<HttpVersion>` - The response version, or no reply for an idle connection.
#[cold]
pub(crate) fn h1_error_response_version(
    stream: &Stream,
    error: &RequestError,
) -> Option<HttpVersion> {
    let line: &str = stream.get_line_buffer();
    if line.is_empty()
        && matches!(
            error,
            RequestError::HttpRequestPartsInsufficient(_) | RequestError::ReadTimeout(_)
        )
    {
        return None;
    }
    if line.split_ascii_whitespace().nth(2) == Some(HTTP_VERSION_1_0) {
        Some(HttpVersion::Http1_0)
    } else {
        Some(HttpVersion::Http1_1)
    }
}

/// Reports a request failure on the wire unless a custom hook owns the response.
///
/// # Arguments
///
/// - `&Server` - The server that owns the request error hooks.
/// - `&mut Stream` - The transport for the failed request.
/// - `&mut Context` - The original context passed unchanged to custom hooks.
/// - `&RequestError` - The failure that maps to the response status.
/// - `Option<HttpVersion>` - The protocol version, or no default reply for an idle connection.
#[cold]
pub(crate) async fn reply_to_request_error(
    server: &Server,
    stream: &mut Stream,
    ctx: &mut Context,
    error: &RequestError,
    version: Option<HttpVersion>,
) {
    if !server.get_request_error().is_empty() {
        server.handle_request_error(stream, ctx, error).await;
        return;
    }
    let Some(version) = version else { return };
    if matches!(
        error,
        RequestError::ClientDisconnected(_)
            | RequestError::ClientClosedConnection(_)
            | RequestError::ServerClosedConnection(_)
    ) {
        return;
    }
    let h1: bool = version.is_http1_0() || version.is_http1_1();
    let mut response: Response = Response::default();
    response
        .set_version(version)
        .set_status_code(error.get_http_status_code());
    if h1 {
        response.set_header(CONNECTION, CLOSE);
    }
    let data: ResponseData = response.build();
    if stream.try_send(data).await.is_ok() && h1 {
        let _: Result<(), ResponseError> = stream.try_flush().await;
    }
}

/// Builds the informational head without a body or framing fields.
///
/// # Returns
///
/// - `ProtocolResponse<()>` - The 100 Continue response head.
#[cold]
pub(crate) fn continue_response() -> ProtocolResponse<()> {
    let mut response: ProtocolResponse<()> = ProtocolResponse::new(());
    *response.status_mut() = StatusCode::CONTINUE;
    response
}

/// Finds the end of an HTTP/1.1 response head inside buffered bytes.
///
/// The header block is terminated by a bare line ending, so a complete head ends
/// at the first empty line. When no such line has arrived the write only extends
/// the buffer and no head is framed yet, which is what keeps a head that arrives
/// in several writes from being framed as a truncated one.
///
/// # Arguments
///
/// - `&[u8]` - The buffered response bytes.
///
/// # Returns
///
/// - `Option<usize>` - The byte offset where the body starts, when a complete
pub fn find_head_end(buffer: &[u8]) -> Option<usize> {
    let mut index: usize = 0;
    while index < buffer.len() {
        let rest: &[u8] = &buffer[index..];
        let line_len: usize = find_line_end(rest)?;
        let is_blank: bool = rest[..line_len]
            .iter()
            .all(|byte: &u8| *byte == CR_U8 || *byte == LF_U8);
        if is_blank {
            return Some(index + line_len);
        }
        index += line_len;
    }
    None
}

/// Finds the length of the next line, including its terminator.
///
/// # Arguments
///
/// - `&[u8]` - The bytes starting at a line boundary.
///
/// # Returns
///
/// - `Option<usize>` - The line length including its terminator, or `None` when
pub fn find_line_end(bytes: &[u8]) -> Option<usize> {
    bytes
        .iter()
        .position(|byte: &u8| *byte == LF_U8)
        .map(|index: usize| index + 1)
}

/// Parses, validates, and filters a serialized HTTP/1.1 response head into a
/// protocol response in a single pass.
///
/// Hop-by-hop headers and any fields nominated by `Connection` are omitted
/// directly while constructing the protocol `HeaderMap`, avoiding intermediate
/// map cloning on the hot path.
///
/// # Arguments
///
/// - `&[u8]` - The serialized response head bytes up to and including the blank line.
/// - `bool` - Whether the incoming request method was `HEAD`.
///
/// # Returns
///
/// - `Result<ParsedProtocolHead, ResponseError>` - The ready protocol head, or
///   a framing error when the head is malformed or unsupported.
pub(crate) fn parse_protocol_response_head(
    head: &[u8],
    is_head: bool,
) -> Result<ParsedProtocolHead, ResponseError> {
    let text: &str =
        from_utf8(head).map_err(|error: Utf8Error| ResponseError::Send(error.to_string()))?;
    let mut lines: SplitLines<'_> = text.split(CRLF);
    let status_line: &str = lines.next().unwrap_or(EMPTY_STR);
    let mut fields: SplitWhitespace<'_> = status_line.split_whitespace();
    let version: &str = fields.next().unwrap_or(EMPTY_STR);
    if !version.starts_with(HTTP_VERSION_PREFIX) {
        return Err(ResponseError::Send(MISSING_RESPONSE_VERSION.to_owned()));
    }
    let status_text: &str = fields.next().unwrap_or(EMPTY_STR);
    if status_text.is_empty() {
        return Err(ResponseError::Send(MISSING_RESPONSE_STATUS.to_owned()));
    }
    let status_code: usize = status_text
        .parse::<usize>()
        .map_err(|error: ParseIntError| ResponseError::Send(error.to_string()))?;
    validate_status_code(status_code)?;
    let status: StatusCode = StatusCode::from_u16(status_code as u16)
        .map_err(|error: InvalidStatusCode| ResponseError::Send(error.to_string()))?;
    let mut response: ProtocolResponse<()> = ProtocolResponse::new(());
    *response.status_mut() = status;
    let headers: &mut ProtocolHeaders = response.headers_mut();
    let suppress_body: bool = is_bodyless_status(status_code, is_head);
    let hide_length: bool = hides_entity_length(status_code, is_head);
    let mut announced_len: Option<usize> = None;
    let mut nominated: Option<Vec<&str>> = None;
    for line in lines {
        let line: &str = line.trim_end_matches(LF_CHAR);
        if line.is_empty() {
            continue;
        }
        let Some((name, value)) = line.split_once(COLON) else {
            return Err(ResponseError::Send(MALFORMED_RESPONSE_HEADER.to_owned()));
        };
        let name: &str = name.trim();
        if name.is_empty() {
            return Err(ResponseError::Send(MALFORMED_RESPONSE_HEADER.to_owned()));
        }
        let trimmed_value: &str = value.trim();
        if name.eq_ignore_ascii_case(CONNECTION) {
            if HeaderValue::from_bytes(trimmed_value.as_bytes()).is_err() {
                return Err(ResponseError::Send(MALFORMED_RESPONSE_HEADER.to_owned()));
            }
            for token in trimmed_value.split(COMMA_STR) {
                let token: &str = token.trim();
                if !token.is_empty()
                    && !token.eq_ignore_ascii_case(KEEP_ALIVE)
                    && !token.eq_ignore_ascii_case(CLOSE)
                {
                    nominated.get_or_insert_with(Vec::new).push(token);
                }
            }
            continue;
        }
        if name.eq_ignore_ascii_case(TRANSFER_ENCODING) {
            return Err(ResponseError::Send(
                UNSUPPORTED_TRANSFER_ENCODING.to_owned(),
            ));
        }
        if name.eq_ignore_ascii_case(CONTENT_TYPE) {
            let header_value: HeaderValue = match trimmed_value {
                STATIC_CT_HTML_UTF8 => HeaderValue::from_static(STATIC_CT_HTML_UTF8),
                STATIC_CT_PLAIN => HeaderValue::from_static(STATIC_CT_PLAIN),
                STATIC_CT_PLAIN_UTF8 => HeaderValue::from_static(STATIC_CT_PLAIN_UTF8),
                STATIC_CT_JSON => HeaderValue::from_static(STATIC_CT_JSON),
                _ => match HeaderValue::from_bytes(trimmed_value.as_bytes()) {
                    Ok(v) => v,
                    Err(_) => {
                        return Err(ResponseError::Send(MALFORMED_RESPONSE_HEADER.to_owned()));
                    }
                },
            };
            headers.append(HeaderName::from_static(CONTENT_TYPE), header_value);
            continue;
        }
        if name.eq_ignore_ascii_case(CONTENT_LENGTH) {
            let length: usize = parse_ascii_length(trimmed_value)
                .map_err(|error: RequestError| ResponseError::Send(error.to_string()))?;
            match announced_len {
                Some(previous) if previous != length => {
                    return Err(ResponseError::Send(OVERFLOWING_RESPONSE_BODY.to_owned()));
                }
                _ => announced_len = Some(length),
            }
            if hide_length {
                continue;
            }
            let Ok(header_value) = HeaderValue::from_bytes(trimmed_value.as_bytes()) else {
                return Err(ResponseError::Send(MALFORMED_RESPONSE_HEADER.to_owned()));
            };
            headers.append(HeaderName::from_static(CONTENT_LENGTH), header_value);
            continue;
        }
        let Ok(header_name) = HeaderName::from_bytes(name.as_bytes()) else {
            return Err(ResponseError::Send(MALFORMED_RESPONSE_HEADER.to_owned()));
        };
        let Ok(header_value) = HeaderValue::from_bytes(trimmed_value.as_bytes()) else {
            return Err(ResponseError::Send(MALFORMED_RESPONSE_HEADER.to_owned()));
        };
        let key: &str = header_name.as_str();
        if key.starts_with(PSEUDO_HEADER_PREFIX) || is_hop_by_hop(key) {
            continue;
        }
        headers.append(header_name, header_value);
    }
    if let Some(tokens) = nominated {
        for token in tokens {
            if let Ok(name) = HeaderName::from_bytes(token.as_bytes()) {
                headers.remove(name);
            }
        }
    }
    Ok(ParsedProtocolHead {
        response,
        announced_len,
        suppress_body,
    })
}

/// Parses a serialized response head into a status code and protocol headers.
///
/// The status line contributes the status code and each header line is decoded
/// on its own. Every line is checked: a head that is already committed to the
/// wire cannot be taken back, so dropping a line the parser does not understand
/// would ship a response that silently lost a header. A line without a
/// separator, a name that is not a valid header name, or a value carrying a byte
/// a field value cannot hold is refused instead.
///
/// A repeated header is appended rather than replaced, so a second `Set-Cookie`
/// survives the parse as its own field value.
///
/// Any HTTP version is accepted in the status line, including the HTTP/2 and
/// HTTP/3 forms the framework writes for a request that arrived on a
/// multiplexed stream. The version on the status line describes the internal
/// serialization of the response object, not the version this stream is
/// answered with, which stays the real protocol on the wire.
///
/// # Arguments
///
/// - `&[u8]` - The response head bytes, up to and including the blank line.
///
/// # Returns
///
/// - `Result<(usize, ProtocolHeaders), ResponseError>` - The status code and the
#[cfg(test)]
pub fn parse_response_head(head: &[u8]) -> Result<(usize, ProtocolHeaders), ResponseError> {
    let text: &str =
        from_utf8(head).map_err(|error: Utf8Error| ResponseError::Send(error.to_string()))?;
    let mut lines: SplitLines<'_> = text.split(CRLF);
    let status_line: &str = lines.next().unwrap_or(EMPTY_STR);
    let mut fields: SplitWhitespace<'_> = status_line.split_whitespace();
    let version: &str = fields.next().unwrap_or(EMPTY_STR);
    if !version.starts_with(HTTP_VERSION_PREFIX) {
        return Err(ResponseError::Send(MISSING_RESPONSE_VERSION.to_owned()));
    }
    let status_text: &str = fields.next().unwrap_or(EMPTY_STR);
    if status_text.is_empty() {
        return Err(ResponseError::Send(MISSING_RESPONSE_STATUS.to_owned()));
    }
    let status_code: usize = status_text
        .parse::<usize>()
        .map_err(|error: ParseIntError| ResponseError::Send(error.to_string()))?;
    let mut headers: ProtocolHeaders = http::HeaderMap::new();
    for line in lines {
        let line: &str = line.trim_end_matches(LF_CHAR);
        if line.is_empty() {
            continue;
        }
        let Some((name, value)) = line.split_once(COLON) else {
            return Err(ResponseError::Send(MALFORMED_RESPONSE_HEADER.to_owned()));
        };
        let name: &str = name.trim();
        if name.is_empty() {
            return Err(ResponseError::Send(MALFORMED_RESPONSE_HEADER.to_owned()));
        }
        let Ok(header_name) = http::HeaderName::from_bytes(name.as_bytes()) else {
            return Err(ResponseError::Send(MALFORMED_RESPONSE_HEADER.to_owned()));
        };
        let Ok(header_value) = http::HeaderValue::from_bytes(value.trim().as_bytes()) else {
            return Err(ResponseError::Send(MALFORMED_RESPONSE_HEADER.to_owned()));
        };
        headers.append(header_name, header_value);
    }
    Ok((status_code, headers))
}

/// Reads the transfer coding a response head announces, if it announces one.
///
/// Only `chunked` can describe a body whose length the head does not carry, and
/// neither HTTP/2 nor HTTP/3 can carry it: their data frames are the framing.
/// A head that announces any coding at all is therefore rejected by the pump
/// before a single frame is emitted, rather than stripped and delivered as a
/// body the client would have to decode.
///
/// # Arguments
///
/// - `&ProtocolHeaders` - The decoded response headers.
///
/// # Returns
///
/// - `bool` - `true` when the head carries a `transfer-encoding` field.
#[cfg(test)]
pub fn has_transfer_encoding(headers: &ProtocolHeaders) -> bool {
    headers.contains_key(TRANSFER_ENCODING)
}

/// Reads the body length a response head announces.
///
/// A head may repeat `content-length`; every occurrence is validated the same
/// way and the values must agree, because two different lengths describe two
/// different bodies and picking one of them hides the disagreement from the
/// client. A repeated field repeating the same value is accepted rather than
/// refused — the request parser makes the same allowance, and a duplicate that
/// agrees is not a disagreement.
///
/// # Arguments
///
/// - `&ProtocolHeaders` - The decoded response headers.
///
/// # Returns
///
/// - `Result<Option<usize>, ResponseError>` - The announced length, `None` when
#[cfg(test)]
pub fn parse_content_length(headers: &ProtocolHeaders) -> Result<Option<usize>, ResponseError> {
    let mut announced: Option<usize> = None;
    for value in headers.get_all(CONTENT_LENGTH).iter() {
        let Ok(value) = value.to_str() else {
            return Err(ResponseError::Send(MALFORMED_RESPONSE_HEADER.to_owned()));
        };
        let length: usize = parse_ascii_length(value)
            .map_err(|error: RequestError| ResponseError::Send(error.to_string()))?;
        match announced {
            Some(previous) if previous != length => {
                return Err(ResponseError::Send(OVERFLOWING_RESPONSE_BODY.to_owned()));
            }
            _ => announced = Some(length),
        }
    }
    Ok(announced)
}

/// Collects the headers a `Connection` field nominates as hop by hop.
///
/// A `Connection` header may name further headers that are hop by hop for this
/// message only, so removing the fixed list alone would forward a header the
/// sender explicitly scoped to the connection that is no longer being
/// negotiated.
///
/// The `connection` field itself is part of the answer: forwarding it would ask
/// the next hop to honour a connection token that no longer describes anything,
/// so it is nominated along with everything it names.
///
/// # Arguments
///
/// - `&ProtocolHeaders` - The headers to inspect.
/// - `&str` - The header name to test.
///
/// # Returns
///
/// - `bool` - Whether the `Connection` header nominates the name.
#[cfg(test)]
pub fn connection_nominates(headers: &ProtocolHeaders, name: &str) -> bool {
    if name.eq_ignore_ascii_case(CONNECTION) {
        return true;
    }
    for value in headers.get_all(CONNECTION).iter() {
        let Ok(value) = value.to_str() else {
            continue;
        };
        for token in value.split(COMMA_STR) {
            if token.trim().eq_ignore_ascii_case(name) {
                return true;
            }
        }
    }
    false
}
