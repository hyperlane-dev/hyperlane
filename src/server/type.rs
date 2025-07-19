use crate::*;

pub type ServerResult = Result<(), ServerError>;
pub type ArcRwLockServerInner = ArcRwLock<ServerInner>;
pub type RwLockReadGuardServerInner<'a> = RwLockReadGuard<'a, ServerInner>;
pub type RwLockWriteGuardServerInner<'a> = RwLockWriteGuard<'a, ServerInner>;
