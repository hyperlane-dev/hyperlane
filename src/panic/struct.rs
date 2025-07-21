use crate::*;

#[derive(CustomDebug, Default, PartialEq, Eq, Clone, Getter, DisplayDebug)]
pub struct Panic {
    pub(super) message: OptionString,
    pub(super) location: OptionString,
    pub(super) payload: OptionString,
}
