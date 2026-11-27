use super::*;

/// Formatting / debug-printing for [`MqError`].
impl Display for MqError {
    /// Formats the [`MqError`] via the supplied formatter.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter<'_>` - The formatter receiving the formatted output.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - A `fmt::Result` value.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            Self::QueueFull => f.write_str(ERR_MQ_QUEUE_FULL),
            Self::UnknownDelivery => f.write_str(ERR_MQ_UNKNOWN_DELIVERY),
            Self::BrokerClosed => f.write_str(ERR_MQ_BROKER_CLOSED),
            Self::MessageExpired => f.write_str(ERR_MQ_EXPIRED),
            Self::Backend(msg) => f.write_str(msg),
        }
    }
}

/// Implements [`Error`] for [`MqError`].
impl Error for MqError {}

/// Default construction for [`MqConfig`].
impl Default for MqConfig {
    /// Constructs a default [`MqConfig`] value.
    fn default() -> Self {
        Self {
            max_queue_len: DEFAULT_MQ_MAX_QUEUE_LEN,
            visibility_timeout_ms: DEFAULT_MQ_VISIBILITY_TIMEOUT_MS,
            default_max_retries: DEFAULT_MQ_MAX_RETRIES,
            dlq_suffix: DEFAULT_MQ_DLQ_SUFFIX.to_owned(),
        }
    }
}

/// Inherent implementation of [`MqConfig`].
impl MqConfig {
    /// Sets the maximum pending queue length per `(topic, group)`.
    ///
    /// # Arguments
    ///
    /// - `usize` - Maximum pending messages.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_max_queue_len(mut self, max_queue_len: usize) -> Self {
        self.set_max_queue_len(max_queue_len);
        self
    }

    /// Sets the visibility timeout in milliseconds.
    ///
    /// # Arguments
    ///
    /// - `u64` - Visibility timeout in milliseconds.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_visibility_timeout_ms(mut self, visibility_timeout_ms: u64) -> Self {
        self.set_visibility_timeout_ms(visibility_timeout_ms);
        self
    }

    /// Sets the default retry count before routing to the dead-letter queue.
    ///
    /// # Arguments
    ///
    /// - `u32` - Default maximum attempts.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_default_max_retries(mut self, default_max_retries: u32) -> Self {
        self.set_default_max_retries(default_max_retries);
        self
    }

    /// Sets the dead-letter queue topic suffix.
    ///
    /// # Arguments
    ///
    /// - `S` - Suffix convertible into `String`.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_dlq_suffix<S>(mut self, dlq_suffix: S) -> Self
    where
        S: Into<String>,
    {
        self.set_dlq_suffix(dlq_suffix.into());
        self
    }
}

/// Inherent implementation of [`MqMessage`].
impl MqMessage {
    /// Creates a new message targeting `topic` with raw `payload` bytes.
    ///
    /// # Arguments
    ///
    /// - `T` - Topic name convertible into `String`.
    /// - `P` - Payload bytes convertible into `Vec<u8>`.
    ///
    /// # Returns
    ///
    /// - `Self` - Newly constructed message stamped with the current epoch millisecond.
    pub fn new<T, P>(topic: T, payload: P) -> Self
    where
        T: Into<String>,
        P: Into<Vec<u8>>,
    {
        Self {
            id: 0,
            topic: topic.into(),
            key: None,
            payload: payload.into(),
            headers: HashMap::new(),
            attempts: 0,
            max_retries: 0,
            created_at_ms: now_epoch_ms(),
            expires_at_ms: None,
        }
    }

    /// Sets an optional routing or partitioning key.
    ///
    /// # Arguments
    ///
    /// - `K` - Routing key convertible into `String`.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated message.
    pub fn with_key<K>(mut self, key: K) -> Self
    where
        K: Into<String>,
    {
        self.set_key(Some(key.into()));
        self
    }

    /// Inserts a metadata header key-value pair on this message.
    ///
    /// # Arguments
    ///
    /// - `K` - Header key convertible into `String`.
    /// - `V` - Header value convertible into `String`.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated message.
    pub fn with_header<K, V>(mut self, key: K, value: V) -> Self
    where
        K: Into<String>,
        V: Into<String>,
    {
        self.get_mut_headers().insert(key.into(), value.into());
        self
    }

