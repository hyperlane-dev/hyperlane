use super::*;

/// A bidirectional in-memory byte pipe joining a handler to a protocol stream.
pub(crate) type DuplexStream = tokio::io::DuplexStream;

/// The response or request headers in their protocol representation.
pub(crate) type ProtocolHeaders = http::HeaderMap;

/// The bidirectional stream one HTTP/3 request runs over.
pub(crate) type RequestH3Stream = h3::server::RequestStream<h3_quinn::BidiStream<Bytes>, Bytes>;

/// The HTTP/2 connection half shared by every request stream of one connection.
pub(crate) type ConnectionH2<T> = h2::server::Connection<T, Bytes>;

/// The failure one HTTP/2 stream operation reports.
pub(crate) type H2Error = h2::Error;

/// The receive half of one HTTP/2 request stream.
pub(crate) type H2RecvStream = h2::RecvStream;

/// The stream half carrying the body of one HTTP/2 response.
pub(crate) type H2SendStream = h2::SendStream<Bytes>;

/// A request head in its protocol representation.
pub(crate) type ProtocolRequest<T> = http::Request<T>;

/// A response head in its protocol representation.
pub(crate) type ProtocolResponse<T> = http::Response<T>;

/// The failure one protocol head builder reports.
pub(crate) type ProtocolBuildError = http::Error;

/// The request method in its protocol representation.
pub(crate) type ProtocolMethod = http::Method;

/// The bare line feed byte.
pub(crate) const LF_U8: u8 = b'\n';

/// The carriage return byte.
pub(crate) const CR_U8: u8 = b'\r';

/// The line separator used when splitting an HTTP/1.1 head into lines.
pub(crate) const CRLF: &str = "\r\n";

/// The prefix every HTTP version string carries in a status line.
pub(crate) const HTTP_VERSION_PREFIX: &str = "HTTP/";

/// The line separator used when splitting a head into lines.
pub(crate) type SplitLines<'a> = Split<'a, &'static str>;

/// The separator between the tokens of a comma separated header value.
pub(crate) const COMMA_STR: &str = ",";

/// The line feed character used when trimming a head line.
pub(crate) const LF_CHAR: char = '\n';

/// The reason returned when a response head carries no version line.
pub(crate) const MISSING_RESPONSE_VERSION: &str = "response head has no version line";

/// The reason returned when a response head carries no status code.
pub(crate) const MISSING_RESPONSE_STATUS: &str = "response head has no status code";

/// The reason returned when a transport carries no socket identity.
pub(crate) const NO_ADDRESS: &str = "transport has no socket address";

/// The reason returned when a collected body disagrees with its declared length.
pub(crate) const CONTENT_LENGTH_MISMATCH: &str =
    "request body length does not match its content-length";
