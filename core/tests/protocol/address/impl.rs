use super::*;

impl ServerHook for TargetEchoRoute {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        ctx.get_mut_request()
            .set_path(TARGET_REWRITTEN_PATH.to_owned());
        let target: String = ctx
            .get_request_target()
            .expect("every request must carry its original target");
        ctx.get_mut_response()
            .set_status_code(200)
            .set_header(CONTENT_TYPE, TEXT_PLAIN)
            .set_header(TARGET_ECHO_HEADER, &target)
            .set_body(&target);
        Status::Continue
    }
}
