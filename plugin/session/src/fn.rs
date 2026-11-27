use super::*;

/// Returns the current Unix epoch timestamp in seconds.
///
/// # Returns
///
/// - `u64` - Seconds since Unix epoch.
pub(crate) fn session_now_secs() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|dur: Duration| dur.as_secs())
        .unwrap_or(0)
}

/// Computes the shard index for `session_id` using `XxHash3_64`.
///
/// # Arguments
///
/// - `&str` - Session identifier.
///
/// # Returns
///
/// - `usize` - Shard index in `0..SESSION_SHARD_COUNT`.
pub(crate) fn session_shard_index(session_id: &str) -> usize {
    let mut hasher: XxHash3_64 = XxHash3_64::default();
    hasher.write(session_id.as_bytes());
    (hasher.finish() as usize) & (SESSION_SHARD_COUNT - 1)
}

/// Applies one SplitMix64 mixing round.
///
/// # Arguments
///
/// - `u64` - Input word.
///
/// # Returns
///
/// - `u64` - Mixed 64-bit output.
pub(crate) fn mix64(mut x: u64) -> u64 {
    x = (x ^ (x >> 30)).wrapping_mul(SPLITMIX_MUL1);
    x = (x ^ (x >> 27)).wrapping_mul(SPLITMIX_MUL2);
    x ^ (x >> 31)
}

/// Generates an unpredictable 128-bit lowercase hexadecimal session identifier.
///
/// # Returns
///
/// - `String` - 32-character hexadecimal session identifier.
pub fn generate_session_id() -> String {
    let seed: u64 = *SESSION_SEED.get_or_init(|| {
        let nanos: u128 = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map(|dur: Duration| dur.as_nanos())
            .unwrap_or(0);
        let pid: u64 = process::id() as u64;
        let probe: u64 = (&nanos as *const u128) as usize as u64;
        mix64((nanos as u64) ^ pid.rotate_left(23) ^ probe)
    });
    let seq: u64 = SESSION_SEQ.fetch_add(1, Ordering::Relaxed);
    let nanos: u64 = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|dur: Duration| dur.as_nanos() as u64)
        .unwrap_or(0);
    let hi: u64 = mix64(seed.wrapping_add(seq.wrapping_mul(SPLITMIX_GAMMA)) ^ nanos);
    let lo: u64 = mix64(hi.wrapping_add(SPLITMIX_GAMMA) ^ seq.rotate_left(31));
    let mut out: String = String::with_capacity(32);
    for word in [hi, lo] {
        for shift in (0..16).rev() {
            let nibble: usize = ((word >> (shift * 4)) & 0x0F) as usize;
            out.push(HEX_DIGITS[nibble] as char);
        }
    }
    out
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
pub(crate) fn sha256(data: &[u8]) -> [u8; 32] {
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
/// - `&[u8; 64]` - Input message block.
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
/// - `[u8; 32]` - 256-bit HMAC tag.
pub(crate) fn hmac_sha256(key: &[u8], data: &[u8]) -> [u8; 32] {
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
pub(crate) fn constant_time_eq(a: &[u8], b: &[u8]) -> bool {
    if a.len() != b.len() {
        return false;
    }
    let mut diff: u8 = 0;
    for i in 0..a.len() {
        diff |= a[i] ^ b[i];
    }
    diff == 0
}

/// Signs `session_id` with `secret` into `"<session_id>.<hex_hmac>"`.
///
/// # Arguments
///
/// - `&str` - Raw session identifier.
/// - `&[u8]` - HMAC-SHA256 signing secret.
///
/// # Returns
///
/// - `String` - Signed cookie value.
pub fn sign_session_cookie(session_id: &str, secret: &[u8]) -> String {
    let mac: [u8; 32] = hmac_sha256(secret, session_id.as_bytes());
    let mut out: String = String::with_capacity(session_id.len() + 1 + 64);
    out.push_str(session_id);
    out.push('.');
    for byte in mac {
        out.push(HEX_DIGITS[(byte >> 4) as usize] as char);
        out.push(HEX_DIGITS[(byte & 0x0F) as usize] as char);
    }
    out
}

/// Verifies a signed session cookie value in constant time and returns the underlying session ID.
///
/// # Arguments
///
/// - `&str` - Raw cookie value (`"<id>.<hex_hmac>"`).
/// - `&[u8]` - HMAC-SHA256 signing secret.
///
/// # Returns
///
/// - `Option<String>` - Verified session identifier, or `None` if tampered or malformed.
pub fn verify_session_cookie(cookie_value: &str, secret: &[u8]) -> Option<String> {
    let (id, provided_sig): (&str, &str) = cookie_value.split_once('.')?;
    if id.is_empty() || provided_sig.len() != 64 {
        return None;
    }
    let expected: String = sign_session_cookie(id, secret);
    let expected_sig: &str = expected.split_once('.')?.1;
    if constant_time_eq(provided_sig.as_bytes(), expected_sig.as_bytes()) {
        Some(id.to_owned())
    } else {
        None
    }
}
