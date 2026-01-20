use crate::*;

#[tokio::test]
async fn context_aborted_and_closed() {
    let ctx: Context = Context::default();
    assert!(!ctx.get_aborted().await);
    ctx.aborted().await;
    assert!(ctx.get_aborted().await);
    ctx.cancel_aborted().await;
    assert!(!ctx.get_aborted().await);
    assert!(!ctx.get_closed().await);
    ctx.closed().await;
    assert!(ctx.get_closed().await);
    ctx.cancel_closed().await;
    assert!(!ctx.get_closed().await);
    assert!(!ctx.is_terminated().await);
    ctx.aborted().await;
    assert!(ctx.is_terminated().await);
    ctx.cancel_aborted().await;
    ctx.closed().await;
    assert!(ctx.is_terminated().await);
}

#[tokio::test]
async fn context_route_params() {
    let ctx: Context = Context::default();
    let mut params: RouteParams = RouteParams::default();
    params.insert("id".to_string(), "123".to_string());
    ctx.set_route_params(params).await;
    let id: Option<String> = ctx.try_get_route_param("id").await;
    assert_eq!(id, Some("123".to_string()));
    let name: Option<String> = ctx.try_get_route_param("name").await;
    assert_eq!(name, None);
}

#[tokio::test]
async fn context_request_and_response() {
    let ctx: Context = Context::default();
    let request: Request = Request::default();
    ctx.set_request(&request).await;
    let fetched_request: Request = ctx.get_request().await;
    assert_eq!(request.get_string(), fetched_request.get_string());
    let response: Response = Response::default();
    ctx.set_response(&response).await;
    let fetched_response: Response = ctx.get_response().await;
    assert_eq!(response.get_string(), fetched_response.get_string());
}
