use super::*;

#[tokio::test]
async fn test_session_signed_cookie_dirty_tracking_regeneration_and_destroy() {
    let secret: Vec<u8> = b"session-hmac-secret-key".to_vec();
    let cfg: SessionConfig = SessionConfig::default()
        .with_signing_secret(secret.clone())
        .with_ttl_secs(120);
    let store: Arc<InMemorySessionStore> = Arc::new(InMemorySessionStore::new(128));
    let plugin: SessionPlugin = SessionPlugin {
        config: cfg,
        store: store.clone(),
    };
    let mut req1: Context = Context::default();
    let h1: SessionHandle = plugin.load_into_context(&mut req1).await;
    plugin.persist_from_context(&mut req1).await;
    assert_eq!(store.len(), 0usize);
    assert!(
        req1.get_response()
            .try_get_header_back(SET_COOKIE)
            .is_none()
    );
    h1.insert("user", "alice");
    let first_id: String = h1.id();
    plugin.persist_from_context(&mut req1).await;
    assert_eq!(store.len(), 1usize);
    let set_cookie1: String = req1.get_response().try_get_header_back(SET_COOKIE).unwrap();
    let raw_cookie_val: &str = set_cookie1
        .split(';')
        .next()
        .unwrap()
        .split_once('=')
        .unwrap()
        .1;
    assert_eq!(
        verify_session_cookie(raw_cookie_val, &secret).as_deref(),
        Some(first_id.as_str())
    );
    assert!(verify_session_cookie(&format!("{raw_cookie_val}0"), &secret).is_none());
    let mut req2: Context = Context::default();
    req2.get_mut_request().get_mut_headers().insert(
        COOKIE.to_owned(),
        VecDeque::from([format!("{DEFAULT_SESSION_COOKIE_NAME}={raw_cookie_val}")]),
    );
    let h2: SessionHandle = plugin.load_into_context(&mut req2).await;
    assert_eq!(h2.get("user").as_deref(), Some("alice"));
    assert!(!h2.is_dirty());
    let rotated_id: String = h2.regenerate_id();
    assert_ne!(rotated_id, first_id);
    plugin.persist_from_context(&mut req2).await;
    assert!(store.load(&first_id).await.is_none());
    assert!(store.load(&rotated_id).await.is_some());
    h2.destroy();
    plugin.persist_from_context(&mut req2).await;
    assert_eq!(store.len(), 0usize);
}
