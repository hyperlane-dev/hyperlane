use crate::*;

#[derive(Clone, Lombok)]
pub struct Server {
    pub(crate) cfg: ArcRwLock<ServerConfig<'static>>,
    pub(crate) router_func: ArcRwLockHashMapRouterFuncBox,
    pub(crate) middleware: ArcRwLockHashMapMiddlewareFuncBox,
    pub(crate) tmp: ArcRwLock<Tmp>,
}
