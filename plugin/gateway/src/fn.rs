use super::*;

/// Returns the current Unix timestamp in milliseconds.
///
/// # Returns
///
/// - `u64` - Milliseconds since Unix epoch.
pub(crate) fn gateway_now_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|dur: Duration| dur.as_millis() as u64)
        .unwrap_or(0)
}

/// Normalizes a route prefix so segment-boundary matching is canonical.
///
/// # Arguments
///
/// - `&str` - Raw route prefix.
///
/// # Returns
///
/// - `String` - Prefix beginning with `/` and without trailing `/` (except `"/"`).
pub(crate) fn normalize_prefix(prefix: &str) -> String {
    let trimmed: &str = prefix.trim();
    if trimmed.is_empty() || trimmed == "/" {
        return "/".to_owned();
    }
    let with_slash: String = if trimmed.starts_with('/') {
        trimmed.to_owned()
    } else {
        format!("/{trimmed}")
    };
    with_slash.trim_end_matches('/').to_owned()
}

/// Reports whether `path` matches `prefix` on an exact path-segment boundary.
///
/// For example, `"/api"` matches `"/api"` and `"/api/orders"`, but never `"/apiv2"`.
///
/// # Arguments
///
/// - `&str` - Incoming request path.
/// - `&str` - Normalized route prefix.
///
/// # Returns
///
/// - `bool` - `true` when `path` falls under `prefix`.
pub fn path_matches_prefix(path: &str, prefix: &str) -> bool {
    if prefix == "/" {
        return true;
    }
    let Some(rest) = path.strip_prefix(prefix) else {
        return false;
    };
    rest.is_empty() || rest.starts_with('/')
}

/// Rewrites an incoming path + optional query string onto `base_url`.
///
/// # Arguments
///
/// - `&str` - Normalized upstream base URL (no trailing slash).
/// - `&str` - Incoming request path.
/// - `&str` - Incoming query string (without leading `?`).
/// - `&str` - Normalized matched prefix.
/// - `bool` - Whether to strip `prefix` from the forwarded path.
///
/// # Returns
///
/// - `String` - Complete upstream URL ready for `http-request`.
pub fn build_upstream_url(
    base_url: &str,
    path: &str,
    query: &str,
    prefix: &str,
    strip_prefix: bool,
) -> String {
    let forward_path: &str = if strip_prefix && prefix != "/" {
        let stripped: &str = path.strip_prefix(prefix).unwrap_or(path);
        if stripped.is_empty() { "/" } else { stripped }
    } else if path.is_empty() {
        "/"
    } else {
        path
    };
    if query.is_empty() {
        format!("{base_url}{forward_path}")
    } else {
        format!("{base_url}{forward_path}?{query}")
    }
}

/// Reports whether `name` is an RFC 9110 hop-by-hop header that proxies must not forward.
///
/// # Arguments
///
/// - `&str` - Header name (any casing).
/// - `&HashSet<String>` - Additional lowercased header names nominated by `Connection`.
///
/// # Returns
///
/// - `bool` - `true` when the header must be stripped before forwarding.
pub fn is_hop_by_hop_header(name: &str, extra_connection_headers: &HashSet<String>) -> bool {
    let lower: String = name.to_ascii_lowercase();
    lower == CONNECTION
        || lower == KEEP_ALIVE
        || lower == PROXY_AUTHENTICATE
        || lower == PROXY_AUTHORIZATION
        || lower == HEADER_PROXY_CONNECTION
        || lower == TE
        || lower == HEADER_TRAILER
        || lower == TRANSFER_ENCODING
        || lower == UPGRADE
        || extra_connection_headers.contains(&lower)
}

/// Extracts lowercased header names listed in `Connection` header values.
///
/// # Arguments
///
/// - `&[String]` - Raw values of any `Connection` headers present.
///
/// # Returns
///
/// - `HashSet<String>` - Lowercased token names to strip alongside standard hop-by-hop headers.
pub(crate) fn extract_connection_header_tokens(values: &[String]) -> HashSet<String> {
    let mut tokens: HashSet<String> = HashSet::new();
    for val in values {
        for part in val.split(',') {
            let trimmed: &str = part.trim();
            if !trimmed.is_empty() {
                tokens.insert(trimmed.to_ascii_lowercase());
            }
        }
    }
    tokens
}
