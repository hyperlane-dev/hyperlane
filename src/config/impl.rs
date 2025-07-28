use crate::*;

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
