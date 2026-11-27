use super::*;

struct StaticCheck {
    name: &'static str,
    outcome: CheckOutcome,
    delay_ms: u64,
    calls: Arc<AtomicUsize>,
}

impl HealthCheck for StaticCheck {
    fn name(&self) -> &str {
        self.name
    }

    fn check(&self) -> HealthFuture<'_> {
        let outcome: CheckOutcome = self.outcome.clone();
        let delay_ms: u64 = self.delay_ms;
        let calls: Arc<AtomicUsize> = Arc::clone(&self.calls);
        Box::pin(async move {
            calls.fetch_add(1, Ordering::Relaxed);
            if delay_ms > 0 {
                sleep(Duration::from_millis(delay_ms)).await;
            }
            outcome
        })
    }
}

#[tokio::test]
async fn test_health_concurrent_probes_cache_timeout_and_draining() {
    let db_calls: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
    let registry: HealthRegistry = HealthRegistry::new()
        .with_check(StaticCheck {
            name: "postgres",
            outcome: CheckOutcome::up(),
            delay_ms: 5,
            calls: Arc::clone(&db_calls),
        })
        .with_check(StaticCheck {
            name: "cache",
            outcome: CheckOutcome::degraded("high latency"),
            delay_ms: 5,
            calls: Arc::new(AtomicUsize::new(0)),
        });
    let report1: HealthReport = registry.evaluate(500, 1_000).await;
    assert_eq!(report1.get_status(), HealthStatus::Degraded);
    assert!(!report1.get_draining());
    assert_eq!(report1.get_checks().len(), 2usize);
    assert_eq!(db_calls.load(Ordering::Relaxed), 1usize);
    let report2: HealthReport = registry.evaluate(500, 1_000).await;
    assert_eq!(report2.get_status(), HealthStatus::Degraded);
    assert_eq!(db_calls.load(Ordering::Relaxed), 1usize);
    registry.mark_draining();
    assert_eq!(registry.liveness_report().get_status(), HealthStatus::Up);
    assert!(registry.liveness_report().get_draining());
    let draining_report: HealthReport = registry.evaluate(500, 1_000).await;
    assert_eq!(draining_report.get_status(), HealthStatus::Down);
    assert!(draining_report.get_draining());
    registry.clear_draining();
    let slow_registry: HealthRegistry = HealthRegistry::new().with_check(StaticCheck {
        name: "slow_dep",
        outcome: CheckOutcome::up(),
        delay_ms: 200,
        calls: Arc::new(AtomicUsize::new(0)),
    });
    let timed_out: HealthReport = slow_registry.evaluate(20, 0).await;
    assert_eq!(timed_out.get_status(), HealthStatus::Down);
    assert_eq!(
        timed_out.get_checks()[0].get_detail().as_deref(),
        Some("probe timed out")
    );
}
