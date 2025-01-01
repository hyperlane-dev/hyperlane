use lombok_macros::*;
use std::thread::JoinHandle;

#[allow(dead_code)]
#[derive(Lombok)]
pub struct Worker {
    pub(super) id: usize,
    pub(super) thread: Option<JoinHandle<()>>,
}
