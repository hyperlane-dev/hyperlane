use crate::*;

pub(crate) async fn error_handler(ctx: Context, error: PanicInfo) {
    eprintln!("{}", error.to_owned());
    eprintln!("Request context: {}", ctx.get_request_string().await);
    let _ = Write::flush(&mut io::stderr());
}
