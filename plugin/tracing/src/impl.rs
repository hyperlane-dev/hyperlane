use super::*;

/// Inherent implementation of [`TraceContext`].
impl TraceContext {
    /// Formats this context as a canonical W3C `traceparent` header value.
    ///
    /// # Returns
    ///
    /// - `String` - Canonical `00-<trace_id>-<span_id>-<flags>` value.
    pub fn to_traceparent(&self) -> String {
        format_traceparent(self.get_trace_id(), self.get_span_id(), self.get_sampled())
    }
}

/// Default construction for [`TracingConfig`].
impl Default for TracingConfig {
    /// Constructs a default [`TracingConfig`] emitting `traceparent`, `X-Request-Id`, and `Server-Timing`.
    fn default() -> Self {
        Self {
            emit_traceparent: true,
            emit_request_id: true,
            emit_server_timing: true,
            server_timing_label: DEFAULT_SERVER_TIMING_LABEL.to_owned(),
            default_sampled: true,
        }
    }
}

/// Inherent builders for [`TracingConfig`].
impl TracingConfig {
    /// Enables or disables emitting `Server-Timing` response headers.
    ///
    /// # Arguments
    ///
    /// - `bool` - Whether to emit `Server-Timing`.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_server_timing(mut self, enabled: bool) -> Self {
        self.set_emit_server_timing(enabled);
        self
    }

    /// Configures the default sampled flag for newly originated traces.
    ///
    /// # Arguments
    ///
    /// - `bool` - Default sampled flag.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_default_sampled(mut self, sampled: bool) -> Self {
        self.set_default_sampled(sampled);
        self
    }
}

/// Inherent implementation of [`TracingPlugin`].
impl TracingPlugin {
    /// Initializes the global [`TracingPlugin`] once with `config` and optional `exporters`.
    ///
    /// # Arguments
    ///
    /// - `TracingConfig` - Header propagation and timing configuration.
    /// - `Vec<SharedSpanExporter>` - Span exporters invoked after response handling.
    ///
    /// # Returns
    ///
    /// - `&'static Self` - Reference to the initialized global plugin.
    pub fn init(config: TracingConfig, exporters: Vec<SharedSpanExporter>) -> &'static Self {
        TRACING_PLUGIN.get_or_init(|| Self {
            config,
            exporters: Arc::new(exporters),
        })
    }

    /// Initializes the global [`TracingPlugin`] once with default configuration and no exporters.
    ///
    /// # Returns
    ///
    /// - `&'static Self` - Reference to the initialized global plugin.
    pub fn init_default() -> &'static Self {
        Self::init(TracingConfig::default(), Vec::new())
    }

    /// Returns the initialized global [`TracingPlugin`], if initialized.
    ///
    /// # Returns
    ///
    /// - `Option<&'static Self>` - Reference to the global plugin, or `None`.
    pub fn get() -> Option<&'static Self> {
        TRACING_PLUGIN.get()
    }

    /// Returns the active [`TracingConfig`].
    ///
    /// # Returns
    ///
    /// - `&TracingConfig` - Active tracing configuration.
    pub fn get_config(&self) -> &TracingConfig {
        &self.config
    }

    /// Retrieves the active [`TraceContext`] stored in `ctx`.
    ///
    /// # Arguments
    ///
    /// - `&Context` - Request context.
    ///
    /// # Returns
    ///
    /// - `Option<TraceContext>` - Active trace context, if tracing ran on this request.
    pub fn current(ctx: &Context) -> Option<TraceContext> {
        ctx.try_get_attribute(ATTR_TRACE_CONTEXT)
    }

    /// Starts tracing on `ctx`, parsing or originating W3C Trace Context and seeding response headers.
    ///
    /// # Arguments
    ///
    /// - `&mut Context` - Incoming request context.
    ///
    /// # Returns
    ///
    /// - `TraceContext` - Established trace context.
    pub fn start_request(&self, ctx: &mut Context) -> TraceContext {
        let incoming_tp: Option<String> = ctx.get_request().try_get_header_back(HEADER_TRACEPARENT);
        let tracestate: Option<String> = ctx.get_request().try_get_header_back(HEADER_TRACESTATE);
        let incoming_req_id: Option<String> = ctx.get_request().try_get_header_back(X_REQUEST_ID);
        let (trace_id, parent_span_id, sampled): (String, Option<String>, bool) =
            match incoming_tp.as_deref().and_then(parse_traceparent) {
                Some((tid, pid, is_sampled)) => (tid, Some(pid), is_sampled),
                None => (
                    generate_trace_id(),
                    None,
                    self.get_config().get_default_sampled(),
                ),
            };
        let span_id: String = generate_span_id();
        let request_id: String = incoming_req_id
            .filter(|id: &String| !id.trim().is_empty())
            .unwrap_or_else(|| trace_id.clone());
        let trace_ctx: TraceContext = TraceContext {
            trace_id,
            span_id,
            parent_span_id,
            request_id,
            sampled,
            tracestate,
        };
        if self.get_config().get_emit_traceparent() {
            ctx.get_mut_response()
                .set_header(HEADER_TRACEPARENT, trace_ctx.to_traceparent());
        }
        if self.get_config().get_emit_request_id() {
            ctx.get_mut_response()
                .set_header(X_REQUEST_ID, trace_ctx.get_request_id());
        }
        ctx.set_attribute(ATTR_TRACE_CONTEXT, trace_ctx.clone());
        ctx.set_attribute(ATTR_TRACE_STARTED_AT, Instant::now());
        trace_ctx
    }

    /// Finalizes tracing on `ctx`, setting `Server-Timing` and exporting a [`SpanRecord`].
    ///
    /// # Arguments
    ///
    /// - `&mut Context` - Active request/response context.
    pub fn finish_request(&self, ctx: &mut Context) {
        let Self {
            config, exporters, ..
        } = self;
        let Some(trace_ctx) = ctx.try_get_attribute::<TraceContext, _>(ATTR_TRACE_CONTEXT) else {
            return;
        };
        let duration_us: u64 = ctx
            .try_get_attribute::<Instant, _>(ATTR_TRACE_STARTED_AT)
            .map(|started: Instant| started.elapsed().as_micros().min(u64::MAX as u128) as u64)
            .unwrap_or(0);
        if config.get_emit_traceparent()
            && ctx
                .get_response()
                .try_get_header_back(HEADER_TRACEPARENT)
                .is_none()
        {
            ctx.get_mut_response()
                .set_header(HEADER_TRACEPARENT, trace_ctx.to_traceparent());
        }
        if config.get_emit_request_id()
            && ctx
                .get_response()
                .try_get_header_back(X_REQUEST_ID)
                .is_none()
        {
            ctx.get_mut_response()
                .set_header(X_REQUEST_ID, trace_ctx.get_request_id());
        }
        if config.get_emit_server_timing() {
            let dur_ms: f64 = (duration_us as f64) / 1_000.0;
            let header_val: String =
                format!("{};dur={dur_ms:.2}", config.get_server_timing_label());
            ctx.get_mut_response().set_header(SERVER_TIMING, header_val);
        }
        if !exporters.is_empty() {
            let record: SpanRecord = SpanRecord {
                trace_id: trace_ctx.get_trace_id().clone(),
                span_id: trace_ctx.get_span_id().clone(),
                parent_span_id: trace_ctx.get_parent_span_id(),
                request_id: trace_ctx.get_request_id().clone(),
                method: ctx.get_request().get_method().to_string(),
                path: ctx.get_request().get_path().clone(),
                status_code: ctx.get_response().get_status_code(),
                duration_us,
                sampled: trace_ctx.get_sampled(),
            };
            for exporter in exporters.iter() {
                exporter.export(&record);
            }
        }
    }
}

