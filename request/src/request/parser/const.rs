/// The URL query separator.
pub(crate) const URL_QUERY: char = '?';

/// The URL authority and path separator.
pub(crate) const URL_SLASH: char = '/';

/// The last ASCII decimal digit.
pub(crate) const ASCII_NINE: u8 = b'9';

/// The first ASCII decimal digit.
pub(crate) const ASCII_ZERO: u8 = b'0';

/// The supported cleartext HTTP URL scheme.
pub(crate) const HTTP_SCHEME: &str = "http";

/// The URL fragment separator.
pub(crate) const URL_FRAGMENT: char = '#';

/// An ambiguous special-scheme authority separator.
pub(crate) const URL_BACKSLASH: char = '\\';

/// The required HTTP URL scheme and authority delimiter.
pub(crate) const SCHEME_SEPARATOR: &str = "://";

/// The prefix identifying the response status line.
pub(crate) const HTTP_STATUS_PREFIX: &[u8] = b"HTTP/";

/// The status code offset after the HTTP-version token and its space.
pub(crate) const HTTP_STATUS_CODE_OFFSET: usize = 9;

/// The non-alphanumeric bytes allowed in an HTTP field name.
pub(crate) const FIELD_NAME_PUNCTUATION: &[u8] = b"!#$%&'*+-.^_`|~";

/// The error for a malformed outgoing header.
pub(crate) const INVALID_REQUEST_HEADER: &str = "Invalid HTTP request header";

/// The maximum accepted size of one buffered response chunk.
pub(crate) const MAX_RESPONSE_CHUNK_SIZE: usize = 64 * 1024 * 1024;

/// The maximum accepted size of an HTTP response header block.
pub(crate) const MAX_RESPONSE_HEAD_BYTES: usize = 64 * 1024;

/// The maximum accepted total size of a buffered HTTP response body.
pub(crate) const MAX_RESPONSE_BODY_BYTES: usize = 64 * 1024 * 1024;

/// The maximum number of consecutive 1xx interim responses accepted before failing.
pub(crate) const MAX_INTERIM_RESPONSES: usize = 16;

/// The error for invalid or conflicting response framing.
pub(crate) const INVALID_RESPONSE_FRAMING: &str = "Invalid or ambiguous HTTP response framing";

/// The error for an HTTP response that exceeds the configured head or body cap.
pub(crate) const RESPONSE_TOO_LARGE: &str = "HTTP response exceeds maximum allowed size";

/// The error for an excessive sequence of 1xx interim responses.
pub(crate) const TOO_MANY_INTERIM_RESPONSES: &str = "Too many interim HTTP responses";

/// The separator before a chunk extension.
pub(crate) const CHUNK_EXTENSION_SEPARATOR: u8 = b';';
