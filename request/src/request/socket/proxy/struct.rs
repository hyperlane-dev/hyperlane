use super::*;

pub struct WebSocketProxyTunnelStream {
    pub(super) inner: BoxAsyncReadWrite,
}

pub struct SyncWebSocketProxyTunnelStream {
    pub(super) inner: BoxReadWrite,
}
