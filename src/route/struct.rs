use crate::*;

#[derive(Debug, Clone, Getter, DisplayDebug)]
pub(crate) struct RoutePattern(#[get(pub(super))] pub(super) VecRouteSegment);

#[derive(Clone, CustomDebug, Getter, GetterMut, DisplayDebug)]
pub(crate) struct RouteMatcher {
    #[debug(skip)]
    #[get(pub(super))]
    #[get_mut(pub(super))]
    pub(super) static_routes: HashMapStringArcFnPinBoxSendSyncXxHash3_64,
    #[debug(skip)]
    #[get(pub(super))]
    #[get_mut(pub(super))]
    pub(super) dynamic_routes: VecRoutePatternArcFnPinBoxSendSync,
    #[debug(skip)]
    #[get(pub(super))]
    #[get_mut(pub(super))]
    pub(super) regex_routes: VecRoutePatternArcFnPinBoxSendSync,
}
