use super::*;

/// Implements `AnySend` for types that are `Any` and `Send`.
///
/// This trait signifies that a type can be safely sent across thread boundaries.
impl<T: Any + Send> AnySend for T {}

/// Implements `AnySendClone` for types that are `Any`, `Send`, and `Clone`.
///
/// This trait signifies that a type can be safely sent across thread boundaries and cloned.
impl<T: Any + Send + Clone> AnySendClone for T {}

/// Implements `AnySync` for types that are `Any` and `Sync`.
///
/// This trait signifies that a type can be safely accessed from multiple threads concurrently.
impl<T: Any + Sync> AnySync for T {}

/// Implements `AnySyncClone` for types that are `Any`, `Sync`, and `Clone`.
///
/// This trait signifies that a type can be safely accessed concurrently and cloned.
impl<T: Any + Sync + Clone> AnySyncClone for T {}

/// Implements `AnySendSync` for types that are `Any`, `Send`, and `Sync`.
///
/// This trait signifies that a type can be safely sent across thread boundaries and accessed concurrently.
impl<T: Any + Send + Sync> AnySendSync for T {}

/// Implements `AnySendSyncClone` for types that are `Any`, `Send`, `Sync`, and `Clone`.
///
/// This trait signifies that a type can be safely sent across thread boundaries, accessed concurrently, and cloned.
impl<T: Any + Send + Sync + Clone> AnySendSyncClone for T {}
