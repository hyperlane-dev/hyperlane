/// Byte pattern for matching 'content-length' header in HTTP requests.
///
/// Used for case-sensitive matching of the content-length header.
pub(crate) const CONTENT_LENGTH_PATTERN: &[u8] = b"content-length:";

/// Byte pattern for matching 'transfer-encoding' header in HTTP requests.
///
/// Used for case-sensitive matching of the transfer-encoding header.
pub(crate) const TRANSFER_ENCODING_PATTERN: &[u8] = b"transfer-encoding:";

/// Byte pattern for matching 'chunked' value in HTTP headers.
///
/// Used for case-sensitive matching of the chunked transfer encoding value.
pub(crate) const CHUNKED_PATTERN: &[u8] = b"chunked";

/// Error message used when a redirect was signalled but no target was found.
pub(crate) const MISSING_REDIRECT_URL: &str = "Missing Redirect URL";

/// Error message used when a redirect target was already visited.
pub(crate) const REDIRECT_URL_DEAD_LOOP: &str = "Redirect URL Dead Loop";

/// Error message used when the redirect budget is exhausted.
pub(crate) const MAX_REDIRECT_TIMES_EXCEEDED: &str = "Max Redirect Times Exceeded";

/// Error message used when a proxy handshake fails.
pub(crate) const INTERNAL_SERVER_ERROR: &str = "Internal Server Error";

/// Error message used when an outgoing HTTP version token is malformed.
pub(crate) const INVALID_HTTP_VERSION: &str = "Invalid HTTP version";

/// Error message used when an outgoing HTTP method token is malformed.
pub(crate) const INVALID_HTTP_METHOD: &str = "Invalid HTTP method";

/// Error message used when proxy credentials or target host exceed protocol limits.
pub(crate) const PROXY_AUTH_ERROR: &str = "Invalid proxy authentication or target";

/// Prefix of an HTTP/1.1 proxy CONNECT status line.
pub(crate) const HTTP_CONNECT_PREFIX_1_1: &[u8] = b"HTTP/1.1 ";

/// Prefix of an HTTP/1.0 proxy CONNECT status line.
pub(crate) const HTTP_CONNECT_PREFIX_1_0: &[u8] = b"HTTP/1.0 ";

/// Effective fallback read buffer size when `RequestConfig::buffer_size` is zero.
pub(crate) const DEFAULT_EFFECTIVE_BUFFER_SIZE: usize = 4096;

/// Maximum response head size accepted from an HTTP CONNECT proxy.
pub(crate) const MAX_CONNECT_HEAD_BYTES: usize = 16 * 1024;

/// Default TCP port for cleartext HTTP URLs.
pub(crate) const HTTP_DEFAULT_PORT: u16 = 80;

/// Default TCP port for HTTPS URLs.
pub(crate) const HTTPS_DEFAULT_PORT: u16 = 443;

/// Sensitive authorization header name stripped on cross-origin or downgrade redirects.
pub(crate) const HEADER_AUTHORIZATION: &str = "authorization";

/// Sensitive proxy authorization header name stripped on cross-origin or downgrade redirects.
pub(crate) const HEADER_PROXY_AUTHORIZATION: &str = "proxy-authorization";

/// Sensitive cookie header name stripped on cross-origin or downgrade redirects.
pub(crate) const HEADER_COOKIE: &str = "cookie";

/// The `": "` separator written between a header name and its value.
pub(crate) const HEADER_VALUE_SEPARATOR: &[u8] = b": ";

/// The carriage-return / line-feed pair terminating an HTTP header line.
pub(crate) const HTTP_CARRIAGE_RETURN_LINE_FEED: &[u8] = b"\r\n";
