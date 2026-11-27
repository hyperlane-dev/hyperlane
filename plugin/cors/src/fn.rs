use super::*;

/// Extracts the lowercased scheme+host authority or host portion of an `Origin` URL for suffix checks.
///
/// # Arguments
///
/// - `&str` - Raw incoming `Origin` header value.
///
/// # Returns
///
/// - `&str` - Host slice without scheme or port, or empty when malformed.
pub(crate) fn extract_origin_host(origin: &str) -> &str {
    let trimmed: &str = origin.trim();
    let without_scheme: &str = match trimmed.split_once("://") {
        Some((_, rest)) => rest,
        None => return "",
    };
    let authority: &str = without_scheme.split('/').next().unwrap_or(without_scheme);
    if authority.starts_with('[') {
        match authority.find(']') {
            Some(end) => &authority[..=end],
            None => "",
        }
    } else {
        authority.split(':').next().unwrap_or(authority)
    }
}

/// Appends `token` to a response's `Vary` header if not already present (ASCII case-insensitive).
///
/// # Arguments
///
/// - `&mut Response` - Target HTTP response.
/// - `&str` - Header token to add to `Vary`.
pub(crate) fn merge_vary_header(response: &mut Response, token: &str) {
    let existing: Option<String> = response.try_get_header_back(VARY);
    match existing {
        None => {
            response.set_header(VARY, token);
        }
        Some(current) => {
            let already: bool = current
                .split(',')
                .any(|part: &str| part.trim().eq_ignore_ascii_case(token));
            if !already {
                let mut merged: String =
                    String::with_capacity(current.len() + COMMA_SPACE.len() + token.len());
                merged.push_str(&current);
                merged.push_str(COMMA_SPACE);
                merged.push_str(token);
                response.set_header(VARY, merged);
            }
        }
    }
}

/// Reports whether a request header token list contains any non-wildcard explicit header name.
///
/// # Arguments
///
/// - `&[String]` - Configured allowed headers.
///
/// # Returns
///
/// - `bool` - `true` when headers should be mirrored from `Access-Control-Request-Headers`.
pub(crate) fn should_mirror_request_headers(headers: &[String]) -> bool {
    headers.is_empty()
        || headers
            .iter()
            .any(|item: &String| item.trim() == CORS_WILDCARD)
}
