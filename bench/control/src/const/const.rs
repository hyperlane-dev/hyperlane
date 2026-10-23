/// The bytes that start a request for the static route.
pub(crate) const PREFIX_ROOT: &[u8] = b"GET / ";

/// The bytes that start a request for the dynamic route.
pub(crate) const PREFIX_USER: &[u8] = b"GET /api/user/";

/// The bytes that start a request for the query route.
pub(crate) const PREFIX_SEARCH: &[u8] = b"GET /api/search";

/// The bytes that start a request for the header route.
pub(crate) const PREFIX_HEADERS: &[u8] = b"GET /api/headers";

/// The answer for the static route.
pub(crate) const RESPONSE_ROOT: &[u8] = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: 25\r\nConnection: keep-alive\r\n\r\nhello hyperlane benchmark";

/// The answer for the dynamic route.
pub(crate) const RESPONSE_USER: &[u8] = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: 10\r\nConnection: keep-alive\r\n\r\nuser 12345";

/// The answer for the query route.
pub(crate) const RESPONSE_SEARCH: &[u8] = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: 11\r\nConnection: keep-alive\r\n\r\nq=hyperlane";

/// The answer for the header route.
pub(crate) const RESPONSE_HEADERS: &[u8] = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: 9\r\nConnection: keep-alive\r\n\r\ntag=alpha";

/// The answer for the body route.
pub(crate) const RESPONSE_ECHO: &[u8] = b"HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: 11\r\nConnection: keep-alive\r\n\r\nhello world";

/// The environment variable that names the address to bind.
pub(crate) const ENV_ADDRESS: &str = "BENCH_ADDR";

/// The address the server binds when the environment names none.
pub(crate) const DEFAULT_ADDRESS: &str = "0.0.0.0:8080";

/// The size of the buffer a connection reads into.
pub(crate) const READ_BUFFER_SIZE: usize = 16 * 1024;
