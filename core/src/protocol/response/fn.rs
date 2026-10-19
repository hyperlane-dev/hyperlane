use super::*;

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
///
/// # Returns
///
/// - `Vec<String>` - The lowercased header names nominated by `Connection`.
pub fn connection_nominated_headers(headers: &ProtocolHeaders) -> Vec<String> {
    let mut nominated: Vec<String> = vec![CONNECTION.to_owned()];
    for value in headers.get_all(CONNECTION).iter() {
        let Ok(value) = value.to_str() else {
            continue;
        };
        for token in value.split(COMMA_STR) {
            let token: String = token.trim().to_ascii_lowercase();
            if !token.is_empty() && !nominated.contains(&token) {
                nominated.push(token);
            }
        }
    }
    nominated
}

/// Waits until the send stream may carry at least one more byte.
///
/// The granted capacity can be smaller than the amount requested, so the
/// returned value is what the caller may send now, not what it asked for. A
/// stream that closes before granting is a failure, not a grant of everything
/// the caller asked for: reporting `wanted` there would hand the framer a
/// window the peer never granted and turn a dead stream into an unbounded loop.
///
/// # Arguments
///
/// - `&mut h2::SendStream<Bytes>` - The stream half to reserve capacity on.
/// - `usize` - The number of bytes the caller intends to send.
///
/// # Returns
///
/// - `Result<usize, ResponseError>` - The granted capacity, or the first
pub(crate) async fn next_h2_capacity(
    send_stream: &mut h2::SendStream<Bytes>,
    wanted: usize,
) -> Result<usize, ResponseError> {
    send_stream.reserve_capacity(wanted);
    loop {
        let granted: Option<Result<usize, h2::Error>> =
            poll_fn(|context: &mut task::Context<'_>| send_stream.poll_capacity(context)).await;
        match granted {
            Some(Ok(capacity)) => {
                if capacity > 0 {
                    return Ok(capacity.min(wanted));
                }
            }
            Some(Err(error)) => return Err(ResponseError::Send(error.to_string())),
            None => return Err(ResponseError::Send(MISSING_SEND_CAPACITY.to_owned())),
        }
    }
}
