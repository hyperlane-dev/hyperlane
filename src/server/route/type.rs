use crate::*;

pub type HashMapRouterFuncBox = HashMap<&'static str, BoxFunc>;
pub type ArcRwLockHashMapRouterFuncBox = ArcRwLock<HashMapRouterFuncBox>;
