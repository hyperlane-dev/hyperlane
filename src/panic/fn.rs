use crate::*;

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
        .set_response_status_code(500)
        .await
        .clear_response_headers()
        .await
        .set_response_header(SERVER, HYPERLANE)
        .await
        .set_response_header(CONTENT_TYPE, content_type)
        .await
        .set_response_body(response_body)
        .await
        .send()
        .await;
}
