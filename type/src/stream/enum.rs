use super::*;

/// Result of inspecting buffered WebSocket wire bytes.
pub(crate) enum WebSocketParseStep {
    /// More bytes must be read from the transport before a frame completes.
    NeedMore,
    /// A complete data message payload was reassembled and validated.
    Message(RequestBody),
    /// A control ping frame arrived and requires a matching pong reply.
    Ping(Vec<u8>),
}
