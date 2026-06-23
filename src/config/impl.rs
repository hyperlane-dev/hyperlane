use crate::*;

/// Provides a default implementation for ServerConfig.
impl Default for ServerConfig {
    /// Creates a new ServerConfig instance with default values.
    ///
    /// # Returns
    ///
    /// - `Self` - A new instance with default configuration.
    #[inline(always)]
    fn default() -> Self {
        Self::new()
    }
}

/// Implementation block for ServerConfig.
impl ServerConfig {
    /// Creates a ServerConfig from a JSON string.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The configuration.
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

    /// Creates a new `ServerConfig` with default values.
    ///
    /// HTTP/2 is enabled by default, HTTP/3 is disabled by default.
    #[inline(always)]
    pub fn new() -> Self {
        Self {
            address: Server::format_bind_address(DEFAULT_HOST, DEFAULT_WEB_PORT),
            nodelay: DEFAULT_NODELAY,
            ttl: DEFAULT_TTI,
            enable_http2: true,
            enable_http3: false,
            cert_path: None,
            key_path: None,
            http3_bind_address: None,
        }
    }

    /// Returns the HTTP/3 bind address, falling back to the TCP address host
    /// with the default HTTP/3 UDP port.
    #[inline(always)]
    pub fn get_effective_http3_bind_address(&self) -> String {
        if let Some(addr) = self.get_http3_bind_address() {
            return addr;
        }
        self.get_address()
            .rsplit_once(COLON)
            .map(|(host, _)| format!("{host}{COLON}{}", Protocol::get_h3_port()))
            .unwrap_or_else(|| self.get_address().clone())
    }
}
