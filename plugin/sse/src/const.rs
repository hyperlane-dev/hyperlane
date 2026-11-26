/// Default SSE `Content-Type` header value with UTF-8 charset.
pub const SSE_CONTENT_TYPE: &str = "text/event-stream; charset=utf-8";

/// Default `Cache-Control` directive for SSE streams.
pub const SSE_CACHE_CONTROL: &str = "no-cache";

/// Reverse-proxy buffering disable header name for SSE streams.
pub const SSE_ACCEL_BUFFERING_HEADER: &str = "x-accel-buffering";

/// Reverse-proxy buffering disable header value for SSE streams.
pub const SSE_ACCEL_BUFFERING_VALUE: &str = "no";

/// Default heartbeat interval in milliseconds.
pub const DEFAULT_SSE_HEARTBEAT_MS: u64 = 15_000;

/// Default client reconnect retry hint in milliseconds.
pub const DEFAULT_SSE_RETRY_MS: u64 = 3_000;

/// Default bounded replay history retained per topic for `Last-Event-ID` resume.
pub const DEFAULT_SSE_REPLAY_CAPACITY: usize = 64;

/// Default broadcast channel capacity per topic.
pub const DEFAULT_SSE_CHANNEL_CAPACITY: usize = 256;

/// Default maximum number of active topics allowed in one [`SseHub`].
pub const DEFAULT_SSE_MAX_TOPICS: usize = 1024;

/// SSE `id` field prefix.
pub(crate) const SSE_FIELD_ID: &str = "id: ";

/// SSE `event` field prefix.
pub(crate) const SSE_FIELD_EVENT: &str = "event: ";

/// SSE `retry` field prefix.
pub(crate) const SSE_FIELD_RETRY: &str = "retry: ";

/// SSE `data` field prefix.
pub(crate) const SSE_FIELD_DATA: &str = "data: ";

/// SSE comment line prefix.
pub(crate) const SSE_COMMENT_PREFIX: &str = ": ";

/// Default heartbeat comment payload.
pub(crate) const SSE_HEARTBEAT_COMMENT: &str = "keep-alive";

/// Error display text when an [`SseHub`] reaches its topic limit.
pub(crate) const ERR_TOPIC_LIMIT: &str = "SSE topic limit reached";

/// Error display text when a topic broadcast channel has closed.
pub(crate) const ERR_CHANNEL_CLOSED: &str = "SSE topic channel closed";
