use crate::*;

pub fn handle_error<T: ToString>(log: &Log, err: T) {
    log.error(err, common_log);
}
