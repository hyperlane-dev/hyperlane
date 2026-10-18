pub(crate) const HEAD_ROUTE_PATH: &str = "/head-body";
pub(crate) const NO_CONTENT_ROUTE_PATH: &str = "/no-content";
pub(crate) const NOT_MODIFIED_ROUTE_PATH: &str = "/not-modified";
pub(crate) const RAW_CHUNKED_ROUTE_PATH: &str = "/raw-chunked";
pub(crate) const PANIC_ROUTE_PATH: &str = "/panic";
pub(crate) const BODY_ECHO_ROUTE_PATH: &str = "/body-echo";

pub(crate) const HEAD_BODY_TEXT: &str = "head-method-body";
pub(crate) const OK_STATUS: usize = 200;
pub(crate) const NO_CONTENT_STATUS: usize = 204;
pub(crate) const NOT_MODIFIED_STATUS: usize = 304;
pub(crate) const TOO_LARGE_STATUS: usize = 413;
pub(crate) const SERVER_ERROR_STATUS: usize = 500;

pub(crate) const WIRE_OK_STATUS: u16 = OK_STATUS as u16;
pub(crate) const WIRE_NO_CONTENT_STATUS: u16 = NO_CONTENT_STATUS as u16;
pub(crate) const WIRE_NOT_MODIFIED_STATUS: u16 = NOT_MODIFIED_STATUS as u16;
pub(crate) const WIRE_SERVER_ERROR_STATUS: u16 = SERVER_ERROR_STATUS as u16;

pub(crate) const CHUNKED_PAYLOAD: &str = "chunked-decoded-payload";
pub(crate) const CHUNK_SIZE_LINE: &str = "10\r\n";
pub(crate) const TASK_PANIC_BODY: &str = "task-panic-handled";
pub(crate) const REQUEST_ERROR_BODY: &str = "request-error-handled";

pub(crate) const MAX_BODY_SIZE: usize = 1024;
pub(crate) const OVERSIZED_BODY_LEN: usize = 4096;
pub(crate) const WRONG_CONTENT_LENGTH: usize = 512;
pub(crate) const DECLARED_BODY_LEN: usize = 32;
pub(crate) const DECLARED_BODY_TEXT: &str = "32";
pub(crate) const READ_TIMEOUT_MS: u64 = 1500;
pub(crate) const CONCURRENT_BUDGET_MS: u64 = 1200;
pub(crate) const INCOMPLETE_BODY_PREFIX: &[u8] = b"incomplete-";
pub(crate) const BODY_ECHO_HEADER: &str = "x-body-len";
pub(crate) const RAW_MARKER_HEADER: &str = "x-raw-marker";
pub(crate) const CHUNKED_TRANSFER_HEADER: &str = "transfer-encoding";
pub(crate) const REQUEST_ERROR_MARKER_HEADER: &str = "x-request-error";
pub(crate) const TASK_PANIC_MARKER_HEADER: &str = "x-task-panic";
