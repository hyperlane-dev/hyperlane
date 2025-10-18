use crate::*;

/// A type alias for server operation result.
///
/// This is commonly used throughout the server's public-facing API.
pub type ServerOperationResult<T> = Result<T, ServerError>;
/// A type alias for task join result.
///
/// This is used when waiting for asynchronous tasks to complete.
pub type TaskJoinResult<T> = Result<T, JoinError>;
/// A type alias for shared server state.
///
/// This is the core mechanism for sharing server state across threads.
pub(crate) type SharedServerState = ArcRwLock<ServerInner>;
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
