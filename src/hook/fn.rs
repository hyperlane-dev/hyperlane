use crate::*;

pub(crate) async fn default_error_hook(ctx: Context) {
    let error: PanicInfo = ctx.get_panic_info().await.unwrap_or_default();
    let request_string: String = ctx.get_request_string().await;
    let error_string: String = error.to_string();
    let mut response_body: String =
        String::with_capacity(error_string.len() + BR.len() + request_string.len());
    response_body.push_str(&error_string);
    response_body.push_str(BR);
    response_body.push_str(&request_string);
    let content_type: String = ContentType::format_content_type_with_charset(TEXT_PLAIN, UTF8);
    eprintln!("{}", response_body);
    let _ = Write::flush(&mut io::stderr());
    let _ = ctx
        .set_response_status_code(500)
        .await
        .clear_response_headers()
        .await
        .set_response_header(CONTENT_TYPE, content_type)
        .await
        .set_response_body(response_body)
        .await
        .send()
        .await;
}
