use super::*;

/// Type aliases for the request side of `http-request`.
/// Mirror the names already used inside `http-type` (`RequestHeaders`,
/// `RequestBody`) so users see consistent vocabulary whether they are
/// reading a server-side `Request` or building a client-side `HttpRequest`.
/// Key type for the request headers map.
pub type RequestHeadersKey = String;
/// Single-value type for a request header.
pub type RequestHeadersValue = String;
/// Map of HTTP request headers (single value per key).
pub type RequestHeaders = HashMapXxHash3_64<RequestHeadersKey, RequestHeadersValue>;
