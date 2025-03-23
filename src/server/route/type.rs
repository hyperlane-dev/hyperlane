use crate::*;

pub type DashMapRouteFuncBox = DashMap<&'static str, BoxFunc, RandomState>;
pub type ArcDashMapRouteFuncBox = Arc<DashMapRouteFuncBox>;
