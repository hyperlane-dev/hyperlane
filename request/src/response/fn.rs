use super::*;

/// Construct an empty response header map.
///
/// # Returns
///
/// - `HttpResponseHeaders` - A new, empty response header map.
pub fn new_response_headers() -> HttpResponseHeaders {
    hash_map_xx_hash3_64()
}
