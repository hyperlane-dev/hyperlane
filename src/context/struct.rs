use crate::*;

#[derive(Clone, Data, Default, CustomDebug, DisplayDebug)]
pub struct ContextInner {
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    aborted: bool,
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    closed: bool,
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    stream: OptionArcRwLockStream,
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    request: Request,
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    response: Response,
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    attributes: HashMapArcAnySendSync,
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    route_params: RouteParams,
}

#[derive(Clone, Default, Getter, CustomDebug, DisplayDebug)]
pub struct Context(#[get(pub(super))] pub(super) ArcRwLock<ContextInner>);
