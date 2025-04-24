use crate::*;

#[derive(Clone, Lombok, Default)]
pub struct InnerContext {
    stream: OptionArcRwLockStream,
    request: Request,
    response: Response,
    log: Log,
    attribute: HashMapArcAnySendSync,
    route_params: ArcRwLockRouteParams,
    #[get(pub(super))]
    #[set(pub(super))]
    aborted: bool,
}

#[derive(Clone, Default)]
pub struct Context(pub(super) ArcRwLock<InnerContext>);
