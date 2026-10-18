use super::*;

const PANIC_PAYLOAD: &str = "limits-panic-payload";

impl ServerHook for HeadBodyRoute {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        ctx.get_mut_response()
            .set_status_code(OK_STATUS)
            .set_header(CONTENT_TYPE, TEXT_PLAIN)
            .set_body(HEAD_BODY_TEXT);
        Status::Continue
    }
}

impl ServerHook for NoContentRoute {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        ctx.get_mut_response()
            .set_status_code(NO_CONTENT_STATUS)
            .set_header("x-no-content", "limits")
            .set_body(CHUNKED_PAYLOAD);
        Status::Continue
    }
}

impl ServerHook for NotModifiedRoute {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        ctx.get_mut_response()
            .set_status_code(NOT_MODIFIED_STATUS)
            .set_header("x-not-modified", "limits")
            .set_body(CHUNKED_PAYLOAD);
        Status::Continue
    }
}

impl ServerHook for RawChunkedRoute {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, stream: &mut Stream, _: &mut Context) -> Status {
        let mut head: String = format!("{HTTP_VERSION_1_1} {OK_STATUS}{HTTP_BR}");
        head.push_str(&format!("{CONTENT_TYPE}: {TEXT_PLAIN}{HTTP_BR}"));
        head.push_str(&format!("{RAW_MARKER_HEADER}: chunked{HTTP_BR}"));
        head.push_str(&format!("{CHUNKED_TRANSFER_HEADER}: {CHUNKED}{HTTP_BR}"));
        head.push_str(&format!("{CONNECTION}: {CLOSE}{HTTP_BR}"));
        head.push_str(HTTP_BR);
        if stream.try_send(head.into_bytes()).await.is_err() {
            stream.set_closed(true);
            return Status::Reject;
        }
        let framed: Vec<u8> =
            format!("{CHUNK_SIZE_LINE}{CHUNKED_PAYLOAD}{HTTP_BR}0{HTTP_BR}{HTTP_BR}").into_bytes();
        if stream.try_send(framed).await.is_err() {
            stream.set_closed(true);
            return Status::Reject;
        }
        stream.set_closed(true);
        Status::Reject
    }
}

impl ServerHook for PanicRoute {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, _: &mut Stream, _: &mut Context) -> Status {
        panic!("{PANIC_PAYLOAD}");
    }
}

impl ServerHook for BodyEchoRoute {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        let body: Vec<u8> = ctx.get_request().get_body().clone();
        let length: String = body.len().to_string();
        ctx.get_mut_response()
            .set_status_code(OK_STATUS)
            .set_header(CONTENT_TYPE, APPLICATION_OCTET_STREAM)
            .set_header(BODY_ECHO_HEADER, &length)
            .set_body(&body);
        Status::Continue
    }
}

impl ServerHook for LimitsResponseSender {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        let data: Vec<u8> = ctx.get_mut_response().build();
        if stream.try_send(data).await.is_err() {
            stream.set_closed(true);
            return Status::Reject;
        }
        Status::Continue
    }
}

impl ServerHook for RequestErrorRecorder {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        mark_request_error();
        let data: Vec<u8> = ctx
            .get_mut_response()
            .set_status_code(TOO_LARGE_STATUS)
            .set_header(REQUEST_ERROR_MARKER_HEADER, "seen")
            .set_body(REQUEST_ERROR_BODY)
            .build();
        if stream.try_send(data).await.is_err() {
            stream.set_closed(true);
            return Status::Reject;
        }
        Status::Reject
    }
}

impl ServerHook for TaskPanicRecorder {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        mark_task_panic();
        let data: Vec<u8> = ctx
            .get_mut_response()
            .set_status_code(SERVER_ERROR_STATUS)
            .set_header(TASK_PANIC_MARKER_HEADER, "seen")
            .set_body(TASK_PANIC_BODY)
            .build();
        if stream.try_send(data).await.is_err() {
            stream.set_closed(true);
            return Status::Reject;
        }
        Status::Reject
    }
}
