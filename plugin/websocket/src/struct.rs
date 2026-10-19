use super::*;

/// Represents a WebSocket instance.
///
/// This struct manages broadcast capabilities and holds the internal broadcast map
/// responsible for handling message distribution to various WebSocket connections.
#[derive(Clone, Data, Debug, Default)]
pub struct WebSocket {
    /// The internal broadcast map.
    ///
    /// This map is used for managing WebSocket message distribution.
    pub broadcast_map: BroadcastMap<Vec<u8>>,
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
#[derive(Data)]
pub struct WebSocketConfig<'a, B: BroadcastTypeTrait> {
    /// The underlying stream for this WebSocket connection.
    #[set(skip)]
    pub stream: &'a mut Stream,
    /// The Hyperlane context.
    ///
    /// This context is associated with this WebSocket connection.
    #[set(skip)]
    pub context: &'a mut Context,
    /// The capacity.
    ///
    /// This is the capacity of the broadcast sender channel.
    #[get(type(copy))]
    #[set(skip)]
    pub capacity: Capacity,
    /// The broadcast type.
    ///
    /// This defines the type of broadcast this WebSocket connection will participate in
    /// (point-to-point or point-to-group).
    #[set(skip)]
    pub broadcast_type: BroadcastType<B>,
    /// The connected hook handler.
    ///
    /// This hook is executed when the WebSocket connection is established.
    #[set(skip)]
    pub connected_hook: ServerHookHandler,
    /// The request hook handler.
    ///
    /// This hook is executed when a new request is received on the WebSocket.
    #[set(skip)]
    pub request_hook: ServerHookHandler,
    /// The sended hook handler.
    ///
    /// This hook is executed after a message has been successfully sent over the WebSocket.
    #[set(skip)]
    pub sended_hook: ServerHookHandler,
    /// The closed hook handler.
    ///
    /// This hook is executed when the WebSocket connection is closed.
    #[set(skip)]
    pub closed_hook: ServerHookHandler,
}
