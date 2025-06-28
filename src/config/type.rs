use crate::*;

pub(crate) type ArcRwLockServerConfig = ArcRwLock<ServerConfig>;
pub(crate) type RwLockReadGuardServerConfig<'a> = RwLockReadGuard<'a, ServerConfig>;
