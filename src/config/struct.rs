use crate::*;

/// Represents the inner, mutable server configuration.
///
/// This structure holds all the settings for the HTTP and WebSocket server,
/// including network parameters and buffer sizes. It is not intended to be used directly
/// by end-users, but rather through the `ServerConfig` wrapper.
#[derive(Clone, CustomDebug, Data, Deserialize, DisplayDebug, Eq, PartialEq, Serialize)]
pub(crate) struct ServerConfigData {
    /// The host address the server will bind to.
    #[get(pub(crate))]
    #[get_mut(pub(super))]
    #[set(pub(super), type(AsRef<str>))]
    pub(super) host: String,
    /// The port number the server will listen on.
    #[get(pub(crate), type(copy))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) port: u16,
    /// The `TCP_NODELAY` option for sockets.
    #[get(pub(crate))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) nodelay: Option<bool>,
    /// The `IP_TTL` option for sockets.
    #[get(pub(crate))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) ttl: Option<u32>,
}

/// Represents the thread-safe, shareable server configuration.
///
/// This structure wraps `ServerConfigData` in an `Arc<RwLock<ServerConfigData>>`
/// to allow for safe concurrent access and modification of the server settings.
#[derive(Clone, CustomDebug, DisplayDebug, Getter)]
pub struct ServerConfig(#[get(pub(super))] pub(super) ArcRwLock<ServerConfigData>);
