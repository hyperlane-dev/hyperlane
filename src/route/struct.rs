use crate::*;

#[derive(Debug, Clone)]
pub(crate) struct RoutePattern(pub(super) VecRouteSegment);

#[derive(Clone, CustomDebug)]
pub(crate) struct RouteMatcher {
    #[debug(skip)]
    pub(super) static_routes: HashMapStringArcFnPinBoxSendSyncXxHash3_64,
    #[debug(skip)]
    pub(super) dynamic_routes: VecRoutePatternArcFnPinBoxSendSync,
    #[debug(skip)]
    pub(super) regex_routes: VecRoutePatternArcFnPinBoxSendSync,
}
