use super::*;

/// A type alias for a heap-allocated read-write lock.
///
/// This type is an alias for `Box<RwLock<T>>`, providing a way to manage
/// a read-write lock allocated on the heap.
pub type BoxRwLock<T> = Box<RwLock<T>>;
