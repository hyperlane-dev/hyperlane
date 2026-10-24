/// The environment variable that names the wire protocol.
pub(crate) const ENV_PROTOCOL: &str = "BENCH_PROTOCOL";

/// The environment variable that names the certificate chain.
pub(crate) const ENV_TLS_CERTIFICATE: &str = "BENCH_TLS_CERT";

/// The environment variable that names the private key.
pub(crate) const ENV_TLS_KEY: &str = "BENCH_TLS_KEY";

/// The protocol name that selects HTTP/1 over a plain socket.
pub(crate) const PROTOCOL_HTTP1: &str = "http1";

/// The protocol name that selects HTTP/1 over TLS.
pub(crate) const PROTOCOL_HTTP1_TLS: &str = "http1-tls";

/// The protocol name that selects HTTP/2 over a plain socket.
pub(crate) const PROTOCOL_HTTP2: &str = "http2";

/// The protocol name that selects HTTP/2 over TLS.
pub(crate) const PROTOCOL_HTTP2_TLS: &str = "http2-tls";

/// The protocol name that selects HTTP/3 over QUIC.
pub(crate) const PROTOCOL_HTTP3: &str = "http3";

/// The identifier the ALPN protocol of HTTP/1.1 is spelled with.
pub(crate) const ALPN_HTTP1: &[u8] = b"http/1.1";

/// The identifier the ALPN protocol of HTTP/2 is spelled with.
pub(crate) const ALPN_HTTP2: &[u8] = b"h2";

/// The identifier the ALPN protocol of HTTP/3 is spelled with.
pub(crate) const ALPN_HTTP3: &[u8] = b"h3";

/// The path the dynamic route lives at.
pub(crate) const PATH_USER: &str = "/api/user/";

/// The path the query route lives at.
pub(crate) const PATH_SEARCH: &str = "/api/search";

/// The path the header route lives at.
pub(crate) const PATH_HEADERS: &str = "/api/headers";

/// The path the body route lives at.
pub(crate) const PATH_ECHO: &str = "/api/echo";

/// The body the control answers the static route with.
pub(crate) const BODY_ROOT: &str = "hello hyperlane benchmark";

/// The prefix of the body the control answers the dynamic route with.
pub(crate) const BODY_USER_PREFIX: &str = "user ";

/// The prefix of the body the control answers the query route with.
pub(crate) const BODY_QUERY_PREFIX: &str = "q=";

/// The prefix of the body the control answers the header route with.
pub(crate) const BODY_TAG_PREFIX: &str = "tag=";

/// The name of the query parameter the query route reads.
pub(crate) const PARAM_QUERY: &str = "q";

/// The name of the request header the header route reads.
pub(crate) const HEADER_TAG: &str = "x-bench-tag";

/// The name of the header that announces the body length.
pub(crate) const HEADER_CONTENT_LENGTH: &str = "content-length";

/// The bytes that separate one query parameter from the next.
pub(crate) const QUERY_SEPARATOR: &str = "&";

/// The bytes that separate one query parameter from its value.
pub(crate) const QUERY_ASSIGN: &str = "=";

/// The bytes that separate the path from the query.
pub(crate) const PATH_QUERY_SEPARATOR: &str = "?";

/// The bytes that separate one path segment from the next.
pub(crate) const PATH_SEPARATOR: &str = "/";

/// The status the control answers with.
pub(crate) const STATUS_OK: u16 = 200;

/// The reason the HTTP/3 protocol could not start without TLS.
pub(crate) const ERROR_HTTP3_NEEDS_TLS: &str = "HTTP/3 requires TLS";

/// The status line every HTTP/1.1 answer starts with.
pub(crate) const RESPONSE_STATUS_LINE: &[u8] = b"HTTP/1.1 200 OK\r\n";

/// The header name and separator the content length answer starts with.
pub(crate) const CONTENT_LENGTH_PREFIX: &[u8] = b"content-length: ";

/// The bytes that end an HTTP head.
pub(crate) const HEAD_TERMINATOR: &[u8] = b"\r\n\r\n";
