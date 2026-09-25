use super::*;

/// Creates a new thread-safe reference-counted read-write lock.
///
/// # Arguments
///
/// - `T` - The data type to be wrapped.
///
/// # Returns
///
/// - `ArcRwLock<T>` - A new atomic reference-counted read-write lock.
#[inline(always)]
pub fn arc_rwlock<T>(data: T) -> ArcRwLock<T> {
    Arc::new(RwLock::new(data))
}
