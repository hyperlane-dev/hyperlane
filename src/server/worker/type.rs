use crate::*;

#[allow(dead_code)]
#[derive(Lombok)]
pub struct Worker {
    pub(super) id: usize,
    pub(super) thread: Option<JoinHandle<()>>,
}
