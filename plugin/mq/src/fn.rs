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
