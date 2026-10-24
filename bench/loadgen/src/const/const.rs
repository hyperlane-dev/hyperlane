/// The command line flag that names the server under test.
pub(crate) const ARG_ADDR: &str = "--addr";

/// The command line flag that sets the number of connections.
pub(crate) const ARG_CONNECTIONS: &str = "--connections";

/// The command line flag that sets the measured window, in seconds.
pub(crate) const ARG_DURATION: &str = "--duration";

/// The command line flag that sets the warm-up window, in seconds.
pub(crate) const ARG_WARMUP: &str = "--warmup";

/// The address the generator drives when none is given.
pub(crate) const DEFAULT_ADDR: &str = "127.0.0.1:8080";

/// The number of connections the generator opens when none is given.
pub(crate) const DEFAULT_CONNECTIONS: usize = 64;

/// The length of the measured window when none is given, in seconds.
pub(crate) const DEFAULT_DURATION_SECS: u64 = 10;

/// The length of the warm-up window when none is given, in seconds.
pub(crate) const DEFAULT_WARMUP_SECS: u64 = 3;

/// The bytes that end an HTTP response head.
pub(crate) const HEAD_TERMINATOR: &[u8] = b"\r\n\r\n";

/// The bytes that end one line of an HTTP response head.
pub(crate) const LINE_TERMINATOR: &[u8] = b"\r\n";

/// The name of the header that announces the body length.
pub(crate) const CONTENT_LENGTH_NAME: &[u8] = b"content-length";

/// The request for the static route.
pub(crate) const REQUEST_ROOT: &str =
    "GET / HTTP/1.1\r\nHost: bench\r\nAccept: */*\r\nUser-Agent: bench/1.0\r\n\r\n";

/// The request for the dynamic route.
pub(crate) const REQUEST_USER: &str =
    "GET /api/user/12345 HTTP/1.1\r\nHost: bench\r\nAccept: */*\r\n\r\n";

/// The request for the query route.
pub(crate) const REQUEST_SEARCH: &str =
    "GET /api/search?q=hyperlane&limit=10 HTTP/1.1\r\nHost: bench\r\nAccept: */*\r\n\r\n";

/// The request for the header route.
pub(crate) const REQUEST_HEADERS: &str =
    "GET /api/headers HTTP/1.1\r\nHost: bench\r\nx-bench-tag: alpha\r\nx-bench-extra: 1\r\n\r\n";

/// The body length the static route answers with.
pub(crate) const EXPECT_ROOT: usize = 25;

/// The body length the dynamic route answers with.
pub(crate) const EXPECT_USER: usize = 10;

/// The body length the query route answers with.
pub(crate) const EXPECT_SEARCH: usize = 11;

/// The body length the header route answers with.
pub(crate) const EXPECT_HEADERS: usize = 9;

/// The body length the body route answers with.
pub(crate) const EXPECT_ECHO: usize = 11;

/// The size of the buffer a connection reads into.
pub(crate) const READ_BUFFER_SIZE: usize = 16 * 1024;

/// The command line flag that names the wire protocol.
pub(crate) const ARG_PROTOCOL: &str = "--protocol";

/// The command line flag that names the connection mode.
pub(crate) const ARG_MODE: &str = "--mode";

/// The command line flag that names the certificate authority.
pub(crate) const ARG_CA: &str = "--ca";

/// The command line flag that caps the connection rate.
pub(crate) const ARG_RATE: &str = "--rate";

/// The connection rate the generator uses when none is given.
///
/// A short-connection row opens one connection per request, and the kernel
/// holds each closed connection in TIME_WAIT for twice the maximum segment
/// lifetime. The ephemeral port range is therefore the ceiling on the
/// connection rate, and an unpaced generator spends the range in seconds and
/// then fails with EADDRNOTAVAIL, which reads as a server defect and is not
/// one.
pub(crate) const DEFAULT_RATE_PER_SEC: u64 = 200;

/// The protocol name that selects HTTP/1 over a plain socket.
pub(crate) const PROTOCOL_HTTP1: &str = "h1";

/// The protocol name that selects HTTP/1 over TLS.
pub(crate) const PROTOCOL_HTTP1_TLS: &str = "h1-tls";

/// The protocol name that selects HTTP/2 over a plain socket.
pub(crate) const PROTOCOL_HTTP2: &str = "h2c";

/// The protocol name that selects HTTP/2 over TLS.
pub(crate) const PROTOCOL_HTTP2_TLS: &str = "h2-tls";

/// The protocol name that selects HTTP/3 over QUIC.
pub(crate) const PROTOCOL_HTTP3: &str = "h3";

/// The connection mode name that keeps the connection open.
pub(crate) const MODE_KEEPALIVE: &str = "keepalive";

/// The connection mode name that opens one connection per request.
pub(crate) const MODE_SHORT: &str = "short";

/// The identifier the ALPN protocol of HTTP/1.1 is spelled with.
pub(crate) const ALPN_HTTP1: &[u8] = b"http/1.1";

/// The identifier the ALPN protocol of HTTP/2 is spelled with.
pub(crate) const ALPN_HTTP2: &[u8] = b"h2";

/// The identifier the ALPN protocol of HTTP/3 is spelled with.
pub(crate) const ALPN_HTTP3: &[u8] = b"h3";

/// The value every request carries in the accept header.
pub(crate) const HEADER_ACCEPT: &str = "*/*";

/// The value every request carries in the user agent header.
pub(crate) const HEADER_USER_AGENT: &str = "bench/1.0";

/// The reason the QUIC connection is closed when the run ends.
pub(crate) const QUIC_CLOSE_REASON: &[u8] = b"done";

/// The method a request uses when its request line names none.
pub(crate) const METHOD_GET: &str = "GET";

/// The head of the request for the body route.
pub(crate) const REQUEST_ECHO_HEAD: &str =
    "POST /api/echo HTTP/1.1\r\nHost: bench\r\nContent-Length: 11\r\n\r\n";

/// The body the body route carries.
pub(crate) const BODY_ECHO: &str = "hello world";
