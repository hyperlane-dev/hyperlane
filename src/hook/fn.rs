use crate::*;

pub async fn default_error_hook(ctx: Context, error: PanicInfo) {
    let response_body: String = format!(
        "{}{}{}",
        error.to_string(),
        BR,
        ctx.get_request_string().await
    );
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
