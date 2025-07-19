use crate::*;

#[derive(Debug, Clone, Getter, DisplayDebug)]
pub(crate) struct RoutePattern(pub(super) VecRouteSegment);

#[derive(Clone, CustomDebug, DisplayDebug)]
pub(crate) struct RouteMatcher {
    #[debug(skip)]
    pub(super) static_routes: HashMapStringArcFnPinBoxSendSyncXxHash3_64,
    #[debug(skip)]
    pub(super) dynamic_routes: VecRoutePatternArcFnPinBoxSendSync,
    #[debug(skip)]
    pub(super) regex_routes: VecRoutePatternArcFnPinBoxSendSync,
}
