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
    let mut url: String =
        String::with_capacity(base_url.len() + forward_path.len() + query.len() + 1);
    url.push_str(base_url);
    url.push_str(forward_path);
    if !query.is_empty() {
        url.push('?');
        url.push_str(query);
    }
    url
}

/// Reports whether `name` is an RFC 9110 hop-by-hop header that proxies must not forward.
///
/// Uses zero-allocation ASCII case-insensitive comparison for standard hop-by-hop headers
/// and only lowercases when `extra_connection_headers` is non-empty.
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
    if name.eq_ignore_ascii_case(CONNECTION)
        || name.eq_ignore_ascii_case(KEEP_ALIVE)
        || name.eq_ignore_ascii_case(PROXY_AUTHENTICATE)
        || name.eq_ignore_ascii_case(PROXY_AUTHORIZATION)
        || name.eq_ignore_ascii_case(HEADER_PROXY_CONNECTION)
        || name.eq_ignore_ascii_case(TE)
        || name.eq_ignore_ascii_case(HEADER_TRAILER)
        || name.eq_ignore_ascii_case(TRANSFER_ENCODING)
        || name.eq_ignore_ascii_case(UPGRADE)
    {
        return true;
    }
    if extra_connection_headers.is_empty() {
        return false;
    }
    extra_connection_headers.contains(&name.to_ascii_lowercase())
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
