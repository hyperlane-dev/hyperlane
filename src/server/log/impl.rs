use std::sync::{Arc, RwLock};

use super::r#type::{Log, LogArcLock};

impl Default for Log {
    fn default() -> Self {
        Self {
            info: Arc::new(RwLock::new(Vec::new())),
            error: Arc::new(RwLock::new(Vec::new())),
            debug: Arc::new(RwLock::new(Vec::new())),
        }
    }
}

impl Log {
    pub fn write(&self, log: LogArcLock) {
        let _ = log.write().and_then(|mut data| {
            for _tem in data.clone() {}
            data.clear();
            Ok(())
        });
    }

    pub fn write_error(&self) {
        self.write(self.error.clone());
    }

    pub fn write_info(&self) {
        self.write(self.info.clone());
    }

    pub fn write_debug(&self) {
        self.write(self.debug.clone());
    }
}
