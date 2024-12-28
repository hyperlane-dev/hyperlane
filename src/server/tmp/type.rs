use crate::server::log::r#type::Log;
use http_type::ArcMutex;
use lombok_macros::*;

#[derive(Clone, Lombok)]
pub struct Tmp {
    pub(crate) running_thread_num: ArcMutex<usize>,
    pub(crate) log: Log,
}
