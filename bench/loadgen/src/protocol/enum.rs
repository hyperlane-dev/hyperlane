/// The wire protocol a connection speaks.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Protocol {
    /// HTTP/1.1 over a plain socket.
    Http1,
    /// HTTP/1.1 over a TLS session.
    Http1Tls,
    /// HTTP/2 with prior knowledge over a plain socket.
    H2c,
    /// HTTP/2 over a TLS session.
    H2Tls,
    /// HTTP/3 over QUIC.
    Http3,
}

/// The way a connection carries requests.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Mode {
    /// One connection carries many requests.
    KeepAlive,
    /// Every request carries its own connection.
    Short,
}
