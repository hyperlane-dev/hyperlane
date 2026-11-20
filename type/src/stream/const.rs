use super::*;

/// The diagnostic for a WebSocket read attempted before parser initialization.
pub(crate) const WEBSOCKET_STATE_ERROR: &str = "WebSocket state must be initialized";

/// The complete HTTP/1.1 interim response with no fields or body.
pub(super) const HTTP_CONTINUE_RESPONSE: &[u8] = b"HTTP/1.1 100 Continue\r\n\r\n";

/// The maximum number of reusable header and query entries retained per worker.
pub(crate) const MAX_POOLED_HEADER_ENTRIES: usize = B_128;

/// The maximum retained capacity of a reusable header or query string.
pub(crate) const MAX_POOLED_HEADER_STRING_CAPACITY: usize = 4 * 1024;

/// The maximum retained capacity of a reusable repeated-value list.
pub(crate) const MAX_POOLED_HEADER_VALUE_CAPACITY: usize = B_4;
