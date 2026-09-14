use super::*;

/// Referer data container storing referer value expressions.
///
/// Used for Referer header matching in request processing.
/// Supports both single and multiple referer values.
pub(crate) struct MultiRefererData {
    /// Vector of referer value expressions to match against.
    pub(crate) referer_values: Vec<Expr>,
}
