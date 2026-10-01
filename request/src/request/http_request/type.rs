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

/// Boxed dynamic async stream.
pub(crate) type BoxAsyncReadWrite = Box<dyn AsyncReadWrite>;
/// Boxed dynamic sync stream.
pub(crate) type BoxReadWrite = Box<dyn ReadWrite>;

/// Result of an HTTP request send.
///
/// Always `Result<HttpResponse, RequestError>` — no boxed trait object,
/// no associated types. The response is a value type: clone it cheaply
/// (`HttpResponse: Clone`) or pass it around by reference.
pub type RequestResult = Result<HttpResponse, RequestError>;
