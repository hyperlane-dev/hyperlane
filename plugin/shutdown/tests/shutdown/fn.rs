use super::*;

struct OrderCallback {
    name: &'static str,
    log: Arc<Mutex<Vec<String>>>,
}

impl ShutdownCallback for OrderCallback {
    fn name(&self) -> &str {
        self.name
    }

    fn on_shutdown(&self) -> ShutdownFuture<'_> {
        let name: String = self.name.to_owned();
        let log: Arc<Mutex<Vec<String>>> = Arc::clone(&self.log);
        Box::pin(async move {
            if let Ok(mut guard) = log.lock() {
                guard.push(name);
            }
        })
    }
}

#[tokio::test]
async fn test_shutdown_drains_in_flight_guards_and_runs_lifo_callbacks() {
    let order: Arc<Mutex<Vec<String>>> = Arc::new(Mutex::new(Vec::new()));
    let coordinator: ShutdownCoordinator = ShutdownCoordinator::new(
        ShutdownConfig::default()
            .with_drain_timeout_ms(500)
            .with_callback_timeout_ms(500),
    )
    .with_callback(OrderCallback {
        name: "first-registered",
        log: Arc::clone(&order),
    })
    .with_callback(OrderCallback {
        name: "second-registered",
        log: Arc::clone(&order),
    });
    let guard: ShutdownGuard = coordinator.enter().unwrap();
    assert_eq!(coordinator.in_flight(), 1usize);
    let stopper: ShutdownCoordinator = coordinator.clone();
    let release: JoinHandle<()> = spawn(async move {
        sleep(Duration::from_millis(20)).await;
        drop(guard);
    });
    let report: ShutdownReport = stopper.execute_shutdown(None).await;
    release.await.unwrap();
    assert!(report.get_drained_cleanly());
    assert_eq!(report.get_remaining_in_flight(), 0usize);
    assert!(coordinator.enter().is_none());
    assert_eq!(
        report.get_completed_callbacks(),
        &vec![
            "second-registered".to_owned(),
            "first-registered".to_owned()
        ]
    );
    assert_eq!(
        *order.lock().unwrap(),
        vec![
            "second-registered".to_owned(),
            "first-registered".to_owned()
        ]
    );
}
