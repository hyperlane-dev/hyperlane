use crate::*;

/// Represents the server configuration.
///
/// This structure holds all the settings for the HTTP and WebSocket server,
/// including network parameters and buffer sizes.
#[derive(Clone, CustomDebug, Data, Deserialize, DisplayDebug, Eq, PartialEq, Serialize)]
pub struct ServerConfig {
    /// The address the server will bind to.
    #[get(pub)]
    #[set(pub, type(AsRef<str>))]
    pub(super) address: String,
    /// The `TCP_NODELAY` option for sockets.
    #[get(pub, type(clone))]
    #[set(pub)]
    pub(super) nodelay: Option<bool>,
    /// The `IP_TTL` option for sockets.
    #[get(pub, type(clone))]
    #[set(pub)]
    pub(super) ttl: Option<u32>,
    /// Enable HTTP/2 support (over TLS when cert/key are configured, otherwise h2c cleartext).
    #[get(pub, type(copy))]
    #[set(pub)]
    pub(super) enable_http2: bool,
    /// Enable HTTP/3 support over QUIC (requires cert/key).
    #[get(pub, type(copy))]
    #[set(pub)]
    pub(super) enable_http3: bool,
    /// Path to the TLS certificate file (PEM format). Required for HTTP/2 over TLS and HTTP/3.
    #[get(pub, type(clone))]
    #[set(pub)]
    pub(super) cert_path: Option<String>,
    /// Path to the TLS private key file (PEM format). Required for HTTP/2 over TLS and HTTP/3.
    #[get(pub, type(clone))]
    #[set(pub)]
    pub(super) key_path: Option<String>,
    /// Optional UDP bind address for HTTP/3. Defaults to the same host as `address` on UDP port 443.
    #[get(pub, type(clone))]
    #[set(pub)]
    pub(super) http3_bind_address: Option<String>,
}
