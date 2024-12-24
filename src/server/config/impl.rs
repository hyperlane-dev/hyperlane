use super::r#type::ServerConfig;
use crate::utils::thread::get_cpu_thread_count;
use http_constant::*;

impl<'a> Default for ServerConfig<'a> {
    fn default() -> Self {
        Self {
            host: DEFAULT_HOST,
            port: DEFAULT_WEB_PORT,
            thread_pool_max_num: get_cpu_thread_count(),
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

    pub fn thread_pool_max_num(&mut self, num: usize) -> &mut Self {
        self.thread_pool_max_num = num;
        self
    }

    pub fn get_thread_pool_max_num(&self) -> usize {
        self.thread_pool_max_num
    }
}
