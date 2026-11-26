use super::*;

#[tokio::test]
async fn test_mq_consumer_groups_ack_nack_dlq_and_backpressure() {
    let mut cfg: MqConfig = MqConfig::default();
    cfg.set_max_queue_len(2)
        .set_default_max_retries(2)
        .set_visibility_timeout_ms(50);
    let broker: InMemoryBroker = InMemoryBroker::new(cfg);
    broker.subscribe_group("orders", "billing").await.unwrap();
    broker.subscribe_group("orders", "analytics").await.unwrap();
    let msg1: MqMessage = MqMessage::new("orders", b"order-1".to_vec())
        .with_key("k1")
        .with_header("trace", "t1");
    let msg2: MqMessage = MqMessage::new("orders", b"order-2".to_vec());
    let msg3: MqMessage = MqMessage::new("orders", b"order-3".to_vec());
    let id1: u64 = broker.publish(msg1).await.unwrap();
    let id2: u64 = broker.publish(msg2).await.unwrap();
    assert_eq!(id2, id1 + 1);
    assert_eq!(broker.publish(msg3).await, Err(MqError::QueueFull));
    let d1: MqDelivery = broker.consume("orders", "billing").await.unwrap().unwrap();
    assert_eq!(d1.get_message().get_payload().as_slice(), b"order-1");
    assert_eq!(d1.get_message().get_attempts(), 1u32);
    broker.ack(d1.get_receipt_id()).await.unwrap();
    assert_eq!(
        broker.ack(d1.get_receipt_id()).await,
        Err(MqError::UnknownDelivery)
    );
    let d2: MqDelivery = broker.consume("orders", "billing").await.unwrap().unwrap();
    broker.nack(d2.get_receipt_id(), true).await.unwrap();
    let d2_retry: MqDelivery = broker.consume("orders", "billing").await.unwrap().unwrap();
    assert_eq!(d2_retry.get_message().get_attempts(), 2u32);
    broker.nack(d2_retry.get_receipt_id(), true).await.unwrap();
    let dlq: Vec<MqMessage> = broker.drain_dlq("orders");
    assert_eq!(dlq.len(), 1usize);
    assert_eq!(dlq[0].get_payload().as_slice(), b"order-2");
    let a1: MqDelivery = broker
        .consume("orders", "analytics")
        .await
        .unwrap()
        .unwrap();
    assert_eq!(a1.get_message().get_payload().as_slice(), b"order-1");
    tokio::time::sleep(Duration::from_millis(70)).await;
    let redelivered: MqDelivery = broker
        .consume("orders", "analytics")
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        redelivered.get_message().get_payload().as_slice(),
        b"order-1"
    );
    assert_eq!(redelivered.get_message().get_attempts(), 2u32);
    broker.ack(redelivered.get_receipt_id()).await.unwrap();
    let stats: MqStats = broker.stats("orders", "billing").await.unwrap();
    assert_eq!(stats.get_published_total(), 2u64);
    assert_eq!(stats.get_acked_total(), 1u64);
}

#[tokio::test]
async fn test_mq_ttl_expiration_drops_stale_messages() {
    let broker: InMemoryBroker = InMemoryBroker::default();
    let mut expired: MqMessage = MqMessage::new("events", b"old".to_vec());
    expired.set_expires_at_ms(Some(1));
    assert_eq!(broker.publish(expired).await, Err(MqError::MessageExpired));
    let short_ttl: MqMessage = MqMessage::new("events", b"ephemeral".to_vec()).with_ttl_ms(20);
    let _: u64 = broker.publish(short_ttl).await.unwrap();
    tokio::time::sleep(Duration::from_millis(35)).await;
    assert!(
        broker
            .consume("events", DEFAULT_MQ_GROUP)
            .await
            .unwrap()
            .is_none()
    );
}
