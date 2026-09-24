use super::*;

/// Boxed synchronous WebSocket trait object.
pub type BoxWebSocketTrait = Box<dyn WebSocketTrait>;
/// Boxed asynchronous WebSocket trait object.
pub type BoxAsyncWebSocketTrait = Box<dyn AsyncWebSocketTrait>;

/// Internal WebSocket connection type.
///
/// Wraps the WebSocket connection in an Arc<AsyncMutex> for thread-safe sharing.
pub(crate) type WebSocketConnection = Arc<Mutex<Option<WebSocketConnectionType>>>;
