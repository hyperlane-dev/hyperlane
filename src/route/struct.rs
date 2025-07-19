use crate::*;

#[derive(Debug, Clone)]
pub(crate) struct RoutePattern(pub(super) VecRouteSegment);

#[derive(Clone, CustomDebug)]
pub(crate) struct RouteMatcher {
    #[debug(skip)]
    pub(super) static_routes: HashMapXxHash3_64<String, ArcFnPinBoxSendSync>,
    #[debug(skip)]
    pub(super) dynamic_routes: Vec<(RoutePattern, ArcFnPinBoxSendSync)>,
    #[debug(skip)]
    pub(super) regex_routes: Vec<(RoutePattern, ArcFnPinBoxSendSync)>,
}
