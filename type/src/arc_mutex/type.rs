use super::*;

/// A type alias for a thread-safe, atomically reference-counted mutex.
///
/// This type is an alias for `Arc<Mutex<T>>`, providing a convenient way to
/// manage shared mutable state across threads.
pub type ArcMutex<T> = Arc<Mutex<T>>;
