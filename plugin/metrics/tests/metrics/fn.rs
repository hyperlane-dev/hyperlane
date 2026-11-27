use super::*;

#[test]
fn test_metrics_lock_free_concurrency_and_prometheus_exposition() {
    let registry: MetricsRegistry = MetricsRegistry::new("svc");
    let custom_counter: Arc<Counter> =
        registry.register_counter("svc_jobs_completed_total", "Completed background jobs.");
    let custom_gauge: Arc<Gauge> = registry.register_gauge("svc_queue_depth", "Pending jobs.");
    let custom_hist: Arc<Histogram> =
        registry.register_histogram("svc_job_seconds", "Job duration.", &[0.01, 0.1, 1.0]);
    let mut handles: Vec<JoinHandle<()>> = Vec::with_capacity(8);
    for _ in 0..8usize {
        let reg: MetricsRegistry = registry.clone();
        let ctr: Arc<Counter> = Arc::clone(&custom_counter);
        let hist: Arc<Histogram> = Arc::clone(&custom_hist);
        handles.push(spawn(move || {
            for _ in 0..100usize {
                reg.record_request_start();
                reg.record_request_finish(200, 128, 0.004);
                ctr.inc();
                hist.observe(0.05);
            }
        }));
    }
    for handle in handles {
        handle.join().unwrap();
    }
    custom_gauge.set(42);
    assert_eq!(registry.total_http_requests(), 800u64);
    assert_eq!(custom_counter.get(), 800u64);
    assert_eq!(custom_hist.count(), 800u64);
    let text: String = registry.render_prometheus();
    assert!(text.contains("svc_http_requests_total{status_class=\"2xx\"} 800"));
    assert!(text.contains("svc_http_requests_in_flight 0"));
    assert!(text.contains("svc_http_response_bytes_total 102400"));
    assert!(text.contains("svc_jobs_completed_total 800"));
    assert!(text.contains("svc_queue_depth 42"));
    assert!(text.contains("svc_job_seconds_bucket{le=\"0.1\"} 800"));
    assert!(text.contains("svc_job_seconds_count 800"));
}
