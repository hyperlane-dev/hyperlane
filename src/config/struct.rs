use crate::*;

/// Represents the server configuration.
///
/// This structure holds all the settings for the HTTP and WebSocket server,
/// including network parameters and buffer sizes.
#[derive(Clone, CustomDebug, Data, Deserialize, DisplayDebug, Eq, New, PartialEq, Serialize)]
pub struct ServerConfig {
    /// The address the server will bind to.
    #[set(type(AsRef<str>))]
    pub(super) address: String,
    /// The `TCP_NODELAY` option for sockets.
    pub(super) nodelay: Option<bool>,
    /// The `IP_TTL` option for sockets.
    pub(super) ttl: Option<u32>,
    /// The `SO_REUSEADDR` option for the listening socket.
    /// When `true`, allows immediate rebind to the same address during restart,
    /// avoiding TIME_WAIT collisions.
    pub(super) reuse_address: Option<bool>,
    /// The listen backlog (maximum number of pending connections) passed to
    /// the underlying `listen()` syscall.
    pub(super) listen_backlog: Option<i32>,
    /// Whether the listening socket is set to non-blocking mode before being
    /// handed off to the tokio reactor.
    pub(super) nonblocking: Option<bool>,
}
