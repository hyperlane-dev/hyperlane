use crate::*;

#[derive(Clone, Getter, Setter)]
pub struct Server {
    #[get(pub(super))]
    #[set(pub(super))]
    pub(super) config: ArcRwLockServerConfig<'static>,
    #[get(pub(super))]
    #[set(pub(super))]
    pub(super) route_matcher: ArcRwLockRouteMatcher,
    #[get(pub(super))]
    #[set(pub(super))]
    pub(super) request_middleware: ArcRwLockVecArcFunc,
    #[get(pub(super))]
    #[set(pub(super))]
    pub(super) response_middleware: ArcRwLockVecArcFunc,
    #[get(pub(super))]
    #[set(pub(super))]
    pub(super) pre_ws_upgrade: ArcRwLockVecArcFunc,
    #[get(pub(super))]
    #[set(pub(super))]
    pub(super) on_ws_connected: ArcRwLockVecArcFunc,
}

#[derive(Clone)]
pub(crate) struct HandlerState<'a> {
    pub(super) stream: &'a ArcRwLockStream,
    pub(super) config: &'a ServerConfig<'a>,
    pub(super) request_middleware: &'a ArcRwLockVecArcFunc,
    pub(super) response_middleware: &'a ArcRwLockVecArcFunc,
    pub(super) route_matcher: &'a ArcRwLockRouteMatcher,
    pub(super) pre_ws_upgrade: &'a ArcRwLockVecArcFunc,
    pub(super) on_ws_connected: &'a ArcRwLockVecArcFunc,
}
