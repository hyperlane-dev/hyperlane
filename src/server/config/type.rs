pub struct ServerConfig<'a> {
    pub(crate) host: &'a str,
    pub(crate) port: usize,
    pub(crate) buffer_size: usize,
}
