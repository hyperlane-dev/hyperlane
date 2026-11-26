use super::*;

/// Asynchronous message broker contract implemented by [`InMemoryBroker`] and external drivers.
pub trait MessageBroker: Send + Sync {
    /// Registers a consumer group on `topic` so future publishes fan out to it.
    ///
    /// # Arguments
    ///
    /// - `&'a self` - Shared reference to the message broker.
    /// - `&'a str` - Topic name.
    /// - `&'a str` - Consumer group name.
    ///
    /// # Returns
    ///
    /// - `BrokerFuture<'a, ()>` - Future resolving when registration succeeds.
    fn subscribe_group<'a>(&'a self, topic: &'a str, group: &'a str) -> BrokerFuture<'a, ()>;

    /// Publishes one message to its target topic and returns the assigned message id.
    ///
    /// # Arguments
    ///
    /// - `MqMessage` - Message to publish.
    ///
    /// # Returns
    ///
    /// - `BrokerFuture<'_, u64>` - Assigned message identifier or [`MqError`].
    fn publish(&self, message: MqMessage) -> BrokerFuture<'_, u64>;

    /// Leases the next available message for `(topic, group)`, if any.
    ///
    /// # Arguments
    ///
    /// - `&'a self` - Shared reference to the message broker.
    /// - `&'a str` - Topic name.
    /// - `&'a str` - Consumer group name.
    ///
    /// # Returns
    ///
    /// - `BrokerFuture<'a, Option<MqDelivery>>` - Next leased delivery or `None` when empty.
    fn consume<'a>(
        &'a self,
        topic: &'a str,
        group: &'a str,
    ) -> BrokerFuture<'a, Option<MqDelivery>>;

    /// Acknowledges successful processing of an active delivery receipt.
    ///
    /// # Arguments
    ///
    /// - `u64` - Delivery receipt identifier.
    ///
    /// # Returns
    ///
    /// - `BrokerFuture<'_, ()>` - `Ok(())` or [`MqError::UnknownDelivery`].
    fn ack(&self, receipt_id: u64) -> BrokerFuture<'_, ()>;

    /// Negatively acknowledges a delivery, either requeuing it or routing it to the DLQ.
    ///
    /// # Arguments
    ///
    /// - `u64` - Delivery receipt identifier.
    /// - `bool` - `true` to requeue when retries remain, `false` to move immediately to DLQ.
    ///
    /// # Returns
    ///
    /// - `BrokerFuture<'_, ()>` - `Ok(())` or [`MqError::UnknownDelivery`].
    fn nack(&self, receipt_id: u64, requeue: bool) -> BrokerFuture<'_, ()>;

    /// Returns queue depth and throughput statistics for `(topic, group)`.
    ///
    /// # Arguments
    ///
    /// - `&'a self` - Shared reference to the message broker.
    /// - `&'a str` - Topic name.
    /// - `&'a str` - Consumer group name.
    ///
    /// # Returns
    ///
    /// - `BrokerFuture<'a, MqStats>` - Current queue and DLQ statistics.
    fn stats<'a>(&'a self, topic: &'a str, group: &'a str) -> BrokerFuture<'a, MqStats>;
}
