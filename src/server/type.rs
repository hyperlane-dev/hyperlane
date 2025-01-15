use crate::*;

pub type AsyncArcRwLock<T> = Arc<tokio::sync::RwLock<T>>;

#[derive(Clone, Lombok)]
pub struct Server {
    pub(crate) cfg: ArcRwLock<ServerConfig<'static>>,
    pub(crate) router_func: ArcRwLockHashMapRouterFuncBox,
    pub(crate) middleware: ArcRwLockHashMapMiddlewareFuncBox,
    pub(crate) async_router_func: AsyncArcRwLockHashMapRouterFuncBox,
    pub(crate) async_middleware: AsyncArcRwLockHashMapMiddlewareFuncBox,
    pub(crate) tmp: ArcRwLock<Tmp>,
}
