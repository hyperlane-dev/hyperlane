use std::sync::{Arc, Mutex};

pub struct Tmp {
    pub(crate) thread_num: Arc<Mutex<usize>>,
}
