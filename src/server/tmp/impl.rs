use std::sync::{Arc, Mutex};

use super::r#type::Tmp;

impl Default for Tmp {
    fn default() -> Self {
        Self {
            running_thread_num: Arc::new(Mutex::new(0)),
        }
    }
}

impl Tmp {
    pub fn add_thread_num(&mut self) {
        let _ = self
            .running_thread_num
            .lock()
            .and_then(|mut running_thread_num| {
                *running_thread_num += 1;
                Ok(())
            });
    }

    pub fn sub_thread_num(&mut self) {
        let _ = self
            .running_thread_num
            .lock()
            .and_then(|mut running_thread_num| {
                *running_thread_num -= 1;
                Ok(())
            });
    }
}
