use super::r#type::ServerConfig;
use crate::*;

impl<'a> Default for ServerConfig<'a> {
    fn default() -> Self {
        Self {
            host: DEFAULT_HOST,
            port: DEFAULT_WEB_PORT,
            thread_pool_size: get_thread_count(),
            log_dir: DEFAULT_LOG_DIR,
            log_size: 1_024_000_000,
        }
    }
}
