use crate::*;

/// The default panic handler provided by the server.
///
/// This function is invoked when a panic occurs during request processing and no custom
/// panic hook has been configured. It logs the panic information to standard error
/// and sends a `500 Internal Server Error` response to the client.
///
/// # Arguments
///
/// - `Context` - The context of the request during which the panic occurred.
pub(crate) async fn default_panic_hook(ctx: Context) {
    let request_string: String = ctx.get_request_string().await;
    let error: Panic = ctx.get_panic().await.unwrap_or_default();
    let mut response_body: String = error.to_string();
    let content_type: String = ContentType::format_content_type_with_charset(TEXT_PLAIN, UTF8);
    if ctx.get_response().await != Response::default() {
        response_body.push_str(BR);
        response_body.push_str(&request_string);
        response_body.push_str(BR);
    }
    eprintln!("{}", response_body);
    let _ = Write::flush(&mut io::stderr());
    let _ = ctx
        .set_response_version(HttpVersion::HTTP1_1)
        .await
        .set_response_status_code(500)
        .await
        .clear_response_headers()
        .await
        .replace_response_header(SERVER, HYPERLANE)
        .await
        .replace_response_header(CONTENT_TYPE, content_type)
        .await
        .set_response_body(response_body)
        .await
        .send()
        .await;
}
