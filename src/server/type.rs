use crate::*;

pub type ServerResult = Result<(), ServerError>;
pub type ServerRequestHandleResult = Result<Request, ServerError>;

#[derive(Clone, Lombok)]
pub struct Server {
    pub(super) cfg: ArcRwLock<ServerConfig<'static>>,
    pub(super) route_func: ArcRwLockHashMapRouteFuncBox,
    pub(super) request_middleware: ArcRwLockMiddlewareFuncBox,
    pub(super) response_middleware: ArcRwLockMiddlewareFuncBox,
    pub(super) tmp: ArcRwLock<Tmp>,
}
