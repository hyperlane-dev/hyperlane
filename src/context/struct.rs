use crate::*;

#[derive(Clone, Data, Default)]
pub struct InnerContext {
    stream: OptionArcRwLockStream,
    request: Request,
    response: Response,
    attribute: HashMapArcAnySendSync,
    route_params: ArcRwLockRouteParams,
    aborted: bool,
    closed: bool,
}

#[derive(Clone, Default)]
pub struct Context(pub(super) ArcRwLock<InnerContext>);
