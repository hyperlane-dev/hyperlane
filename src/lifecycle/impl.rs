use super::*;

impl Lifecycle {
    /// Creates a new `Lifecycle` instance, defaulting to a continued and keep-alive state.
    ///
    /// # Returns
    ///
    /// A new `Lifecycle::Continue(true)` instance.
    pub(crate) fn new() -> Self {
        Self::Continue(true)
    }

    /// Creates a new `Lifecycle` instance with a `Continue` state.
    ///
    /// # Arguments
    ///
    /// - `keep_alive` - A boolean indicating whether the connection should be kept alive.
    ///
    /// # Returns
    ///
    /// A new `Lifecycle::Continue` instance with the specified keep-alive status.
    pub(crate) fn new_continue(keep_alive: bool) -> Self {
        Self::Continue(keep_alive)
    }

    /// Updates the lifecycle status based on whether the connection is aborted or should be kept alive.
    ///
    /// # Arguments
    ///
    /// - `aborted` - A boolean indicating if the request processing has been aborted.
    /// - `keep_alive` - A boolean indicating if the connection should be kept alive.
    pub(crate) fn update_status(&mut self, aborted: bool, keep_alive: bool) {
        *self = if aborted {
            Lifecycle::Abort(keep_alive)
        } else {
            Lifecycle::Continue(keep_alive)
        };
    }

    /// Checks if the lifecycle state is `Abort`.
    ///
    /// # Returns
    ///
    /// `true` if the lifecycle is in the `Abort` state, otherwise `false`.
    pub(crate) fn is_abort(&self) -> bool {
        matches!(self, Lifecycle::Abort(_))
    }

    /// Checks if the connection should be kept alive, regardless of whether it is aborted or continued.
    ///
    /// # Returns
    ///
    /// `true` if the keep-alive flag is set, otherwise `false`.
    pub(crate) fn is_keep_alive(&self) -> bool {
        matches!(self, Lifecycle::Continue(true) | Lifecycle::Abort(true))
    }

    /// Returns the keep-alive status of the connection.
    ///
    /// # Returns
    ///
    /// The boolean value of the keep-alive flag from either the `Continue` or `Abort` state.
    pub(crate) fn keep_alive(&self) -> bool {
        match self {
            Lifecycle::Continue(res) | Lifecycle::Abort(res) => *res,
        }
    }
}
