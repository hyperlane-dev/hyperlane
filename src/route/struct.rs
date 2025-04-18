use crate::*;

#[derive(Debug, Clone)]
pub(crate) enum RouteSegment {
    Static(String),
    Dynamic(String),
}

#[derive(Debug, Clone)]
pub(crate) struct RoutePattern(pub(super) VecRouteSegment);

#[derive(Clone)]
pub(crate) struct RouteMatcher(pub(super) VecRoutePatternArcFunc);
