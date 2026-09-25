use super::*;

#[derive(Clone, Debug, Default)]
pub struct WebSocketBuilder {
    pub(crate) websocket: WebSocket,
    pub(crate) builder: WebSocket,
}
