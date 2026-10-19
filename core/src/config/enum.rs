use super::*;

/// The wire protocol a server connection speaks.
///
/// The variant is selected at runtime from the server configuration, never at
/// compile time: one binary serves every protocol it is configured for.
#[derive(Clone, Copy, Debug, Default, Deserialize, DisplayDebug, Eq, PartialEq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum ServerProtocol {
    /// HTTP/1.0 and HTTP/1.1 over a plain or TLS terminated byte stream.
    #[default]
    Http1,
    /// HTTP/2, either with prior knowledge over a plain socket or negotiated
    /// through `h2` ALPN over TLS.
    Http2,
    /// HTTP/3 over QUIC, which always terminates TLS inside its own handshake.
    Http3,
}
