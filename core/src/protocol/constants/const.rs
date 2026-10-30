/// Pseudo-header prefix used by HTTP/2 and HTTP/3 header maps.
pub(crate) const PSEUDO_HEADER_PREFIX: &str = ":";

/// Size of the buffer used to read a protocol request body chunk by chunk.
pub(crate) const PROTOCOL_BODY_CHUNK: usize = 8 * 1024;

/// The character separating the path from the query in a request target.
pub(crate) const QUESTION_MARK: char = '?';

/// The character separating the userinfo from the host in an authority.
pub(crate) const AT_SIGN: char = '@';

/// The header naming the trailers section of a message.
pub(crate) const TRAILER_HEADER: &str = "trailer";

/// Buffer size of the duplex pipe joining the pipeline to a protocol stream.
pub(crate) const PROTOCOL_STREAM_CAPACITY: usize = 32 * 1024;

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
