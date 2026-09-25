use super::*;

impl Default for WebSocketConfig {
    #[inline(always)]
    fn default() -> Self {
        Self {
            timeout: DEFAULT_HIGH_SECURITY_READ_TIMEOUT_MS,
            url_obj: HttpUrlComponents::default(),
            buffer: DEFAULT_BUFFER_SIZE,
            protocols: Vec::new(),
            proxy: None,
        }
    }
}
