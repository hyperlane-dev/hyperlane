use super::*;

/// Returns the current Unix epoch timestamp in whole seconds.
///
/// # Returns
///
/// - `u64` - Seconds since Unix epoch.
pub fn auth_now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|dur: Duration| dur.as_secs())
        .unwrap_or(0)
}

/// Computes the FIPS 180-4 SHA-256 digest of `data`.
///
/// # Arguments
///
/// - `&[u8]` - Input byte slice.
///
/// # Returns
///
/// - `[u8; 32]` - 256-bit SHA-256 digest.
pub fn sha256(data: &[u8]) -> [u8; 32] {
    let mut state: [u32; 8] = SHA256_INIT;
    let bit_len: u64 = (data.len() as u64).wrapping_mul(8);
    let mut offset: usize = 0;
    while data.len() - offset >= 64 {
        let mut block: [u8; 64] = [0u8; 64];
        block.copy_from_slice(&data[offset..offset + 64]);
        sha256_compress(&mut state, &block);
        offset += 64;
    }
    let rem: usize = data.len() - offset;
    let mut tail: [u8; 128] = [0u8; 128];
    tail[..rem].copy_from_slice(&data[offset..]);
    tail[rem] = 0x80;
    let total_tail: usize = if rem < 56 { 64 } else { 128 };
    tail[total_tail - 8..total_tail].copy_from_slice(&bit_len.to_be_bytes());
    let mut b0: [u8; 64] = [0u8; 64];
    b0.copy_from_slice(&tail[..64]);
    sha256_compress(&mut state, &b0);
    if total_tail == 128 {
        let mut b1: [u8; 64] = [0u8; 64];
        b1.copy_from_slice(&tail[64..128]);
        sha256_compress(&mut state, &b1);
    }
    let mut out: [u8; 32] = [0u8; 32];
    for (i, word) in state.iter().enumerate() {
        out[i * 4..i * 4 + 4].copy_from_slice(&word.to_be_bytes());
    }
    out
}

/// Compresses one 64-byte SHA-256 block into `state`.
///
/// # Arguments
///
/// - `&mut [u32; 8]` - Running hash state.
/// - `&[u8; 64]` - Input block.
pub(crate) fn sha256_compress(state: &mut [u32; 8], block: &[u8; 64]) {
    let mut w: [u32; 64] = [0u32; 64];
    for i in 0..16usize {
        w[i] = u32::from_be_bytes([
            block[i * 4],
            block[i * 4 + 1],
            block[i * 4 + 2],
            block[i * 4 + 3],
        ]);
    }
    for i in 16..64usize {
        let s0: u32 = w[i - 15].rotate_right(7) ^ w[i - 15].rotate_right(18) ^ (w[i - 15] >> 3);
        let s1: u32 = w[i - 2].rotate_right(17) ^ w[i - 2].rotate_right(19) ^ (w[i - 2] >> 10);
        w[i] = w[i - 16]
            .wrapping_add(s0)
            .wrapping_add(w[i - 7])
            .wrapping_add(s1);
    }
    let mut a: u32 = state[0];
    let mut b: u32 = state[1];
    let mut c: u32 = state[2];
    let mut d: u32 = state[3];
    let mut e: u32 = state[4];
    let mut f: u32 = state[5];
    let mut g: u32 = state[6];
    let mut h: u32 = state[7];
    for i in 0..64usize {
        let s1: u32 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
        let ch: u32 = (e & f) ^ ((!e) & g);
        let temp1: u32 = h
            .wrapping_add(s1)
            .wrapping_add(ch)
            .wrapping_add(SHA256_K[i])
            .wrapping_add(w[i]);
        let s0: u32 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
        let maj: u32 = (a & b) ^ (a & c) ^ (b & c);
        let temp2: u32 = s0.wrapping_add(maj);
        h = g;
        g = f;
        f = e;
        e = d.wrapping_add(temp1);
        d = c;
        c = b;
        b = a;
        a = temp1.wrapping_add(temp2);
    }
    state[0] = state[0].wrapping_add(a);
    state[1] = state[1].wrapping_add(b);
    state[2] = state[2].wrapping_add(c);
    state[3] = state[3].wrapping_add(d);
    state[4] = state[4].wrapping_add(e);
    state[5] = state[5].wrapping_add(f);
    state[6] = state[6].wrapping_add(g);
    state[7] = state[7].wrapping_add(h);
}

