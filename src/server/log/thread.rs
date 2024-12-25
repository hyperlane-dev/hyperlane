use super::r#type::Log;
use std::thread::spawn;

pub fn run() {
    spawn(|| {
        let log: Log = Log::default();
        loop {
            log.write_error();
            log.write_info();
            log.write_debug();
        }
    });
}
