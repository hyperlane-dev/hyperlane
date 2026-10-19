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