/// Computes the RFC 4231 HMAC-SHA256 tag of `data` under `key`.
///
/// # Arguments
///
/// - `&[u8]` - Secret key bytes.
/// - `&[u8]` - Input message bytes.
///
/// # Returns
///
/// - `[u8; 32]` - 256-bit HMAC-SHA256 tag.
pub fn hmac_sha256(key: &[u8], data: &[u8]) -> [u8; 32] {
    let mut key_block: [u8; 64] = [0u8; 64];
    if key.len() > 64 {
        let hashed: [u8; 32] = sha256(key);
        key_block[..32].copy_from_slice(&hashed);
    } else {
        key_block[..key.len()].copy_from_slice(key);
    }
    let mut inner: Vec<u8> = Vec::with_capacity(64 + data.len());
    for byte in key_block {
        inner.push(byte ^ 0x36);
    }
    inner.extend_from_slice(data);
    let inner_hash: [u8; 32] = sha256(&inner);
    let mut outer: [u8; 96] = [0u8; 96];
    for i in 0..64usize {
        outer[i] = key_block[i] ^ 0x5c;
    }
    outer[64..96].copy_from_slice(&inner_hash);
    sha256(&outer)
}

/// Compares two byte slices in constant time with respect to their contents.
///
/// # Arguments
///
/// - `&[u8]` - First byte slice.
/// - `&[u8]` - Second byte slice.
///
/// # Returns
///
/// - `bool` - `true` when lengths and bytes are identical.
pub fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff: u8 = 0;
    for i in 0..a.len() {
        diff |= a[i] ^ b[i];
    }
    diff == 0
}

/// Encodes `input` using unpadded RFC 4648 base64url.
///
/// # Arguments
///
/// - `&[u8]` - Raw bytes to encode.
///
/// # Returns
///
/// - `String` - Unpadded base64url string.
pub fn base64url_encode(input: &[u8]) -> String {
    let mut out: String = String::with_capacity(input.len().div_ceil(3) * 4);
    let mut i: usize = 0;
    while i + 3 <= input.len() {
        let triple: u32 =
            ((input[i] as u32) << 16) | ((input[i + 1] as u32) << 8) | (input[i + 2] as u32);
        out.push(BASE64URL_ALPHABET[((triple >> 18) & 0x3F) as usize] as char);
        out.push(BASE64URL_ALPHABET[((triple >> 12) & 0x3F) as usize] as char);
        out.push(BASE64URL_ALPHABET[((triple >> 6) & 0x3F) as usize] as char);
        out.push(BASE64URL_ALPHABET[(triple & 0x3F) as usize] as char);
        i += 3;
    }
    let rem: usize = input.len() - i;
    if rem == 1 {
        let val: u32 = (input[i] as u32) << 16;
        out.push(BASE64URL_ALPHABET[((val >> 18) & 0x3F) as usize] as char);
        out.push(BASE64URL_ALPHABET[((val >> 12) & 0x3F) as usize] as char);
    } else if rem == 2 {
        let val: u32 = ((input[i] as u32) << 16) | ((input[i + 1] as u32) << 8);
        out.push(BASE64URL_ALPHABET[((val >> 18) & 0x3F) as usize] as char);
        out.push(BASE64URL_ALPHABET[((val >> 12) & 0x3F) as usize] as char);
        out.push(BASE64URL_ALPHABET[((val >> 6) & 0x3F) as usize] as char);
    }
    out
}

/// Decodes a single base64 or base64url alphabet character into its 6-bit value.
///
/// # Arguments
///
/// - `u8` - ASCII byte to decode.
/// - `bool` - When `true`, accepts URL-safe `-` and `_`; when `false`, accepts standard `+` and `/`.
///
/// # Returns
///
/// - `Option<u8>` - Decoded 6-bit value, or `None` when invalid.
pub(crate) fn decode_b64_char(byte: u8, url_safe: bool) -> Option<u8> {
    match byte {
        b'A'..=b'Z' => Some(byte - b'A'),
        b'a'..=b'z' => Some(byte - b'a' + 26),
        b'0'..=b'9' => Some(byte - b'0' + 52),
        b'-' if url_safe => Some(62),
        b'_' if url_safe => Some(63),
        b'+' if !url_safe => Some(62),
        b'/' if !url_safe => Some(63),
        _ => None,
    }
}

