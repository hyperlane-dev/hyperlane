/// The diagnostic for a WebSocket read attempted before parser initialization.
pub(crate) const WEBSOCKET_STATE_ERROR: &str = "WebSocket state must be initialized";

/// The complete HTTP/1.1 interim response with no fields or body.
pub(super) const HTTP_CONTINUE_RESPONSE: &[u8] = b"HTTP/1.1 100 Continue\r\n\r\n";
