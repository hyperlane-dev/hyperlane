use crate::*;

#[derive(CustomDebug, Default, PartialEq, Eq, Clone, Getter, DisplayDebug)]
pub struct Panic {
    pub(super) message: String,
    pub(super) location: String,
    pub(super) payload: String,
}
