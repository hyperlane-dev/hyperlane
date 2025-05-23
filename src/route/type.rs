use crate::*;

pub(crate) type VecRouteSegment = Vec<RouteSegment>;
pub(crate) type VecRoutePatternArcFunc = Vec<(RoutePattern, ArcFunc)>;
pub(crate) type ArcRwLockRouteMatcher = ArcRwLock<RouteMatcher>;
pub(crate) type RwLockReadGuardRouteMatcher<'a> = RwLockReadGuard<'a, RouteMatcher>;
pub(crate) type HashMapRouteFuncBox = HashMap<String, ArcFunc, BuildHasherDefault<XxHash3_64>>;
pub(crate) type RwLockReadGuardHashMapRouteFuncBox<'a> = RwLockReadGuard<'a, HashMapRouteFuncBox>;
pub(crate) type ArcRwLockHashMapRouteFuncBox = ArcRwLock<HashMapRouteFuncBox>;
pub(crate) type TupleArcFuncRouteParams = (ArcFunc, RouteParams);
pub(crate) type OptionTupleArcFuncRouteParams = Option<TupleArcFuncRouteParams>;
pub(crate) type ResultAddRoute = Result<(), RouteError>;

pub(crate) type RouteParams = HashMap<String, String, BuildHasherDefault<XxHash3_64>>;
pub(crate) type ArcRwLockRouteParams = ArcRwLock<RouteParams>;
