use super::*;

/// Active W3C Trace Context and request correlation metadata stored in Hyperlane [`Context`].
#[derive(Clone, Data, Debug, Eq, PartialEq)]
pub struct TraceContext {
    /// 32-character lowercase hex 128-bit W3C trace identifier.
    pub(crate) trace_id: String,
    /// 16-character lowercase hex 64-bit W3C span identifier for this hop.
    pub(crate) span_id: String,
    /// Upstream parent span identifier if an incoming `traceparent` was propagated.
    #[get(type(clone))]
    pub(crate) parent_span_id: Option<String>,
    /// Correlation `X-Request-Id` value (reused from incoming header or minted from `trace_id`).
    pub(crate) request_id: String,
    /// Whether this trace carries the W3C sampled flag (`0x01`).
    #[get(type(copy))]
    pub(crate) sampled: bool,
    /// Optional incoming `tracestate` propagated unchanged.
    #[get(type(clone))]
    pub(crate) tracestate: Option<String>,
}

/// Completed request span emitted to registered [`SpanExporter`] backends.
#[derive(Clone, Data, Debug, Eq, PartialEq)]
pub struct SpanRecord {
    /// 128-bit hex trace identifier.
    pub(crate) trace_id: String,
    /// 64-bit hex span identifier.
    pub(crate) span_id: String,
    /// Optional parent span identifier.
    #[get(type(clone))]
    pub(crate) parent_span_id: Option<String>,
    /// Correlation request identifier.
    pub(crate) request_id: String,
    /// HTTP request method string.
    pub(crate) method: String,
    /// HTTP request path.
    pub(crate) path: String,
    /// HTTP response status code.
    #[get(type(copy))]
    pub(crate) status_code: usize,
    /// Total request processing time in microseconds.
    #[get(type(copy))]
    pub(crate) duration_us: u64,
    /// Whether this trace was marked sampled.
    #[get(type(copy))]
    pub(crate) sampled: bool,
}

/// Configuration for [`TracingPlugin`].
#[derive(Clone, Data, Debug, Eq, PartialEq)]
pub struct TracingConfig {
    /// Emits `traceparent` on responses when `true`.
    #[get(type(copy))]
    pub(crate) emit_traceparent: bool,
    /// Emits `X-Request-Id` on responses when `true`.
    #[get(type(copy))]
    pub(crate) emit_request_id: bool,
    /// Emits `Server-Timing` latency header on responses when `true`.
    #[get(type(copy))]
    pub(crate) emit_server_timing: bool,
    /// Metric label used in `Server-Timing` (default `"app"`).
    pub(crate) server_timing_label: String,
    /// Default sampled flag assigned to newly originated traces.
    #[get(type(copy))]
    pub(crate) default_sampled: bool,
}

/// Process-wide [`TracingPlugin`] singleton storing [`TracingConfig`] and [`SpanExporter`] hooks.
#[derive(Clone)]
pub struct TracingPlugin {
    /// Active tracing configuration.
    pub(crate) config: TracingConfig,
    /// Registered span exporters invoked upon response completion.
    pub(crate) exporters: Arc<Vec<SharedSpanExporter>>,
}

/// Opt-in request middleware establishing [`TraceContext`] and setting correlation headers early.
#[derive(Clone, Copy, Debug, Default)]
#[hyperlane::request_middleware(-130)]
pub struct TracingRequestHook;

/// Opt-in response middleware recording latency, setting `Server-Timing`, and exporting spans.
#[derive(Clone, Copy, Debug, Default)]
#[hyperlane::response_middleware(90)]
pub struct TracingResponseHook;
