use super::*;

/// Parameters describing one asynchronous HTTP/2 client exchange target.
pub(crate) struct H2ExchangeTarget<'a> {
    /// Parsed URL components for the current hop.
    pub(crate) url_obj: &'a HttpUrlComponents,
    /// Validated HTTP method token.
    pub(crate) method_token: &'a str,
    /// Whether the request method is `HEAD`.
    pub(crate) is_head: bool,
    /// Target hostname or IP literal.
    pub(crate) host: &'a str,
    /// Effective TCP port.
    pub(crate) port: u16,
    /// Whether the target scheme is HTTPS.
    pub(crate) is_https: bool,
    /// Pool key identifying the multiplexed HTTP/2 connection.
    pub(crate) pool_key: PoolKey,
}
