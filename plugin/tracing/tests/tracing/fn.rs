use super::*;

struct MemoryExporter {
    spans: Arc<Mutex<Vec<SpanRecord>>>,
}

impl SpanExporter for MemoryExporter {
    fn export(&self, span: &SpanRecord) {
        if let Ok(mut guard) = self.spans.lock() {
            guard.push(span.clone());
        }
    }
}

#[test]
fn test_tracing_w3c_parse_propagate_and_span_export() {
    let valid_tp: &str = "00-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01";
    let parsed: Option<(String, String, bool)> = parse_traceparent(valid_tp);
    assert_eq!(
        parsed,
        Some((
            "4bf92f3577b34da6a3ce929d0e0e4736".to_owned(),
            "00f067aa0ba902b7".to_owned(),
            true
        ))
    );
    assert!(parse_traceparent("00-00000000000000000000000000000000-00f067aa0ba902b7-01").is_none());
    assert!(parse_traceparent("ff-4bf92f3577b34da6a3ce929d0e0e4736-00f067aa0ba902b7-01").is_none());
    let spans: Arc<Mutex<Vec<SpanRecord>>> = Arc::new(Mutex::new(Vec::new()));
    let plugin: TracingPlugin = TracingPlugin {
        config: TracingConfig::default(),
        exporters: Arc::new(vec![Arc::new(MemoryExporter {
            spans: Arc::clone(&spans),
        })]),
    };
    let mut ctx: Context = Context::default();
    ctx.get_mut_request()
        .set_method(Method::Get)
        .set_path("/orders".to_owned());
    ctx.get_mut_request().get_mut_headers().insert(
        HEADER_TRACEPARENT.to_owned(),
        VecDeque::from([valid_tp.to_owned()]),
    );
    let started: TraceContext = plugin.start_request(&mut ctx);
    assert_eq!(started.get_trace_id(), "4bf92f3577b34da6a3ce929d0e0e4736");
    assert_eq!(
        started.get_parent_span_id().as_deref(),
        Some("00f067aa0ba902b7")
    );
    assert_ne!(started.get_span_id(), "00f067aa0ba902b7");
    assert_eq!(
        TracingPlugin::current(&ctx).unwrap().get_trace_id(),
        "4bf92f3577b34da6a3ce929d0e0e4736"
    );
    ctx.get_mut_response().set_status_code(200);
    plugin.finish_request(&mut ctx);
    assert!(
        ctx.get_response()
            .try_get_header_back(SERVER_TIMING)
            .is_some()
    );
    assert!(
        ctx.get_response()
            .try_get_header_back(X_REQUEST_ID)
            .is_some()
    );
    let recorded: Vec<SpanRecord> = spans.lock().unwrap().clone();
    assert_eq!(recorded.len(), 1usize);
    assert_eq!(recorded[0].get_path(), "/orders");
    assert_eq!(recorded[0].get_status_code(), 200usize);
}
