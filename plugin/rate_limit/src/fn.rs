use super::*;

/// Computes the shard index for `key` using `XxHash3_64`.
///
/// # Arguments
///
/// - `&str` - Bucket key.
///
/// # Returns
///
/// - `usize` - Shard index in `0..RATE_LIMIT_SHARD_COUNT`.
pub(crate) fn rate_limit_shard_index(key: &str) -> usize {
    let mut hasher: XxHash3_64 = XxHash3_64::default();
    hasher.write(key.as_bytes());
    (hasher.finish() as usize) & (RATE_LIMIT_SHARD_COUNT - 1)
}

/// Converts nanoseconds upward to whole seconds (minimum `1` when `nanos > 0`).
///
/// # Arguments
///
/// - `u64` - Duration in nanoseconds.
///
/// # Returns
///
/// - `u64` - Ceiling whole seconds.
pub(crate) fn ceil_nanos_to_secs(nanos: u64) -> u64 {
    if nanos == 0 {
        return 0;
    }
    nanos.div_ceil(NANOS_PER_SEC)
}

/// Extracts the first client IP token from `X-Forwarded-For`, `X-Real-Ip`, or peer socket address.
///
/// # Arguments
///
/// - `&Stream` - Transport stream.
/// - `&Context` - Request context.
/// - `bool` - Whether forwarding headers (`X-Forwarded-For` / `X-Real-Ip`) are trusted.
///
/// # Returns
///
/// - `String` - Resolved client key.
pub(crate) fn resolve_client_ip(stream: &Stream, ctx: &Context, trust_proxy: bool) -> String {
    if trust_proxy {
        if let Some(xff) = ctx.get_request().try_get_header_back(X_FORWARDED_FOR)
            && let Some(first) = xff.split(',').next()
        {
            let trimmed: &str = first.trim();
            if !trimmed.is_empty() {
                return trimmed.to_owned();
            }
        }
        if let Some(real_ip) = ctx.get_request().try_get_header_back(X_REAL_IP) {
            let trimmed: &str = real_ip.trim();
            if !trimmed.is_empty() {
                return trimmed.to_owned();
            }
        }
    }
    stream
        .get_peer_addr()
        .ok()
        .map(|addr: SocketAddr| addr.ip().to_string())
        .unwrap_or_else(|| FALLBACK_CLIENT_KEY.to_owned())
}
