use lombok_macros::Lombok;
use std::thread::JoinHandle;

#[allow(dead_code)]
#[derive(Lombok)]
pub struct Worker {
    pub(super) id: usize,
    pub(super) thread: Option<JoinHandle<()>>,
}