/// Implements [`ServerHook`] for [`TracingRequestHook`].
impl ServerHook for TracingRequestHook {
    /// Constructs a per-request [`TracingRequestHook`].
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - Connection stream.
    /// - `&mut Context` - Request context.
    ///
    /// # Returns
    ///
    /// - `Self` - Hook instance.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    /// Initializes W3C Trace Context and correlation headers at the very start of the pipeline.
    ///
    /// # Arguments
    ///
    /// - `Self` - Hook instance.
    /// - `&mut Stream` - Connection stream.
    /// - `&mut Context` - Request context.
    ///
    /// # Returns
    ///
    /// - `Status` - Always `Status::Continue`.
    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        let Some(plugin) = TracingPlugin::get() else {
            return Status::Continue;
        };
        let _: TraceContext = plugin.start_request(ctx);
        Status::Continue
    }
}

/// Implements [`ServerHook`] for [`TracingResponseHook`].
impl ServerHook for TracingResponseHook {
    /// Constructs a per-response [`TracingResponseHook`].
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - Connection stream.
    /// - `&mut Context` - Request context.
    ///
    /// # Returns
    ///
    /// - `Self` - Hook instance.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    /// Records `Server-Timing` and exports the completed request span.
    ///
    /// # Arguments
    ///
    /// - `Self` - Hook instance.
    /// - `&mut Stream` - Connection stream.
    /// - `&mut Context` - Request context.
    ///
    /// # Returns
    ///
    /// - `Status` - Always `Status::Continue`.
    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        let Some(plugin) = TracingPlugin::get() else {
            return Status::Continue;
        };
        plugin.finish_request(ctx);
        Status::Continue
    }
}
