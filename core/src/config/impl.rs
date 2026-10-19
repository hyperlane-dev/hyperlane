use super::*;

/// Implementation block for ServerProtocol.
impl ServerProtocol {
    /// Returns the ALPN protocol identifier used to negotiate this protocol.
    ///
    /// The value is sent verbatim in the TLS handshake, so it must be the
    /// wire identifier and not the human-readable version string.
    ///
    /// # Returns
    ///
    /// - `&'static [u8]` - The ALPN protocol identifier bytes.
    #[inline(always)]
    pub fn get_alpn_protocol(&self) -> &'static [u8] {
        match self {
            Self::Http1 => ALPN_HTTP1,
            Self::Http2 => ALPN_HTTP2,
            Self::Http3 => ALPN_HTTP3,
        }
    }

    /// Returns the HTTP version negotiated by this protocol.
    ///
    /// # Returns
    ///
    /// - `HttpVersion` - The protocol's HTTP version.
    #[inline(always)]
    pub fn get_version(&self) -> HttpVersion {
        match self {
            Self::Http1 => HttpVersion::Http1_1,
            Self::Http2 => HttpVersion::Http2,
            Self::Http3 => HttpVersion::Http3,
        }
    }

    /// Returns `true` when this protocol runs over QUIC rather than TCP.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` for QUIC transports, `false` for TCP transports.
    #[inline(always)]
    pub fn is_quic(&self) -> bool {
        matches!(self, Self::Http3)
    }
}

/// Provides a default implementation for ServerConfig.
impl Default for ServerConfig {
    /// Creates a new ServerConfig instance with default values.
    ///
    /// # Returns
    ///
    /// - `Self` - A new instance with default configuration.
    #[inline(always)]
    fn default() -> Self {
        Self {
            address: Server::format_bind_address(DEFAULT_HOST, DEFAULT_WEB_PORT),
            nodelay: DEFAULT_NODELAY,
            ttl: DEFAULT_TTI,
            protocol: ServerProtocol::default(),
            tls: None,
        }
    }
}

/// Implementation block for ServerConfig.
impl ServerConfig {
    /// Creates a ServerConfig from a JSON string.
    ///
    /// # Arguments
    ///
    /// - `C` - The configuration.
    ///
    /// # Returns
    ///
    /// - `Result<Self, serde_json::Error>` - A `Result` containing either the `ServerConfig` or a `serde_json::Error`.
    pub fn from_json<C>(json: C) -> Result<Self, serde_json::Error>
    where
        C: AsRef<str>,
    {
        serde_json::from_str(json.as_ref())
    }

    /// Returns the negotiated ALPN protocol identifier for this configuration.
    ///
    /// # Returns
    ///
    /// - `&'static [u8]` - The ALPN identifier of the configured protocol.
    #[inline(always)]
    pub fn get_alpn_protocol(&self) -> &'static [u8] {
        self.get_protocol().get_alpn_protocol()
    }
}
