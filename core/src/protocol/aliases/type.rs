use super::*;

/// The response or request headers in their protocol representation.
pub(crate) type ProtocolHeaders = http::HeaderMap;

/// The HTTP/2 connection half shared by every request stream of one connection.
pub(crate) type ConnectionH2<T> = h2::server::Connection<T, Bytes>;

/// The failure one HTTP/2 stream operation reports.
pub(crate) type H2Error = h2::Error;

/// The receive half of one HTTP/2 request stream.
pub(crate) type H2RecvStream = h2::RecvStream;

/// The stream half carrying the body of one HTTP/2 response.
pub(crate) type H2SendStream = h2::SendStream<Bytes>;

/// The response head sender for an incoming HTTP/2 request.
pub(crate) type H2SendResponse = SendResponse<Bytes>;

/// The HTTP/2 server handshake builder.
pub(crate) type H2ServerBuilder = Builder;

/// Standard library blocking TCP listener converted into a Tokio listener.
pub(crate) type StdTcpListener = net::TcpListener;

/// Standard library blocking UDP socket wrapped for exclusive multi-shard QUIC dispatch.
pub(crate) type StdUdpSocket = net::UdpSocket;

/// Established QUIC connection served by the native HTTP/3 engine.
pub(crate) type QuicConnection = Connection;

/// Incoming QUIC stream half for HTTP/3 requests and unidirectional streams.
pub(crate) type QuicRecvStream = RecvStream;

/// Outgoing QUIC stream half for HTTP/3 responses and control streams.
pub(crate) type QuicSendStream = SendStream;

/// Error returned while writing onto a QUIC stream.
pub(crate) type QuicWriteError = WriteError;

/// Standard library OS thread builder avoiding collision with `h2::server::Builder`.
pub(crate) type ThreadBuilder = thread::Builder;

/// Standard library OS thread join handle avoiding collision with `tokio::task::JoinHandle`.
pub(crate) type ThreadJoinHandle<T> = thread::JoinHandle<T>;

/// Tokio runtime builder avoiding collision with `h2::server::Builder`.
pub(crate) type RuntimeBuilder = runtime::Builder;

/// Standard library read-write lock for the startup inline hook registry.
pub(crate) type StdRwLock<T> = sync::RwLock<T>;

/// Atomic memory ordering alias avoiding collision with `std::cmp::Ordering`.
pub(crate) type AtomicOrdering = sync::atomic::Ordering;

/// A request head in its protocol representation.
pub(crate) type ProtocolRequest<T> = http::Request<T>;

/// A response head in its protocol representation.
pub(crate) type ProtocolResponse<T> = http::Response<T>;

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

/// The reason returned when a collected body disagrees with its declared length.
pub(crate) const CONTENT_LENGTH_MISMATCH: &str =
    "request body length does not match its content-length";
