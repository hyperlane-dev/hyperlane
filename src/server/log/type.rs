use super::{
    constant::{DEBUG_DIR, ERROR_DIR, INFO_DIR, LOG_EXTION, SLASH},
    r#trait::{LogDataTrait, LogFuncTrait},
    utils::{get_current_date, get_file_size, get_second_element_from_filename, write_to_file},
};
use http_constant::HTTP_BR;
use http_type::ArcRwLock;
use lazy_static::lazy_static;
use lombok_macros::Lombok;
use std::{
    fmt::Debug,
    sync::{Arc, RwLock},
};

pub type LogArcLock = ArcRwLock<Vec<(String, BoxLogFunc)>>;
pub type LogFunc = dyn LogFuncTrait;
pub type BoxLogFunc = Box<LogFunc>;

#[derive(Debug, Clone, Lombok)]
pub struct Log {
    pub(super) path: &'static str,
    pub(super) file_size: usize,
}

lazy_static! {
    static ref LOG_ERROR_QUEUE: LogArcLock = Arc::new(RwLock::new(Vec::new()));
    static ref LOG_INFO_QUEUE: LogArcLock = Arc::new(RwLock::new(Vec::new()));
    static ref LOG_DEBUG_QUEUE: LogArcLock = Arc::new(RwLock::new(Vec::new()));
}

impl Log {
    fn write(list: &mut Vec<(String, BoxLogFunc)>, path: &str) {
        for (log_string, func) in list.iter() {
            let out: String = func(log_string);
            write_to_file(path, &out);
        }
        list.clear();
    }

    fn add_data<T, L>(log_queue: &LogArcLock, data: T, func: L)
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        if let Ok(mut queue) = log_queue.write() {
            let data_string: String = format!("{:?}{}", data, HTTP_BR);
            queue.push((data_string, Box::new(func)));
        }
    }

    fn get_file_name(&self, idx: usize) -> String {
        format!("{}{}.{}.{}", SLASH, get_current_date(), idx, LOG_EXTION)
    }

    fn get_log_path(&self, system_dir: &str) -> String {
        let base_path: &&str = self.get_path();
        let mut combined_path: String = base_path.trim_end_matches(SLASH).to_string();
        if !system_dir.starts_with(SLASH) {
            combined_path.push_str(SLASH);
        }
        combined_path.push_str(system_dir.trim_start_matches(SLASH).trim_end_matches(SLASH));
        let idx: usize = get_second_element_from_filename(&combined_path);
        let mut combined_path_clone: String = combined_path.clone();
        combined_path.push_str(&self.get_file_name(idx));
        let file_size: usize = get_file_size(&combined_path);
        if file_size <= self.file_size {
            return combined_path;
        }
        combined_path_clone.push_str(&self.get_file_name(idx + 1));
        combined_path_clone
    }

    pub(super) fn write_error(&self) {
        if let Ok(mut error) = LOG_ERROR_QUEUE.write() {
            Self::write(&mut *error, &self.get_log_path(ERROR_DIR));
        }
    }

    pub(super) fn write_info(&self) {
        if let Ok(mut info) = LOG_INFO_QUEUE.write() {
            Self::write(&mut *info, &self.get_log_path(INFO_DIR));
        }
    }

    pub(super) fn write_debug(&self) {
        if let Ok(mut debug) = LOG_DEBUG_QUEUE.write() {
            Self::write(&mut *debug, &self.get_log_path(DEBUG_DIR));
        }
    }

    pub fn log_error<T, L>(&self, data: T, func: L)
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        Self::add_data(&LOG_ERROR_QUEUE, data, func);
    }

    pub fn log_info<T, L>(&self, data: T, func: L)
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        Self::add_data(&LOG_INFO_QUEUE, data, func);
    }

    pub fn log_debug<T, L>(&self, data: T, func: L)
    where
        T: LogDataTrait,
        L: LogFuncTrait,
    {
        Self::add_data(&LOG_DEBUG_QUEUE, data, func);
    }
}
