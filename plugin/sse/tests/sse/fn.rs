use super::*;

#[test]
fn test_sse_event_encode_sanitizes_crlf_and_splits_multiline_data() {
    let event: SseEvent = SseEvent::new("line1\r\nline2\rline3")
        .with_id("42\r\ndata: injected")
        .with_event("update\nretry: 1")
        .with_retry(2500);
    let encoded: String = String::from_utf8(event.encode()).unwrap();
    assert!(encoded.contains("id: 42data: injected\n"));
    assert!(encoded.contains("event: updateretry: 1\n"));
    assert!(encoded.contains("retry: 2500\n"));
    assert!(encoded.contains("data: line1\ndata: line2\ndata: line3\n\n"));
    let comment: SseEvent = SseEvent::comment_only("ping\r\nevent: fake");
    let comment_wire: String = String::from_utf8(comment.encode()).unwrap();
    assert_eq!(comment_wire, ": pingevent: fake\n\n");
}

#[tokio::test]
async fn test_sse_hub_pubsub_replay_topic_limit_and_cleanup() {
    let mut cfg: SseConfig = SseConfig::default();
    cfg.set_max_topics(2).set_replay_capacity(4);
    let hub: SseHub = SseHub::new(cfg);
    let seq1: u64 = hub.publish("orders", SseEvent::new("first")).unwrap();
    let seq2: u64 = hub.publish("orders", SseEvent::new("second")).unwrap();
    assert_eq!(seq2, seq1 + 1);
    let mut resumed: SseSubscription = hub.subscribe("orders", Some(&seq1.to_string())).unwrap();
    let replayed: SseEvent = resumed.recv().await.unwrap();
    assert_eq!(replayed.get_data(), "second");
    assert_eq!(hub.subscriber_count("orders"), 1usize);
    let _: u64 = hub.publish("orders", SseEvent::new("live")).unwrap();
    let live: SseEvent = resumed.recv().await.unwrap();
    assert_eq!(live.get_data(), "live");
    let ephem: SseSubscription = hub.subscribe("ephemeral", None).unwrap();
    assert_eq!(hub.topic_count(), 2usize);
    assert_eq!(
        hub.publish("overflow", SseEvent::new("blocked")),
        Err(SseError::TopicLimitReached)
    );
    drop(ephem);
    assert_eq!(hub.topic_count(), 1usize);
    let mut ctx: Context = Context::default();
    SseStream::prepare(&mut ctx);
    assert_eq!(
        ctx.get_response()
            .try_get_header_back(CONTENT_TYPE)
            .as_deref(),
        Some(SSE_CONTENT_TYPE)
    );
    assert_eq!(
        ctx.get_response()
            .try_get_header_back(CACHE_CONTROL)
            .as_deref(),
        Some(SSE_CACHE_CONTROL)
    );
}
