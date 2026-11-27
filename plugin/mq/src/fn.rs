use super::*;

/// Returns the current Unix timestamp in milliseconds.
///
/// # Returns
///
/// - `u64` - Milliseconds since Unix epoch (`0` if the system clock precedes the epoch).
pub(crate) fn now_epoch_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|dur: Duration| dur.as_millis() as u64)
        .unwrap_or(0)
}

/// Computes a deterministic shard slot for `topic` using 64-bit FNV-1a hashing.
///
/// # Arguments
///
/// - `&str` - Topic name to hash.
///
/// # Returns
///
/// - `usize` - Shard index in `0..MQ_SHARD_COUNT`.
pub(crate) fn mq_topic_shard_index(topic: &str) -> usize {
    let mut hash: u64 = FNV1A_OFFSET_BASIS;
    for &b in topic.as_bytes() {
        hash ^= u64::from(b);
        hash = hash.wrapping_mul(FNV1A_PRIME);
    }
    (hash as usize) & (MQ_SHARD_COUNT - 1)
}

/// Computes a shard slot for `receipt_id` using fast Fibonacci bit mixing.
///
/// # Arguments
///
/// - `u64` - Delivery receipt identifier.
///
/// # Returns
///
/// - `usize` - Shard index in `0..MQ_SHARD_COUNT`.
pub(crate) fn mq_receipt_shard_index(receipt_id: u64) -> usize {
    let mixed: u64 = receipt_id.wrapping_mul(0x9e37_79b9_7f4a_7c15);
    ((mixed >> 58) as usize) & (MQ_SHARD_COUNT - 1)
}

/// Returns whether a message has passed its optional TTL deadline.
///
/// # Arguments
///
/// - `&MqMessage` - Message to inspect.
/// - `u64` - Current Unix timestamp in milliseconds.
///
/// # Returns
///
/// - `bool` - `true` when `expires_at_ms` is set and `<= now_ms`.
pub(crate) fn is_message_expired(message: &MqMessage, now_ms: u64) -> bool {
    message
        .get_expires_at_ms()
        .is_some_and(|deadline: u64| now_ms >= deadline)
}

/// Formats the dead-letter queue key for `topic` using `config`.
///
/// # Arguments
///
/// - `&str` - Base topic name.
/// - `&MqConfig` - Broker configuration holding `dlq_suffix`.
///
/// # Returns
///
/// - `String` - Dead-letter queue key.
pub(crate) fn dlq_topic_name(topic: &str, config: &MqConfig) -> String {
    format!("{topic}{}", config.get_dlq_suffix())
}
