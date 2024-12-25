pub struct ServerConfig<'a> {
    pub(crate) host: &'a str,
    pub(crate) port: usize,
    pub(crate) thread_pool_size: usize,
    pub(crate) log_path: &'a str,
}
