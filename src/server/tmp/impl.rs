use std::sync::{Arc, RwLock};

use super::r#type::Tmp;

impl<'a> Default for Tmp {
    fn default() -> Self {
        Self {
            thread_pool_num: Arc::new(RwLock::new(0)),
        }
    }
}

impl Tmp {
    pub fn get_thread_pool_num(&self) -> usize {
        let thread_pool_num_res = self.thread_pool_num.read();
        if thread_pool_num_res.is_err() {
            return 0;
        }
        let thread_pool_num = thread_pool_num_res.unwrap();
        *thread_pool_num
    }

    pub fn thread_num_add(&mut self) -> &mut Self {
        if let Ok(mut thread_pool_num) = self.thread_pool_num.write() {
            *thread_pool_num += 1;
        }
        self
    }

    pub fn thread_num_sub(&mut self) -> &mut Self {
        if let Ok(mut thread_pool_num) = self.thread_pool_num.write() {
            *thread_pool_num = (*thread_pool_num - 1).max(0);
        }
        self
    }
}
