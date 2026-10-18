use super::*;

impl ServerHook for ProbeMiddleware {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        let request: &Request = ctx.get_request();
        let path: String = request.get_path().clone();
        let method: String = request.get_method().to_string();
        let query: String = joined_query(request, PROBE_QUERY_KEYS);
        let header: String = joined_header(request, PROBE_HEADER_KEY);
        let version: String = format!("{:?}", request.get_version());
        ctx.get_mut_response()
            .set_status_code(200)
            .set_header(CONTENT_TYPE, TEXT_PLAIN)
            .set_header(PROBE_METHOD_HEADER, &method)
            .set_header(PROBE_PATH_HEADER, &path)
            .set_header(PROBE_QUERY_HEADER, &query)
            .set_header(PROBE_HEADER_ECHO_HEADER, &header)
            .set_header(PROBE_VERSION_HEADER, &version)
            .set_body(String::new());
        Status::Continue
    }
}

impl ServerHook for ProbeResponseMiddleware {
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

impl ServerHook for EchoRoute {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        let body: Vec<u8> = ctx.get_request().get_body().clone();
        let version: String = format!("{:?}", ctx.get_request().get_version());
        let payload: Vec<u8> = body
            .iter()
            .map(|value: &u8| value.wrapping_add(1))
            .collect::<Vec<u8>>();
        ctx.get_mut_response()
            .set_status_code(200)
            .set_header(CONTENT_TYPE, APPLICATION_OCTET_STREAM)
            .set_header("x-request-version", &version)
            .set_body(&payload);
        Status::Continue
    }
}

impl ServerHook for GatedSlowRoute {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        let gate: &RouteGate = route_gate(GateSlot::SlowHttp2);
        gate.mark_slow_entered();
        gate.await_slow_release().await;
        ctx.get_mut_response()
            .set_status_code(200)
            .set_header(CONTENT_TYPE, TEXT_PLAIN)
            .set_body(SLOW_DONE_BODY);
        Status::Continue
    }
}

impl ServerHook for GatedSlowRouteHttp3 {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        let gate: &RouteGate = route_gate(GateSlot::SlowHttp3);
        gate.mark_slow_entered();
        gate.await_slow_release().await;
        ctx.get_mut_response()
            .set_status_code(200)
            .set_header(CONTENT_TYPE, TEXT_PLAIN)
            .set_body(SLOW_DONE_BODY);
        Status::Continue
    }
}

impl ServerHook for SplitWriteRoute {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        let data: Vec<u8> = ctx
            .get_mut_response()
            .set_status_code(200)
            .set_header(CONTENT_TYPE, TEXT_EVENT_STREAM)
            .set_header(CACHE_CONTROL, NO_CACHE)
            .set_body(Vec::new())
            .build();
        if stream.try_send(data).await.is_err() {
            stream.set_closed(true);
            return Status::Reject;
        }
        for piece in fragment_writes(SPLIT_HEAD_BODY.as_bytes()) {
            if stream.try_send(piece).await.is_err() {
                stream.set_closed(true);
                return Status::Reject;
            }
        }
        if stream.try_send(SPLIT_TAIL_BODY).await.is_err() {
            stream.set_closed(true);
            return Status::Reject;
        }
        stream.set_closed(true);
        Status::Reject
    }
}

impl ServerHook for DuplicateCookieRoute {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        let first: String = CookieBuilder::new(FIRST_COOKIE_NAME, FIRST_COOKIE_VALUE)
            .http_only()
            .build();
        let second: String = CookieBuilder::new(SECOND_COOKIE_NAME, SECOND_COOKIE_VALUE)
            .http_only()
            .build();
        ctx.get_mut_response()
            .set_status_code(200)
            .set_header(CONTENT_TYPE, TEXT_PLAIN)
            .add_header(SET_COOKIE, &first)
            .add_header(SET_COOKIE, &second)
            .set_header(CONNECTION, KEEP_ALIVE)
            .set_body(COOKIE_SENT_BODY);
        Status::Continue
    }
}

impl ServerHook for LargeResponseRoute {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        let payload: Vec<u8> = large_binary_payload();
        ctx.get_mut_response()
            .set_status_code(200)
            .set_header(CONTENT_TYPE, APPLICATION_OCTET_STREAM)
            .set_body(&payload);
        Status::Continue
    }
}

impl ServerHook for BinaryEchoRoute {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        let body: Vec<u8> = ctx.get_request().get_body().clone();
        ctx.get_mut_response()
            .set_status_code(200)
            .set_header(CONTENT_TYPE, APPLICATION_OCTET_STREAM)
            .set_body(&body);
        Status::Continue
    }
}

impl ServerHook for GatedSseStreamRoute {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        let data: Vec<u8> = ctx
            .get_mut_response()
            .set_status_code(200)
            .set_header(CONTENT_TYPE, TEXT_EVENT_STREAM)
            .set_header(CACHE_CONTROL, NO_CACHE)
            .set_body(Vec::new())
            .build();
        if stream.try_send(data).await.is_err() {
            stream.set_closed(true);
            return Status::Reject;
        }
        if stream.try_send(SSE_FIRST_CHUNK).await.is_err() {
            stream.set_closed(true);
            return Status::Reject;
        }
        let gate: &RouteGate = route_gate(GateSlot::SseHttp1);
        gate.mark_sse_first_chunk_sent();
        gate.await_sse_release().await;
        if stream.try_send(SSE_SECOND_CHUNK).await.is_err() {
            stream.set_closed(true);
            return Status::Reject;
        }
        stream.set_closed(true);
        Status::Reject
    }
}

