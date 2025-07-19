use crate::*;

#[derive(Clone, Data, Default, CustomDebug, DisplayDebug)]
pub struct InnerContext {
    aborted: bool,
    closed: bool,
    stream: OptionArcRwLockStream,
    request: Request,
    response: Response,
    attributes: HashMapArcAnySendSync,
    route_params: RouteParams,
}

#[derive(Clone, Default, CustomDebug, DisplayDebug)]
pub struct Context(pub(super) ArcRwLock<InnerContext>);
