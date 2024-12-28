use super::{
    constant::{DEFAULT_LOG_DIR, DEFAULT_LOG_FILE_SIZE},
    r#type::Log,
};

impl Default for Log {
    fn default() -> Self {
        Self {
            path: DEFAULT_LOG_DIR,
            file_size: DEFAULT_LOG_FILE_SIZE,
        }
    }
}

impl Log {
    pub fn new(path: &'static str, file_size: usize) -> Self {
        Self { path, file_size }
    }
}
