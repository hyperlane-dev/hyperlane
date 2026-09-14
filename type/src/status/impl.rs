use super::*;

/// Implementation block for `Status`.
///
/// Provides convenience methods for checking the status variant.
impl Status {
    /// Returns `true` if the status is `Continue`.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the status is `Continue`, `false` otherwise.
    #[inline(always)]
    pub fn is_continue(&self) -> bool {
        matches!(self, Status::Continue)
    }

    /// Returns `true` if the status is `Reject`.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the status is `Reject`, `false` otherwise.
    #[inline(always)]
    pub fn is_reject(&self) -> bool {
        matches!(self, Status::Reject)
    }
}
