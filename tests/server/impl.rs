use super::*;

impl ServerHook for TestSendRoute {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, _: &mut Stream, _: &mut Context) -> Status {
        Status::Continue
    }
}

impl ServerHook for TaskPanicHook {
    async fn new(_: &mut Stream, ctx: &mut Context) -> Self {
        let error: PanicData = ctx.try_get_task_panic_data().unwrap_or_default();
        let response_body: String = error.to_string();
        let content_type: String = ContentType::format_content_type_with_charset(TEXT_PLAIN, UTF8);
        Self {
            response_body,
            content_type,
        }
    }

    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        let data: Vec<u8> = ctx
            .get_mut_response()
            .set_version(HttpVersion::Http1_1)
            .set_status_code(500)
            .clear_headers()
            .set_header(SERVER, HYPERLANE)
            .set_header(CONTENT_TYPE, &self.content_type)
            .set_body(&self.response_body)
            .build();
        if stream.try_send(data).await.is_err() {
            stream.set_closed(true);
            return Status::Reject;
        }
        Status::Continue
    }
}

impl ServerHook for RequestErrorHook {
    async fn new(_: &mut Stream, ctx: &mut Context) -> Self {
        let request_error: RequestError = ctx.try_get_request_error_data().unwrap_or_default();
        Self {
            response_status_code: request_error.get_http_status_code(),
            response_body: request_error.to_string(),
        }
    }

    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        let data: Vec<u8> = ctx
            .get_mut_response()
            .set_version(HttpVersion::Http1_1)
            .set_status_code(self.response_status_code)
            .set_body(self.response_body)
            .build();
        if stream.try_send(data).await.is_err() {
            stream.set_closed(true);
            return Status::Reject;
        }
        Status::Continue
    }
}

impl ServerHook for RequestMiddleware {
    async fn new(stream: &mut Stream, _: &mut Context) -> Self {
        let mut socket_addr: String = String::new();

        socket_addr = stream
            .get_stream()
            .peer_addr()
            .map(|data| data.to_string())
            .unwrap_or_default();

        Self { socket_addr }
    }

    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        ctx.get_mut_response()
            .set_version(HttpVersion::Http1_1)
            .set_status_code(200)
            .set_header(SERVER, HYPERLANE)
            .set_header(CONNECTION, KEEP_ALIVE)
            .set_header(CONTENT_TYPE, TEXT_PLAIN)
            .set_header(ACCESS_CONTROL_ALLOW_ORIGIN, WILDCARD_ANY)
            .set_header("SocketAddr", &self.socket_addr);
        Status::Continue
    }
}

impl ServerHook for UpgradeMiddleware {
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
                .set_header(CONNECTION, UPGRADE)
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

impl ServerHook for ResponseMiddleware {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        if ctx.get_request().is_ws_upgrade_type() {
            return Status::Continue;
        }
        let data: Vec<u8> = ctx.get_mut_response().build();
        if stream.try_send(data).await.is_err() {
            stream.set_closed(true);
            return Status::Reject;
        }
        Status::Continue
    }
}

impl ServerHook for RootRoute {
    async fn new(_: &mut Stream, ctx: &mut Context) -> Self {
        let response_body: String = format!("Hello hyperlane => {}", ctx.get_request().get_path());
        let cookie1: String = CookieBuilder::new("key1", "value1").http_only().build();
        let cookie2: String = CookieBuilder::new("key2", "value2").http_only().build();
        Self {
            response_body,
            cookie1,
            cookie2,
        }
    }

    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        ctx.get_mut_response()
            .add_header(SET_COOKIE, &self.cookie1)
            .add_header(SET_COOKIE, &self.cookie2)
            .set_body(&self.response_body);
        Status::Continue
    }
}

impl ServerHook for SseRoute {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        let data: Vec<u8> = ctx
            .get_mut_response()
            .set_header(CONTENT_TYPE, TEXT_EVENT_STREAM)
            .set_body(Vec::new())
            .build();
        if stream.try_send(data).await.is_err() {
            stream.set_closed(true);
            return Status::Reject;
        }
        for i in 0..10 {
            let body: String = format!("data:{i}{HTTP_DOUBLE_BR}");
            if stream.try_send(&body).await.is_err() {
                break;
            }
        }
        stream.set_closed(true);
        Status::Reject
    }
}

impl WebsocketRoute {
    pub async fn try_send_body_hook(
        &self,
        stream: &mut Stream,
        ctx: &mut Context,
    ) -> Result<(), ResponseError> {
        let send_result: Result<(), ResponseError> = if ctx.get_request().is_ws_upgrade_type() {
            let body: &ResponseBody = ctx.get_response().get_body();
            let frame_list: Vec<ResponseBody> = WebSocketFrame::create_frame_list(body);
            stream.try_send_list(&frame_list).await
        } else {
            let body: &Vec<u8> = ctx.get_response().get_body();
            stream.try_send(body).await
        };
        if send_result.is_err() {
            stream.set_closed(true);
        }
        send_result
    }
}

impl ServerHook for WebsocketRoute {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        while let Ok(body) = stream.try_get_websocket_request().await {
            ctx.get_mut_response().set_body(body);
            if self.try_send_body_hook(stream, ctx).await.is_err() {
                return Status::Reject;
            }
        }
        Status::Continue
    }
}

impl ServerHook for DynamicRoute {
    async fn new(_: &mut Stream, ctx: &mut Context) -> Self {
        Self {
            params: ctx.get_route_params().clone(),
        }
    }

    async fn handle(mut self, _: &mut Stream, _: &mut Context) -> Status {
        self.params.insert("key".to_owned(), "value".to_owned());
        panic!("Test panic {:?}", self.params);
    }
}

impl ServerHook for GetAllRoutes {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        if let Some(server) = SERVER_REF.get() {
            let route_matcher: &RouteMatcher = server.get_route_matcher();
            let mut response_body: String = String::new();
            for key in route_matcher.get_static_route().keys() {
                response_body.push_str(&format!("Static route: {key}\n"));
            }
            for value in route_matcher.get_dynamic_route().values() {
                for (route_pattern, _) in value {
                    response_body.push_str(&format!("Dynamic route: {route_pattern}\n"));
                }
            }
            for value in route_matcher.get_regex_route().values() {
                for (route_pattern, _) in value {
                    response_body.push_str(&format!("Regex route: {route_pattern}\n"));
                }
            }
            ctx.get_mut_response().set_body(&response_body);
        }
        Status::Continue
    }
}
