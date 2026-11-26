use super::*;

/// Resolves the transfer coding of an HTTP/1.1 request.
///
/// # Arguments
///
/// - `&RequestHeadersValue` - The values in wire order.
/// # Returns
///
/// - `Result<RequestFraming, RequestError>` - Chunked framing or the reason it cannot be decoded.
#[cold]
pub(super) fn parse_transfer_encoding(
    values: &RequestHeadersValue,
) -> Result<RequestFraming, RequestError> {
    let invalid: RequestError = RequestError::HttpRead(HttpStatus::BadRequest);
    let mut chunked: bool = false;
    let mut unsupported: bool = false;
    for value in values {
        for item in value.split(COMMA) {
            let coding: &str = item.trim_matches(HTTP_HEADER_OPTIONAL_WHITESPACE);
            if coding.is_empty() {
                continue;
            }
            if chunked {
                return Err(invalid);
            }
            if coding.eq_ignore_ascii_case(CHUNKED) {
                chunked = true;
            } else if coding.bytes().all(|byte: u8| {
                byte.is_ascii_alphanumeric() || HTTP_HEADER_TOKEN_PUNCTUATION.contains(&byte)
            }) {
                unsupported = true;
            } else {
                return Err(invalid);
            }
        }
    }
    if !chunked {
        return Err(invalid);
    }
    if unsupported {
        return Err(RequestError::HttpRead(HttpStatus::NotImplemented));
    }
    Ok(RequestFraming::Chunked)
}

/// Reads one LF-terminated HTTP line with a strict upper bound to prevent unbounded memory growth.
///
/// # Arguments
///
/// - `&mut R` - The buffered reader to consume from.
/// - `&mut String` - The destination string buffer (cleared before reading).
/// - `usize` - The maximum line length in bytes (`usize::MAX` disables the cap).
/// - `RequestError` - The error returned when the line exceeds `max_len`.
///
/// # Returns
///
/// - `Result<usize, RequestError>` - The number of bytes read (`0` on clean EOF), or a limit/UTF-8/I/O error.
pub(crate) async fn read_http_line_bounded<R>(
    reader: &mut R,
    out: &mut String,
    max_len: usize,
    overflow_error: RequestError,
) -> Result<usize, RequestError>
where
    R: AsyncBufReadExt + Unpin,
{
    out.clear();
    let mut raw: Vec<u8> = Vec::new();
    loop {
        let available: &[u8] = reader.fill_buf().await?;
        if available.is_empty() {
            break;
        }
        let budget: usize = max_len.saturating_sub(raw.len());
        if budget == 0 {
            return Err(overflow_error);
        }
        let scan_len: usize = available.len().min(budget);
        let newline: Option<usize> = available[..scan_len]
            .iter()
            .position(|byte: &u8| *byte == BR_U8);
        let consumed: usize = newline.map_or(scan_len, |index: usize| index + 1);
        raw.extend_from_slice(&available[..consumed]);
        reader.consume(consumed);
        if newline.is_some() {
            break;
        }
        if raw.len() >= max_len && max_len != usize::MAX {
            return Err(overflow_error);
        }
    }
    let text: &str =
        from_utf8(&raw).map_err(|_| RequestError::ReadConnection(HttpStatus::BadRequest))?;
    out.push_str(text);
    Ok(raw.len())
}

/// Validates an HTTP/1.1 `Host` header value per RFC 9110 Section 7.2 (`uri-host [ ":" port ]`).
///
/// Rejects userinfo (`@`), whitespace, path/query delimiters, control characters, and
/// malformed IPv6 bracket literals or ports.
///
/// # Arguments
///
/// - `&str` - The trimmed `Host` header value.
///
/// # Returns
///
/// - `bool` - `true` when the `Host` value is syntactically valid.
#[inline(always)]
pub(crate) fn is_valid_http_host(value: &str) -> bool {
    if value.is_empty() {
        return true;
    }
    let bytes: &[u8] = value.as_bytes();
    let (host_part, port_part): (&[u8], Option<&[u8]>) = if bytes[0] == b'[' {
        let Some(close_idx) = bytes.iter().position(|byte: &u8| *byte == b']') else {
            return false;
        };
        let literal: &[u8] = &bytes[1..close_idx];
        if literal.is_empty()
            || !literal
                .iter()
                .all(|byte: &u8| byte.is_ascii_hexdigit() || *byte == b':' || *byte == b'.')
        {
            return false;
        }
        let rest: &[u8] = &bytes[close_idx + 1..];
        if rest.is_empty() {
            (literal, None)
        } else if let Some(port) = rest.strip_prefix(b":") {
            (literal, Some(port))
        } else {
            return false;
        }
    } else {
        let mut colon_pos: Option<usize> = None;
        for (idx, &byte) in bytes.iter().enumerate() {
            if byte == b':' {
                if colon_pos.is_some() {
                    return false;
                }
                colon_pos = Some(idx);
            }
        }
        match colon_pos {
            Some(idx) => (&bytes[..idx], Some(&bytes[idx + 1..])),
            None => (bytes, None),
        }
    };
    if bytes[0] != b'[' {
        if host_part.is_empty() {
            return false;
        }
        let valid_reg_name: bool = host_part.iter().all(|byte: &u8| {
            byte.is_ascii_alphanumeric()
                || matches!(
                    *byte,
                    b'-' | b'.'
                        | b'_'
                        | b'~'
                        | b'%'
                        | b'!'
                        | b'$'
                        | b'&'
                        | b'\''
                        | b'('
                        | b')'
                        | b'*'
                        | b'+'
                        | b','
                        | b';'
                        | b'='
                )
        });
        if !valid_reg_name {
            return false;
        }
    }
    if let Some(port) = port_part {
        if port.is_empty() || !port.iter().all(|byte: &u8| byte.is_ascii_digit()) {
            return false;
        }
        let Ok(port_str) = from_utf8(port) else {
            return false;
        };
        if port_str.parse::<u16>().is_err() {
            return false;
        }
    }
    true
}

/// Parses one entity-tag prefix without allocating or interpreting escapes.
///
/// # Arguments
///
/// - `&[u8]` - The input beginning with a strong or weak entity-tag.
///
/// # Returns
///
/// - `Option<(&[u8], &[u8])>` - The opaque value and unconsumed suffix, or `None` for invalid syntax.
pub(super) fn parse_entity_tag(input: &[u8]) -> Option<(&[u8], &[u8])> {
    let bytes: &[u8] = input.strip_prefix(ENTITY_TAG_WEAK_PREFIX).unwrap_or(input);
    let opaque: &[u8] = bytes.strip_prefix(&[ENTITY_TAG_QUOTE])?;
    for (index, &byte) in opaque.iter().enumerate() {
        if byte == ENTITY_TAG_QUOTE {
            return Some((&opaque[..index], &opaque[index + 1..]));
        }
        if byte <= HTTP_HEADER_CONTROL_LIMIT || byte == HTTP_HEADER_DELETE {
            return None;
        }
    }
    None
}
