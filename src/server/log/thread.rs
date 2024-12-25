use std::thread::spawn;

use super::r#type::{write_debug, write_error, write_info};

pub fn run() {
    spawn(|| loop {
        write_error();
        write_info();
        write_debug();
    });
}
