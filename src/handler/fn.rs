use crate::*;

pub(crate) async fn error_handler(error: PanicInfo) {
    eprintln!("{}", error.to_owned());
    let _ = std::io::Write::flush(&mut std::io::stderr());
}
