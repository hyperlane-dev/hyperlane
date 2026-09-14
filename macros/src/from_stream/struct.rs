use super::*;

/// Represents data for stream processing.
///
/// This struct holds the variable name for stream processing.
pub(crate) struct FromStreamData {
    /// The variable name to store the read data.
    pub(crate) variable_name: Option<Expr>,
}
