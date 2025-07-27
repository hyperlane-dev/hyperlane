use crate::*;

/// A type alias for a write guard on the inner context data.
///
/// This provides exclusive, mutable access to the `ContextInner` data.
pub(crate) type RwLockWriteContextInner<'a> = RwLockWriteGuard<'a, ContextInner>;

/// A type alias for a read guard on the inner context data.
///
/// This provides shared, immutable access to the `ContextInner` data.
pub(crate) type RwLockReadContextInner<'a> = RwLockReadGuard<'a, ContextInner>;
