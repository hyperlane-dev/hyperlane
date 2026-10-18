/// One TLS version the handshake may offer.
pub(crate) type SupportedProtocolVersion = rustls::SupportedProtocolVersion;

/// The TLS configuration one listener hands to a rustls handshake.
pub(crate) type TlsServerConfig = rustls::ServerConfig;

/// The failure one rustls configuration step reports.
pub(crate) type TlsError = rustls::Error;
