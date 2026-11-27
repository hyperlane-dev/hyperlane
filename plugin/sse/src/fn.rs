use super::*;

/// Computes the shard index for `topic` using `XxHash3_64`.
///
/// # Arguments
///
/// - `&str` - Topic name to hash.
///
/// # Returns
///
/// - `usize` - Shard slot in `0..SSE_SHARD_COUNT`.
pub(crate) fn sse_shard_index(topic: &str) -> usize {
    let mut hasher: XxHash3_64 = XxHash3_64::default();
    hasher.write(topic.as_bytes());
    (hasher.finish() as usize) & (SSE_SHARD_COUNT - 1)
}

/// Writes sanitized single-line SSE header bytes directly into `out`, stripping `\r`, `\n`, and NUL.
///
/// # Arguments
///
/// - `&str` - Raw field text.
/// - `&mut Vec<u8>` - Destination wire buffer.
pub(crate) fn write_sanitized_sse_bytes(raw: &str, out: &mut Vec<u8>) {
    let bytes: &[u8] = raw.as_bytes();
    let has_forbidden: bool = bytes
        .iter()
        .any(|b: &u8| *b == b'\r' || *b == b'\n' || *b == 0);
    if !has_forbidden {
        out.extend_from_slice(bytes);
        return;
    }
    for &b in bytes {
        if b != b'\r' && b != b'\n' && b != 0 {
            out.push(b);
        }
    }
}

/// Writes an unsigned 64-bit decimal integer directly into `out` without allocating.
///
/// # Arguments
///
/// - `u64` - Unsigned integer to format.
/// - `&mut Vec<u8>` - Destination wire buffer.
pub(crate) fn write_u64_decimal(mut value: u64, out: &mut Vec<u8>) {
    if value == 0 {
        out.push(b'0');
        return;
    }
    let mut digits: [u8; 20] = [0u8; 20];
    let mut pos: usize = digits.len();
    while value > 0 {
        pos -= 1;
        digits[pos] = b'0' + (value % 10) as u8;
        value /= 10;
    }
    out.extend_from_slice(&digits[pos..]);
}

/// Parses a `Last-Event-ID` string into a numeric sequence id when valid.
///
/// # Arguments
///
/// - `Option<&str>` - Raw `Last-Event-ID` header value.
///
/// # Returns
///
/// - `Option<u64>` - Parsed sequence id, or `None` when absent or non-numeric.
pub(crate) fn parse_last_event_id(raw: Option<&str>) -> Option<u64> {
    raw.and_then(|text: &str| text.trim().parse::<u64>().ok())
}
