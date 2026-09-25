use super::*;

/// Represents attributes for the Hyperlane macro.
///
/// Used to store parsed variable-type pairs from macro input.
/// Supports both single and multiple pairs.
pub(crate) struct MultiHyperlaneAttr {
    /// Vector of variable-type pairs.
    pub(crate) params: Vec<(Ident, Ident)>,
}
