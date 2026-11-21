use super::*;

#[derive(Default)]
pub(crate) struct UringDropState {
    pub(super) started: Notify,
    pub(super) entered_uring: AtomicBool,
    pub(super) dropped: AtomicBool,
    pub(super) borrow_available: AtomicBool,
}

pub(crate) struct UringDropProbe {
    pub(super) state: Arc<UringDropState>,
}

pub(crate) struct UringPendingDropRoute {
    pub(super) probe: UringDropProbe,
}
