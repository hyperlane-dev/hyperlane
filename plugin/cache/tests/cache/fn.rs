use super::*;

#[tokio::test]
async fn test_cache_singleflight_coalescing_ttl_and_byte_budget() {
    let cache: ShardedMemoryCache = ShardedMemoryCache::new(
        CacheConfig::default()
            .with_default_ttl_ms(200)
            .with_max_entries(4)
            .with_max_bytes(512),
    );
    let origin_calls: Arc<AtomicUsize> = Arc::new(AtomicUsize::new(0));
    let mut tasks: Vec<JoinHandle<Option<CachedValue>>> = Vec::with_capacity(16);
    for _ in 0..16usize {
        let c: ShardedMemoryCache = cache.clone();
        let calls: Arc<AtomicUsize> = Arc::clone(&origin_calls);
        tasks.push(spawn(async move {
            c.get_or_load("hot-item", 500, || async move {
                calls.fetch_add(1, Ordering::Relaxed);
                sleep(Duration::from_millis(25)).await;
                Some(CachedValue::bytes(b"computed-once".to_vec()))
            })
            .await
        }));
    }
    for task in tasks {
        let res: CachedValue = task.await.unwrap().unwrap();
        assert_eq!(res.get_body().as_slice(), b"computed-once");
    }
    assert_eq!(origin_calls.load(Ordering::Relaxed), 1usize);
    assert_eq!(cache.len(), 1usize);
    cache.put_sync(
        "short-lived".to_owned(),
        CachedValue::bytes(b"tmp".to_vec()),
        25,
    );
    assert!(cache.get_sync("short-lived").is_some());
    sleep(Duration::from_millis(40)).await;
    assert!(cache.get_sync("short-lived").is_none());
    assert!(cache.remove("hot-item").await);
    assert!(cache.is_empty());
}
