use super::*;

/// Creates a new boxed read-write lock.
///
/// # Arguments
///
/// - `T` - The data type to be wrapped.
///
/// # Returns
///
/// - `BoxRwLock<T>` - A new boxed read-write lock.
#[inline(always)]
pub fn box_rwlock<T>(data: T) -> BoxRwLock<T> {
    Box::new(RwLock::new(data))
}
