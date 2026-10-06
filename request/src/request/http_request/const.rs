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

/// Error message used when the request method is neither GET nor POST.
pub(crate) const METHOD_NOT_ALLOWED: &str = "Method Not Allowed";

/// Error message used when a redirect was signalled but no target was found.
pub(crate) const MISSING_REDIRECT_URL: &str = "Missing Redirect URL";

/// Error message used when redirect handling is disabled by configuration.
pub(crate) const REDIRECT_NOT_ENABLED: &str = "Redirect Not Enabled";

/// Error message used when a redirect target was already visited.
pub(crate) const REDIRECT_URL_DEAD_LOOP: &str = "Redirect URL Dead Loop";

/// Error message used when the redirect budget is exhausted.
pub(crate) const MAX_REDIRECT_TIMES_EXCEEDED: &str = "Max Redirect Times Exceeded";

/// Error message used when a proxy handshake fails.
pub(crate) const INTERNAL_SERVER_ERROR: &str = "Internal Server Error";

/// Prefix of a successful HTTP/1.1 proxy response status line.
pub(crate) const HTTP_1_1_OK_PREFIX: &str = "HTTP/1.1 200";

/// Prefix of a successful HTTP/1.0 proxy response status line.
pub(crate) const HTTP_1_0_OK_PREFIX: &str = "HTTP/1.0 200";

/// Separator marking the end of an HTTP header block.
pub(crate) const HEADER_TERMINATOR: &str = "\r\n\r\n";

/// HTTP request-line method token for POST.
pub(crate) const HTTP_METHOD_POST: &str = "POST";
