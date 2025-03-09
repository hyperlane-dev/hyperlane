use crate::*;

#[derive(Clone, Debug, Lombok)]
pub struct ServerConfig<'a> {
    pub(super) host: &'a str,
    pub(super) port: usize,
    pub(super) log_dir: &'a str,
    pub(super) log_size: usize,
    pub(super) interval_millis: usize,
    pub(super) print: bool,
    pub(super) websocket_buffer_size: usize,
}
