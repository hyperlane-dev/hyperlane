use super::*;

impl ServerHook for RejectGuardMiddleware {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        if ctx.get_request().get_path() == "/deny" {
            ctx.get_mut_response()
                .set_status_code(HttpStatus::Forbidden.code())
                .set_body("denied");
            let wire: ResponseData = ctx.get_mut_response().build();
            let _: Result<(), ResponseError> = stream.try_send(&wire).await;
            return Status::Reject;
        }
        Status::Continue
    }
}

impl ServerHook for SetAttrRoute {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        ctx.set_attribute("secret", "leaked".to_owned());
        ctx.get_mut_response()
            .set_header("x-leaked", "yes")
            .set_body("attr-set");
        Status::Continue
    }
}

impl ServerHook for CheckIsolationRoute {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        let leaked_attr: bool = ctx.try_get_attribute::<String, _>("secret").is_some();
        let leaked_param: bool = !ctx.get_route_params().is_empty();
        let leaked_hdr: bool = ctx.get_response().has_header("x-leaked");
        let body: String = format!("{leaked_attr},{leaked_param},{leaked_hdr}");
        ctx.get_mut_response().set_body(body);
        Status::Continue
    }
}

impl ServerHook for PanicRoute {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, _: &mut Stream, _: &mut Context) -> Status {
        panic!("intentional security isolation panic");
    }
}

impl ServerHook for RegexEchoRoute {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        let item: String = ctx.try_get_route_param("item").unwrap_or_default();
        ctx.get_mut_response().set_body(format!("matched:{item}"));
        Status::Continue
    }
}

impl ServerHook for AdminRoute {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        ctx.get_mut_response().set_body("admin-secret");
        Status::Continue
    }
}

impl ServerHook for SecuritySendMiddleware {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        let wire: ResponseData = ctx.get_mut_response().build();
        let _: Result<(), ResponseError> = stream.try_send(&wire).await;
        Status::Continue
    }
}
