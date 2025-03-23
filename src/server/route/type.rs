use crate::*;

pub type DashMapRouteFuncBox = DashMap<&'static str, BoxFunc, BuildHasherDefault<XxHash3_64>>;
pub type ArcDashMapRouteFuncBox = Arc<DashMapRouteFuncBox>;
