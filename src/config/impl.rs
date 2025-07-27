use crate::*;

/// Provides default values for `ServerConfig`.
impl Default for ServerConfig {
    /// This implementation sets up the server with sensible defaults for common use cases.
    ///
    /// # Returns
    ///
    /// A `ServerConfig` instance with default values.
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