impl ServerHook for GatedSseStreamRouteHttp2 {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        let data: Vec<u8> = ctx
            .get_mut_response()
            .set_status_code(200)
            .set_header(CONTENT_TYPE, TEXT_EVENT_STREAM)
            .set_header(CACHE_CONTROL, NO_CACHE)
            .set_body(Vec::new())
            .build();
        if stream.try_send(data).await.is_err() {
            stream.set_closed(true);
            return Status::Reject;
        }
        if stream.try_send(SSE_FIRST_CHUNK).await.is_err() {
            stream.set_closed(true);
            return Status::Reject;
        }
        let gate: &RouteGate = route_gate(GateSlot::SseHttp2);
        gate.mark_sse_first_chunk_sent();
        gate.await_sse_release().await;
        if stream.try_send(SSE_SECOND_CHUNK).await.is_err() {
            stream.set_closed(true);
            return Status::Reject;
        }
        stream.set_closed(true);
        Status::Reject
    }
}

impl ServerHook for GatedSseStreamRouteHttp3 {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        let data: Vec<u8> = ctx
            .get_mut_response()
            .set_status_code(200)
            .set_header(CONTENT_TYPE, TEXT_EVENT_STREAM)
            .set_header(CACHE_CONTROL, NO_CACHE)
            .set_body(Vec::new())
            .build();
        if stream.try_send(data).await.is_err() {
            stream.set_closed(true);
            return Status::Reject;
        }
        if stream.try_send(SSE_FIRST_CHUNK).await.is_err() {
            stream.set_closed(true);
            return Status::Reject;
        }
        let gate: &RouteGate = route_gate(GateSlot::SseHttp3);
        gate.mark_sse_first_chunk_sent();
        gate.await_sse_release().await;
        if stream.try_send(SSE_SECOND_CHUNK).await.is_err() {
            stream.set_closed(true);
            return Status::Reject;
        }
        stream.set_closed(true);
        Status::Reject
    }
}

impl ServerHook for VersionRoute {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        let version: String = format!("{:?}", ctx.get_request().get_version());
        ctx.get_mut_response()
            .set_status_code(200)
            .set_header(CONTENT_TYPE, TEXT_PLAIN)
            .set_body(&version);
        Status::Continue
    }
}

impl ServerHook for HopByHopStrippingMiddleware {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        let named: String = format!("{INTERNAL_CONNECTION_HEADER}, {KEEP_ALIVE}");
        ctx.get_mut_response()
            .set_status_code(200)
            .set_header(CONTENT_TYPE, TEXT_PLAIN)
            .set_header(CONNECTION, &named)
            .set_header(INTERNAL_CONNECTION_HEADER, INTERNAL_CONNECTION_VALUE)
            .set_header(TRANSFER_ENCODING, CHUNKED)
            .set_header(UPGRADE, "h2c")
            .set_header(PRESERVED_HEADER_KEY, PRESERVED_HEADER_VALUE)
            .set_body(HOP_BY_HOP_SENT_BODY);
        Status::Continue
    }
}

impl ServerHook for HopByHopStrippingRoute {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        let named: String = format!("{INTERNAL_CONNECTION_HEADER}, {KEEP_ALIVE}");
        ctx.get_mut_response()
            .set_status_code(200)
            .set_header(CONTENT_TYPE, TEXT_PLAIN)
            .set_header(CONNECTION, &named)
            .set_header(KEEP_ALIVE, "timeout=5")
            .set_header(INTERNAL_CONNECTION_HEADER, INTERNAL_CONNECTION_VALUE)
            .set_header(UPGRADE, "h2c")
            .set_header(TE, "trailers")
            .set_header(PRESERVED_HEADER_KEY, PRESERVED_HEADER_VALUE)
            .set_body(HOP_BY_HOP_SENT_BODY);
        Status::Continue
    }
}

impl ServerHook for SocketMetadataRoute {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        let local: SocketAddr = stream
            .get_local_addr()
            .expect("a serving stream must report its local address");
        let peer: SocketAddr = stream
            .get_peer_addr()
            .expect("a serving stream must report its peer address");
        let body: String = format!("{local}|{peer}");
        ctx.get_mut_response()
            .set_status_code(200)
            .set_header(CONTENT_TYPE, TEXT_PLAIN)
            .set_body(&body);
        Status::Continue
    }
}

impl ServerHook for UnframedBodyRoute {
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        let data: Vec<u8> = ctx
            .get_mut_response()
            .set_status_code(200)
            .set_header(CONTENT_TYPE, TEXT_EVENT_STREAM)
            .set_body(Vec::new())
            .build();
        if stream.try_send(data).await.is_err() {
            stream.set_closed(true);
            return Status::Reject;
        }
        for piece in fragment_writes(UNFRAMED_BODY.as_bytes()) {
            if stream.try_send(piece).await.is_err() {
                stream.set_closed(true);
                return Status::Reject;
            }
        }
        stream.set_closed(true);
        Status::Reject
    }
}
