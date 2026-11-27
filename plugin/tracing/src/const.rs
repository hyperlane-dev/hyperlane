/// Standard W3C Trace Context `traceparent` header name.
pub const HEADER_TRACEPARENT: &str = "traceparent";

/// Standard W3C Trace Context `tracestate` header name.
pub const HEADER_TRACESTATE: &str = "tracestate";

/// Context attribute key storing the active [`crate::TraceContext`].
pub const ATTR_TRACE_CONTEXT: &str = "hyperlane_plugin_tracing_context";

/// Context attribute key storing the request start `Instant` for latency measurement.
pub(crate) const ATTR_TRACE_STARTED_AT: &str = "hyperlane_plugin_tracing_started_at";

/// W3C Trace Context version `00`.
pub(crate) const TRACEPARENT_VERSION: &str = "00";

/// W3C trace flags byte value indicating sampled (`01`).
pub(crate) const TRACE_FLAGS_SAMPLED: &str = "01";

/// W3C trace flags byte value indicating not sampled (`00`).
pub(crate) const TRACE_FLAGS_UNSAMPLED: &str = "00";

/// Default `Server-Timing` metric label.
pub(crate) const DEFAULT_SERVER_TIMING_LABEL: &str = "app";

/// Lowercase hexadecimal alphabet used for fast stack-allocated trace/span formatting.
pub(crate) const HEX_DIGITS: &[u8; 16] = b"0123456789abcdef";

/// SplitMix64 Weyl increment constant.
pub(crate) const SPLITMIX_GAMMA: u64 = 0x9E37_79B9_7F4A_7C15;

/// SplitMix64 first mixing multiplier.
pub(crate) const SPLITMIX_MUL1: u64 = 0xBF58_476D_1CE4_E5B9;

/// SplitMix64 second mixing multiplier.
pub(crate) const SPLITMIX_MUL2: u64 = 0x94D0_49BB_1331_11EB;
