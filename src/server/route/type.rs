use crate::*;

pub type HashMapRouteFuncBox = HashMap<String, BoxFunc, BuildHasherDefault<XxHash3_64>>;
pub type ArcRwLockHashMapRouteFuncBox = ArcRwLock<HashMapRouteFuncBox>;
