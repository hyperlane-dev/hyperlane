use crate::*;

#[derive(Clone, Lombok)]
pub struct Server {
    pub cfg: ArcRwLock<ServerConfig<'static>>,
    pub route_func: ArcRwLockHashMapRouteFuncBox,
    pub request_middleware: ArcRwLockHashMapMiddlewareFuncBox,
    pub response_middleware: ArcRwLockHashMapMiddlewareFuncBox,
    pub tmp: ArcRwLock<Tmp>,
}
