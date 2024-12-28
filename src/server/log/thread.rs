use crate::Log;
use std::thread::spawn;

pub fn run(log: &Log) {
    let log_clone: Log = log.clone();
    spawn(move || loop {
        log_clone.write_error();
        log_clone.write_info();
        log_clone.write_debug();
    });
}
