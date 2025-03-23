use crate::*;

#[derive(Clone, Lombok)]
pub struct Server {
    pub(super) cfg: ArcRwLock<ServerConfig<'static>>,
    pub(super) route_func: ArcDashMapRouteFuncBox,
    pub(super) request_middleware: ArcRwLockMiddlewareFuncBox,
    pub(super) response_middleware: ArcRwLockMiddlewareFuncBox,
    pub(super) tmp: ArcRwLock<Tmp>,
}
