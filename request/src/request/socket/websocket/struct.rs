use super::*;

/// Represents different types of WebSocket connections.
///
/// This enum encapsulates both direct and proxy-based WebSocket connections,
/// providing a unified interface for WebSocket operations.
#[derive(Debug)]
pub enum WebSocketConnectionType {
    Direct(WebSocketStream<MaybeTlsStream<http_type::tokio::net::TcpStream>>),
    Proxy(WebSocketStream<WebSocketProxyTunnelStream>),
}

/// Represents a WebSocket client connection.
///
/// This struct manages the WebSocket lifecycle including:
/// - Connection state
/// - Message sending/receiving
/// - Configuration
#[derive(Debug)]
pub struct WebSocket {
    /// The WebSocket server URL.
    pub(crate) url: Arc<String>,
    /// HTTP headers for the WebSocket handshake.
    pub(crate) header: Arc<RequestHeaders>,
    /// Configuration settings for the WebSocket connection.
    pub(crate) config: ArcRwLock<WebSocketConfig>,
    /// Atomic flag indicating connection status.
    pub(crate) connected: Arc<AtomicBool>,
    /// The underlying WebSocket connection.
    pub(crate) connection: WebSocketConnection,
}

/// Clone implementation for WebSocket.
///
/// Creates a new WebSocket instance with cloned configuration but resets:
/// - Connection status to false
/// - Connection to None
impl Clone for WebSocket {
    fn clone(&self) -> Self {
        Self {
            url: self.url.clone(),
            header: self.header.clone(),
            config: self.config.clone(),
            connected: Arc::new(AtomicBool::new(false)),
            connection: Arc::new(http_type::tokio::sync::Mutex::new(None)),
        }
    }
}

/// Default implementation for WebSocket.
///
/// Creates a WebSocket with:
/// - Empty URL
/// - Empty headers
/// - Default configuration
/// - Disconnected state
/// - No active connection
impl Default for WebSocket {
    #[inline(always)]
    fn default() -> Self {
        Self {
            url: Arc::new(String::new()),
            header: Arc::new(hash_map_xx_hash3_64()),
            config: Arc::new(RwLock::new(WebSocketConfig::default())),
            connected: Arc::new(AtomicBool::new(false)),
            connection: Arc::new(http_type::tokio::sync::Mutex::new(None)),
        }
    }
}

/// Stream implementation for WebSocketConnectionType.
///
/// Allows polling for incoming WebSocket messages.
/// Handles both direct and proxy connections uniformly.
impl Stream for WebSocketConnectionType {
    type Item =
        Result<tokio_tungstenite::tungstenite::Message, tokio_tungstenite::tungstenite::Error>;

    fn poll_next(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        match &mut *self {
            WebSocketConnectionType::Direct(stream) => Pin::new(stream).poll_next(cx),
            WebSocketConnectionType::Proxy(stream) => Pin::new(stream).poll_next(cx),
        }
    }
}

/// Sink implementation for WebSocketConnectionType.
///
/// Allows sending WebSocket messages.
/// Handles both direct and proxy connections uniformly.
impl Sink<tokio_tungstenite::tungstenite::Message> for WebSocketConnectionType {
    type Error = tokio_tungstenite::tungstenite::Error;

    fn poll_ready(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        match &mut *self {
            WebSocketConnectionType::Direct(stream) => Pin::new(stream).poll_ready(cx),
            WebSocketConnectionType::Proxy(stream) => Pin::new(stream).poll_ready(cx),
        }
    }

    fn start_send(
        mut self: Pin<&mut Self>,
        item: tokio_tungstenite::tungstenite::Message,
    ) -> Result<(), Self::Error> {
        match &mut *self {
            WebSocketConnectionType::Direct(stream) => Pin::new(stream).start_send(item),
            WebSocketConnectionType::Proxy(stream) => Pin::new(stream).start_send(item),
        }
    }

    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        match &mut *self {
            WebSocketConnectionType::Direct(stream) => Pin::new(stream).poll_flush(cx),
            WebSocketConnectionType::Proxy(stream) => Pin::new(stream).poll_flush(cx),
        }
    }

    fn poll_close(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        match &mut *self {
            WebSocketConnectionType::Direct(stream) => Pin::new(stream).poll_close(cx),
            WebSocketConnectionType::Proxy(stream) => Pin::new(stream).poll_close(cx),
        }
    }
}
