use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WebSocketConfig {
    pub(crate) timeout: u64,
    pub(crate) url_obj: HttpUrlComponents,
    pub(crate) buffer: usize,
    pub(crate) protocols: Vec<String>,
    pub(crate) proxy: Option<ProxyConfig>,
}
