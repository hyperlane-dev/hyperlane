use crate::*;

/// Represents a parsed and structured route pattern.
///
/// This struct wraps a vector of `RouteSegment`s, which are the individual components
/// of a URL path. It is used internally by the `RouteMatcher` to perform efficient
/// route matching against incoming requests.
#[derive(Debug, Clone, Getter, DisplayDebug)]
pub(crate) struct RoutePattern(
    /// The collection of segments that make up the route pattern.
    #[get(pub(super))]
    pub(super) RouteSegmentList,
);

/// The core routing engine responsible for matching request paths to their corresponding handlers.
///
/// The matcher categorizes routes into three types for optimized performance:
/// 1.  `static_routes`- For exact path matches, offering the fastest lookups.
/// 2.  `dynamic_routes`- For paths with variable segments.
/// 3.  `regex_routes`- For complex matching based on regular expressions.
///
/// When a request comes in, the matcher checks these categories in order to find the appropriate handler.
#[derive(Clone, CustomDebug, Getter, GetterMut, DisplayDebug)]
pub(crate) struct RouteMatcher {
    /// A hash map for storing and quickly retrieving handlers for static routes.
    /// These are routes without any variable path segments.
    #[debug(skip)]
    #[get(pub(super))]
    #[get_mut(pub(super))]
    pub(super) static_routes: ServerHookMap,
    /// A vector of routes that contain dynamic segments.
    /// These are evaluated sequentially if no static route matches.
    #[debug(skip)]
    #[get(pub(super))]
    #[get_mut(pub(super))]
    pub(super) dynamic_routes: ServerHookPatternRoutes,
    /// A vector of routes that use regular expressions for matching.
    /// These provide the most flexibility but are evaluated last due to their performance overhead.
    #[debug(skip)]
    #[get(pub(super))]
    #[get_mut(pub(super))]
    pub(super) regex_routes: ServerHookPatternRoutes,
}
