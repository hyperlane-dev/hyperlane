use super::r#type::ServerConfig;
use http_constant::*;

impl<'a> Default for ServerConfig<'a> {
    fn default() -> Self {
        Self {
            host: DEFAULT_HOST,
            port: DEFAULT_WEB_PORT,
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
}
