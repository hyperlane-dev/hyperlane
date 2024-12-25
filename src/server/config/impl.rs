use super::r#type::ServerConfig;
use http_constant::*;

impl<'a> Default for ServerConfig<'a> {
    fn default() -> Self {
        Self {
            host: DEFAULT_HOST,
            port: DEFAULT_WEB_PORT,
            thread_pool_size: 0,
            log_path: "",
        }
    }
}

impl<'a> ServerConfig<'a> {
    pub fn host(&mut self, host: &'a str) -> &mut Self {
        self.host = host;
        self
    }

    pub fn port(&mut self, port: usize) -> &mut Self {
        self.port = port;
        self
    }

    pub fn thread_pool_size(&mut self, thread_pool_size: usize) -> &mut Self {
        self.thread_pool_size = thread_pool_size;
        self
    }

    pub fn log_path(&mut self, log_path: &'a str) -> &mut Self {
        self.log_path = log_path;
        self
    }
}
