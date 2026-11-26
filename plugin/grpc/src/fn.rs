use super::*;

/// Parses a gRPC `grpc-timeout` header value (`<1..8 digits><H|M|S|m|u|n>`) into a [`Duration`].
///
/// # Arguments
///
/// - `&str` - Raw `grpc-timeout` header value.
///
/// # Returns
///
/// - `Option<Duration>` - Parsed timeout duration, or `None` when malformed.
pub fn parse_grpc_timeout(raw: &str) -> Option<Duration> {
    let trimmed: &str = raw.trim();
    if trimmed.len() < 2 || trimmed.len() > 9 {
        return None;
    }
    let (digits, unit): (&str, &str) = trimmed.split_at(trimmed.len() - 1);
    if !digits.bytes().all(|b: u8| b.is_ascii_digit()) {
        return None;
    }
    let value: u64 = digits.parse::<u64>().ok()?;
    match unit {
        "H" => Some(Duration::from_secs(value.saturating_mul(3600))),
        "M" => Some(Duration::from_secs(value.saturating_mul(60))),
        "S" => Some(Duration::from_secs(value)),
        "m" => Some(Duration::from_millis(value)),
        "u" => Some(Duration::from_micros(value)),
        "n" => Some(Duration::from_nanos(value)),
        _ => None,
    }
}

/// Percent-encodes a gRPC status message per the gRPC over HTTP/2 specification.
///
/// # Arguments
///
/// - `&str` - Raw UTF-8 status message.
///
/// # Returns
///
/// - `String` - Wire-safe percent-encoded header value.
pub fn percent_encode_grpc_message(raw: &str) -> String {
    let mut out: String = String::with_capacity(raw.len());
    for &byte in raw.as_bytes() {
        let safe: bool = (0x20..=0x7e).contains(&byte) && byte != b'%';
        if safe {
            out.push(char::from(byte));
        } else {
            out.push('%');
            out.push(char::from(GRPC_HEX_UPPER[(byte >> 4) as usize]));
            out.push(char::from(GRPC_HEX_UPPER[(byte & 0x0f) as usize]));
        }
    }
    out
}

/// Reports whether a `Content-Type` header value matches `application/grpc` (or `+proto`/`+json`).
///
/// # Arguments
///
/// - `&str` - Raw `Content-Type` header value.
///
/// # Returns
///
/// - `bool` - `true` when the media type is gRPC.
pub fn is_grpc_content_type(content_type: &str) -> bool {
    let base: &str = content_type.split(';').next().unwrap_or_default().trim();
    if base.len() < GRPC_CONTENT_TYPE.len() {
        return false;
    }
    let (prefix, rest): (&str, &str) = base.split_at(GRPC_CONTENT_TYPE.len());
    prefix.eq_ignore_ascii_case(GRPC_CONTENT_TYPE) && (rest.is_empty() || rest.starts_with('+'))
}

/// Parses a `grpc-encoding` header token into a [`Compress`] algorithm.
///
/// # Arguments
///
/// - `Option<&str>` - Raw `grpc-encoding` header value.
///
/// # Returns
///
/// - `Compress` - Parsed compression algorithm (`Compress::Unknown` for identity or absent).
pub(crate) fn parse_grpc_encoding(raw: Option<&str>) -> Compress {
    let Some(token) = raw.map(str::trim) else {
        return Compress::Unknown;
    };
    if token.is_empty() || token.eq_ignore_ascii_case(GRPC_IDENTITY_ENCODING) {
        return Compress::Unknown;
    }
    Compress::from_str(token).unwrap_or(Compress::Unknown)
}
