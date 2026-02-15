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
        Self {
            address: Server::format_bind_address(DEFAULT_HOST, DEFAULT_WEB_PORT),
            nodelay: DEFAULT_NODELAY,
            ttl: DEFAULT_TTI,
        }
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
}
