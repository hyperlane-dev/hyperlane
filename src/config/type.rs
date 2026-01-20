use crate::*;

/// A type alias for configuration read guard.
///
/// This provides read-only access to the `ServerConfigData` wrapped in a `RwLock`.
pub(crate) type ConfigReadGuard<'a> = RwLockReadGuard<'a, ServerConfigData>;

/// A type alias for configuration write guard.
///
/// This provides mutable access to the `ServerConfigData` wrapped in a `RwLock`.
pub(crate) type ConfigWriteGuard<'a> = RwLockWriteGuard<'a, ServerConfigData>;
