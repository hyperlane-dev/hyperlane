use crate::*;

#[inline]
pub fn log_handler<T: ToString>(log_data: &T) -> String {
    let write_data: String = format!("{}: {}\n", current_time(), log_data.to_string());
    write_data.clone()
}

#[inline]
pub fn log_debug_handler<T: std::fmt::Debug>(log_data: &T) -> String {
    let write_data: String = format!("{}: {:?}\n", current_time(), log_data);
    write_data.clone()
}

#[inline]
pub fn log_debug_format_handler<T: std::fmt::Debug>(log_data: &T) -> String {
    let write_data: String = format!("{}: {:#?}\n", current_time(), log_data);
    write_data.clone()
}
