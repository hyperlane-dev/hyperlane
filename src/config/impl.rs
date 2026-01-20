use crate::*;

/// Implements the `Default` trait for `ServerConfigData`.
///
/// This provides a default configuration for the server with predefined values.
impl Default for ServerConfigData {
    /// Creates a default `ServerConfigData`.
    ///
    /// # Returns
    ///
    /// - `Self` - A `ServerConfigData` instance with default settings.
    #[inline(always)]
    fn default() -> Self {
        Self {
            host: DEFAULT_HOST.to_owned(),
            port: DEFAULT_WEB_PORT,
            nodelay: DEFAULT_NODELAY,
            ttl: DEFAULT_TTI,
        }
    }
}

/// Implements the `Default` trait for `ServerConfig`.
///
/// This wraps the default `ServerConfigData` in an `Arc<RwLock>`.
impl Default for ServerConfig {
    /// Creates a default `ServerConfig`.
    ///
    /// # Returns
    ///
    /// - `Self` - A `ServerConfig` instance with default settings.
    #[inline(always)]
    fn default() -> Self {
        Self(arc_rwlock(ServerConfigData::default()))
    }
}

/// Implements the `PartialEq` trait for `ServerConfig`.
///
/// This allows for comparing two `ServerConfig` instances for equality.
impl PartialEq for ServerConfig {
    /// Checks if two `ServerConfig` instances are equal.
    ///
    /// It first checks for pointer equality for performance. If the pointers are not equal,
    /// it compares the inner `ServerConfigData` values.
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

/// Implementation of `From` trait for `ServerConfig`.
impl From<ServerConfigData> for ServerConfig {
    /// Creates a `ServerConfig` from a `ServerConfigData`.
    ///
    /// # Arguments
    ///
    /// - `ServerConfigData` - The configuration.
    ///
    /// # Returns
    ///
    /// - `ServerConfig` - A `ServerConfig` instance.
    #[inline(always)]
    fn from(ctx: ServerConfigData) -> Self {
        Self(arc_rwlock(ctx))
    }
}

/// Implementation block for `ServerConfig`.
impl ServerConfig {
    /// Creates a new `ServerConfig` with default values.
    ///
    /// # Returns
    ///
    /// - `Self` - A new `ServerConfig` instance.
    pub async fn new() -> Self {
        Self::default()
    }

    /// Acquires a read lock on the server configuration.
    ///
    /// # Returns
    ///
    /// - `ConfigReadGuard` - A `ConfigReadGuard` for the inner configuration.
    async fn read(&self) -> ConfigReadGuard<'_> {
        self.get_0().read().await
    }

    /// Acquires a write lock on the server configuration.
    ///
    /// # Returns
    ///
    /// - `ConfigWriteGuard` - A `ConfigWriteGuard` for the inner configuration.
    async fn write(&self) -> ConfigWriteGuard<'_> {
        self.get_0().write().await
    }

    /// Retrieves a clone of the inner server configuration.
    ///
    /// This function provides a snapshot of the current configuration by acquiring a read lock
    /// and cloning the inner `ServerConfigData`.
    ///
    /// # Returns
    ///
    /// - `ServerConfigData` - A `ServerConfigData` instance containing the current server configuration.
    pub(crate) async fn get_data(&self) -> ServerConfigData {
        self.read().await.clone()
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
    pub async fn host<H>(&self, host: H) -> &Self
    where
        H: AsRef<str>,
    {
        self.write().await.set_host(host);
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
    pub async fn port(&self, port: u16) -> &Self {
        self.write().await.set_port(port);
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
    pub async fn nodelay(&self, nodelay: bool) -> &Self {
        self.write().await.set_nodelay(Some(nodelay));
        self
    }

    /// Enables the `TCP_NODELAY` option.
    ///
    /// # Returns
    ///
    /// - `&Self` - A reference to `Self` for method chaining.
    pub async fn enable_nodelay(&self) -> &Self {
        self.nodelay(true).await
    }

    /// Disables the `TCP_NODELAY` option.
    ///
    /// # Returns
    ///
    /// - `&Self` - A reference to `Self` for method chaining.
    pub async fn disable_nodelay(&self) -> &Self {
        self.nodelay(false).await
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
    pub async fn ttl(&self, ttl: u32) -> &Self {
        self.write().await.set_ttl(Some(ttl));
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
    /// - `AsRef<str>` - The configuration.
    ///
    /// # Returns
    ///
    /// - `Result<ServerConfig, serde_json::Error>` - A `Result<ServerConfig, serde_json::Error>` which is a `Result` containing either the `ServerConfig` or a `serde_json::Error`.
    pub fn from_json<C>(json: C) -> Result<ServerConfig, serde_json::Error>
    where
        C: AsRef<str>,
    {
        serde_json::from_str(json.as_ref()).map(|data: ServerConfigData| Self::from(data))
    }
}
