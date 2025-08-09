use crate::*;

/// Implementation for `ServerConfig`.
impl ServerConfig {
    /// Creates a `ServerConfig` from a string slice.
    ///
    /// # Arguments
    ///
    /// - `config_str` - A string slice that holds the JSON representation of the server configuration.
    ///
    /// # Returns
    ///
    /// - `ServerConfigResult` - A `Result` containing the `ServerConfig` or an error.
    pub fn from_str(config_str: &str) -> ServerConfigResult {
        serde_json::from_str(config_str)
    }
}

/// Implementation of `Default` trait for `ServerConfig`.
impl Default for ServerConfig {
    /// Creates a default `ServerConfig` with predefined values.
    ///
    /// # Returns
    ///
    /// - `ServerConfig` - The default server configuration.
    fn default() -> Self {
        Self {
            host: DEFAULT_HOST.to_owned(),
            port: DEFAULT_WEB_PORT,
            ws_buffer: DEFAULT_BUFFER_SIZE,
            http_buffer: DEFAULT_BUFFER_SIZE,
            nodelay: DEFAULT_NODELAY,
            linger: DEFAULT_LINGER,
            ttl: DEFAULT_TTI,
        }
    }
}
