use super::*;

impl Lifecycle {
    pub(crate) fn new() -> Self {
        Self::Continue(true)
    }

    pub(crate) fn new_continue(keep_alive: bool) -> Self {
        Self::Continue(keep_alive)
    }

    pub(crate) fn update_status(&mut self, aborted: bool, keep_alive: bool) {
        *self = if aborted {
            Lifecycle::Abort(keep_alive)
        } else {
            Lifecycle::Continue(keep_alive)
        };
    }

    pub(crate) fn is_abort(&self) -> bool {
        matches!(self, Lifecycle::Abort(_))
    }

    pub(crate) fn is_keep_alive(&self) -> bool {
        matches!(self, Lifecycle::Continue(true) | Lifecycle::Abort(true))
    }

    pub(crate) fn keep_alive(&self) -> bool {
        match self {
            Lifecycle::Continue(res) | Lifecycle::Abort(res) => *res,
        }
    }
}
