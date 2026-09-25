use super::*;

/// Represents data for a response header.
///
/// This struct holds the key, value, and operation for a response header.
pub(crate) struct ResponseHeaderData {
    /// The header key.
    pub(crate) key: Expr,
    /// The header value.
    pub(crate) value: Expr,
    /// The operation to perform on the header (add or set).
    pub(crate) operation: HeaderOperation,
}

/// Represents data for a response body.
///
/// This struct holds the expression for the response body.
pub(crate) struct ResponseBodyData {
    /// The response body expression.
    pub(crate) body: Expr,
}
