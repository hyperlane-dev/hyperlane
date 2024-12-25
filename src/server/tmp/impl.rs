use std::sync::{Arc, Mutex};

use super::r#type::Tmp;

impl Default for Tmp {
    fn default() -> Self {
        Self {
            thread_num: Arc::new(Mutex::new(0)),
        }
    }
}

impl Tmp {
    pub fn add_thread_num(&mut self) {
        let _ = self.thread_num.lock().and_then(|mut thread_num| {
            *thread_num += 1;
            println!("{}", thread_num);
            Ok(())
        });
    }

    pub fn sub_thread_num(&mut self) {
        let _ = self.thread_num.lock().and_then(|mut thread_num| {
            *thread_num -= 1;
            println!("{}", thread_num);
            Ok(())
        });
    }
}
