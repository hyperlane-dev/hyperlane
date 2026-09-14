use super::*;

/// Host data container storing host value expressions.
///
/// Used for host matching in request processing.
/// Supports both single and multiple host values.
pub(crate) struct MultiHostData {
    /// Vector of host value expressions to match against.
    pub(crate) host_values: Vec<Expr>,
}
