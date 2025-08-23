use crate::*;

/// Extracts a comparable key from a `RoutePattern`.
///
/// This function iterates over all segments in the pattern and converts
/// each `RouteSegment` into a string slice. For `Regex` segments, only
/// the parameter name is included, ignoring the actual compiled regex.
///
/// # Arguments
///
/// - `&RoutePattern` - A reference to the `RoutePattern` to extract keys from.
///
/// # Returns
///
/// - `Vec<&str>` - A vector of string slices representing each segment of the route.
///   This vector can be used for comparison or hashing purposes.
pub(crate) fn segment_key(pattern: &RoutePattern) -> Vec<&str> {
    pattern
        .get_0()
        .iter()
        .map(|seg| match seg {
            RouteSegment::Static(key) => key.as_str(),
            RouteSegment::Dynamic(key) => key.as_str(),
            RouteSegment::Regex(key, _) => key.as_str(),
        })
        .collect::<Vec<_>>()
}
