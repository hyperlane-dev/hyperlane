use crate::*;

#[derive(Data, Clone, CustomDebug, DisplayDebug)]
pub struct ServerInner {
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) config: ServerConfig,
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) route_matcher: RouteMatcher,
    #[debug(skip)]
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) request_middleware: VecArcFnPinBoxSendSync,
    #[debug(skip)]
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) response_middleware: VecArcFnPinBoxSendSync,
    #[debug(skip)]
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) pre_upgrade_hook: VecArcFnPinBoxSendSync,
    #[debug(skip)]
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) connected_hook: VecArcFnPinBoxSendSync,
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) disable_http_hook: HashSetXxHash3_64<String>,
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) disable_ws_hook: HashSetXxHash3_64<String>,
    #[debug(skip)]
    #[get(pub(super))]
    #[get_mut(pub(super))]
    #[set(pub(super))]
    pub(super) error_hook: ArcErrorHandlerSendSync,
}

#[derive(Clone, Getter, CustomDebug, DisplayDebug)]
pub struct Server(pub(super) ArcRwLockServerInner);

#[derive(Clone, CustomDebug, DisplayDebug)]
pub(crate) struct HandlerState<'a> {
    pub(super) stream: &'a ArcRwLockStream,
    pub(super) ctx: &'a Context,
}