    /// Configures a relative time-to-live in milliseconds from creation time.
    ///
    /// # Arguments
    ///
    /// - `u64` - TTL in milliseconds.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated message with `expires_at_ms` set.
    pub fn with_ttl_ms(mut self, ttl_ms: u64) -> Self {
        let base: u64 = self.get_created_at_ms();
        self.set_expires_at_ms(Some(base.saturating_add(ttl_ms)));
        self
    }

    /// Overrides the maximum delivery attempts before dead-lettering.
    ///
    /// # Arguments
    ///
    /// - `u32` - Maximum attempts allowed for this message.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated message.
    pub fn with_max_retries(mut self, max_retries: u32) -> Self {
        self.set_max_retries(max_retries);
        self
    }
}

/// Inherent implementation of [`InMemoryBroker`].
impl InMemoryBroker {
    /// Creates a new 64-shard [`InMemoryBroker`] governed by `config`.
    ///
    /// # Arguments
    ///
    /// - `MqConfig` - Queue capacity, retry, and visibility timeout settings.
    ///
    /// # Returns
    ///
    /// - `Self` - Ready in-memory message broker.
    pub fn new(config: MqConfig) -> Self {
        let topic_shards: TopicShardArray =
            Box::new(from_fn(|_: usize| RwLock::new(HashMap::new())));
        let receipt_shards: ReceiptShardArray =
            Box::new(from_fn(|_: usize| RwLock::new(HashMap::new())));
        Self {
            inner: Arc::new(InMemoryBrokerInner {
                config,
                next_msg_id: AtomicU64::new(1),
                next_receipt_id: AtomicU64::new(1),
                topic_shards,
                receipt_shards,
            }),
        }
    }

    /// Returns a reference to this broker's [`MqConfig`].
    ///
    /// # Returns
    ///
    /// - `&MqConfig` - Active broker configuration.
    pub fn get_config(&self) -> &MqConfig {
        &self.inner.config
    }

    /// Looks up an existing [`TopicBroker`] for `topic` via read-fast sharding.
    ///
    /// # Arguments
    ///
    /// - `&str` - Topic name.
    ///
    /// # Returns
    ///
    /// - `Option<Arc<TopicBroker>>` - Shared topic broker, if present.
    fn get_topic(&self, topic: &str) -> Option<Arc<TopicBroker>> {
        let Self { inner } = self;
        let idx: usize = mq_topic_shard_index(topic);
        inner.topic_shards[idx].read().ok().and_then(
            |map: RwLockReadGuard<'_, HashMap<String, Arc<TopicBroker>>>| map.get(topic).cloned(),
        )
    }

    /// Looks up or inserts the [`TopicBroker`] for `topic` via read-fast sharding.
    ///
    /// # Arguments
    ///
    /// - `&str` - Topic name.
    ///
    /// # Returns
    ///
    /// - `Result<Arc<TopicBroker>, MqError>` - Shared topic broker or [`MqError::BrokerClosed`].
    fn get_or_create_topic(&self, topic: &str) -> Result<Arc<TopicBroker>, MqError> {
        if let Some(existing) = self.get_topic(topic) {
            return Ok(existing);
        }
        let Self { inner } = self;
        let idx: usize = mq_topic_shard_index(topic);
        let Ok(mut write_map) = inner.topic_shards[idx].write() else {
            return Err(MqError::BrokerClosed);
        };
        if let Some(existing) = write_map.get(topic) {
            return Ok(Arc::clone(existing));
        }
        let created: Arc<TopicBroker> = Arc::new(TopicBroker {
            published_total: AtomicU64::new(0),
            state: Mutex::new(TopicBrokerState {
                groups: HashSet::new(),
                queues: HashMap::new(),
                in_flight: HashMap::new(),
                deadlines: BinaryHeap::new(),
                dlq: VecDeque::new(),
                acked_totals: HashMap::new(),
            }),
        });
        write_map.insert(topic.to_owned(), Arc::clone(&created));
        Ok(created)
    }

    /// Registers a `receipt_id -> topic_broker` mapping in the sharded receipt index.
    ///
    /// # Arguments
    ///
    /// - `u64` - Delivery receipt identifier.
    /// - `Arc<TopicBroker>` - Owning topic broker.
    fn index_receipt(&self, receipt_id: u64, topic_broker: Arc<TopicBroker>) {
        let Self { inner } = self;
        let idx: usize = mq_receipt_shard_index(receipt_id);
        if let Ok(mut map) = inner.receipt_shards[idx].write() {
            map.insert(receipt_id, topic_broker);
        }
    }

