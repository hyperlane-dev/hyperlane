use chrono::{DateTime, Local};
use std::{
    fs::{self, metadata, read_dir, OpenOptions},
    io::Write,
};

use super::constant::DEFAULT_LOG_FILE_START_IDX;

pub(crate) fn write_to_file(file_path: &str, content: &str) {
    if let Some(parent_dir) = std::path::Path::new(file_path).parent() {
        let _ = fs::create_dir_all(parent_dir);
    }
    let _ = OpenOptions::new()
        .write(true)
        .append(true)
        .create(true) // Create the file if it does not exist.
        .open(file_path)
        .and_then(|mut file| {
            let _ = file.write_all(content.as_bytes());
            Ok(())
        });
}

pub(crate) fn get_current_date() -> String {
    let today: DateTime<Local> = Local::now();
    today.format("%Y_%m_%d").to_string()
}

pub(crate) fn get_file_size(file_path: &str) -> usize {
    metadata(file_path)
        .and_then(|metadata| Ok(metadata.len()))
        .unwrap_or_default() as usize
}

pub(crate) fn get_second_element_from_filename(dir_path: &str) -> usize {
    let mut res_idx: usize = DEFAULT_LOG_FILE_START_IDX;
    if let Ok(entries) = read_dir(dir_path) {
        for entry in entries.filter_map(Result::ok) {
            let file_name: String = entry.file_name().to_string_lossy().to_string();
            let parts: Vec<&str> = file_name.split('.').collect();
            if parts.len() > 1 {
                if let Ok(second_element) = parts[1].parse::<usize>() {
                    res_idx = second_element.max(res_idx);
                }
            }
        }
    }
    res_idx.max(DEFAULT_LOG_FILE_START_IDX)
}
