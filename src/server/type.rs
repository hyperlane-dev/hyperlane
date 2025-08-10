use crate::*;

/// A type alias for a `Result` that returns a `ServerError` on failure.
/// This is commonly used throughout the server's public-facing API.
pub type ServerResult<T> = Result<T, ServerError>;

/// A type alias for a `Result` that returns a `JoinError` on failure.
/// This is used when waiting for asynchronous tasks to complete.
pub type ResultJoinError<T> = Result<T, JoinError>;

/// A type alias for a pinned, boxed, `Send`-able future with no output.
/// This is often used to represent an asynchronous task that can be sent across threads.
pub type PinBoxFutureSend = Pin<Box<dyn FutureSend>>;

/// A type alias for a thread-safe, reference-counted closure that produces a `FnPinBoxFutureSendSync`.
/// This is useful for creating and sharing asynchronous task factories.
pub type ArcFnPinBoxFutureSendSync = Arc<dyn FnPinBoxFutureSendSync>;

/// A type alias for a thread-safe, reference-counted read-write lock over `ServerInner`.
/// This is the core mechanism for sharing server state across threads.
pub(crate) type ArcRwLockServerInner = ArcRwLock<ServerInner>;

/// A type alias for a read guard on the `ServerInner`'s `RwLock`.
/// This provides read-only access to the server's internal state.
pub(crate) type RwLockReadGuardServerInner<'a> = RwLockReadGuard<'a, ServerInner>;

/// A type alias for a write guard on the `ServerInner`'s `RwLock`.
/// This provides mutable access to the server's internal state.
pub(crate) type RwLockWriteGuardServerInner<'a> = RwLockWriteGuard<'a, ServerInner>;
