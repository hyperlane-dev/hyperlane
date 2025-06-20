use crate::*;

pub(super) static ROUTE_PARAMS_POOL: OnceLock<ObjectPool<RouteParams>> = OnceLock::new();
