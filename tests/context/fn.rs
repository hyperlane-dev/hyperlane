use crate::*;

#[test]
fn context_ref_from_address() {
    let ctx: Context = Context::default();
    let ctx_address: usize = (&ctx).into();
    let ctx_ref: &Context = ctx_address.into();
    assert_eq!(&ctx, ctx_ref);
}

#[test]
fn context_mut_from_address() {
    let mut ctx: Context = Context::default();
    let ctx_address: usize = (&mut ctx).into();
    let ctx_mut: &mut Context = ctx_address.into();
    assert_eq!(&mut ctx, ctx_mut);
}

#[test]
fn context_ref_into_address() {
    let ctx: Context = Context::default();
    let ctx_address: usize = (&ctx).into();
    assert!(ctx_address > 0);
}

#[test]
fn context_mut_into_address() {
    let mut ctx: Context = Context::default();
    let ctx_address: usize = (&mut ctx).into();
    assert!(ctx_address > 0);
}

#[test]
fn context_route_params() {
    let mut ctx: Context = Context::default();
    let mut params: RouteParams = RouteParams::default();
    params.insert("id".to_string(), "123".to_string());
    ctx.set_route_params(params);
    let id: Option<String> = ctx.try_get_route_param("id");
    assert_eq!(id, Some("123".to_string()));
    let name: Option<String> = ctx.try_get_route_param("name");
    assert_eq!(name, None);
}

#[test]
fn context_request_and_response_string() {
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

#[test]
fn context_as_ref() {
    let ctx: Context = Context::default();
    let ctx_ref: &Context = ctx.as_ref();
    assert_eq!(ctx.get_request(), ctx_ref.get_request());
    assert_eq!(ctx.get_response(), ctx_ref.get_response());
}

#[test]
fn context_as_mut() {
    let mut ctx: Context = Context::default();
    let new_ctx: Context = ctx.as_mut().clone();
    assert_eq!(ctx, new_ctx);
}

#[test]
fn get_panic_from_context() {
    let mut ctx: Context = Context::default();
    let set_panic: PanicData = PanicData::new(
        Some("test".to_string()),
        Some("test".to_string()),
        Some("test".to_string()),
    );
    ctx.set_task_panic(set_panic.clone());
    let get_panic: PanicData = ctx.try_get_task_panic_data().unwrap();
    assert_eq!(set_panic, get_panic);
}

#[test]
fn context_attributes() {
    let mut ctx: Context = Context::default();
    ctx.set_attribute("key1", "value1".to_string());
    let value: Option<String> = ctx.try_get_attribute("key1");
    assert_eq!(value, Some("value1".to_string()));
    ctx.remove_attribute("key1");
    let value: Option<String> = ctx.try_get_attribute("key1");
    assert_eq!(value, None);
    ctx.set_attribute("key2", 123);
    ctx.clear_attribute();
    let value: Option<i32> = ctx.try_get_attribute("key2");
    assert_eq!(value, None);
}

#[test]
fn run_set_func() {
    let mut ctx: Context = Context::default();
    const KEY: &str = "string";
    const PARAM: &str = "test";
    let func: &(dyn Fn(&str) -> String + Send + Sync) = &|msg: &str| msg.to_string();
    ctx.set_attribute(KEY, func);
    let get_key: &(dyn Fn(&str) -> String + Send + Sync) = ctx.try_get_attribute(KEY).unwrap();
    assert_eq!(get_key(PARAM), func(PARAM));
    let func: &(dyn Fn(&str) + Send + Sync) = &|msg: &str| {
        assert_eq!(msg, PARAM);
    };
    ctx.set_attribute(KEY, func);
    let hyperlane = ctx.get_attribute::<&(dyn Fn(&str) + Send + Sync)>(KEY);
    hyperlane(PARAM);
}
