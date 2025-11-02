use crate::*;

/// A type alias for a hash map that stores captured route parameters.
///
/// The key is the parameter name and the value is the captured string.
pub type RouteParams = HashMapXxHash3_64<String, String>;
/// A type alias for a list of route segments.
///
/// This is used to represent a parsed route.
pub type RouteSegmentList = Vec<RouteSegment>;
/// A type alias for a map of route segments.
///
/// This is used to store static and dynamic routes.
pub(crate) type OptionAhoCorasick = Option<AhoCorasick>;
/// A type alias for a list of path components.
///
/// This is often used for path components.
pub(crate) type PathComponentList<'a> = Vec<&'a str>;
/// A type alias for route registration result.
///
/// This indicates success or a `RouteError`.
pub(crate) type RouteRegistrationResult = Result<(), RouteError>;
/// A type alias for route parsing result.
///
/// This yields a vector of `RouteSegment`s or a `RouteError`.
pub(crate) type RouteParseResult = Result<RouteSegmentList, RouteError>;
/// A type alias for route pattern creation result.
///
/// This can fail with a `RouteError`.
pub(crate) type RoutePatternResult = Result<RoutePattern, RouteError>;
/// A type alias for optional route parameters.
///
/// It is `Some` if a dynamic or regex route matches and captures parameters, and `None` otherwise.
pub(crate) type OptionalRouteParameters = Option<RouteParams>;
