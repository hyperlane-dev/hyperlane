use super::*;

impl ServerHook for LargeRoute {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        let body: String = BODY_UNIT.repeat(BODY_REPEATS);
        ctx.get_mut_response()
            .set_status_code(HttpStatus::Ok.code())
            .set_body(body);
        Status::Continue
    }
}

impl ServerHook for SmallRoute {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        ctx.get_mut_response()
            .set_status_code(HttpStatus::Ok.code())
            .set_body(SMALL_BODY);
        Status::Continue
    }
}

impl ServerHook for CodedRoute {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        let body: String = BODY_UNIT.repeat(BODY_REPEATS);
        ctx.get_mut_response()
            .set_status_code(HttpStatus::Ok.code())
            .set_header(CONTENT_ENCODING, BROTLI_TOKEN)
            .set_body(body);
        Status::Continue
    }
}

impl ServerHook for StreamRoute {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        let body: String = BODY_UNIT.repeat(BODY_REPEATS);
        ctx.get_mut_response()
            .set_status_code(HttpStatus::Ok.code())
            .set_header(CONTENT_TYPE, TEXT_EVENT_STREAM)
            .set_body(body);
        Status::Continue
    }
}

impl ServerHook for NoContentRoute {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        ctx.get_mut_response()
            .set_status_code(HttpStatus::NoContent.code())
            .set_body(BODY_UNIT);
        Status::Continue
    }
}

impl ServerHook for VaryRoute {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        let body: String = BODY_UNIT.repeat(BODY_REPEATS);
        ctx.get_mut_response()
            .set_status_code(HttpStatus::Ok.code())
            .set_header(VARY, EXISTING_VARY)
            .set_body(body);
        Status::Continue
    }
}

impl ServerHook for ValidatedRoute {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        let body: String = BODY_UNIT.repeat(BODY_REPEATS);
        ctx.get_mut_response()
            .set_status_code(HttpStatus::Ok.code())
            .set_header(ETAG, APP_VALIDATOR)
            .set_body(body);
        Status::Continue
    }
}

impl ServerHook for SenderMiddleware {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        let data: Vec<u8> = ctx.build_response();
        let _: Result<(), ResponseError> = stream.try_send(data).await;
        Status::Continue
    }
}
