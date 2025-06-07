use crate::*;

#[derive(Clone, Getter, Setter)]
pub struct Server {
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(super) config: ArcRwLockServerConfig<'static>,
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(super) route_matcher: ArcRwLockRouteMatcher,
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(super) request_middleware: ArcRwLockVecArcFunc,
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(super) response_middleware: ArcRwLockVecArcFunc,
    #[get(pub(crate))]
    #[set(pub(crate))]
    pub(super) on_ws_handshake: ArcRwLockVecArcFunc,
}

#[derive(Clone)]
pub(crate) struct RequestHandlerImmutableParams<'a> {
    pub(super) stream: &'a ArcRwLockStream,
    pub(super) config: &'a ServerConfig<'a>,
    pub(super) request_middleware: &'a ArcRwLockVecArcFunc,
    pub(super) response_middleware: &'a ArcRwLockVecArcFunc,
    pub(super) route_matcher: &'a ArcRwLockRouteMatcher,
    pub(super) on_ws_handshake: &'a ArcRwLockVecArcFunc,
}
