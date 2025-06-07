use crate::*;

pub(crate) type VecRouteSegment = Vec<RouteSegment>;
pub(crate) type VecRoutePatternArcFunc = Vec<(RoutePattern, ArcFunc)>;
pub(crate) type ArcRwLockRouteMatcher = ArcRwLock<RouteMatcher>;
pub(crate) type ResultAddRoute = Result<(), RouteError>;
pub(crate) type RouteParams = HashMap<String, String, BuildHasherDefault<XxHash3_64>>;
pub(crate) type ArcRwLockRouteParams = ArcRwLock<RouteParams>;
pub(crate) type ResultVecRouteSegmentRouteError = Result<VecRouteSegment, RouteError>;
pub(crate) type ResultRoutePatternRouteError = Result<RoutePattern, RouteError>;
pub(crate) type OptionRouteParams = Option<RouteParams>;
