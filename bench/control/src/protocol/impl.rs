use super::*;

/// Implements the protocol selection the environment drives.
impl Protocol {
    /// Reads the protocol from its environment name.
    ///
    /// # Arguments
    ///
    /// - `&str` - The name the environment carries.
    ///
    /// # Returns
    ///
    /// - `Option<Protocol>` - The protocol, when the name names one.
    pub(crate) fn from_name(name: &str) -> Option<Protocol> {
        match name {
            PROTOCOL_HTTP1 => Some(Protocol::Http1),
            PROTOCOL_HTTP1_TLS => Some(Protocol::Http1Tls),
            PROTOCOL_HTTP2 => Some(Protocol::H2c),
            PROTOCOL_HTTP2_TLS => Some(Protocol::H2Tls),
            PROTOCOL_HTTP3 => Some(Protocol::Http3),
            _ => None,
        }
    }

    /// Reports whether the protocol is carried over TLS.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the protocol terminates TLS.
    pub(crate) fn uses_tls(&self) -> bool {
        match self {
            Protocol::Http1Tls | Protocol::H2Tls | Protocol::Http3 => true,
            Protocol::Http1 | Protocol::H2c => false,
        }
    }

    /// The ALPN identifier the protocol advertises.
    ///
    /// # Returns
    ///
    /// - `&'static [u8]` - The identifier.
    pub(crate) fn alpn(&self) -> &'static [u8] {
        match self {
            Protocol::Http1 | Protocol::Http1Tls => ALPN_HTTP1,
            Protocol::H2c | Protocol::H2Tls => ALPN_HTTP2,
            Protocol::Http3 => ALPN_HTTP3,
        }
    }
}
