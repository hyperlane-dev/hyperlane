use crate::*;

/// A type alias for a `Result<ServerConfig, serde_json::Error>`.
///
/// This is used for operations that can fail during `ServerConfig` deserialization.
pub type ServerConfigResult = Result<ServerConfig, serde_json::Error>;

/// A type alias for `RwLockReadGuard<'a, ServerConfigInner>`.
///
/// This provides read-only access to the `ServerConfigInner` wrapped in a `RwLock`.
pub(crate) type RwLockReadGuardServerConfigInner<'a> = RwLockReadGuard<'a, ServerConfigInner>;

/// A type alias for `RwLockWriteGuard<'a, ServerConfigInner>`.
///
/// This provides mutable access to the `ServerConfigInner` wrapped in a `RwLock`.
pub(crate) type RwLockWriteGuardServerConfigInner<'a> = RwLockWriteGuard<'a, ServerConfigInner>;
