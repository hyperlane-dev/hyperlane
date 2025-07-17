use crate::*;

#[derive(Debug, Clone)]
pub struct RequestContext {
    pub(super) request_id: String,
    pub(super) method: String,
    pub(super) path: String,
    pub(super) remote_addr: OptionString,
    pub(super) user_agent: OptionString,
    pub(super) start_time: std::time::Instant,
}

#[derive(Debug, Default)]
pub struct RequestIdGenerator {
    pub(super) counter: std::sync::atomic::AtomicU64,
}
