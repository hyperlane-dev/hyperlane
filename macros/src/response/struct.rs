use super::*;

/// Represents data for a send operation.
///
/// This struct holds the data to send.
pub(crate) struct SendData {
    /// The data to send.
    pub(crate) data: Expr,
}
