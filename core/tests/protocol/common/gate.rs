use super::*;

#[derive(Clone, Copy)]
pub(crate) enum GateSlot {
    SlowHttp2,
    SlowHttp3,
    SseHttp1,
    SseHttp2,
    SseHttp3,
}

pub(crate) struct RouteGate {
    slow_entered: Notify,
    slow_release: Notify,
    sse_first_chunk_sent: Notify,
    sse_release: Notify,
}

impl RouteGate {
    fn new() -> Self {
        Self {
            slow_entered: Notify::new(),
            slow_release: Notify::new(),
            sse_first_chunk_sent: Notify::new(),
            sse_release: Notify::new(),
        }
    }
}

pub(crate) fn route_gate(slot: GateSlot) -> &'static RouteGate {
    static SLOW_HTTP2: OnceLock<RouteGate> = OnceLock::new();
    static SLOW_HTTP3: OnceLock<RouteGate> = OnceLock::new();
    static SSE_HTTP1: OnceLock<RouteGate> = OnceLock::new();
    static SSE_HTTP2: OnceLock<RouteGate> = OnceLock::new();
    static SSE_HTTP3: OnceLock<RouteGate> = OnceLock::new();
    match slot {
        GateSlot::SlowHttp2 => SLOW_HTTP2.get_or_init(RouteGate::new),
        GateSlot::SlowHttp3 => SLOW_HTTP3.get_or_init(RouteGate::new),
        GateSlot::SseHttp1 => SSE_HTTP1.get_or_init(RouteGate::new),
        GateSlot::SseHttp2 => SSE_HTTP2.get_or_init(RouteGate::new),
        GateSlot::SseHttp3 => SSE_HTTP3.get_or_init(RouteGate::new),
    }
}

pub(crate) type GateLease = tokio::sync::MutexGuard<'static, ()>;

pub(crate) async fn gate_lease(slot: GateSlot) -> GateLease {
    slot_mutex(slot).lock().await
}

fn slot_mutex(slot: GateSlot) -> &'static tokio::sync::Mutex<()> {
    static SLOW_HTTP2: OnceLock<tokio::sync::Mutex<()>> = OnceLock::new();
    static SLOW_HTTP3: OnceLock<tokio::sync::Mutex<()>> = OnceLock::new();
    static SSE_HTTP1: OnceLock<tokio::sync::Mutex<()>> = OnceLock::new();
    static SSE_HTTP2: OnceLock<tokio::sync::Mutex<()>> = OnceLock::new();
    static SSE_HTTP3: OnceLock<tokio::sync::Mutex<()>> = OnceLock::new();
    match slot {
        GateSlot::SlowHttp2 => SLOW_HTTP2.get_or_init(|| tokio::sync::Mutex::new(())),
        GateSlot::SlowHttp3 => SLOW_HTTP3.get_or_init(|| tokio::sync::Mutex::new(())),
        GateSlot::SseHttp1 => SSE_HTTP1.get_or_init(|| tokio::sync::Mutex::new(())),
        GateSlot::SseHttp2 => SSE_HTTP2.get_or_init(|| tokio::sync::Mutex::new(())),
        GateSlot::SseHttp3 => SSE_HTTP3.get_or_init(|| tokio::sync::Mutex::new(())),
    }
}

impl RouteGate {
    pub(crate) fn mark_slow_entered(&self) {
        self.slow_entered.notify_one();
    }

    pub(crate) async fn await_slow_entered(&self) {
        let bound: Duration = Duration::from_secs(GATE_TIMEOUT_SECS);
        timeout(bound, self.slow_entered.notified())
            .await
            .expect("the gated slow route must be entered within the gate timeout");
    }

    pub(crate) async fn await_slow_release(&self) {
        let bound: Duration = Duration::from_secs(GATE_TIMEOUT_SECS);
        timeout(bound, self.slow_release.notified())
            .await
            .expect("the gated slow route must be released within the gate timeout");
    }

    pub(crate) fn release_slow(&self) {
        self.slow_release.notify_one();
    }

    pub(crate) fn mark_sse_first_chunk_sent(&self) {
        self.sse_first_chunk_sent.notify_one();
    }

    pub(crate) async fn await_first_chunk_sent(&self) {
        let bound: Duration = Duration::from_secs(GATE_TIMEOUT_SECS);
        timeout(bound, self.sse_first_chunk_sent.notified())
            .await
            .expect("the first sse chunk must be sent within the gate timeout");
    }

    pub(crate) async fn await_sse_release(&self) {
        let bound: Duration = Duration::from_secs(GATE_TIMEOUT_SECS);
        timeout(bound, self.sse_release.notified())
            .await
            .expect("the sse handler must be released within the gate timeout");
    }

    pub(crate) fn release_sse(&self) {
        self.sse_release.notify_one();
    }
}

pub(crate) fn joined_query(request: &Request, keys: [&str; 2]) -> String {
    let mut collected: Vec<String> = Vec::new();
    for key in keys {
        let value: Option<VecDeque<String>> = request.try_get_query(key);
        match value {
            Some(entries) => {
                for entry in entries {
                    collected.push(format!("{key}={entry}"));
                }
            }
            None => collected.push(format!("{key}=")),
        }
    }
    collected.join(";")
}

pub(crate) fn joined_header(request: &Request, key: &str) -> String {
    let value: Option<VecDeque<String>> = request.try_get_header(key);
    match value {
        Some(entries) => entries.into_iter().collect::<Vec<String>>().join(";"),
        None => String::new(),
    }
}

pub(crate) fn fragment_writes(data: &[u8]) -> Vec<Vec<u8>> {
    let mut pieces: Vec<Vec<u8>> = Vec::new();
    let mut offset: usize = 0;
    while offset < data.len() {
        let end: usize = usize::min(offset + SPLIT_FRAGMENT_LEN, data.len());
        pieces.push(data[offset..end].to_vec());
        offset = end;
    }
    pieces
}
