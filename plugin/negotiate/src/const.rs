/// The body length below which a response is answered without a content coding.
pub(crate) const DEFAULT_MIN_LENGTH: usize = 256;

/// The prefix marking a weak HTTP entity-tag per RFC 9110.
pub(crate) const WEAK_ETAG_PREFIX: &str = "W/";

/// Lowercase hexadecimal digits used for stack-allocated strong ETag formatting.
pub(crate) const HEX_LOWER: &[u8; 16] = b"0123456789abcdef";

/// The parameter delimiter inside a `Content-Type` header value.
pub(crate) const MEDIA_TYPE_PARAM_DELIMITER: char = ';';

/// The `Cache-Control` directive forbidding response body transformation.
pub(crate) const CACHE_CONTROL_NO_TRANSFORM: &str = "no-transform";

/// The wildcard token in a `Vary` header field.
pub(crate) const VARY_WILDCARD: &str = "*";

/// Error reported when `CompressMiddleware::init` is called more than once.
pub(crate) const COMPRESS_CONFIG_ALREADY_SET: &str = "the compress configuration was already set";
