use crate::*;

#[test]
fn context_aborted_and_closed() {
    let ctx: Context = Context::default();
    assert!(!ctx.get_aborted());
    ctx.aborted();
    assert!(ctx.get_aborted());
    ctx.cancel_aborted();
    assert!(!ctx.get_aborted());
    assert!(!ctx.get_closed());
    ctx.closed();
    assert!(ctx.get_closed());
    ctx.cancel_closed();
    assert!(!ctx.get_closed());
    assert!(!ctx.is_terminated());
    ctx.aborted();
    assert!(ctx.is_terminated());
    ctx.cancel_aborted();
    ctx.closed();
    assert!(ctx.is_terminated());
}

#[test]
fn context_route_params() {
    let ctx: Context = Context::default();
    let mut params: RouteParams = RouteParams::default();
    params.insert("id".to_string(), "123".to_string());
    ctx.set_route_params(params);
    let id: Option<String> = ctx.try_get_route_param("id");
    assert_eq!(id, Some("123".to_string()));
    let name: Option<String> = ctx.try_get_route_param("name");
    assert_eq!(name, None);
}

#[test]
fn context_request_and_response() {
    let ctx: Context = Context::default();
    let request: Request = Request::default();
    ctx.set_request(&request);
    let fetched_request: Request = ctx.get_request();
    assert_eq!(request.get_string(), fetched_request.get_string());
    let response: Response = Response::default();
    ctx.set_response(&response);
    let fetched_response: Response = ctx.get_response();
    assert_eq!(response.get_string(), fetched_response.get_string());
}
