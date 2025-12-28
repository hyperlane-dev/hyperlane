use crate::*;

/// Represents the inner, mutable server configuration.
///
/// This structure holds all the settings for the HTTP and WebSocket server,
/// including network parameters and buffer sizes. It is not intended to be used directly
/// by end-users, but rather through the `ServerConfig` wrapper.
#[derive(Clone, Data, CustomDebug, DisplayDebug, PartialEq, Eq, Deserialize, Serialize)]
pub(crate) struct ServerConfigInner {
    /// The host address the server will bind to.
    #[get(pub(crate))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) host: String,
    /// The port number the server will listen on.
    #[get(pub(crate))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) port: u16,
    /// The configuration for HTTP request.
    #[get(pub(crate))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) request_config: RequestConfig,
    /// The `TCP_NODELAY` option for sockets.
    #[get(pub(crate))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) nodelay: Option<bool>,
    /// The `SO_LINGER` option for sockets.
    #[get(pub(crate))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) linger: Option<Duration>,
    /// The `IP_TTL` option for sockets.
    #[get(pub(crate))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) ttl: Option<u32>,
}

/// Represents the thread-safe, shareable server configuration.
///
/// This structure wraps `ServerConfigInner` in an `Arc<RwLock<ServerConfigInner>>`
/// to allow for safe concurrent access and modification of the server settings.
#[derive(Clone, Getter, CustomDebug, DisplayDebug)]
pub struct ServerConfig(#[get(pub(super))] pub(super) SharedServerConfig);
