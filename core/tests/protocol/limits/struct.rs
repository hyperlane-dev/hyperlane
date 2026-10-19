use super::*;

pub(crate) struct HeadBodyRoute;

pub(crate) struct NoContentRoute;

pub(crate) struct NotModifiedRoute;

pub(crate) struct RawChunkedRoute;

pub(crate) struct PanicRoute;

pub(crate) struct BodyEchoRoute;

pub(crate) struct LimitsResponseSender;

pub(crate) struct RequestErrorRecorder;

pub(crate) struct TaskPanicRecorder;

pub(crate) enum LimitsOutcome {
    Replied(u16, http::HeaderMap, Vec<u8>),
    H2Reset(h2::Error),
    H3Reset(h3::error::StreamError),
    TimedOut,
    SetupFailed,
}

struct LimitsSignals {
    request_error: Mutex<bool>,
    task_panic: Mutex<bool>,
}

fn limits_signals() -> &'static LimitsSignals {
    static SIGNALS: OnceLock<LimitsSignals> = OnceLock::new();
    SIGNALS.get_or_init(|| LimitsSignals {
        request_error: Mutex::new(false),
        task_panic: Mutex::new(false),
    })
}

fn flag_set(signal: &Mutex<bool>) {
    if let Ok(mut held) = signal.lock() {
        *held = true;
    }
}

fn flag_cleared(signal: &Mutex<bool>) {
    if let Ok(mut held) = signal.lock() {
        *held = false;
    }
}

fn flag_read(signal: &Mutex<bool>) -> bool {
    match signal.lock() {
        Ok(held) => *held,
        Err(_) => false,
    }
}

pub(crate) fn mark_request_error() {
    flag_set(&limits_signals().request_error);
}

pub(crate) fn mark_task_panic() {
    flag_set(&limits_signals().task_panic);
}

pub(crate) fn request_error_seen() -> bool {
    flag_read(&limits_signals().request_error)
}

pub(crate) fn task_panic_seen() -> bool {
    flag_read(&limits_signals().task_panic)
}

pub(crate) fn reset_signals() {
    flag_cleared(&limits_signals().request_error);
    flag_cleared(&limits_signals().task_panic);
}

pub(crate) fn limits_test_lock() -> &'static tokio::sync::Mutex<()> {
    static LOCK: tokio::sync::Mutex<()> = tokio::sync::Mutex::const_new(());
    &LOCK
}
