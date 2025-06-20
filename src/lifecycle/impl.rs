use super::*;

impl Lifecycle {
    pub(crate) fn is_abort(&self) -> bool {
        matches!(self, Lifecycle::Abort(_))
    }
}
