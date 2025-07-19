use crate::*;

#[derive(Data)]
pub struct ServerBuilder {
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(super) config: ServerConfig,
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(super) route_matcher: RouteMatcher,
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(super) request_middleware: VecArcFnPinBoxSendSync,
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(super) response_middleware: VecArcFnPinBoxSendSync,
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(super) pre_upgrade_hook: VecArcFnPinBoxSendSync,
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(super) connected_hook: VecArcFnPinBoxSendSync,
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(super) disable_http_hook: HashSetXxHash3_64<String>,
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(super) disable_ws_hook: HashSetXxHash3_64<String>,
    #[get(pub(crate))]
    #[get_mut(pub(crate))]
    #[set(pub(crate))]
    pub(super) error_hook: ArcErrorHandlerSendSync,
}

#[derive(Clone)]
pub struct Server(pub(super) Arc<ServerInner>);

pub struct ServerInner {
    pub(super) config: ServerConfig,
    pub(super) route_matcher: RouteMatcher,
    pub(super) request_middleware: VecArcFnPinBoxSendSync,
    pub(super) response_middleware: VecArcFnPinBoxSendSync,
    pub(super) pre_upgrade_hook: VecArcFnPinBoxSendSync,
    pub(super) connected_hook: VecArcFnPinBoxSendSync,
    pub(super) disable_http_hook: HashSetXxHash3_64<String>,
    pub(super) disable_ws_hook: HashSetXxHash3_64<String>,
    pub(super) error_hook: ArcErrorHandlerSendSync,
}

#[derive(Clone)]
pub(crate) struct HandlerState<'a> {
    pub(super) stream: &'a ArcRwLockStream,
    pub(super) ctx: &'a Context,
}
