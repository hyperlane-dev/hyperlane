use crate::*;

pub(crate) type RwLockWriteContextInner<'a> = RwLockWriteGuard<'a, ContextInner>;
pub(crate) type RwLockReadContextInner<'a> = RwLockReadGuard<'a, ContextInner>;
