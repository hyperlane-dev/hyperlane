use crate::*;

pub type HashMapRouteFuncBox = HashMap<&'static str, BoxFunc>;
pub type ArcRwLockHashMapRouteFuncBox = ArcRwLock<HashMapRouteFuncBox>;
