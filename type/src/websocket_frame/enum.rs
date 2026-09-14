use super::*;

/// WebSocket frame opcode types
#[derive(Clone, Copy, Debug, Deserialize, Eq, PartialEq, Serialize)]
#[repr(u8)]
pub enum WebSocketOpcode {
    /// Represents a continuation frame (0x0).
    Continuation = 0x0,
    /// Represents a text frame (0x1).
    Text = 0x1,
    /// Represents a binary frame (0x2).
    Binary = 0x2,
    /// Represents a connection close frame (0x8).
    Close = 0x8,
    /// Represents a ping frame (0x9).
    Ping = 0x9,
    /// Represents a pong frame (0xA).
    Pong = 0xA,
    /// Represents a reserved opcode for future use, including the specific byte value.
    Reserved(u8),
}
