use crate::*;

/// A type alias for a `Result` that returns a `ServerError` on failure.
/// This is commonly used throughout the server's public-facing API.
pub type ServerResult<T> = Result<T, ServerError>;

/// A type alias for a thread-safe, reference-counted read-write lock over `ServerInner`.
/// This is the core mechanism for sharing server state across threads.
pub type ArcRwLockServerInner = ArcRwLock<ServerInner>;

/// A type alias for a read guard on the `ServerInner`'s `RwLock`.
/// This provides read-only access to the server's internal state.
pub type RwLockReadGuardServerInner<'a> = RwLockReadGuard<'a, ServerInner>;

/// A type alias for a write guard on the `ServerInner`'s `RwLock`.
/// This provides mutable access to the server's internal state.
pub type RwLockWriteGuardServerInner<'a> = RwLockWriteGuard<'a, ServerInner>;

/// A type alias for a `Result` that returns a `JoinError` on failure.
/// This is used when waiting for asynchronous tasks to complete.
pub type ResultJoinError<T> = Result<T, JoinError>;
