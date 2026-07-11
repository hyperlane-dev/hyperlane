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
}
