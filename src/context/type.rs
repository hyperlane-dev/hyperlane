use crate::*;

/// A type alias for a write guard on the context data.
///
/// This provides exclusive, mutable access to the `ContextData` data.
pub(crate) type ContextWriteGuard<'a> = RwLockWriteGuard<'a, ContextData>;

/// A type alias for a read guard on the context data.
///
/// This provides shared, immutable access to the `ContextData` data.
pub(crate) type ContextReadGuard<'a> = RwLockReadGuard<'a, ContextData>;
