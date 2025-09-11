use super::*;

/// Implementation of methods for the `Lifecycle` enum.
impl Lifecycle {
    /// Creates a new Lifecycle instance with Continue state.
    ///
    /// # Arguments
    ///
    /// - `bool` - Whether the connection should be kept alive.
    ///
    /// # Returns
    ///
    /// - `Lifecycle` - A new Lifecycle::Continue instance.
    pub(crate) fn new(keep_alive: bool) -> Self {
        Self::Continue(keep_alive)
    }

    /// Updates the lifecycle status based on abort and keep-alive flags.
    ///
    /// # Arguments
    ///
    /// - `&mut self` - A mutable reference to the `Lifecycle` instance.
    /// - `bool` - Whether the request processing has been aborted.
    /// - `bool` - Whether the connection should be kept alive.
    pub(crate) fn update_status(&mut self, aborted: bool, keep_alive: bool) {
        *self = if aborted {
            Lifecycle::Abort(keep_alive)
        } else {
            Lifecycle::Continue(keep_alive)
        };
    }

    /// Checks if the lifecycle state is Abort.
    ///
    /// # Returns
    ///
    /// - `bool` - true if in Abort state, false otherwise.
    pub(crate) fn is_abort(&self) -> bool {
        matches!(self, Lifecycle::Abort(_))
    }

    /// Checks if the connection should be kept alive.
    ///
    /// # Returns
    ///
    /// - `bool` - true if keep-alive flag is set, false otherwise.
    pub(crate) fn is_keep_alive(&self) -> bool {
        matches!(self, Lifecycle::Continue(true) | Lifecycle::Abort(true))
    }

    /// Returns the keep-alive status of the connection.
    ///
    /// # Returns
    ///
    /// - `bool` - The keep-alive flag value.
    pub(crate) fn keep_alive(&self) -> bool {
        match self {
            Lifecycle::Continue(res) | Lifecycle::Abort(res) => *res,
        }
    }
}
