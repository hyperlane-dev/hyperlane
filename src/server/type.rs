use crate::*;

#[derive(Clone, Lombok, Default)]
pub struct Server {
    cfg: ArcRwLock<ServerConfig<'static>>,
    route_func: ArcRwLockHashMapRouteFuncBox,
    request_middleware: ArcRwLockMiddlewareFuncBox,
    response_middleware: ArcRwLockMiddlewareFuncBox,
    tmp: ArcRwLock<Tmp>,
}
