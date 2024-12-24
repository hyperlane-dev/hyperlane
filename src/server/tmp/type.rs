use std::sync::{Arc, RwLock};

pub struct Tmp {
    pub(crate) thread_pool_num: Arc<RwLock<usize>>,
}
