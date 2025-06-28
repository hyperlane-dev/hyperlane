use crate::*;

#[derive(Debug, Default, Clone, Getter)]
pub struct PanicInfo {
    pub(super) message: String,
    pub(super) location: Option<String>,
    pub(super) payload: String,
}
