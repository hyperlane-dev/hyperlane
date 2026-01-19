use crate::*;

/// Implements the `Default` trait for `ServerConfigInner`.
///
/// This provides a default configuration for the server with predefined values.
impl Default for ServerConfigInner {
    /// Creates a default `ServerConfigInner`.
    ///
    /// # Returns
    ///
    /// - `Self` - A `ServerConfigInner` instance with default settings.
    #[inline(always)]
    fn default() -> Self {
        Self {
            host: DEFAULT_HOST.to_owned(),
            port: DEFAULT_WEB_PORT,
            request_config: RequestConfig::default(),
            nodelay: DEFAULT_NODELAY,
            ttl: DEFAULT_TTI,
        }
    }
}

/// Implements the `Default` trait for `ServerConfig`.
///
/// This wraps the default `ServerConfigInner` in an `Arc<RwLock>`.
impl Default for ServerConfig {
    /// Creates a default `ServerConfig`.
    ///
    /// # Returns
    ///
    /// - `Self` - A `ServerConfig` instance with default settings.
    #[inline(always)]
    fn default() -> Self {
        Self(arc_rwlock(ServerConfigInner::default()))
    }
}

/// Implements the `PartialEq` trait for `ServerConfig`.
///
/// This allows for comparing two `ServerConfig` instances for equality.
impl PartialEq for ServerConfig {
    /// Checks if two `ServerConfig` instances are equal.
    ///
    /// It first checks for pointer equality for performance. If the pointers are not equal,
    /// it compares the inner `ServerConfigInner` values.
    ///
    /// # Arguments
    ///
    /// - `&Self`- The other `ServerConfig` to compare against.
    ///
    /// # Returns
    ///
    /// - `bool` - Indicating whether the configurations are equal.
    #[inline(always)]
    fn eq(&self, other: &Self) -> bool {
        if Arc::ptr_eq(self.get_0(), other.get_0()) {
            return true;
        }
        if let (Ok(s), Ok(o)) = (self.get_0().try_read(), other.get_0().try_read()) {
            *s == *o
        } else {
            false
        }
    }
}

/// Implements the `Eq` trait for `ServerConfig`.
///
/// This indicates that `ServerConfig` has a total equality relation.
impl Eq for ServerConfig {}

/// Implementation block for `ServerConfig`.
impl ServerConfig {
    /// Creates a new `ServerConfig` with default values.
    ///
    /// # Returns
    ///
    /// - `Self` - A new `ServerConfig` instance.
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Acquires a read lock on the server configuration.
    ///
    /// # Returns
    ///
    /// - `ConfigReadGuard` - A `ConfigReadGuard` for the inner configuration.
    #[inline(always)]
    fn read(&self) -> ConfigReadGuard<'_> {
        self.get_0().try_read().unwrap()
    }

    /// Acquires a write lock on the server configuration.
    ///
    /// # Returns
    ///
    /// - `ConfigWriteGuard` - A `ConfigWriteGuard` for the inner configuration.
    #[inline(always)]
    fn write(&self) -> ConfigWriteGuard<'_> {
        self.get_0().try_write().unwrap()
    }

    /// Retrieves a clone of the inner server configuration.
    ///
    /// This function provides a snapshot of the current configuration by acquiring a read lock
    /// and cloning the inner `ServerConfigInner`.
    ///
    /// # Returns
    ///
    /// - `ServerConfigInner` - A `ServerConfigInner` instance containing the current server configuration.
    #[inline(always)]
    pub(crate) fn get_inner(&self) -> ServerConfigInner {
        self.read().clone()
    }

    /// Sets the host address for the server.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>`- The host address to set.
    ///
    /// # Returns
    ///
    /// - `&Self` - A reference to `Self` for method chaining.
    #[inline(always)]
    pub fn host<H>(&self, host: H) -> &Self
    where
        H: AsRef<str>,
    {
        self.write().set_host(host);
        self
    }

    /// Sets the port for the server.
    ///
    /// # Arguments
    ///
    /// - `u16`- The port number to set.
    ///
    /// # Returns
    ///
    /// - `&Self` - A reference to `Self` for method chaining.
    #[inline(always)]
    pub fn port(&self, port: u16) -> &Self {
        self.write().set_port(port);
        self
    }

    /// Sets the HTTP request config.
    ///
    /// # Arguments
    ///
    /// - `RequestConfig`- The HTTP request config to set.
    ///
    /// # Returns
    ///
    /// - `&Self` - A reference to `Self` for method chaining.
    #[inline(always)]
    pub fn request_config(&self, request_config: RequestConfig) -> &Self {
        self.write().set_request_config(request_config);
        self
    }

    /// Sets the `TCP_NODELAY` option.
    ///
    /// # Arguments
    ///
    /// - `bool`- The `bool` value for `TCP_NODELAY`.
    ///
    /// # Returns
    ///
    /// - `&Self` - A reference to `Self` for method chaining.
    #[inline(always)]
    pub fn nodelay(&self, nodelay: bool) -> &Self {
        self.write().set_nodelay(Some(nodelay));
        self
    }

    /// Enables the `TCP_NODELAY` option.
    ///
    /// # Returns
    ///
    /// - `&Self` - A reference to `Self` for method chaining.
    #[inline(always)]
    pub fn enable_nodelay(&self) -> &Self {
        self.nodelay(true)
    }

    /// Disables the `TCP_NODELAY` option.
    ///
    /// # Returns
    ///
    /// - `&Self` - A reference to `Self` for method chaining.
    #[inline(always)]
    pub fn disable_nodelay(&self) -> &Self {
        self.nodelay(false)
    }

    /// Sets the `IP_TTL` option.
    ///
    /// # Arguments
    ///
    /// - `u32`- The `u32` value for `IP_TTL`.
    ///
    /// # Returns
    ///
    /// - `&Self` - A reference to `Self` for method chaining.
    #[inline(always)]
    pub fn ttl(&self, ttl: u32) -> &Self {
        self.write().set_ttl(Some(ttl));
        self
    }

    /// Creates a `ServerConfig` from a JSON string.
    ///
    /// # Arguments
    ///
    /// - `&str`- The JSON string to parse.
    ///
    /// # Returns
    ///
    /// - `Result<ServerConfig, serde_json::Error>` - A `Result<ServerConfig, serde_json::Error>` which is a `Result` containing either the `ServerConfig` or a `serde_json::Error`.
    ///   Creates a `ServerConfig` from a JSON string.
    ///
    /// # Arguments
    ///
    /// - `config_str` - The JSON string to parse.
    ///
    /// # Returns
    ///
    /// - `Result<ServerConfig, serde_json::Error>` - A `Result<ServerConfig, serde_json::Error>` which is a `Result` containing either the `ServerConfig` or a `serde_json::Error`.
    pub fn from_json_str(config_str: &str) -> Result<ServerConfig, serde_json::Error> {
        serde_json::from_str(config_str).map(|config: ServerConfigInner| Self(arc_rwlock(config)))
    }
}
