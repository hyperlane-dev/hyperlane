use crate::*;

pub type HashMapRouteFuncBox = HashMap<String, ArcFunc, BuildHasherDefault<XxHash3_64>>;
pub type ArcRwLockHashMapRouteFuncBox = ArcRwLock<HashMapRouteFuncBox>;
pub type RouteParams = HashMap<String, String, BuildHasherDefault<XxHash3_64>>;
pub type VecRouteSegment = Vec<RouteSegment>;
pub type TupleArcFuncRouteParams = (ArcFunc, RouteParams);
pub type OptionTupleArcFuncRouteParams = Option<TupleArcFuncRouteParams>;
pub type VecRoutePatternArcFunc = Vec<(RoutePattern, ArcFunc)>;
pub type ArcRwLockRouteMatcher = ArcRwLock<RouteMatcher>;

#[derive(Debug, Clone, PartialEq)]
pub enum RouteSegment {
    Static(String),
    Dynamic(String),
}

#[derive(Debug, Clone)]
pub struct RoutePattern(pub(super) VecRouteSegment);

#[derive(Clone)]
pub struct RouteMatcher(pub(super) VecRoutePatternArcFunc);
