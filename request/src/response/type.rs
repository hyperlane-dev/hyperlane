use super::*;

/// HTTP response headers — single-value `HashMap<String, String>`.
pub type HttpResponseHeaders = HashMapXxHash3_64<String, String>;

/// Raw response body bytes (alias for `Vec<u8>` to match `http-type`).
pub type ResponseBody = Vec<u8>;

/// Raw bytes of the full serialized HTTP response.
pub type ResponseData = Vec<u8>;

/// UTF-8 form of the full serialized HTTP response.
pub type ResponseDataString = String;

/// Construct an empty response header map.
pub fn new_response_headers() -> HttpResponseHeaders {
    hash_map_xx_hash3_64()
}
