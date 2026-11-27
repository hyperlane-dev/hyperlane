use super::*;

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

/// Processes one 64-byte SHA-256 message block into `state`.
///
/// # Arguments
///
/// - `&mut [u32; 8]` - Running SHA-256 hash state.
/// - `&[u8; 64]` - 512-bit input block.
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

/// Computes the RFC 4231 HMAC-SHA256 authentication code of `data` under `key`.
///
/// # Arguments
///
/// - `&[u8]` - Secret HMAC key bytes.
/// - `&[u8]` - Message payload bytes.
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
    let mut inner_input: Vec<u8> = Vec::with_capacity(64 + data.len());
    for byte in key_block {
        inner_input.push(byte ^ 0x36);
    }
    inner_input.extend_from_slice(data);
    let inner_hash: [u8; 32] = sha256(&inner_input);
    let mut outer_input: [u8; 96] = [0u8; 96];
    for i in 0..64usize {
        outer_input[i] = key_block[i] ^ 0x5c;
    }
    outer_input[64..96].copy_from_slice(&inner_hash);
    sha256(&outer_input)
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
/// - `bool` - `true` iff both slices have identical length and contents.
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

/// Encodes a 32-byte digest into a 64-character lowercase hexadecimal string.
///
/// # Arguments
///
/// - `&[u8; 32]` - 256-bit digest.
///
/// # Returns
///
/// - `String` - Lowercase hexadecimal encoding.
pub(crate) fn encode_hex_32(bytes: &[u8; 32]) -> String {
    let mut out: String = String::with_capacity(64);
    for byte in bytes {
        out.push(HEX_DIGITS[(byte >> 4) as usize] as char);
        out.push(HEX_DIGITS[(byte & 0x0F) as usize] as char);
    }
    out
}

/// Issues a stateless HMAC-SHA256 signed CSRF token `"<nonce>.<hex_mac>"`.
///
/// # Arguments
///
/// - `&[u8]` - Server-side CSRF HMAC secret.
/// - `&str` - Caller-supplied unique nonce (must not contain `'.'`).
///
/// # Returns
///
/// - `String` - Signed CSRF token suitable for double-submit cookie and header transport.
pub fn issue_csrf_token(secret: &[u8], nonce: &str) -> String {
    let mac: [u8; 32] = hmac_sha256(secret, nonce.as_bytes());
    let sig: String = encode_hex_32(&mac);
    let mut out: String = String::with_capacity(nonce.len() + 1 + sig.len());
    out.push_str(nonce);
    out.push('.');
    out.push_str(&sig);
    out
}

/// Verifies a stateless HMAC-SHA256 signed CSRF token in constant time.
///
/// # Arguments
///
/// - `&[u8]` - Server-side CSRF HMAC secret.
/// - `&str` - Candidate CSRF token (`"<nonce>.<hex_mac>"`).
///
/// # Returns
///
/// - `bool` - `true` when the token signature is valid under `secret`.
pub fn verify_csrf_token(secret: &[u8], token: &str) -> bool {
    let Some((nonce, provided_sig)) = token.split_once('.') else {
        return false;
    };
    if nonce.is_empty() || provided_sig.len() != 64 {
        return false;
    }
    let expected_mac: [u8; 32] = hmac_sha256(secret, nonce.as_bytes());
    let expected_sig: String = encode_hex_32(&expected_mac);
    constant_time_eq(provided_sig.as_bytes(), expected_sig.as_bytes())
}

/// Detects raw or percent-encoded path traversal (`..`, `%2e%2e`), backslashes, or NUL bytes.
///
/// # Arguments
///
/// - `&str` - Incoming URL path.
///
/// # Returns
///
/// - `bool` - `true` when a suspicious path-traversal sequence is present.
pub fn has_path_traversal(path: &str) -> bool {
    if path.contains("..") || path.contains('\\') || path.contains('\0') {
        return true;
    }
    let lower: String = path.to_ascii_lowercase();
    lower.contains(ENCODED_NUL)
        || lower.contains(ENCODED_BACKSLASH_LOWER)
        || lower.contains(&format!("{ENCODED_DOT_LOWER}{ENCODED_DOT_LOWER}"))
        || lower.contains(&format!(".{ENCODED_DOT_LOWER}"))
        || lower.contains(&format!("{ENCODED_DOT_LOWER}."))
}

/// Reports whether `method` is a safe read-only HTTP method exempt from CSRF checks.
///
/// # Arguments
///
/// - `&Method` - Request HTTP method.
///
/// # Returns
///
/// - `bool` - `true` for `GET`, `HEAD`, `OPTIONS`, or `TRACE`.
pub(crate) fn is_csrf_safe_method(method: &Method) -> bool {
    method.is_get() || method.is_head() || method.is_options() || method.is_trace()
}
