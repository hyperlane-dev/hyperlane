use super::*;

/// Returns the current Unix epoch timestamp in milliseconds.
///
/// # Returns
///
/// - `u64` - Milliseconds since Unix epoch.
pub(crate) fn cache_now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|dur: Duration| dur.as_millis() as u64)
        .unwrap_or(0)
}

/// Computes the shard index for `key` using `XxHash3_64`.
///
/// # Arguments
///
/// - `&str` - Cache key.
///
/// # Returns
///
/// - `usize` - Shard index in `0..CACHE_SHARD_COUNT`.
pub(crate) fn cache_shard_index(key: &str) -> usize {
    let mut hasher: XxHash3_64 = XxHash3_64::default();
    hasher.write(key.as_bytes());
    (hasher.finish() as usize) & (CACHE_SHARD_COUNT - 1)
}

/// Computes the approximate heap footprint in bytes of a [`CachedValue`].
///
/// # Arguments
///
/// - `&str` - Cache key.
/// - `&CachedValue` - Cached response payload and headers.
///
/// # Returns
///
/// - `usize` - Estimated memory footprint in bytes.
pub(crate) fn estimate_entry_bytes(key: &str, value: &CachedValue) -> usize {
    let header_bytes: usize = value
        .get_headers()
        .iter()
        .map(|(k, v): &(String, String)| k.len() + v.len())
        .sum();
    key.len() + header_bytes + value.get_body().len() + 64
}

/// Parses `Cache-Control` directives to check whether caching is prohibited and extract optional `max-age` / `s-maxage`.
///
/// # Arguments
///
/// - `Option<&str>` - Raw `Cache-Control` header value.
///
/// # Returns
///
/// - `(bool, Option<u64>)` - `(cacheable, ttl_override_ms)`.
pub(crate) fn parse_cache_control(header: Option<&str>) -> (bool, Option<u64>) {
    let Some(raw) = header else {
        return (true, None);
    };
    let mut s_maxage_secs: Option<u64> = None;
    let mut max_age_secs: Option<u64> = None;
    for directive in raw.split(',') {
        let trimmed: &str = directive.trim();
        if trimmed.eq_ignore_ascii_case(CC_NO_STORE)
            || trimmed.eq_ignore_ascii_case(CC_NO_CACHE)
            || trimmed.eq_ignore_ascii_case(CC_PRIVATE)
        {
            return (false, None);
        }
        let lower: String = trimmed.to_ascii_lowercase();
        if let Some(secs_str) = lower.strip_prefix(CC_S_MAXAGE_PREFIX)
            && let Ok(secs) = secs_str.trim().parse::<u64>()
        {
            s_maxage_secs = Some(secs);
        } else if let Some(secs_str) = lower.strip_prefix(CC_MAX_AGE_PREFIX)
            && let Ok(secs) = secs_str.trim().parse::<u64>()
        {
            max_age_secs = Some(secs);
        }
    }
    let chosen: Option<u64> = s_maxage_secs.or(max_age_secs);
    if chosen == Some(0) {
        return (false, None);
    }
    (true, chosen.map(|secs: u64| secs.saturating_mul(1_000)))
}

/// Builds a canonical HTTP cache key from method, path, and sorted query parameters.
///
/// # Arguments
///
/// - `&Context` - Incoming HTTP request context.
///
/// # Returns
///
/// - `String` - Deterministic cache key.
pub(crate) fn build_http_cache_key(ctx: &Context) -> String {
    let path: &str = ctx.get_request().get_path();
    let querys: &RequestQuerys = ctx.get_request().get_querys();
    if querys.is_empty() {
        return path.to_owned();
    }
    let mut pairs: Vec<(&String, &VecDeque<String>)> = querys.iter().collect();
    pairs
        .sort_by(|a: &(&String, &VecDeque<String>), b: &(&String, &VecDeque<String>)| a.0.cmp(b.0));
    let mut key: String = String::with_capacity(path.len() + 32);
    key.push_str(path);
    key.push('?');
    let mut first: bool = true;
    for (k, vals) in pairs {
        for v in vals {
            if !first {
                key.push('&');
            }
            first = false;
            key.push_str(k);
            key.push('=');
            key.push_str(v);
        }
    }
    key
}
