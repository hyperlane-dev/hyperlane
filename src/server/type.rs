use crate::*;

pub type ServerResult<T> = Result<T, ServerError>;
pub type ArcRwLockServerInner = ArcRwLock<ServerInner>;
pub type RwLockReadGuardServerInner<'a> = RwLockReadGuard<'a, ServerInner>;
pub type RwLockWriteGuardServerInner<'a> = RwLockWriteGuard<'a, ServerInner>;
pub type ResultJoinError<T> = Result<T, JoinError>;
