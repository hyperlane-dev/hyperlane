use crate::*;

pub(crate) type VecRouteSegment = Vec<RouteSegment>;
pub(crate) type VecRoutePatternArcFunc = Vec<(RoutePattern, ArcFunc)>;
pub(crate) type ArcRwLockRouteMatcher = ArcRwLock<RouteMatcher>;
pub(crate) type HashMapRouteFuncBox = HashMap<String, ArcFunc, BuildHasherDefault<XxHash3_64>>;
pub(crate) type ArcRwLockHashMapRouteFuncBox = ArcRwLock<HashMapRouteFuncBox>;
pub(crate) type TupleArcFuncRouteParams = (ArcFunc, RouteParams);
pub(crate) type OptionTupleArcFuncRouteParams = Option<TupleArcFuncRouteParams>;
pub(crate) type ResultAddRoute = Result<(), RouteError>;

pub type RouteParams = HashMap<String, String, BuildHasherDefault<XxHash3_64>>;
pub type ArcRwLockRouteParams = ArcRwLock<RouteParams>;

#[derive(Debug, Clone)]
pub(crate) enum RouteSegment {
    Static(String),
    Dynamic(String),
}

impl PartialEq for RouteSegment {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (RouteSegment::Static(segment1), RouteSegment::Static(segment2)) => {
                segment1 == segment2
            }
            (RouteSegment::Dynamic(_), RouteSegment::Dynamic(_)) => true,
            _ => false,
        }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct RoutePattern(pub(super) VecRouteSegment);

impl PartialEq for RoutePattern {
    fn eq(&self, other: &Self) -> bool {
        if self.0.len() != other.0.len() {
            return false;
        }
        self.0
            .iter()
            .zip(other.0.iter())
            .all(|(segment1, segment2)| segment1 == segment2)
    }
}

#[derive(Clone)]
pub(crate) struct RouteMatcher(pub(super) VecRoutePatternArcFunc);
