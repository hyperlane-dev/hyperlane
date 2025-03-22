use crate::*;

#[derive(Clone, Lombok)]
pub struct Server {
    pub cfg: ArcRwLock<ServerConfig<'static>>,
    pub route_func: ArcRwLockDashMapRouteFuncBox,
    pub request_middleware: ArcRwLockDashMapMiddlewareFuncBox,
    pub response_middleware: ArcRwLockDashMapMiddlewareFuncBox,
    pub tmp: ArcRwLock<Tmp>,
}
