use crate::*;

/// A type alias for a write guard on the context data.
///
/// This provides exclusive, mutable access to the `ContextInner` data.
pub(crate) type ContextWriteGuard<'a> = RwLockWriteGuard<'a, ContextInner>;
/// A type alias for a read guard on the context data.
///
/// This provides shared, immutable access to the `ContextInner` data.
pub(crate) type ContextReadGuard<'a> = RwLockReadGuard<'a, ContextInner>;
