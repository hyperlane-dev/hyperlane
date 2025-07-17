use crate::*;

#[derive(Debug, Default, Clone, Getter)]
pub struct PanicInfo {
    pub(super) message: String,
    pub(super) location: Option<String>,
    pub(super) payload: String,
    pub(super) request_id: Option<String>,
    pub(super) request_method: Option<String>,
    pub(super) request_path: Option<String>,
    pub(super) remote_addr: Option<String>,
    pub(super) user_agent: Option<String>,
    pub(super) request_duration: Option<Duration>,
}
