pub(crate) fn print_error_handle(error: String) {
    eprint!("{}", error);
    let _ = std::io::Write::flush(&mut std::io::stderr());
}
