use crate::*;

#[derive(Clone, Lombok, Default)]
pub struct Server {
    cfg: ArcRwLock<ServerConfig<'static>>,
    route_func: ArcRwLockDashMapRouteFuncBox,
    request_middleware: ArcRwLockDashMapMiddlewareFuncBox,
    response_middleware: ArcRwLockDashMapMiddlewareFuncBox,
    tmp: ArcRwLock<Tmp>,
}
