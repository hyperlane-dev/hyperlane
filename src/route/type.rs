use crate::*;

/// A type alias for a hash map that stores captured route parameters.
///
/// The key is the parameter name and the value is the captured string.
pub type RouteParams = HashMapXxHash3_64<String, String>;

/// A type alias for a list of route segments.
///
/// This is used to represent a parsed route.
pub type RouteSegmentList = Vec<RouteSegment>;

/// A type alias for a list of path components.
///
/// This is often used for path components.
pub(crate) type PathComponentList<'a> = Vec<&'a str>;
