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
