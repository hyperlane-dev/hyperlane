use crate::*;

pub(crate) type VecRouteSegment = Vec<RouteSegment>;
pub(crate) type VecRoutePatternArcFunc = Vec<(RoutePattern, ArcFunc)>;
pub(crate) type ArcRwLockRouteMatcher = ArcRwLock<RouteMatcher>;
pub(crate) type HashMapRouteFuncBox = HashMap<String, ArcFunc, BuildHasherDefault<XxHash3_64>>;
pub(crate) type ArcRwLockHashMapRouteFuncBox = ArcRwLock<HashMapRouteFuncBox>;
pub(crate) type TupleArcFuncRouteParams = (ArcFunc, RouteParams);
pub(crate) type OptionTupleArcFuncRouteParams = Option<TupleArcFuncRouteParams>;

pub type RouteParams = HashMap<String, String, BuildHasherDefault<XxHash3_64>>;
pub type ArcRwLockRouteParams = ArcRwLock<RouteParams>;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum RouteSegment {
    Static(String),
    Dynamic(String),
}

#[derive(Debug, Clone)]
pub(crate) struct RoutePattern(pub(super) VecRouteSegment);

#[derive(Clone)]
pub(crate) struct RouteMatcher(pub(super) VecRoutePatternArcFunc);
