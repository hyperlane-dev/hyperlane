use super::*;

/// Cache key identifying a reusable upstream transport channel.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct PoolKey {
    /// Whether the target connection uses TLS.
    pub(crate) is_https: bool,
    /// Lowercased target host or IP literal.
    pub(crate) host: String,
    /// Target TCP port.
    pub(crate) port: u16,
    /// Optional proxy identity (`None` for direct connections).
    pub(crate) proxy: Option<ProxyKey>,
    /// Deterministic fingerprint of configured TLS root certificates (`0` for default WebPKI roots).
    pub(crate) root_cert_fingerprint: u64,
    /// Configured synchronous socket timeout in milliseconds (`0` for asynchronous pools).
    pub(crate) sync_timeout_ms: u64,
}

/// Hashable proxy identity component for [`PoolKey`].
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub(crate) struct ProxyKey {
    /// Proxy protocol family tag (`0 = Http`, `1 = Https`, `2 = Socks5`).
    pub(crate) proxy_type_tag: u8,
    /// Proxy hostname or IP.
    pub(crate) host: String,
    /// Proxy port.
    pub(crate) port: u16,
    /// Optional username.
    pub(crate) username: Option<String>,
    /// Optional password.
    pub(crate) password: Option<String>,
}

/// One idle asynchronous HTTP/1.1 connection stored in [`ASYNC_H1_POOL`].
pub(crate) struct PooledAsyncConn {
    /// The keep-alive async transport.
    pub(crate) stream: BoxAsyncReadWrite,
    /// Timestamp when this connection became idle.
    pub(crate) idle_since: Instant,
}

/// One idle synchronous HTTP/1.1 connection stored in [`SYNC_H1_POOL`].
pub(crate) struct PooledSyncConn {
    /// The keep-alive sync transport.
    pub(crate) stream: BoxReadWrite,
    /// Timestamp when this connection became idle.
    pub(crate) idle_since: Instant,
}
