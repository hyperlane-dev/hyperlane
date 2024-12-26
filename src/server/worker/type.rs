use std::thread::JoinHandle;

pub struct Worker {
    pub(super) id: usize,
    pub(super) thread: Option<JoinHandle<()>>,
}
