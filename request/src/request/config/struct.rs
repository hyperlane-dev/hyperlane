use super::*;

/// Configuration for HTTP requests.
///
/// Contains settings like timeout, redirect handling, and proxy configuration.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct Config {
    /// Request timeout in milliseconds.
    pub(crate) timeout: u64,
    /// Parsed URL components for the request.
    pub(crate) url_obj: HttpUrlComponents,
    /// Whether to follow redirects automatically.
    pub(crate) redirect: bool,
    /// Maximum number of redirects to follow.
    pub(crate) max_redirect_times: usize,
    /// Current number of redirects followed.
    pub(crate) redirect_times: usize,
    /// HTTP version to use (1.1 or 2).
    pub(crate) http_version: HttpVersion,
    /// Buffer size for reading responses.
    pub(crate) buffer: usize,
    /// Whether to decode response bodies automatically.
    pub(crate) decode: bool,
    /// Optional proxy configuration.
    pub(crate) proxy: Option<ProxyConfig>,
}

/// Proxy server configuration.
///
/// Contains proxy type, host/port, and optional authentication.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct ProxyConfig {
    /// Type of proxy (HTTP/HTTPS/SOCKS5).
    pub(crate) proxy_type: ProxyType,
    /// Proxy server hostname or IP address.
    pub(crate) host: String,
    /// Proxy server port number.
    pub(crate) port: u16,
    /// Optional username for proxy authentication.
    pub(crate) username: Option<String>,
    /// Optional password for proxy authentication.
    pub(crate) password: Option<String>,
}

/// Supported proxy types.
#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) enum ProxyType {
    /// HTTP proxy.
    Http,
    /// HTTPS proxy.
    Https,
    /// SOCKS5 proxy.
    Socks5,
}
