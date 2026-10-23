pub(crate) const FIXTURE_DIR_NAME: &str = "tests/fixtures/tls";

pub(crate) const CA_PEM_NAME: &str = "ca.pem";
pub(crate) const LEAF_CHAIN_PEM_NAME: &str = "leaf-chain.pem";
pub(crate) const LEAF_KEY_NAME: &str = "leaf.key";
pub(crate) const MISMATCHED_LEAF_KEY_NAME: &str = "mismatched-leaf.key";
pub(crate) const MALFORMED_CERTIFICATE_PEM_NAME: &str = "malformed-certificate.pem";
pub(crate) const TRUNCATED_CERTIFICATE_PEM_NAME: &str = "truncated-certificate.pem";
pub(crate) const INVALID_PRIVATE_KEY_PEM_NAME: &str = "invalid-private-key.pem";
pub(crate) const MISSING_PEM_NAME: &str = "this-file-does-not-exist.pem";

pub(crate) const WIRE_HTTP1: &str = "http1";
pub(crate) const WIRE_HTTP2: &str = "http2";
pub(crate) const WIRE_HTTP3: &str = "http3";
pub(crate) const WIRE_UNKNOWN: &str = "http9";

pub(crate) const ALPN_HTTP1: &[u8] = b"http/1.1";
pub(crate) const ALPN_HTTP2: &[u8] = b"h2";
pub(crate) const ALPN_HTTP3: &[u8] = b"h3";

pub(crate) const LOOPBACK_HOST: &str = "127.0.0.1";
pub(crate) const TLS_SERVER_NAME: &str = "localhost";
pub(crate) const PLACEHOLDER_ADDRESS: &str = "127.0.0.1:0";

pub(crate) const BIND_ATTEMPTS: usize = 8;
pub(crate) const CONNECT_TIMEOUT_SECS: u64 = 15;
pub(crate) const REQUEST_TIMEOUT_SECS: u64 = 20;
pub(crate) const GATE_TIMEOUT_SECS: u64 = 20;
pub(crate) const SERVER_BOOT_MILLIS: u64 = 120;
pub(crate) const SHUTDOWN_PROBE_MILLIS: u64 = 250;
pub(crate) const SHUTDOWN_TIMEOUT_SECS: u64 = 15;
pub(crate) const FAST_STREAM_BUDGET_MILLIS: u64 = 1500;

pub(crate) const PROBE_PATH: &str = "/probe";
pub(crate) const PROBE_QUERY: &str = "name=hyperlane&kind=protocol";
pub(crate) const PROBE_QUERY_KEYS: [&str; 2] = ["name", "kind"];
pub(crate) const PROBE_QUERY_EXPECTED: &str = "name=hyperlane;kind=protocol";
pub(crate) const PROBE_METHOD: &str = "GET";
pub(crate) const PROBE_HEADER_KEY: &str = "x-probe";
pub(crate) const PROBE_HEADER_VALUE: &str = "probe-value";
pub(crate) const PROBE_REPEATED_HEADERS: [&str; 2] = [PROBE_HEADER_VALUE, PROBE_SECOND_VALUE];
pub(crate) const PROBE_HEADER_EXPECTED: &str = "probe-value;probe-second";
pub(crate) const PROBE_METHOD_HEADER: &str = "x-probe-method";
pub(crate) const PROBE_PATH_HEADER: &str = "x-probe-path";
pub(crate) const PROBE_QUERY_HEADER: &str = "x-probe-query";
pub(crate) const PROBE_HEADER_ECHO_HEADER: &str = "x-probe-header";
pub(crate) const PROBE_VERSION_HEADER: &str = "x-probe-version";

pub(crate) const ECHO_ROUTE_PATH: &str = "/echo";
pub(crate) const SLOW_ROUTE_PATH: &str = "/slow";
pub(crate) const SPLIT_ROUTE_PATH: &str = "/split";
pub(crate) const COOKIE_ROUTE_PATH: &str = "/cookie";
pub(crate) const LARGE_ROUTE_PATH: &str = "/large";
pub(crate) const BINARY_ROUTE_PATH: &str = "/binary";
pub(crate) const SSE_ROUTE_PATH: &str = "/sse";
pub(crate) const SSE_ROUTE_PATH_HTTP2: &str = "/sse-h2";
pub(crate) const SSE_ROUTE_PATH_HTTP3: &str = "/sse-h3";
pub(crate) const SLOW_ROUTE_PATH_HTTP3: &str = "/slow-h3";
pub(crate) const VERSION_ROUTE_PATH: &str = "/version";
pub(crate) const HOP_BY_HOP_ROUTE_PATH: &str = "/hop-by-hop";
pub(crate) const HOP_BY_HOP_CLEAN_ROUTE_PATH: &str = "/hop-by-hop-clean";
pub(crate) const SOCKET_METADATA_ROUTE_PATH: &str = "/socket-metadata";
pub(crate) const UNFRAMED_BODY_ROUTE_PATH: &str = "/unframed";

pub(crate) const SLOW_DONE_BODY: &str = "slow-done";
pub(crate) const COOKIE_SENT_BODY: &str = "cookie-sent";
pub(crate) const HOP_BY_HOP_SENT_BODY: &str = "hop-by-hop-sent";
pub(crate) const FIRST_COOKIE_NAME: &str = "dup";
pub(crate) const FIRST_COOKIE_VALUE: &str = "one";
pub(crate) const SECOND_COOKIE_NAME: &str = "dup2";
pub(crate) const SECOND_COOKIE_VALUE: &str = "two";
pub(crate) const PRESERVED_HEADER_KEY: &str = "x-end-to-end";
pub(crate) const PRESERVED_HEADER_VALUE: &str = "kept";
pub(crate) const INTERNAL_CONNECTION_HEADER: &str = "x-internal";
pub(crate) const INTERNAL_CONNECTION_VALUE: &str = "secret";

pub(crate) const HOSTILE_BYTES_LEN: usize = 1024;
pub(crate) const ABOVE_FLOW_CONTROL_WINDOW_LEN: usize = 1024 * 1024;

pub(crate) const SSE_FIRST_CHUNK: &str = "data:first\n\n";
pub(crate) const SSE_SECOND_CHUNK: &str = "data:second\n\n";
pub(crate) const SPLIT_HEAD_BODY: &str = "head-part";
pub(crate) const SPLIT_TAIL_BODY: &str = "tail-part";
pub(crate) const SPLIT_BODY_LEN: usize = 18;
pub(crate) const SPLIT_FRAGMENT_LEN: usize = 5;
pub(crate) const UNFRAMED_BODY: &str = "body-with-no-content-length";
pub(crate) const SOCKET_METADATA_SEPARATOR: char = '|';
pub(crate) const SERVER_CONFIG_LEGACY_ADDRESS: &str = "127.0.0.1:8080";

const PROBE_SECOND_VALUE: &str = "probe-second";
