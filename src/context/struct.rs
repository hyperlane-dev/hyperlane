use crate::*;

#[derive(Clone, Data, Default)]
pub struct InnerContext {
    aborted: bool,
    closed: bool,
    stream: OptionArcRwLockStream,
    request: Request,
    response: Response,
    attributes: HashMapArcAnySendSync,
    route_params: RouteParams,
    server: Server,
}

#[derive(Clone, Default)]
pub struct Context(pub(super) ArcRwLock<InnerContext>);
