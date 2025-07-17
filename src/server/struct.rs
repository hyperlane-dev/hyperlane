use crate::*;

#[derive(Clone, Getter, Setter)]
pub struct Server {
    #[get(pub(super))]
    #[set(pub(super))]
    pub(super) config: ArcRwLockServerConfig,
    #[get(pub(super))]
    #[set(pub(super))]
    pub(super) route_matcher: ArcRwLockRouteMatcher,
    #[get(pub(super))]
    #[set(pub(super))]
    pub(super) request_middleware: ArcRwLockVecArcFnPinBoxSendSync,
    #[get(pub(super))]
    #[set(pub(super))]
    pub(super) response_middleware: ArcRwLockVecArcFnPinBoxSendSync,
    #[get(pub(super))]
    #[set(pub(super))]
    pub(super) pre_upgrade_hook: ArcRwLockVecArcFnPinBoxSendSync,
    #[get(pub(super))]
    #[set(pub(super))]
    pub(super) ws_connected_hook: ArcRwLockVecArcFnPinBoxSendSync,
}

#[derive(Clone)]
pub(crate) struct HandlerState<'a> {
    pub(super) stream: &'a ArcRwLockStream,
}
