use crate::*;

pub fn handle_error(tmp: &Tmp, err_str: &str) {
    tmp.get_log().error(err_str, common_log);
}
