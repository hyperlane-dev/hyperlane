use super::*;

/// Represents different upgrade types.
#[derive(Clone, Debug, Deserialize, Eq, PartialEq, Serialize)]
pub enum UpgradeType {
    /// Represents an upgrade to the WebSocket protocol.
    WebSocket,
    /// Represents an upgrade to HTTP/2 cleartext (h2c).
    H2c,
    /// Represents a TLS upgrade, which is rare and experimental. It includes the specific TLS protocol string.
    Tls(String),
    /// Represents other custom or unknown upgrade protocols, including the protocol string.
    Unknown(String),
}
