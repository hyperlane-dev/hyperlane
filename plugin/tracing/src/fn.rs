use super::*;

/// Applies one SplitMix64 avalanche mix step.
///
/// # Arguments
///
/// - `u64` - Input word.
///
/// # Returns
///
/// - `u64` - Mixed 64-bit word.
pub(crate) fn mix64(mut x: u64) -> u64 {
    x = (x ^ (x >> 30)).wrapping_mul(SPLITMIX_MUL1);
    x = (x ^ (x >> 27)).wrapping_mul(SPLITMIX_MUL2);
    x ^ (x >> 31)
}

/// Returns a process-unique entropy seed.
///
/// # Returns
///
/// - `u64` - Initialized 64-bit seed.
pub(crate) fn process_trace_seed() -> u64 {
    *TRACE_SEED.get_or_init(|| {
        let nanos: u128 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|dur: Duration| dur.as_nanos())
            .unwrap_or(0);
        let pid: u64 = process::id() as u64;
        let probe: u64 = (&nanos as *const u128) as usize as u64;
        mix64((nanos as u64) ^ pid.rotate_left(29) ^ probe)
    })
}

/// Formats a 64-bit value into 16 lowercase hexadecimal ASCII bytes.
///
/// # Arguments
///
/// - `u64` - 64-bit word.
/// - `&mut [u8]` - Destination buffer of at least 16 bytes.
pub(crate) fn write_hex_u64(word: u64, dst: &mut [u8]) {
    for (idx, slot) in dst.iter_mut().take(16).enumerate() {
        let shift: usize = (15 - idx) * 4;
        let nibble: usize = ((word >> shift) & 0x0F) as usize;
        *slot = HEX_DIGITS[nibble];
    }
}

/// Generates a non-zero 128-bit W3C trace ID encoded as 32 lowercase hexadecimal characters.
///
/// # Returns
///
/// - `String` - 32-character hex trace ID.
pub fn generate_trace_id() -> String {
    let seed: u64 = process_trace_seed();
    let seq: u64 = TRACE_SEQ.fetch_add(1, Ordering::Relaxed);
    let nanos: u64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|dur: Duration| dur.as_nanos() as u64)
        .unwrap_or(0);
    let hi: u64 = mix64(seed.wrapping_add(seq.wrapping_mul(SPLITMIX_GAMMA)) ^ nanos);
    let lo: u64 = mix64(hi.wrapping_add(SPLITMIX_GAMMA) ^ seq.rotate_left(17)) | 1;
    let mut buf: [u8; 32] = [0u8; 32];
    write_hex_u64(hi, &mut buf[0..16]);
    write_hex_u64(lo, &mut buf[16..32]);
    String::from_utf8_lossy(&buf).into_owned()
}

/// Generates a non-zero 64-bit W3C span ID encoded as 16 lowercase hexadecimal characters.
///
/// # Returns
///
/// - `String` - 16-character hex span ID.
pub fn generate_span_id() -> String {
    let seed: u64 = process_trace_seed();
    let seq: u64 = TRACE_SEQ.fetch_add(1, Ordering::Relaxed);
    let word: u64 = mix64(seed ^ seq.wrapping_mul(SPLITMIX_GAMMA)) | 1;
    let mut buf: [u8; 16] = [0u8; 16];
    write_hex_u64(word, &mut buf);
    String::from_utf8_lossy(&buf).into_owned()
}

/// Reports whether `slice` consists of exact `expected_len` lowercase/uppercase ASCII hex digits and is non-zero.
///
/// # Arguments
///
/// - `&str` - Candidate hexadecimal string.
/// - `usize` - Expected byte length (`32` for trace ID, `16` for span ID).
///
/// # Returns
///
/// - `bool` - `true` when valid and non-zero.
pub(crate) fn is_valid_nonzero_hex(slice: &str, expected_len: usize) -> bool {
    if slice.len() != expected_len {
        return false;
    }
    let mut any_nonzero: bool = false;
    for byte in slice.bytes() {
        if !byte.is_ascii_hexdigit() {
            return false;
        }
        if byte != b'0' {
            any_nonzero = true;
        }
    }
    any_nonzero
}

/// Parses a W3C `traceparent` header (`00-<32hex>-<16hex>-<2hex>`).
///
/// # Arguments
///
/// - `&str` - Raw `traceparent` header value.
///
/// # Returns
///
/// - `Option<(String, String, bool)>` - `(trace_id, parent_span_id, sampled)` when valid.
pub fn parse_traceparent(header: &str) -> Option<(String, String, bool)> {
    let mut parts: Split<'_, char> = header.trim().split('-');
    let version: &str = parts.next()?;
    let trace_id: &str = parts.next()?;
    let parent_id: &str = parts.next()?;
    let flags: &str = parts.next()?;
    let version_hex: bool = version.bytes().all(|b: u8| b.is_ascii_hexdigit());
    if parts.next().is_some()
        || version.len() != 2
        || version.eq_ignore_ascii_case("ff")
        || !version_hex
        || !is_valid_nonzero_hex(trace_id, 32)
        || !is_valid_nonzero_hex(parent_id, 16)
        || flags.len() != 2
    {
        return None;
    }
    let flags_byte: u8 = u8::from_str_radix(flags, 16).ok()?;
    let sampled: bool = (flags_byte & 0x01) != 0;
    Some((
        trace_id.to_ascii_lowercase(),
        parent_id.to_ascii_lowercase(),
        sampled,
    ))
}

/// Formats a W3C `traceparent` header string (`00-<trace_id>-<span_id>-<flags>`).
///
/// # Arguments
///
/// - `&str` - 32-char lowercase hex trace ID.
/// - `&str` - 16-char lowercase hex span ID.
/// - `bool` - Whether the trace is sampled.
///
/// # Returns
///
/// - `String` - Canonical 55-character `traceparent` value.
pub fn format_traceparent(trace_id: &str, span_id: &str, sampled: bool) -> String {
    let flags: &str = if sampled {
        TRACE_FLAGS_SAMPLED
    } else {
        TRACE_FLAGS_UNSAMPLED
    };
    let mut out: String = String::with_capacity(55);
    out.push_str(TRACEPARENT_VERSION);
    out.push('-');
    out.push_str(trace_id);
    out.push('-');
    out.push_str(span_id);
    out.push('-');
    out.push_str(flags);
    out
}
