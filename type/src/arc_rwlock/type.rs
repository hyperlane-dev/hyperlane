use super::*;

/// A type alias for a thread-safe, atomically reference-counted read-write lock.
///
/// This type is an alias for `Arc<RwLock<T>>`, providing a convenient way to
/// manage shared mutable state with read-write locking capabilities across threads.
pub type ArcRwLock<T> = Arc<RwLock<T>>;
