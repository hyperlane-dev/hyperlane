use crate::*;

pub type ServerResult = Result<(), ServerError>;
pub type ServerRequestHandleResult = Result<Request, ServerError>;

#[derive(Clone, Lombok)]
pub struct Server {
    pub(super) cfg: ArcRwLock<ServerConfig<'static>>,
    pub(super) route: ArcRwLockHashMapRouteFuncBox,
    pub(super) request_middleware: ArcRwLockMiddlewareFuncBox,
    pub(super) response_middleware: ArcRwLockMiddlewareFuncBox,
    pub(super) tmp: ArcRwLockTmp,
}

#[derive(Clone)]
pub struct RequestHandlerParams<'a> {
    pub(super) stream: &'a ArcRwLockStream,
    pub(super) request: Request,
    pub(super) log: &'a Log,
    pub(super) buffer_size: usize,
    pub(super) request_middleware: &'a ArcRwLockMiddlewareFuncBox,
    pub(super) response_middleware: &'a ArcRwLockMiddlewareFuncBox,
    pub(super) route_func: &'a ArcRwLockHashMapRouteFuncBox,
}
