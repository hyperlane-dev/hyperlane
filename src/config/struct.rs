use crate::*;

/// Represents the server configuration.
///
/// This structure holds all the settings for the HTTP and WebSocket server,
/// including network parameters and buffer sizes.
#[derive(Clone, Data, CustomDebug, DisplayDebug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ServerConfig {
    /// The host address the server will bind to.
    pub(super) host: String,
    /// The port number the server will listen on.
    pub(super) port: usize,
    /// The buffer size for WebSocket connections.
    pub(super) ws_buffer: usize,
    /// The buffer size for HTTP connections.
    pub(super) http_buffer: usize,
    /// The TCP_NODELAY option for sockets.
    pub(super) nodelay: OptionBool,
    /// The SO_LINGER option for sockets.
    pub(super) linger: OptionDuration,
    /// The IP_TTL option for sockets.
    pub(super) ttl: OptionU32,
}
