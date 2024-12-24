use super::r#type::ServerConfig;

impl<'a> Default for ServerConfig<'a> {
    fn default() -> Self {
        Self {
            host: "0.0.0.0",
            port: 80,
            buffer_size: 1024,
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

    pub fn buffer_size(&mut self, buffer_size: usize) -> &mut Self {
        self.buffer_size = buffer_size;
        self
    }
}
