use crate::*;

/// A type alias for shared server configuration.
///
/// This is the core mechanism for sharing server config state across threads.
pub(crate) type SharedServerConfig = ArcRwLock<ServerConfigInner>;

/// A type alias for server state read guard.
///
/// This provides read-only access to the server's internal state.
pub(crate) type ServerStateReadGuard<'a> = RwLockReadGuard<'a, ServerInner>;

/// A type alias for server state write guard.
///
/// This provides mutable access to the server's internal state.
pub(crate) type ServerStateWriteGuard<'a> = RwLockWriteGuard<'a, ServerInner>;
