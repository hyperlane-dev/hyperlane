use super::*;

/// Implements the protocol selection the command line drives.
impl Protocol {
    /// Reads the protocol from its command line name.
    ///
    /// # Arguments
    ///
    /// - `&str` - The name the command line carries.
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

    /// The scheme a request of the protocol carries.
    ///
    /// # Returns
    ///
    /// - `&'static str` - The scheme.
    pub(crate) fn scheme(&self) -> &'static str {
        match self {
            Protocol::Http1 | Protocol::H2c => SCHEME_CLEARTEXT,
            Protocol::Http1Tls | Protocol::H2Tls | Protocol::Http3 => SCHEME_SECURE,
        }
    }
}

/// Implements the connection mode the command line drives.
impl Mode {
    /// Reads the connection mode from its command line name.
    ///
    /// # Arguments
    ///
    /// - `&str` - The name the command line carries.
    ///
    /// # Returns
    ///
    /// - `Option<Mode>` - The mode, when the name names one.
    pub(crate) fn from_name(name: &str) -> Option<Mode> {
        match name {
            MODE_KEEPALIVE => Some(Mode::KeepAlive),
            MODE_SHORT => Some(Mode::Short),
            _ => None,
        }
    }
}
