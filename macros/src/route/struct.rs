use super::*;

/// Represents the attributes for the `route` macro.
///
/// This struct parses the input attributes for the `route` macro,
/// specifically extracting the path for the route.
pub(crate) struct RouteAttr {
    /// The path expression for the route.
    pub(crate) path: Expr,
}
