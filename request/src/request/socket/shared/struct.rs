use super::*;

#[derive(Clone, Debug)]
pub(crate) struct SharedWebSocketBuilder;

#[derive(Clone, Debug)]
pub struct WebSocketError {
    pub(crate) kind: WebSocketErrorKind,
    pub(crate) message: String,
}

#[derive(Clone, Debug, PartialEq)]
pub enum WebSocketErrorKind {
    Connection,
    String,
    Timeout,
    InvalidUrl,
    Io,
    Tls,
}
