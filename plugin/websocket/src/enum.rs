use super::*;

/// Represents the type of broadcast for WebSocket messages.
///
/// This enum allows specifying whether a message is intended for a direct
/// point-to-point communication between two entities or for a group of entities.
///
/// # Type Parameters
///
/// - `T`: The type used to identify points or groups, which must implement `BroadcastTypeTrait`.
#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
pub enum BroadcastType<T: BroadcastTypeTrait> {
    /// Indicates a point-to-point broadcast between two specific entities.
    ///
    /// The tuple contains the identifiers of the two entities involved in the communication.
    PointToPoint(T, T),
    /// Indicates a broadcast to a specific group of entities.
    ///
    /// The tuple contains the identifier of the group.
    PointToGroup(T),
    /// Represents an unknown or unhandled broadcast type.
    ///
    /// This variant is used as a default or fallback for unhandled cases.
    Unknown,
}
