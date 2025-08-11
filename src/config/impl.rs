use crate::*;

/// Implements the `Default` trait for `ServerConfigInner`.
///
/// This provides a default configuration for the server with predefined values.
impl Default for ServerConfigInner {
    /// Creates a default `ServerConfigInner`.
    ///
    /// # Returns
    ///
    /// A `ServerConfigInner` instance with default settings.
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

/// Implements the `Default` trait for `ServerConfig`.
///
/// This wraps the default `ServerConfigInner` in an `Arc<RwLock>`.
impl Default for ServerConfig {
    /// Creates a default `ServerConfig`.
    ///
    /// # Returns
    ///
    /// A `ServerConfig` instance with default settings.
    fn default() -> Self {
        Self(arc_rwlock(ServerConfigInner::default()))
    }
}

/// Implements the `Eq` trait for `ServerConfig`.
impl Eq for ServerConfig {}

/// Implements the `PartialEq` trait for `ServerConfig`.
///
/// This allows for comparing two `ServerConfig` instances for equality.
impl PartialEq for ServerConfig {
    /// Checks if two `ServerConfig` instances are equal.
    ///
    /// It first checks for pointer equality for performance. If the pointers are not equal,
    /// it compares the inner `ServerConfigInner` values.
    ///
    /// # Parameters
    ///
    /// - `&Self`: The other `ServerConfig` to compare against.
    ///
    /// # Returns
    ///
    /// A `bool` indicating whether the configurations are equal.
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

/// Implementation block for `ServerConfig`.
impl ServerConfig {
    /// Creates a new `ServerConfig` with default values.
    ///
    /// # Returns
    ///
    /// A new `ServerConfig` instance.
    pub fn new() -> Self {
        Self::default()
    }

    /// Acquires a read lock on the server configuration.
    ///
    /// # Returns
    ///
    /// A `RwLockReadGuardServerConfigInner` for the inner configuration.
    async fn read(&self) -> RwLockReadGuardServerConfigInner {
        self.get_0().read().await
    }

    /// Acquires a write lock on the server configuration.
    ///
    /// # Returns
    ///
    /// A `RwLockWriteGuardServerConfigInner` for the inner configuration.
    async fn write(&self) -> RwLockWriteGuardServerConfigInner {
        self.get_0().write().await
    }

    /// Retrieves a clone of the inner server configuration.
    ///
    /// This function provides a snapshot of the current configuration by acquiring a read lock
    /// and cloning the inner `ServerConfigInner`.
    ///
    /// # Returns
    ///
    /// A `ServerConfigInner` instance containing the current server configuration.
    pub(crate) async fn get_inner(&self) -> ServerConfigInner {
        self.read().await.clone()
    }

    /// Sets the host address for the server.
    ///
    /// # Parameters
    ///
    /// - `H`: The host address to set.
    ///
    /// # Returns
    ///
    /// A reference to `Self` for method chaining.
    pub async fn host<H: ToString>(&self, host: H) -> &Self {
        self.write().await.set_host(host.to_string());
        self
    }

    /// Sets the port for the server.
    ///
    /// # Parameters
    ///
    /// - `usize`: The port number to set.
    ///
    /// # Returns
    ///
    /// A reference to `Self` for method chaining.
    pub async fn port(&self, port: usize) -> &Self {
        self.write().await.set_port(port);
        self
    }

    /// Sets the WebSocket buffer size.
    ///
    /// # Parameters
    ///
    /// - `usize`: The WebSocket buffer size to set.
    ///
    /// # Returns
    ///
    /// A reference to `Self` for method chaining.
    pub async fn ws_buffer(&self, ws_buffer: usize) -> &Self {
        self.write().await.set_ws_buffer(ws_buffer);
        self
    }

    /// Sets the HTTP buffer size.
    ///
    /// # Parameters
    ///
    /// - `usize`: The HTTP buffer size to set.
    ///
    /// # Returns
    ///
    /// A reference to `Self` for method chaining.
    pub async fn http_buffer(&self, http_buffer: usize) -> &Self {
        self.write().await.set_http_buffer(http_buffer);
        self
    }

    /// Sets the `TCP_NODELAY` option.
    ///
    /// # Parameters
    ///
    /// - `OptionBool`: The `Option<bool>` value for `TCP_NODELAY`.
    ///
    /// # Returns
    ///
    /// A reference to `Self` for method chaining.
    pub async fn nodelay(&self, nodelay: OptionBool) -> &Self {
        self.write().await.set_nodelay(nodelay);
        self
    }

    /// Enables the `TCP_NODELAY` option.
    ///
    /// # Returns
    ///
    /// A reference to `Self` for method chaining.
    pub async fn enable_nodelay(&self) -> &Self {
        self.nodelay(Some(true)).await
    }

    /// Disables the `TCP_NODELAY` option.
    ///
    /// # Returns
    ///
    /// A reference to `Self` for method chaining.
    pub async fn disable_nodelay(&self) -> &Self {
        self.nodelay(Some(false)).await
    }

    /// Sets the `SO_LINGER` option.
    ///
    /// # Parameters
    ///
    /// - `OptionDuration`: The `Option<Duration>` value for `SO_LINGER`.
    ///
    /// # Returns
    ///
    /// A reference to `Self` for method chaining.
    pub async fn linger(&self, linger: OptionDuration) -> &Self {
        self.write().await.set_linger(linger);
        self
    }

    /// Enables the `SO_LINGER` option with a specified duration.
    ///
    /// # Parameters
    ///
    /// - `Duration`: The `Duration` for `SO_LINGER`.
    ///
    /// # Returns
    ///
    /// A reference to `Self` for method chaining.
    pub async fn enable_linger(&self, linger: Duration) -> &Self {
        self.linger(Some(linger)).await
    }

    /// Disables the `SO_LINGER` option.
    ///
    /// # Returns
    ///
    /// A reference to `Self` for method chaining.
    pub async fn disable_linger(&self) -> &Self {
        self.linger(None).await
    }

    /// Sets the `IP_TTL` option.
    ///
    /// # Parameters
    ///
    /// - `OptionU32`: The `Option<u32>` value for `IP_TTL`.
    ///
    /// # Returns
    ///
    /// A reference to `Self` for method chaining.
    pub async fn ttl(&self, ttl: OptionU32) -> &Self {
        self.write().await.set_ttl(ttl);
        self
    }

    /// Enables the `IP_TTL` option with a specified value.
    ///
    /// # Parameters
    ///
    /// - `u32`: The `u32` TTL value.
    ///
    /// # Returns
    ///
    /// A reference to `Self` for method chaining.
    pub async fn enable_ttl(&self, ttl: u32) -> &Self {
        self.ttl(Some(ttl)).await
    }

    /// Disables the `IP_TTL` option.
    ///
    /// # Returns
    ///
    /// A reference to `Self` for method chaining.
    pub async fn disable_ttl(&self) -> &Self {
        self.ttl(None).await
    }

    /// Creates a `ServerConfig` from a JSON string.
    ///
    /// # Parameters
    ///
    /// - `&str`: The JSON string to parse.
    ///
    /// # Returns
    ///
    /// A `ServerConfigResult` which is a `Result` containing either the `ServerConfig` or a `serde_json::Error`.
    pub fn from_str(config_str: &str) -> ServerConfigResult {
        serde_json::from_str(config_str).map(|config: ServerConfigInner| Self(arc_rwlock(config)))
    }
}
