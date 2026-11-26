/// The QUIC application error code sent to peers when the server stops.
///
/// A close frame still needs an error code even for an orderly shutdown, and a
/// zero code is the one every implementation reports as an unspecified error
/// rather than as an application condition of its own.
pub(crate) const QUIC_CLOSE_CODE: u32 = 0;

/// The human readable reason carried by the shutdown close frame.
///
/// It is sent as the close frame's reason string so a peer logs why its
/// connection ended instead of an opaque code.
pub(crate) const QUIC_CLOSE_REASON: &[u8] = b"hyperlane server shutdown";

/// The value the accept loop publishes once it has stopped serving.
///
/// The flag is stored rather than merely signalled: a caller that reaches `wait`
/// after the loop already finished must observe the finished state, while a
/// caller that arrives first must block until the loop really stops.
pub(crate) const SERVER_STOPPED: bool = true;

/// The value the accept loop publishes while it is still serving.
pub(crate) const SERVER_RUNNING: bool = false;

/// The reason recorded when the HTTP/2 preface never arrives.
///
/// The handshake is bounded like every other read, so a client that connects
/// and stays silent is reported as an elapsed handshake rather than holding a
/// connection slot until the process exits.
pub(crate) const H2_HANDSHAKE_TIMEOUT: &str = "http2 handshake exceeded the read timeout";

/// The read timeout that means "no timeout at all".
///
/// The request configuration expresses "read without a deadline" as the
/// sentinel value `u64::MAX`. Mapping it to a `Duration` of the same number of
/// milliseconds would be a ~584,000 year deadline, so the sentinel is kept and
/// read as a real absence of a bound rather than as an enormous one.
pub(crate) const NO_TIMEOUT_MS: u64 = u64::MAX;

/// Backlog size passed to `listen(2)` for bound TCP listeners.
pub(crate) const TCP_LISTEN_BACKLOG: i32 = 1024;

/// Backoff delay in milliseconds after a non-peer TCP `accept()` error such as `EMFILE`.
pub(crate) const TCP_ACCEPT_BACKOFF_MS: u64 = 10;

/// Per-stream initial HTTP/2 flow-control window in bytes.
pub(crate) const H2_INITIAL_STREAM_WINDOW_SIZE: u32 = 1024 * 1024;

/// Connection-level initial HTTP/2 flow-control window in bytes.
pub(crate) const H2_INITIAL_CONNECTION_WINDOW_SIZE: u32 = 4 * 1024 * 1024;

/// Maximum concurrent HTTP/2 streams advertised per connection.
pub(crate) const H2_MAX_CONCURRENT_STREAMS: u32 = 1024;

/// Maximum concurrent reset streams tracked by the HTTP/2 server (mitigates Rapid Reset).
pub(crate) const H2_MAX_CONCURRENT_RESET_STREAMS: usize = 64;

/// Maximum pending accept reset streams before HTTP/2 tears down the connection.
pub(crate) const H2_MAX_PENDING_ACCEPT_RESET_STREAMS: usize = 64;

/// Minimum HTTP/2 `SETTINGS_MAX_HEADER_LIST_SIZE` floor so framework error hooks still fire.
pub(crate) const H2_MIN_MAX_HEADER_LIST_SIZE: u32 = 64 * 1024;

/// Upper cap for HTTP/2 `SETTINGS_MAX_HEADER_LIST_SIZE` even under relaxed configs.
pub(crate) const H2_MAX_HEADER_LIST_SIZE_CAP: u32 = 1024 * 1024;
