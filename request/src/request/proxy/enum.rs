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
