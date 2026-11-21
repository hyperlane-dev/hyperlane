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
/// ```rust
/// use http_request::{Proxy, ProxyType};
///
/// let https = Proxy::https("proxy.example.com", 7890);
/// assert_eq!(https.get_proxy_type(), &ProxyType::Https);
/// assert_eq!(https.get_port(), 7890);
///
/// let socks = Proxy::socks5("127.0.0.1", 1080).auth("example-user", "example-password");
/// assert_eq!(socks.get_proxy_type(), &ProxyType::Socks5);
/// assert_eq!(socks.get_username(), Some("example-user".to_string()));
/// ```
#[derive(Clone, Data, Debug, Eq, PartialEq)]
pub struct Proxy {
    /// Proxy protocol family.
    pub(crate) proxy_type: ProxyType,
    /// Proxy server hostname or IP.
    pub(crate) host: String,
    /// Proxy server port.
    #[get(type(copy))]
    pub(crate) port: u16,
    /// Optional username for proxy auth.
    #[get(type(clone))]
    pub(crate) username: Option<String>,
    /// Optional password for proxy auth.
    #[get(type(clone))]
    pub(crate) password: Option<String>,
}

/// Async tunnel stream wrapping another async stream, with a buffer of
/// pre-read bytes that are returned before delegating to the inner stream.
#[derive(Data)]
pub struct ProxyTunnelStream {
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) inner: BoxAsyncReadWrite,
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) pre_read_data: Vec<u8>,
}

/// Sync tunnel stream wrapping another sync stream, with a buffer of
/// pre-read bytes that are returned before delegating to the inner stream.
#[derive(Data)]
pub struct SyncProxyTunnelStream {
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) inner: BoxReadWrite,
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) pre_read_data: Vec<u8>,
}