/// Decodes unpadded (or padded) base64 / base64url text into raw bytes.
///
/// # Arguments
///
/// - `&str` - Encoded string.
/// - `bool` - `true` for base64url, `false` for standard base64.
///
/// # Returns
///
/// - `Option<Vec<u8>>` - Decoded bytes, or `None` on malformed input.
pub(crate) fn decode_base64_inner(input: &str, url_safe: bool) -> Option<Vec<u8>> {
    let trimmed: &str = input.trim_end_matches('=');
    let bytes: &[u8] = trimmed.as_bytes();
    if bytes.len() % 4 == 1 {
        return None;
    }
    let mut out: Vec<u8> = Vec::with_capacity((bytes.len() * 3) / 4);
    let mut i: usize = 0;
    while i + 4 <= bytes.len() {
        let a: u32 = decode_b64_char(bytes[i], url_safe)? as u32;
        let b: u32 = decode_b64_char(bytes[i + 1], url_safe)? as u32;
        let c: u32 = decode_b64_char(bytes[i + 2], url_safe)? as u32;
        let d: u32 = decode_b64_char(bytes[i + 3], url_safe)? as u32;
        let quad: u32 = (a << 18) | (b << 12) | (c << 6) | d;
        out.push(((quad >> 16) & 0xFF) as u8);
        out.push(((quad >> 8) & 0xFF) as u8);
        out.push((quad & 0xFF) as u8);
        i += 4;
    }
    let rem: usize = bytes.len() - i;
    if rem == 2 {
        let a: u32 = decode_b64_char(bytes[i], url_safe)? as u32;
        let b: u32 = decode_b64_char(bytes[i + 1], url_safe)? as u32;
        let val: u32 = (a << 18) | (b << 12);
        out.push(((val >> 16) & 0xFF) as u8);
    } else if rem == 3 {
        let a: u32 = decode_b64_char(bytes[i], url_safe)? as u32;
        let b: u32 = decode_b64_char(bytes[i + 1], url_safe)? as u32;
        let c: u32 = decode_b64_char(bytes[i + 2], url_safe)? as u32;
        let val: u32 = (a << 18) | (b << 12) | (c << 6);
        out.push(((val >> 16) & 0xFF) as u8);
        out.push(((val >> 8) & 0xFF) as u8);
    }
    Some(out)
}

/// Decodes an unpadded RFC 4648 base64url string.
///
/// # Arguments
///
/// - `&str` - Base64url-encoded string.
///
/// # Returns
///
/// - `Option<Vec<u8>>` - Decoded bytes, or `None` if malformed.
pub fn base64url_decode(input: &str) -> Option<Vec<u8>> {
    decode_base64_inner(input, true)
}

/// Decodes a standard RFC 4648 base64 string (used by HTTP Basic authentication).
///
/// # Arguments
///
/// - `&str` - Standard base64-encoded string.
///
/// # Returns
///
/// - `Option<Vec<u8>>` - Decoded bytes, or `None` if malformed.
pub fn base64_decode(input: &str) -> Option<Vec<u8>> {
    decode_base64_inner(input, false)
}

/// Extracts a case-insensitive prefix-stripped token from an `Authorization` header.
///
/// # Arguments
///
/// - `&'a str` - Raw `Authorization` header value.
/// - `&str` - Expected lowercase scheme prefix ending with space (for example `"bearer "`).
///
/// # Returns
///
/// - `Option<&'a str>` - Trimmed credential payload, or `None` when the scheme does not match.
pub(crate) fn strip_auth_scheme<'a>(header: &'a str, lowercase_prefix: &str) -> Option<&'a str> {
    let trimmed: &'a str = header.trim();
    if trimmed.len() < lowercase_prefix.len() {
        return None;
    }
    let (head, tail): (&'a str, &'a str) = trimmed.split_at(lowercase_prefix.len());
    if head.eq_ignore_ascii_case(lowercase_prefix) {
        let token: &'a str = tail.trim();
        if token.is_empty() { None } else { Some(token) }
    } else {
        None
    }
}
