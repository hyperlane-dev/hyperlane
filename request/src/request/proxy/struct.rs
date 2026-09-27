use super::*;

/// Proxy protocol family.
///
/// Distinguishes the wire format used to reach the proxy server:
/// plain `HTTP CONNECT` for HTTP proxies, TLS-wrapped `HTTP CONNECT` for
/// HTTPS proxies, and SOCKS5 handshake for SOCKS5 proxies.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProxyType {
    /// HTTP proxy (plain TCP, CONNECT method).
    Http,
    /// HTTPS proxy (TLS-wrapped CONNECT method).
    Https,
    /// SOCKS5 proxy.
    Socks5,
}

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
#[derive(Clone, Debug, Eq, PartialEq)]
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

impl Proxy {
    /// Plain HTTP proxy.
    pub fn http<H: AsRef<str>>(host: H, port: u16) -> Self {
        Self::new(ProxyType::Http, host, port)
    }

    /// HTTPS proxy (TLS-wrapped).
    pub fn https<H: AsRef<str>>(host: H, port: u16) -> Self {
        Self::new(ProxyType::Https, host, port)
    }

    /// SOCKS5 proxy.
    pub fn socks5<H: AsRef<str>>(host: H, port: u16) -> Self {
        Self::new(ProxyType::Socks5, host, port)
    }

    /// Attach username / password to this proxy.
    pub fn auth<U: AsRef<str>, P: AsRef<str>>(mut self, username: U, password: P) -> Self {
        self.set_username(Some(username.as_ref().to_owned()));
        self.set_password(Some(password.as_ref().to_owned()));
        self
    }

    /// Sets the proxy username.
    ///
    /// # Arguments
    ///
    /// - `Option<String>` - The username, or None to clear.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to self for chaining.
    pub fn set_username(&mut self, value: Option<String>) -> &mut Self {
        self.username = value;
        self
    }

    /// Sets the proxy password.
    ///
    /// # Arguments
    ///
    /// - `Option<String>` - The password, or None to clear.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - A mutable reference to self for chaining.
    pub fn set_password(&mut self, value: Option<String>) -> &mut Self {
        self.password = value;
        self
    }

    fn new<H: AsRef<str>>(proxy_type: ProxyType, host: H, port: u16) -> Self {
        Self {
            proxy_type,
            host: host.as_ref().to_owned(),
            port,
            username: None,
            password: None,
        }
    }
}

/// Async tunnel stream wrapping another async stream, with a buffer of
/// pre-read bytes that are returned before delegating to the inner stream.
pub struct ProxyTunnelStream {
    pub(super) inner: BoxAsyncReadWrite,
    pub(super) pre_read_data: Vec<u8>,
}

/// Sync tunnel stream wrapping another sync stream, with a buffer of
/// pre-read bytes that are returned before delegating to the inner stream.
pub struct SyncProxyTunnelStream {
    pub(super) inner: BoxReadWrite,
    pub(super) pre_read_data: Vec<u8>,
}

impl ProxyTunnelStream {
    /// Returns the inner stream mutably.
    ///
    /// # Returns
    ///
    /// - `&mut BoxAsyncReadWrite` - The mutable inner stream.
    pub(crate) fn get_inner_mut(&mut self) -> &mut BoxAsyncReadWrite {
        &mut self.inner
    }

    /// Returns the buffered pre-read bytes.
    ///
    /// # Returns
    ///
    /// - `&[u8]` - The pre-read bytes not yet consumed.
    pub(crate) fn get_pre_read_data_ref(&self) -> &[u8] {
        &self.pre_read_data
    }

    /// Returns the buffered pre-read bytes mutably.
    ///
    /// # Returns
    ///
    /// - `&mut Vec<u8>` - The mutable pre-read buffer.
    pub(crate) fn get_pre_read_data_mut(&mut self) -> &mut Vec<u8> {
        &mut self.pre_read_data
    }
}

impl SyncProxyTunnelStream {
    /// Returns the inner stream mutably.
    ///
    /// # Returns
    ///
    /// - `&mut BoxReadWrite` - The mutable inner stream.
    pub(crate) fn get_inner_mut(&mut self) -> &mut BoxReadWrite {
        &mut self.inner
    }

    /// Returns the buffered pre-read bytes.
    ///
    /// # Returns
    ///
    /// - `&[u8]` - The pre-read bytes not yet consumed.
    pub(crate) fn get_pre_read_data_ref(&self) -> &[u8] {
        &self.pre_read_data
    }

    /// Returns the buffered pre-read bytes mutably.
    ///
    /// # Returns
    ///
    /// - `&mut Vec<u8>` - The mutable pre-read buffer.
    pub(crate) fn get_pre_read_data_mut(&mut self) -> &mut Vec<u8> {
        &mut self.pre_read_data
    }
}
