/// Errors returned by [`crate::MessageBroker`] operations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum MqError {
    /// Target topic queue reached `MqConfig::max_queue_len`.
    QueueFull,
    /// Receipt identifier does not correspond to any active in-flight delivery.
    UnknownDelivery,
    /// Broker backend is closed or lock-poisoned.
    BrokerClosed,
    /// Message TTL expired before delivery.
    MessageExpired,
    /// Backend-specific transport or storage error.
    Backend(String),
}
