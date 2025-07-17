use crate::*;

pub(crate) type RwLockWriteInnerContext<'a> = RwLockWriteGuard<'a, InnerContext>;
pub(crate) type RwLockReadInnerContext<'a> = RwLockReadGuard<'a, InnerContext>;
