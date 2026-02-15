use crate::*;

#[tokio::test]
async fn context_aborted_and_closed() {
    let mut ctx: Context = Context::default();
    assert!(!ctx.get_aborted());
    ctx.set_aborted(true);
    assert!(ctx.get_aborted());
    ctx.set_aborted(false);
    assert!(!ctx.get_aborted());
    assert!(!ctx.get_closed());
    ctx.set_closed(true);
    assert!(ctx.get_closed());
    ctx.set_closed(false);
    assert!(!ctx.get_closed());
    assert!(!ctx.is_terminated());
    ctx.set_aborted(true);
    assert!(ctx.is_terminated());
    ctx.set_aborted(false);
    ctx.set_closed(true);
    assert!(ctx.is_terminated());
}

#[tokio::test]
async fn context_route_params() {
    let mut ctx: Context = Context::default();
    let mut params: RouteParams = RouteParams::default();
    params.insert("id".to_string(), "123".to_string());
    ctx.set_route_params(params);
    let id: Option<String> = ctx.try_get_route_param("id");
    assert_eq!(id, Some("123".to_string()));
    let name: Option<String> = ctx.try_get_route_param("name");
    assert_eq!(name, None);
}

#[tokio::test]
async fn context_request_and_response_string() {
    let mut ctx: Context = Context::default();
    let request: Request = Request::default();
    ctx.set_request(request.clone());
    let fetched_request: &Request = ctx.get_request();
    assert_eq!(request.to_string(), fetched_request.to_string());
    let response: Response = Response::default();
    ctx.set_response(response.clone());
    let fetched_response: &Response = ctx.get_response();
    assert_eq!(response.to_string(), fetched_response.to_string());
}
