use super::*;

impl ServerHook for WebsocketUpgradeMiddleware {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        if !ctx.get_request().is_ws_upgrade_type() {
            return Status::Continue;
        }
        if let Some(key) = &ctx.get_request().try_get_header_back(SEC_WEBSOCKET_KEY) {
            let accept_key: String = WebSocketFrame::generate_accept_key(key);
            let data: Vec<u8> = ctx
                .get_mut_response()
                .set_version(HttpVersion::Http1_1)
                .set_status_code(101)
                .set_header(UPGRADE, WEBSOCKET)
                .set_header(CONNECTION, WEBSOCKET_CONNECTION_VALUE)
                .set_header(SEC_WEBSOCKET_ACCEPT, &accept_key)
                .set_body(Vec::new())
                .build();
            if stream.try_send(data).await.is_err() {
                stream.set_closed(true);
                return Status::Reject;
            }
        }
        Status::Continue
    }
}

impl WebsocketEchoRoute {
    async fn try_send_frame(&self, stream: &mut Stream, body: &[u8]) -> Result<(), ResponseError> {
        let frame_list: Vec<ResponseBody> = WebSocketFrame::create_frame_list(body);
        stream.try_send_list(&frame_list).await
    }
}

impl ServerHook for WebsocketEchoRoute {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, stream: &mut Stream, _: &mut Context) -> Status {
        while let Ok(body) = stream.try_get_websocket_request().await {
            let payload: Vec<u8> = body;
            if self.try_send_frame(stream, &payload).await.is_err() {
                stream.set_closed(true);
                return Status::Reject;
            }
        }
        Status::Continue
    }
}
