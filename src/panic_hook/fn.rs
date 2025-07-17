use crate::*;

pub(crate) fn panic_hook() -> &'static PanicHook {
    &PANIC_HOOK
}
