use super::*;

/// Synchronous WebSocket operations trait.
///
/// Defines the interface for synchronous WebSocket operations including:
/// - Sending messages (text, binary, ping, pong)
/// - Receiving messages
/// - Closing connections
/// - Checking connection status
pub trait WebSocketTrait: Send + Sync {
    /// Sends a text message synchronously.
    ///
    /// # Arguments
    ///
    /// - `&str` - The text message to send.
    ///
    /// # Returns
    ///
    /// - `WebSocketResult` - Result indicating success or failure.
    fn send_text(&mut self, text: &str) -> WebSocketResult;
    /// Sends a binary message synchronously.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The binary data to send.
    ///
    /// # Returns
    ///
    /// - `WebSocketResult` - Result indicating success or failure.
    fn send_binary(&mut self, data: &[u8]) -> WebSocketResult;
    /// Sends a ping message synchronously.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The ping data to send.
    ///
    /// # Returns
    ///
    /// - `WebSocketResult` - Result indicating success or failure.
    fn send_ping(&mut self, data: &[u8]) -> WebSocketResult;
    /// Sends a pong message synchronously.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The pong data to send.
    ///
    /// # Returns
    ///
    /// - `WebSocketResult` - Result indicating success or failure.
    fn send_pong(&mut self, data: &[u8]) -> WebSocketResult;
    /// Receives a message synchronously.
    ///
    /// # Returns
    ///
    /// - `WebSocketMessageResult` - Result containing the received message or error.
    fn receive(&mut self) -> WebSocketMessageResult;
    /// Closes the WebSocket connection synchronously.
    ///
    /// # Returns
    ///
    /// - `WebSocketResult` - Result indicating success or failure.
    fn close(&mut self) -> WebSocketResult;
    /// Checks if the WebSocket is currently connected.
    ///
    /// # Returns
    ///
    /// - `bool` - True if connected, false otherwise.
    fn is_connected(&self) -> bool;
}

/// Asynchronous WebSocket operations trait.
///
/// Defines the interface for asynchronous WebSocket operations including:
/// - Sending messages (text, binary, ping, pong)
/// - Receiving messages
/// - Closing connections
/// - Checking connection status
pub trait AsyncWebSocketTrait: Send + Sync {
    /// Sends a text message asynchronously.
    ///
    /// # Arguments
    ///
    /// - `&str` - The text message to send.
    ///
    /// # Returns
    ///
    /// - `WebSocketResult` - Result indicating success or failure.
    fn send_text<'a>(
        &'a mut self,
        text: &'a str,
    ) -> Pin<Box<dyn Future<Output = WebSocketResult> + Send + 'a>>;
    /// Sends a binary message asynchronously.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The binary data to send.
    ///
    /// # Returns
    ///
    /// - `WebSocketResult` - Result indicating success or failure.
    fn send_binary<'a>(
        &'a mut self,
        data: &'a [u8],
    ) -> Pin<Box<dyn Future<Output = WebSocketResult> + Send + 'a>>;
    /// Sends a ping message asynchronously.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The ping data to send.
    ///
    /// # Returns
    ///
    /// - `WebSocketResult` - Result indicating success or failure.
    fn send_ping<'a>(
        &'a mut self,
        data: &'a [u8],
    ) -> Pin<Box<dyn Future<Output = WebSocketResult> + Send + 'a>>;
    /// Sends a pong message asynchronously.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - The pong data to send.
    ///
    /// # Returns
    ///
    /// - `WebSocketResult` - Result indicating success or failure.
    fn send_pong<'a>(
        &'a mut self,
        data: &'a [u8],
    ) -> Pin<Box<dyn Future<Output = WebSocketResult> + Send + 'a>>;
    /// Receives a message asynchronously.
    ///
    /// # Returns
    ///
    /// - `WebSocketMessageResult` - Result containing the received message or error.
    fn receive<'a>(
        &'a mut self,
    ) -> Pin<Box<dyn Future<Output = WebSocketMessageResult> + Send + 'a>>;
    /// Closes the WebSocket connection asynchronously.
    ///
    /// # Returns
    ///
    /// - `WebSocketResult` - Result indicating success or failure.
    fn close<'a>(&'a mut self) -> Pin<Box<dyn Future<Output = WebSocketResult> + Send + 'a>>;
    /// Checks if the WebSocket is currently connected.
    ///
    /// # Returns
    ///
    /// - `bool` - True if connected, false otherwise.
    fn is_connected(&self) -> bool;
}
