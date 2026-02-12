use crate::*;

/// A type alias for server state read guard.
///
/// This provides read-only access to the server's internal state.
pub(crate) type ServerStateReadGuard<'a> = RwLockReadGuard<'a, ServerData>;

/// A type alias for server state write guard.
///
/// This provides mutable access to the server's internal state.
pub(crate) type ServerStateWriteGuard<'a> = RwLockWriteGuard<'a, ServerData>;
