use crate::*;

#[derive(Clone, Data, Default, CustomDebug)]
pub struct InnerContext {
    aborted: bool,
    closed: bool,
    stream: OptionArcRwLockStream,
    request: Request,
    response: Response,
    attributes: HashMapArcAnySendSync,
    route_params: RouteParams,
}

#[derive(Clone, Default, CustomDebug)]
pub struct Context(pub(super) ArcRwLock<InnerContext>);
