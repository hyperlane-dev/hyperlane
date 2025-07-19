use crate::*;

#[derive(CustomDebug, Default, Clone, Getter, DisplayDebug)]
pub struct PanicInfo {
    pub(super) message: String,
    pub(super) location: Option<String>,
    pub(super) payload: String,
}
