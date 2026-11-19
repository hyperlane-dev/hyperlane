use super::*;

impl ServerHook for SendRoute {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[send]
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        ctx.get_mut_request().set_method(Method::Get);
        ctx.get_mut_response()
            .set_status_code(HttpStatus::Ok.code())
            .set_body(SEND_BODY);
        Status::Reject
    }
}

impl ServerHook for TrySendRoute {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[try_send]
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        ctx.get_mut_request().set_method(Method::Head);
        ctx.get_mut_response()
            .set_status_code(HttpStatus::Ok.code())
            .set_body(TRY_SEND_BODY);
        Status::Reject
    }
}

impl ServerHook for RawSendRoute {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[send(RAW_SEND_RESPONSE)]
    async fn handle(self, stream: &mut Stream, _ctx: &mut Context) -> Status {
        Status::Reject
    }
}

impl ServerHook for RawTrySendRoute {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    #[try_send(RAW_TRY_SEND_RESPONSE)]
    async fn handle(self, stream: &mut Stream, _ctx: &mut Context) -> Status {
        Status::Reject
    }
}
