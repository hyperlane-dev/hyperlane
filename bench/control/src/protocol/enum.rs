/// The wire protocol the control server speaks.
///
/// The control is the machine-ceiling reference of the measurement matrix, so
/// it has to speak the same protocol as the row it references: a control that
/// speaks HTTP/1.1 cannot say what the machine allowed for HTTP/3.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum Protocol {
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
