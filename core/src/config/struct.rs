use super::*;

/// TLS material a server needs to terminate a connection.
///
/// The certificate and key are stored as filesystem paths and only read when a
/// listener is created, so a configuration can be deserialized, compared and
/// cloned without touching the disk.
///
/// The two path fields are part of the public surface: `TlsConfig::new` is the
/// documented constructor and applications read back what they configured.
#[derive(Clone, CustomDebug, Data, Deserialize, DisplayDebug, Eq, New, PartialEq, Serialize)]
pub struct TlsConfig {
    /// Path to the PEM encoded certificate chain presented to clients.
    pub certificate_path: String,
    /// Path to the PEM encoded private key matching the leaf certificate.
    pub private_key_path: String,
}

/// Represents the server configuration.
///
/// This structure holds all the settings for the HTTP and WebSocket server,
/// including network parameters and buffer sizes.
#[derive(Clone, CustomDebug, Data, Deserialize, DisplayDebug, Eq, New, PartialEq, Serialize)]
pub struct ServerConfig {
    /// The address the server will bind to.
    #[set(type(AsRef<str>))]
    pub address: String,
    /// The `TCP_NODELAY` option for sockets.
    #[get(type(copy))]
    pub nodelay: Option<bool>,
    /// The `IP_TTL` option for sockets.
    #[get(type(copy))]
    pub ttl: Option<u32>,
    /// The wire protocol the listener speaks, defaulting to HTTP/1.
    ///
    /// Skipped by `new` so the three argument constructor keeps its signature;
    /// a configuration built that way speaks HTTP/1 until a setter changes it.
    #[get(type(copy))]
    #[new(skip)]
    #[serde(default)]
    pub protocol: ServerProtocol,
    /// TLS material used to terminate connections, or `None` for plaintext.
    ///
    /// Skipped by `new` for the same reason as `protocol`; the setters take an
    /// `Option`, so `set_tls(None)` restores plaintext without a separate call.
    #[get(type(clone))]
    #[new(skip)]
    #[serde(default)]
    pub tls: Option<TlsConfig>,
}
