use crate::*;

pub fn handle_error(tmp: &Tmp, err_string: String) {
    tmp.get_log().error(err_string, common_log);
}
