use super::*;

/// Represents a WebSocket instance.
///
/// This struct manages broadcast capabilities and holds the internal broadcast map
/// responsible for handling message distribution to various WebSocket connections.
#[derive(Clone, Debug, Default)]
pub struct WebSocket {
    /// The internal broadcast map.
    ///
    /// This map is used for managing WebSocket message distribution.
    pub(super) broadcast_map: BroadcastMap<Vec<u8>>,
}

/// Configuration for a WebSocket connection.
///
/// This struct encapsulates all necessary parameters for setting up and managing
/// a WebSocket connection, including context, buffer sizes, capacity, broadcast type,
/// and hook handlers for different lifecycle events.
///
/// # Type Parameters
///
/// - `B`: The type used for broadcast keys, which must implement `BroadcastTypeTrait`.
pub struct WebSocketConfig<'a, B: BroadcastTypeTrait> {
    /// The underlying stream for this WebSocket connection.
    pub(super) stream: &'a mut Stream,
    /// The Hyperlane context.
    ///
    /// This context is associated with this WebSocket connection.
    pub(super) context: &'a mut Context,
    /// The capacity.
    ///
    /// This is the capacity of the broadcast sender channel.
    pub(super) capacity: Capacity,
    /// The broadcast type.
    ///
    /// This defines the type of broadcast this WebSocket connection will participate in
    /// (point-to-point or point-to-group).
    pub(super) broadcast_type: BroadcastType<B>,
    /// The connected hook handler.
    ///
    /// This hook is executed when the WebSocket connection is established.
    pub(super) connected_hook: ServerHookHandler,
    /// The request hook handler.
    ///
    /// This hook is executed when a new request is received on the WebSocket.
    pub(super) request_hook: ServerHookHandler,
    /// The sended hook handler.
    ///
    /// This hook is executed after a message has been successfully sent over the WebSocket.
    pub(super) sended_hook: ServerHookHandler,
    /// The closed hook handler.
    ///
    /// This hook is executed when the WebSocket connection is closed.
    pub(super) closed_hook: ServerHookHandler,
}
