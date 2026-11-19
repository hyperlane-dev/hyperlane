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
