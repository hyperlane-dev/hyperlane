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
    "GET /api/search?q=hyperlane&limit=10 HTTP/1.1\r\nHost: bench\r\nAccept: */*\n\r\n";

/// The request for the header route.
pub(crate) const REQUEST_HEADERS: &str =
    "GET /api/headers HTTP/1.1\r\nHost: bench\r\nx-bench-tag: alpha\r\nx-bench-extra: 1\r\n\r\n";

/// The request for the body route.
pub(crate) const REQUEST_ECHO: &str =
    "POST /api/echo HTTP/1.1\r\nHost: bench\r\nContent-Length: 11\r\n\r\nhello world";

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
