use crate::*;

#[inline]
pub fn common_log<T: ToString>(data: &T) -> String {
    format!("{}: {}{}", current_time(), data.to_string(), BR)
}

#[inline]
pub fn log_handler<T: ToString>(log_data: &T) -> String {
    common_log(log_data)
}

#[inline]
pub fn log_debug_handler<T: std::fmt::Debug>(log_data: &T) -> String {
    let write_data: String = format!("{}: {:?}{}", current_time(), log_data, BR);
    write_data.clone()
}

#[inline]
pub fn log_debug_format_handler<T: std::fmt::Debug>(log_data: &T) -> String {
    let write_data: String = format!("{}: {:#?}{}", current_time(), log_data, BR);
    write_data.clone()
}
