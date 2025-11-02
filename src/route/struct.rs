use crate::*;

/// Represents a parsed and structured route pattern.
///
/// This struct wraps a vector of `RouteSegment`s, which are the individual components
/// of a URL path. It is used internally by the `RouteMatcher` to perform efficient
/// route matching against incoming requests.
#[derive(Debug, Clone, Getter, DisplayDebug)]
pub struct RoutePattern(
    /// The collection of segments that make up the route pattern.
    #[get]
    pub(super) RouteSegmentList,
);

/// The core routing engine responsible for matching request paths to their corresponding handlers.
///
/// The matcher categorizes route into three types for optimized performance:
/// 1.  `static_route`- For exact path matches, offering the fastest lookups.
/// 2.  `dynamic_route`- For paths with variable segments.
/// 3.  `regex_route`- For complex matching based on regular expressions.
///
/// When a request comes in, the matcher checks these categories in order to find the appropriate handler.
#[derive(Clone, CustomDebug, Getter, GetterMut, DisplayDebug, Setter)]
pub struct RouteMatcher {
    /// A hash map for storing and quickly retrieving handlers for static route.
    /// These are route without any variable path segments.
    #[get]
    #[set(skip)]
    #[get_mut(pub(super))]
    #[debug(skip)]
    pub(super) static_route: ServerHookMap,
    /// A layered map of dynamic routes grouped by segment count.
    /// Routes are organized by path segment count for efficient filtering during matching.
    #[get]
    #[set(skip)]
    #[get_mut(pub(super))]
    #[debug(skip)]
    pub(super) dynamic_route: ServerHookPatternRoute,
    /// A layered map of regex routes grouped by segment count.
    /// Routes with tail regex patterns can match paths with more segments.
    #[get]
    #[set(skip)]
    #[get_mut(pub(super))]
    #[debug(skip)]
    pub(super) regex_route: ServerHookPatternRoute,
    /// AC automaton for fast dynamic/regex route static segment matching.
    /// Used to quickly filter candidate routes by matching static segments.
    #[get]
    #[set(pub(super))]
    #[debug(skip)]
    pub(super) ac_automaton: OptionAhoCorasick,
    /// Static segment patterns extracted from dynamic/regex routes for AC automaton.
    /// Maps pattern index to (segment_count, route_index, is_regex).
    #[get]
    #[set(pub(super))]
    #[debug(skip)]
    pub(super) ac_pattern_map: Vec<(usize, usize, bool)>,
}
