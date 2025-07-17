use crate::*;

#[derive(Clone, Data, Default, Debug)]
pub struct InnerContext {
    aborted: bool,
    closed: bool,
    stream: OptionArcRwLockStream,
    request: Request,
    response: Response,
    attributes: HashMapArcAnySendSync,
    route_params: RouteParams,
}

#[derive(Clone, Default, Debug)]
pub struct Context(pub(super) ArcRwLock<InnerContext>);