    /// Removes and returns the owning [`TopicBroker`] for `receipt_id`.
    ///
    /// # Arguments
    ///
    /// - `u64` - Delivery receipt identifier.
    ///
    /// # Returns
    ///
    /// - `Option<Arc<TopicBroker>>` - Owning topic broker if the receipt was active.
    fn take_receipt_owner(&self, receipt_id: u64) -> Option<Arc<TopicBroker>> {
        let Self { inner } = self;
        let idx: usize = mq_receipt_shard_index(receipt_id);
        inner.receipt_shards[idx].write().ok().and_then(
            |mut map: RwLockWriteGuard<'_, HashMap<u64, Arc<TopicBroker>>>| map.remove(&receipt_id),
        )
    }

    /// Reclaims expired in-flight leases for one topic in `O(k log N)` via its min-heap.
    ///
    /// # Arguments
    ///
    /// - `&mut TopicBrokerState` - Locked per-topic broker state.
    /// - `&MqConfig` - Broker configuration.
    /// - `u64` - Current Unix timestamp in milliseconds.
    ///
    /// # Returns
    ///
    /// - `Vec<u64>` - Reclaimed receipt ids to remove from the receipt shard index.
    fn reclaim_expired_topic_leases(
        state: &mut TopicBrokerState,
        config: &MqConfig,
        now_ms: u64,
    ) -> Vec<u64> {
        let mut reclaimed: Vec<u64> = Vec::new();
        while let Some(Reverse((deadline, receipt_id))) = state.deadlines.peek().copied() {
            if deadline > now_ms {
                break;
            }
            let _: Option<Reverse<(u64, u64)>> = state.deadlines.pop();
            let Some(entry) = state.in_flight.remove(&receipt_id) else {
                continue;
            };
            reclaimed.push(receipt_id);
            if is_message_expired(&entry.message, now_ms) {
                continue;
            }
            let max_attempts: u32 = if entry.message.get_max_retries() > 0 {
                entry.message.get_max_retries()
            } else {
                config.get_default_max_retries().max(1)
            };
            if entry.message.get_attempts() >= max_attempts {
                state.dlq.push_back(entry.message);
            } else {
                state
                    .queues
                    .entry(entry.group)
                    .or_default()
                    .push_front(entry.message);
            }
        }
        reclaimed
    }

    /// Cleans up reclaimed receipt identifiers from the receipt shard index.
    ///
    /// # Arguments
    ///
    /// - `&[u64]` - Receipt identifiers removed during expiration reclaim.
    fn prune_receipt_index(&self, receipt_ids: &[u64]) {
        for &receipt_id in receipt_ids {
            let _: Option<Arc<TopicBroker>> = self.take_receipt_owner(receipt_id);
        }
    }

    /// Drains and returns all dead-lettered messages currently buffered for `topic`.
    ///
    /// # Arguments
    ///
    /// - `&str` - Base topic name.
    ///
    /// # Returns
    ///
    /// - `Vec<MqMessage>` - All dead-lettered messages in FIFO order.
    pub fn drain_dlq(&self, topic: &str) -> Vec<MqMessage> {
        let Some(topic_broker) = self.get_topic(topic) else {
            return Vec::new();
        };
        let Ok(mut state) = topic_broker.state.lock() else {
            return Vec::new();
        };
        state.dlq.drain(..).collect()
    }

