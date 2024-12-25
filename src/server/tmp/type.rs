use std::sync::{Arc, Mutex};

pub struct Tmp {
    pub(crate) running_thread_num: Arc<Mutex<usize>>,
}
