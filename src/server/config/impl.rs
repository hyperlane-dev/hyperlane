use crate::*;

impl<'a> Default for ServerConfig<'a> {
    #[inline]
    fn default() -> Self {
        Self {
            host: DEFAULT_HOST,
            port: DEFAULT_WEB_PORT,
            log_dir: DEFAULT_LOG_DIR,
            log_size: DEFAULT_LOG_FILE_SIZE,
            interval_millis: DEFAULT_LOG_INTERVAL_MILLIS,
            print: DEFAULT_PRINT,
        }
    }
}
