use super::*;

/// Proxy configuration for an outgoing HTTP / WebSocket request.
///
/// `Proxy` is a value type with no `Arc<RwLock>` wrapping — exactly like
/// `hyperlane-core`'s plain fields. Construct one with the convenience
/// constructors below, then pass to `RequestBuilder::proxy(...)` or assign
/// directly to `RequestConfig::proxy`.
///
/// # Examples
///
/// ```ignore
/// use http_request::Proxy;
///
/// let p = Proxy::https("proxy.example.com", 7890);
/// let auth = Proxy::socks5("127.0.0.1", 1080).auth("user", "pass");
/// ```
#[derive(Clone, Data, Debug, Eq, PartialEq)]
pub struct Proxy {
    /// Proxy protocol family.
    pub proxy_type: ProxyType,
    /// Proxy server hostname or IP.
    pub host: String,
    /// Proxy server port.
    pub port: u16,
    /// Optional username for proxy auth.
    pub username: Option<String>,
    /// Optional password for proxy auth.
    pub password: Option<String>,
}

/// Async tunnel stream wrapping another async stream, with a buffer of
/// pre-read bytes that are returned before delegating to the inner stream.
#[derive(Data)]
pub struct ProxyTunnelStream {
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(super) inner: BoxAsyncReadWrite,
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(super) pre_read_data: Vec<u8>,
}

/// Sync tunnel stream wrapping another sync stream, with a buffer of
/// pre-read bytes that are returned before delegating to the inner stream.
#[derive(Data)]
pub struct SyncProxyTunnelStream {
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(super) inner: BoxReadWrite,
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(super) pre_read_data: Vec<u8>,
}
