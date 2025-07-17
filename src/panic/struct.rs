use crate::*;

#[derive(Getter)]
pub(crate) struct PanicHook {
    pub(super) error_hook: AtomicErrorHandlerPtr,
    pub(super) is_initialized: AtomicUsize,
}
