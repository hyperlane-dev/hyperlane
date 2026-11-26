/// Hexadecimal uppercase digit table used by percent-encoding and multipart boundaries.
pub(crate) const HEX_UPPER: &[u8; 16] = b"0123456789ABCDEF";

/// Default `application/x-www-form-urlencoded` media type for form request bodies.
pub(crate) const FORM_URLENCODED_CONTENT_TYPE: &str = "application/x-www-form-urlencoded";

/// Default `multipart/form-data` boundary prefix.
pub(crate) const MULTIPART_BOUNDARY_PREFIX: &str = "hyperlane-boundary-";

/// `Authorization` header prefix for Bearer tokens.
pub(crate) const BEARER_PREFIX: &str = "Bearer ";

/// `Authorization` header prefix for HTTP Basic credentials.
pub(crate) const BASIC_PREFIX: &str = "Basic ";

/// HTTP/1.0 version prefix used to distinguish default keep-alive semantics.
pub(crate) const HTTP_1_0_PREFIX: &[u8] = b"HTTP/1.0 ";

/// Error message returned when an HTTP/2 TLS handshake fails to negotiate `h2` ALPN.
pub(crate) const H2_ALPN_MISMATCH: &str = "Server did not negotiate HTTP/2 via ALPN";

/// Maximum stack buffer length for zero-allocation lowercase header lookups.
pub(crate) const RESPONSE_HEADER_STACK_LIMIT: usize = 64;

/// Maximum exponential backoff delay between automatic retries (milliseconds).
pub(crate) const MAX_RETRY_BACKOFF_MS: u64 = 1000;

/// Base exponential backoff delay for the first retry attempt (milliseconds).
pub(crate) const BASE_RETRY_BACKOFF_MS: u64 = 50;
