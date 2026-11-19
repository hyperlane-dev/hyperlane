use super::*;

pub(super) const SEND_BODY: &str = "send payload";
pub(super) const SEND_PATH: &str = "/send";
pub(super) const WIRE_TIMEOUT: Duration = Duration::from_secs(5);
pub(super) const BIND_ATTEMPTS: usize = 3;
pub(super) const RAW_SEND_PATH: &str = "/raw-send";
pub(super) const TRY_SEND_BODY: &str = "try_send payload";
pub(super) const TRY_SEND_PATH: &str = "/try-send";
pub(super) const OK_STATUS_LINE: &str = "HTTP/1.1 200 OK\r\n";
pub(super) const RAW_TRY_SEND_PATH: &str = "/raw-try-send";
pub(super) const RAW_SEND_RESPONSE: &str =
    "HTTP/1.1 200 OK\r\nX-Raw: send\r\nContent-Length: 5\r\n\r\n";
pub(super) const RAW_TRY_SEND_RESPONSE: &str =
    "HTTP/1.1 200 OK\r\nX-Raw: try_send\r\nContent-Length: 8\r\n\r\n";
