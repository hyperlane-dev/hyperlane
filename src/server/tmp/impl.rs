use super::r#type::Tmp;
use crate::*;

impl Default for Tmp {
    #[inline]
    fn default() -> Self {
        Self {
            running_thread_num: Arc::new(Mutex::new(0)),
            log: Log::default(),
        }
    }
}

impl Tmp {
    #[inline]
    pub fn add_thread_num(&mut self) {
        let _ = self
            .running_thread_num
            .lock()
            .and_then(|mut running_thread_num| {
                *running_thread_num += 1;
                Ok(())
            });
    }

    #[inline]
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
