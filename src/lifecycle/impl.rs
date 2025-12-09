use super::*;

/// Implementation of methods for the `RequestLifecycle` enum.
impl RequestLifecycle {
    /// Creates a new RequestLifecycle instance with Continuing state.
    ///
    /// # Arguments
    ///
    /// - `bool` - Whether the connection should be kept alive.
    ///
    /// # Returns
    ///
    /// - `RequestLifecycle` - A new RequestLifecycle::Continuing instance.
    #[inline(always)]
    pub(crate) fn new(keep_alive: bool) -> Self {
        Self::Continuing(keep_alive)
    }

    /// Updates the lifecycle status based on abort and keep-alive flags.
    ///
    /// # Arguments
    ///
    /// - `&mut self` - A mutable reference to the `RequestLifecycle` instance.
    /// - `bool` - Whether the request processing has been aborted.
    /// - `bool` - Whether the connection should be kept alive.
    #[inline(always)]
    pub(crate) fn update_status(&mut self, aborted: bool, keep_alive: bool) {
        *self = if aborted {
            RequestLifecycle::Aborted(keep_alive)
        } else {
            RequestLifecycle::Continuing(keep_alive)
        };
    }

    /// Checks if the lifecycle state is Aborted.
    ///
    /// # Returns
    ///
    /// - `bool` - true if in Aborted state, false otherwise.
    #[inline(always)]
    pub(crate) fn is_aborted(&self) -> bool {
        matches!(self, RequestLifecycle::Aborted(_))
    }

    /// Checks if the connection should be kept alive.
    ///
    /// # Returns
    ///
    /// - `bool` - true if keep-alive flag is set, false otherwise.
    #[inline(always)]
    pub(crate) fn is_keep_alive(&self) -> bool {
        matches!(
            self,
            RequestLifecycle::Continuing(true) | RequestLifecycle::Aborted(true)
        )
    }

    /// Returns the keep-alive status of the connection.
    ///
    /// # Returns
    ///
    /// - `bool` - The keep-alive flag value.
    #[inline(always)]
    pub(crate) fn keep_alive(&self) -> bool {
        match self {
            RequestLifecycle::Continuing(res) | RequestLifecycle::Aborted(res) => *res,
        }
    }
}
