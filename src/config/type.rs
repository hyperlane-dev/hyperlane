use crate::*;

/// A type alias for configuration loading result.
///
/// This is used for operations that can fail during `ServerConfig` deserialization.
pub type ConfigLoadResult = Result<ServerConfig, serde_json::Error>;
/// A type alias for configuration read guard.
///
/// This provides read-only access to the `ServerConfigInner` wrapped in a `RwLock`.
pub(crate) type ConfigReadGuard<'a> = RwLockReadGuard<'a, ServerConfigInner>;
/// A type alias for configuration write guard.
///
/// This provides mutable access to the `ServerConfigInner` wrapped in a `RwLock`.
pub(crate) type ConfigWriteGuard<'a> = RwLockWriteGuard<'a, ServerConfigInner>;
