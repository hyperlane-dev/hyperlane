use super::*;

/// Represents a type that can be dynamically downcast and is safe to send across threads.
///
/// This trait combines the capabilities of `Any` and `Send`, ensuring that types implementing
/// it can be safely sent across thread boundaries.
pub trait AnySend: Any + Send {}

/// Represents a type that can be dynamically downcast, is safe to send across threads, and can be cloned.
///
/// This trait combines the capabilities of `Any`, `Send`, and `Clone`, ensuring that types implementing
/// it can be safely sent across thread boundaries and can be cloned.
pub trait AnySendClone: Any + Send + Clone {}

/// Represents a type that can be dynamically downcast and is safe to share across threads.
///
/// This trait combines the capabilities of `Any` and `Sync`, ensuring that types implementing
/// it can be safely shared across thread boundaries.
pub trait AnySync: Any + Sync {}

/// Represents a type that can be dynamically downcast, is safe to share across threads, and can be cloned.
///
/// This trait combines the capabilities of `Any`, `Sync`, and `Clone`, ensuring that types implementing
/// it can be safely shared across thread boundaries and can be cloned.
pub trait AnySyncClone: Any + Sync + Clone {}

/// Represents a type that can be dynamically downcast and is safe to both send and share across threads.
///
/// This trait combines the capabilities of `Any`, `Send`, and `Sync`, ensuring that types implementing
/// it can be safely sent and shared across thread boundaries.
pub trait AnySendSync: Any + Send + Sync {}

/// Represents a type that can be dynamically downcast, is safe to both send and share across threads, and can be cloned.
///
/// This trait combines the capabilities of `Any`, `Send`, `Sync`, and `Clone`, ensuring that types implementing
/// it can be safely sent and shared across thread boundaries and can be cloned.
pub trait AnySendSyncClone: Any + Send + Sync + Clone {}