    /// Returns the number of dead-lettered messages currently buffered for `topic`.
    ///
    /// # Arguments
    ///
    /// - `&str` - Base topic name.
    ///
    /// # Returns
    ///
    /// - `usize` - Count of messages waiting in the dead-letter queue.
    pub fn dlq_len(&self, topic: &str) -> usize {
        let Some(topic_broker) = self.get_topic(topic) else {
            return 0;
        };
        topic_broker
            .state
            .lock()
            .map(|state: MutexGuard<'_, TopicBrokerState>| state.dlq.len())
            .unwrap_or(0)
    }

    /// Returns the canonical dead-letter topic name for `topic` using this broker's config.
    ///
    /// # Arguments
    ///
    /// - `&str` - Base topic name.
    ///
    /// # Returns
    ///
    /// - `String` - Configured dead-letter topic name.
    pub fn dlq_name(&self, topic: &str) -> String {
        dlq_topic_name(topic, self.get_config())
    }
}

/// Default construction for [`InMemoryBroker`].
impl Default for InMemoryBroker {
    /// Constructs a default [`InMemoryBroker`] value.
    fn default() -> Self {
        Self::new(MqConfig::default())
    }
}

/// Implements [`MessageBroker`] for [`InMemoryBroker`].
impl MessageBroker for InMemoryBroker {
    /// Implements the `subscribe_group` operation on [`InMemoryBroker`].
    ///
    /// # Arguments
    ///
    /// - `&'a self` - Shared reference to a `'a self`.
    /// - `&'a str` - Shared reference to a `'a str`.
    /// - `&'a str` - Shared reference to a `'a str`.
    ///
    /// # Returns
    ///
    /// - `BrokerFuture<'a, ()>` - A `BrokerFuture<'a, ()>` value.
    fn subscribe_group<'a>(&'a self, topic: &'a str, group: &'a str) -> BrokerFuture<'a, ()> {
        Box::pin(async move {
            let topic_broker: Arc<TopicBroker> = self.get_or_create_topic(topic)?;
            let Ok(mut state) = topic_broker.state.lock() else {
                return Err(MqError::BrokerClosed);
            };
            state.groups.insert(group.to_owned());
            state.queues.entry(group.to_owned()).or_default();
            Ok(())
        })
    }

    /// Implements the `publish` operation on [`InMemoryBroker`].
    ///
    /// # Arguments
    ///
    /// - `MqMessage` - A `MqMessage` parameter.
    ///
    /// # Returns
    ///
    /// - `BrokerFuture<'_, u64>` - A `BrokerFuture<'_, u64>` value.
    fn publish(&self, mut message: MqMessage) -> BrokerFuture<'_, u64> {
        Box::pin(async move {
            let Self { inner } = self;
            let now_ms: u64 = now_epoch_ms();
            if is_message_expired(&message, now_ms) {
                return Err(MqError::MessageExpired);
            }
            let id: u64 = inner.next_msg_id.fetch_add(1, Ordering::Relaxed);
            message.set_id(id);
            if message.get_max_retries() == 0 {
                message.set_max_retries(inner.config.get_default_max_retries().max(1));
            }
            let topic_broker: Arc<TopicBroker> = self.get_or_create_topic(message.get_topic())?;
            let max_len: usize = inner.config.get_max_queue_len();
            let Ok(mut state) = topic_broker.state.lock() else {
                return Err(MqError::BrokerClosed);
            };
            if state.groups.is_empty() {
                let current_len: usize =
                    state.queues.get(DEFAULT_MQ_GROUP).map_or(0, VecDeque::len);
                if current_len >= max_len {
                    return Err(MqError::QueueFull);
                }
                state
                    .queues
                    .entry(DEFAULT_MQ_GROUP.to_owned())
                    .or_default()
                    .push_back(message);
            } else {
                for group in &state.groups {
                    let current_len: usize = state.queues.get(group).map_or(0, VecDeque::len);
                    if current_len >= max_len {
                        return Err(MqError::QueueFull);
                    }
                }
                let mut groups: Vec<String> = state.groups.iter().cloned().collect();
                if let Some(last_group) = groups.pop() {
                    for group in groups {
                        state
                            .queues
                            .entry(group)
                            .or_default()
                            .push_back(message.clone());
                    }
                    state
                        .queues
                        .entry(last_group)
                        .or_default()
                        .push_back(message);
                }
            }
            topic_broker.published_total.fetch_add(1, Ordering::Relaxed);
            Ok(id)
        })
    }

    /// Implements the `consume` operation on [`InMemoryBroker`].
    ///
    /// # Arguments
    ///
    /// - `&'a self` - Shared reference to a `'a self`.
    /// - `&'a str` - Shared reference to a `'a str`.
    /// - `&'a str` - Shared reference to a `'a str`.
    ///
    /// # Returns
    ///
    /// - `BrokerFuture<'a, Option<MqDelivery>>` - A `BrokerFuture<'a, Option<MqDelivery>>` value.
    fn consume<'a>(
        &'a self,
        topic: &'a str,
        group: &'a str,
    ) -> BrokerFuture<'a, Option<MqDelivery>> {
        Box::pin(async move {
            let Self { inner } = self;
            let Some(topic_broker) = self.get_topic(topic) else {
                return Ok(None);
            };
            let now_ms: u64 = now_epoch_ms();
            let (delivery, reclaimed): (Option<MqDelivery>, Vec<u64>) = {
                let Ok(mut state) = topic_broker.state.lock() else {
                    return Err(MqError::BrokerClosed);
                };
                let reclaimed_ids: Vec<u64> =
                    Self::reclaim_expired_topic_leases(&mut state, &inner.config, now_ms);
                let mut next_msg: Option<MqMessage> = None;
                if let Some(queue) = state.queues.get_mut(group) {
                    while let Some(candidate) = queue.pop_front() {
                        if !is_message_expired(&candidate, now_ms) {
                            next_msg = Some(candidate);
                            break;
                        }
                    }
                }
                let leased: Option<MqDelivery> = if let Some(mut msg) = next_msg {
                    msg.set_attempts(msg.get_attempts().saturating_add(1));
                    let receipt_id: u64 = inner.next_receipt_id.fetch_add(1, Ordering::Relaxed);
                    let visible_after_ms: u64 =
                        now_ms.saturating_add(inner.config.get_visibility_timeout_ms());
                    state.in_flight.insert(
                        receipt_id,
                        InFlightEntry {
                            group: group.to_owned(),
                            message: msg.clone(),
                        },
                    );
                    state
                        .deadlines
                        .push(Reverse((visible_after_ms, receipt_id)));
                    Some(MqDelivery {
                        message: msg,
                        receipt_id,
                        group: group.to_owned(),
                    })
                } else {
                    None
                };
                (leased, reclaimed_ids)
            };
            if !reclaimed.is_empty() {
                self.prune_receipt_index(&reclaimed);
            }
            if let Some(ref leased) = delivery {
                self.index_receipt(leased.get_receipt_id(), topic_broker);
            }
            Ok(delivery)
        })
    }

    /// Implements the `ack` operation on [`InMemoryBroker`].
    ///
    /// # Arguments
    ///
    /// - `u64` - A `u64` parameter.
    ///
    /// # Returns
    ///
    /// - `BrokerFuture<'_, ()>` - A `BrokerFuture<'_, ()>` value.
    fn ack(&self, receipt_id: u64) -> BrokerFuture<'_, ()> {
        Box::pin(async move {
            let Some(topic_broker) = self.take_receipt_owner(receipt_id) else {
                return Err(MqError::UnknownDelivery);
            };
            let Ok(mut state) = topic_broker.state.lock() else {
                return Err(MqError::BrokerClosed);
            };
            let Some(entry) = state.in_flight.remove(&receipt_id) else {
                return Err(MqError::UnknownDelivery);
            };
            let acked: &mut u64 = state.acked_totals.entry(entry.group).or_insert(0);
            *acked = acked.saturating_add(1);
            Ok(())
        })
    }

    /// Implements the `nack` operation on [`InMemoryBroker`].
    ///
    /// # Arguments
    ///
    /// - `u64` - A `u64` parameter.
    /// - `bool` - A boolean (`bool`).
    ///
    /// # Returns
    ///
    /// - `BrokerFuture<'_, ()>` - A `BrokerFuture<'_, ()>` value.
    fn nack(&self, receipt_id: u64, requeue: bool) -> BrokerFuture<'_, ()> {
        Box::pin(async move {
            let Self { inner } = self;
            let Some(topic_broker) = self.take_receipt_owner(receipt_id) else {
                return Err(MqError::UnknownDelivery);
            };
            let now_ms: u64 = now_epoch_ms();
            let Ok(mut state) = topic_broker.state.lock() else {
                return Err(MqError::BrokerClosed);
            };
            let Some(entry) = state.in_flight.remove(&receipt_id) else {
                return Err(MqError::UnknownDelivery);
            };
            if is_message_expired(&entry.message, now_ms) {
                return Ok(());
            }
            let max_attempts: u32 = if entry.message.get_max_retries() > 0 {
                entry.message.get_max_retries()
            } else {
                inner.config.get_default_max_retries().max(1)
            };
            if requeue && entry.message.get_attempts() < max_attempts {
                state
                    .queues
                    .entry(entry.group)
                    .or_default()
                    .push_back(entry.message);
            } else {
                state.dlq.push_back(entry.message);
            }
            Ok(())
        })
    }

    /// Implements the `stats` operation on [`InMemoryBroker`].
    ///
    /// # Arguments
    ///
    /// - `&'a self` - Shared reference to a `'a self`.
    /// - `&'a str` - Shared reference to a `'a str`.
    /// - `&'a str` - Shared reference to a `'a str`.
    ///
    /// # Returns
    ///
    /// - `BrokerFuture<'a, MqStats>` - A `BrokerFuture<'a, MqStats>` value.
    fn stats<'a>(&'a self, topic: &'a str, group: &'a str) -> BrokerFuture<'a, MqStats> {
        Box::pin(async move {
            let Self { inner } = self;
            let Some(topic_broker) = self.get_topic(topic) else {
                return Ok(MqStats::default());
            };
            let now_ms: u64 = now_epoch_ms();
            let (stats, reclaimed): (MqStats, Vec<u64>) = {
                let Ok(mut state) = topic_broker.state.lock() else {
                    return Err(MqError::BrokerClosed);
                };
                let reclaimed_ids: Vec<u64> =
                    Self::reclaim_expired_topic_leases(&mut state, &inner.config, now_ms);
                let queued: usize = state.queues.get(group).map_or(0, VecDeque::len);
                let in_flight: usize = state
                    .in_flight
                    .values()
                    .filter(|e: &&InFlightEntry| e.group == group)
                    .count();
                let dead_lettered: usize = state.dlq.len();
                let published_total: u64 = topic_broker.published_total.load(Ordering::Relaxed);
                let acked_total: u64 = state.acked_totals.get(group).copied().unwrap_or(0);
                (
                    MqStats {
                        queued,
                        in_flight,
                        dead_lettered,
                        published_total,
                        acked_total,
                    },
                    reclaimed_ids,
                )
            };
            if !reclaimed.is_empty() {
                self.prune_receipt_index(&reclaimed);
            }
            Ok(stats)
        })
    }
}

