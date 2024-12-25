use http_type::ArcRwLock;
use lazy_static::lazy_static;
use std::{
    fmt::Debug,
    sync::{Arc, RwLock},
};

pub type LogArcLock = ArcRwLock<Vec<String>>;

lazy_static! {
    static ref LOG_ERROR_QUEUE: LogArcLock = Arc::new(RwLock::new(Vec::new()));
    static ref LOG_INFO_QUEUE: LogArcLock = Arc::new(RwLock::new(Vec::new()));
    static ref LOG_DEBUG_QUEUE: LogArcLock = Arc::new(RwLock::new(Vec::new()));
}

fn write(list: &mut Vec<String>) {
    for tem in list.iter() {
        println!("{}", tem);
    }
    list.clear();
}

fn add_data<T>(log_queue: &LogArcLock, data: T)
where
    T: Debug,
{
    if let Ok(mut queue) = log_queue.write() {
        let data_string: String = format!("{:?}", data);
        queue.push(data_string);
    }
}

pub(super) fn write_error() {
    if let Ok(mut error) = LOG_ERROR_QUEUE.write() {
        write(&mut *error);
    }
}

pub(super) fn write_info() {
    if let Ok(mut info) = LOG_INFO_QUEUE.write() {
        write(&mut *info);
    }
}

pub(super) fn write_debug() {
    if let Ok(mut debug) = LOG_DEBUG_QUEUE.write() {
        write(&mut *debug);
    }
}

pub(crate) fn add_error_data<T>(data: T)
where
    T: Debug,
{
    add_data(&LOG_ERROR_QUEUE, data);
}

pub(crate) fn add_info_data<T>(data: T)
where
    T: Debug,
{
    add_data(&LOG_INFO_QUEUE, data);
}

pub(crate) fn add_debug_data<T>(data: T)
where
    T: Debug,
{
    add_data(&LOG_DEBUG_QUEUE, data);
}
