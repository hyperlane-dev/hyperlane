use crate::*;

pub type DashMapRouteFuncBox = DashMap<&'static str, BoxFunc>;
pub type ArcRwLockDashMapRouteFuncBox = ArcRwLock<DashMapRouteFuncBox>;
