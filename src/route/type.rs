use crate::*;

/// A type alias for a hash map that stores captured route parameters.
///
/// The key is the parameter name and the value is the captured string.
pub type RouteParams = HashMapXxHash3_64<String, String>;
/// A type alias for a vector of `RouteSegment`s.
///
/// This is used to represent a parsed route.
pub(crate) type VecRouteSegment = Vec<RouteSegment>;
/// A type alias for a vector of string slices.
///
/// This is often used for path components.
pub(crate) type VecStrRef<'a> = Vec<&'a str>;
/// A type alias for a vector containing tuples of a `RoutePattern` and its associated handler function.
///
/// This is used for storing dynamic and regex routes.
pub(crate) type VecRoutePatternArcFnPinBoxSendSync<T = ()> =
    Vec<(RoutePattern, ArcFnContextPinBoxSendSync<T>)>;
/// A type alias for a hash map that stores static routes and their handlers.
///
/// The key is the exact path string.
pub(crate) type HashMapStringArcFnPinBoxSendSyncXxHash3_64 =
    HashMapXxHash3_64<String, ArcFnContextPinBoxSendSync<()>>;
/// A type alias for a `Result` returned when adding a new route.
///
/// This indicates success or a `RouteError`.
pub(crate) type ResultAddRoute = Result<(), RouteError>;
/// A type alias for a `Result` from parsing a route string.
///
/// This yields a vector of `RouteSegment`s or a `RouteError`.
pub(crate) type ResultVecRouteSegmentRouteError = Result<VecRouteSegment, RouteError>;
/// A type alias for a `Result` from creating a `RoutePattern`.
///
/// This can fail with a `RouteError`.
pub(crate) type ResultRoutePatternRouteError = Result<RoutePattern, RouteError>;
/// A type alias for an optional `RouteParams` map.
///
/// It is `Some` if a dynamic or regex route matches and captures parameters, and `None` otherwise.
pub(crate) type OptionRouteParams = Option<RouteParams>;
