use super::*;

/// Represents a decoded WebSocket frame.
#[derive(Clone, Debug, DisplayDebug, Eq, Getter, PartialEq)]
pub struct WebSocketFrame {
    /// FIN flag indicating if this is the final frame.
    #[get(type(copy))]
    pub fin: bool,
    /// Opcode indicating the frame type (text, binary, etc.).
    pub opcode: WebSocketOpcode,
    /// Mask flag indicating if the payload is masked.
    #[get(type(copy))]
    pub mask: bool,
    /// The payload data of the frame.
    pub payload_data: Vec<u8>,
}
