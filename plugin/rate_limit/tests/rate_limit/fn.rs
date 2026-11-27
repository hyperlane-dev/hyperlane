use super::*;

#[test]
fn test_rate_limit_gcra_burst_and_multithreaded_exactness() {
    let limiter: Arc<ShardedRateLimiter> = Arc::new(ShardedRateLimiter::new(
        RateLimitPolicy::new(10, 10_000)
            .with_burst(20)
            .with_algorithm(RateLimitAlgorithm::Gcra),
    ));
    let allowed_count: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
    let mut threads: Vec<JoinHandle<()>> = Vec::with_capacity(8);
    for _ in 0..8usize {
        let lim: Arc<ShardedRateLimiter> = Arc::clone(&limiter);
        let counter: Arc<AtomicUsize> = Arc::clone(&allowed_count);
        threads.push(spawn(move || {
            for _ in 0..10usize {
                if lim.check("hot-client").get_allowed() {
                    counter.fetch_add(1, Ordering::Relaxed);
                }
            }
        }));
    }
    for handle in threads {
        handle.join().unwrap();
    }
    assert_eq!(allowed_count.load(Ordering::Relaxed), 20usize);
    let denied: RateLimitDecision = limiter.check("hot-client");
    assert!(!denied.get_allowed());
    assert_eq!(denied.get_remaining(), 0u64);
    assert!(denied.get_retry_after_secs() >= 1u64);
}

#[tokio::test]
async fn test_rate_limit_fixed_window_reset_and_max_keys_eviction() {
    let limiter: ShardedRateLimiter = ShardedRateLimiter::new(
        RateLimitPolicy::new(2, 40)
            .with_algorithm(RateLimitAlgorithm::FixedWindow)
            .with_max_keys(2),
    );
    assert!(limiter.check("k1").get_allowed());
    assert!(limiter.check("k1").get_allowed());
    assert!(!limiter.check("k1").get_allowed());
    assert!(limiter.check("k2").get_allowed());
    assert!(!limiter.check("k3").get_allowed());
    assert_eq!(limiter.tracked_keys(), 2usize);
    sleep(Duration::from_millis(55)).await;
    assert!(limiter.check("k1").get_allowed());
    assert!(limiter.check("k3").get_allowed());
    let evicted: usize = limiter.purge_expired();
    assert_eq!(evicted, 0usize);
}
