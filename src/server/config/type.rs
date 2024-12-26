use lombok_macros::*;

#[derive(Clone, Debug, Lombok)]
pub struct ServerConfig<'a> {
    pub(super) host: &'a str,
    pub(super) port: usize,
    pub(super) thread_pool_size: usize,
    pub(super) log_path: &'a str,
}