/// Inherent implementation of [`MqPlugin`].
impl MqPlugin {
    /// Initializes the global [`MqPlugin`] once with `config`.
    ///
    /// # Arguments
    ///
    /// - `MqConfig` - Message queue broker configuration.
    ///
    /// # Returns
    ///
    /// - `&'static Self` - Reference to the initialized global plugin instance.
    pub fn init(config: MqConfig) -> &'static Self {
        MQ_PLUGIN.get_or_init(|| Self {
            broker: InMemoryBroker::new(config.clone()),
            config,
            custom_broker: None,
        })
    }

    /// Initializes the global [`MqPlugin`] once with a custom pluggable [`MessageBroker`] backend.
    ///
    /// # Arguments
    ///
    /// - `MqConfig` - Message queue broker configuration.
    /// - `SharedBroker` - Custom message broker implementation.
    ///
    /// # Returns
    ///
    /// - `&'static Self` - Reference to the initialized global plugin instance.
    pub fn init_with_broker(config: MqConfig, broker: SharedBroker) -> &'static Self {
        MQ_PLUGIN.get_or_init(|| Self {
            broker: InMemoryBroker::new(config.clone()),
            config,
            custom_broker: Some(broker),
        })
    }

    /// Returns the initialized global [`MqPlugin`], if [`MqPlugin::init`] was called.
    ///
    /// # Returns
    ///
    /// - `Option<&'static Self>` - Reference to the global plugin, or `None`.
    pub fn get() -> Option<&'static Self> {
        MQ_PLUGIN.get()
    }

    /// Returns the plugin's [`MqConfig`].
    ///
    /// # Returns
    ///
    /// - `&MqConfig` - Active configuration.
    pub fn get_config(&self) -> &MqConfig {
        &self.config
    }

    /// Returns the plugin's shared [`InMemoryBroker`].
    ///
    /// # Returns
    ///
    /// - `&InMemoryBroker` - Process-wide in-memory broker.
    pub fn get_broker(&self) -> &InMemoryBroker {
        &self.broker
    }

    /// Returns the custom pluggable [`SharedBroker`] backend when installed via [`MqPlugin::init_with_broker`].
    ///
    /// # Returns
    ///
    /// - `Option<&SharedBroker>` - Active custom broker backend, or `None`.
    pub fn get_custom_broker(&self) -> Option<&SharedBroker> {
        self.custom_broker.as_ref()
    }
}
