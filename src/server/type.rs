use crate::*;

#[derive(Clone, Lombok)]
pub struct Server {
    pub(crate) cfg: ArcRwLock<ServerConfig<'static>>,
    pub(crate) route_func: ArcRwLockHashMapRouteFuncBox,
    pub(crate) request_middleware: ArcRwLockHashMapMiddlewareFuncBox,
    pub(crate) response_middleware: ArcRwLockHashMapMiddlewareFuncBox,
    pub(crate) tmp: ArcRwLock<Tmp>,
}
