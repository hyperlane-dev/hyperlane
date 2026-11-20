/// Byte delimiting a quoted HTTP field member.
pub(crate) const HTTP_QUOTED_FIELD_QUOTE: u8 = b'"';

/// Byte escaping the next character inside a quoted HTTP field member.
pub(crate) const HTTP_QUOTED_FIELD_ESCAPE: u8 = b'\\';

/// Pseudo-header prefix used by HTTP/2 and HTTP/3 header maps.
pub(crate) const PSEUDO_HEADER_PREFIX: &str = ":";

/// The character separating the path from the query in a request target.
pub(crate) const QUESTION_MARK: char = '?';

/// The character separating the userinfo from the host in an authority.
pub(crate) const AT_SIGN: char = '@';

/// The header naming the trailers section of a message.
pub(crate) const TRAILER_HEADER: &str = "trailer";

/// URI scheme used for cleartext HTTP.
pub(crate) const HTTP_SCHEME: &str = "http";

/// URI scheme used for TLS-protected HTTP.
pub(crate) const HTTPS_SCHEME: &str = "https";

/// Maximum size of a serialized response head accepted from the pipeline.
pub(crate) const RESPONSE_HEAD_MAX: usize = 16 * 1024;

/// The reason returned when a response head exceeds its bound.
pub(crate) const RESPONSE_HEAD_TOO_LONG: &str = "response head exceeds its bound";

/// The reason returned when the pipeline closed without writing a head.
pub(crate) const MISSING_RESPONSE_HEAD: &str = "response ended before a head was written";

/// The reason returned when a head announces a transfer coding the framing pump
/// cannot reproduce.
///
/// HTTP/2 and HTTP/3 do not carry a chunked body: the data frames are the
/// framing. Forwarding the chunk size lines as body bytes would hand the client
/// a body it must not have to decode, so the head is refused before any frame is
/// emitted and the stream can be reset with a real error.
pub(crate) const UNSUPPORTED_TRANSFER_ENCODING: &str =
    "response transfer-encoding is not supported on this protocol";

/// Static `text/html; charset=utf-8` media type for zero-allocation protocol header reuse.
pub(crate) const STATIC_CT_HTML_UTF8: &str = "text/html; charset=utf-8";

/// Static `text/plain` media type for zero-allocation protocol header reuse.
pub(crate) const STATIC_CT_PLAIN: &str = "text/plain";

/// Static `text/plain; charset=utf-8` media type for zero-allocation protocol header reuse.
pub(crate) const STATIC_CT_PLAIN_UTF8: &str = "text/plain; charset=utf-8";

/// Static `application/json` media type for zero-allocation protocol header reuse.
pub(crate) const STATIC_CT_JSON: &str = "application/json";

/// The reason returned when a status line carries no code the protocol allows.
pub(crate) const INVALID_RESPONSE_STATUS: &str = "response head has an unusable status code";

/// The reason returned when a header line is not `name: value`.
pub(crate) const MALFORMED_RESPONSE_HEADER: &str = "response head has a malformed header line";

/// The reason returned when the body is shorter than the announced length.
pub(crate) const TRUNCATED_RESPONSE_BODY: &str =
    "response ended before the announced content length arrived";

/// The reason returned when the body runs past the announced length.
pub(crate) const OVERFLOWING_RESPONSE_BODY: &str =
    "response carried more bytes than the announced content length";

/// The reason returned when a send stream is gone before capacity is granted.
pub(crate) const MISSING_SEND_CAPACITY: &str =
    "the send stream was closed before capacity was granted";

/// The smallest status code a protocol can carry.
pub(crate) const MIN_STATUS_CODE: usize = 100;

/// The largest status code a protocol can carry.
pub(crate) const MAX_STATUS_CODE: usize = 999;
