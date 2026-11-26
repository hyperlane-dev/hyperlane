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
    /// Creates a new [`InMemoryBroker`] governed by `config`.
    ///
    /// # Arguments
    ///
    /// - `MqConfig` - Queue capacity, retry, and visibility timeout settings.
    ///
    /// # Returns
    ///
    /// - `Self` - Ready in-memory message broker.
    pub fn new(config: MqConfig) -> Self {
        Self {
            inner: Arc::new(InMemoryBrokerInner {
                config,
                next_msg_id: AtomicU64::new(1),
                next_receipt_id: AtomicU64::new(1),
                state: Mutex::new(BrokerState {
                    topic_groups: HashMap::new(),
                    queues: HashMap::new(),
                    in_flight: HashMap::new(),
                    dlqs: HashMap::new(),
                    published_totals: HashMap::new(),
                    acked_totals: HashMap::new(),
                }),
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

    /// Reclaims any in-flight messages whose visibility timeout has expired.
    ///
    /// # Arguments
    ///
    /// - `&mut BrokerState` - Locked broker state.
    /// - `&MqConfig` - Broker configuration.
    /// - `u64` - Current Unix timestamp in milliseconds.
    fn reclaim_expired_in_flight(state: &mut BrokerState, config: &MqConfig, now_ms: u64) {
        let expired_ids: Vec<u64> = state
            .in_flight
            .iter()
            .filter_map(|(id, entry): (&u64, &InFlightEntry)| {
                (now_ms >= entry.visible_after_ms).then_some(*id)
            })
            .collect();
        for receipt_id in expired_ids {
            let Some(entry) = state.in_flight.remove(&receipt_id) else {
                continue;
            };
            if is_message_expired(&entry.message, now_ms) {
                continue;
            }
            let max_attempts: u32 = if entry.message.get_max_retries() > 0 {
                entry.message.get_max_retries()
            } else {
                config.get_default_max_retries().max(1)
            };
            if entry.message.get_attempts() >= max_attempts {
                let dlq_name: String = dlq_topic_name(entry.message.get_topic(), config);
                state
                    .dlqs
                    .entry(dlq_name)
                    .or_default()
                    .push_back(entry.message);
            } else {
                let key: (String, String) = (entry.message.get_topic().clone(), entry.group);
                state
                    .queues
                    .entry(key)
                    .or_default()
                    .push_front(entry.message);
            }
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
        let Self { inner } = self;
        let dlq_name: String = dlq_topic_name(topic, &inner.config);
        let Ok(mut state) = inner.state.lock() else {
            return Vec::new();
        };
        state
            .dlqs
            .remove(&dlq_name)
            .map(|queue: VecDeque<MqMessage>| queue.into_iter().collect())
            .unwrap_or_default()
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
            let Self { inner } = self;
            let Ok(mut state) = inner.state.lock() else {
                return Err(MqError::BrokerClosed);
            };
            state
                .topic_groups
                .entry(topic.to_owned())
                .or_default()
                .insert(group.to_owned());
            state
                .queues
                .entry((topic.to_owned(), group.to_owned()))
                .or_default();
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
            let Ok(mut state) = inner.state.lock() else {
                return Err(MqError::BrokerClosed);
            };
            let topic: String = message.get_topic().clone();
            let groups: Vec<String> = match state.topic_groups.get(&topic) {
                Some(set) if !set.is_empty() => set.iter().cloned().collect(),
                _ => vec![DEFAULT_MQ_GROUP.to_owned()],
            };
            let max_len: usize = inner.config.get_max_queue_len();
            for group in &groups {
                let current_len: usize = state
                    .queues
                    .get(&(topic.clone(), group.clone()))
                    .map_or(0, VecDeque::len);
                if current_len >= max_len {
                    return Err(MqError::QueueFull);
                }
            }
            for group in groups {
                state
                    .queues
                    .entry((topic.clone(), group))
                    .or_default()
                    .push_back(message.clone());
            }
            let total: &mut u64 = state.published_totals.entry(topic).or_insert(0);
            *total = total.saturating_add(1);
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
            let now_ms: u64 = now_epoch_ms();
            let Ok(mut state) = inner.state.lock() else {
                return Err(MqError::BrokerClosed);
            };
            Self::reclaim_expired_in_flight(&mut state, &inner.config, now_ms);
            let queue_key: (String, String) = (topic.to_owned(), group.to_owned());
            let Some(queue) = state.queues.get_mut(&queue_key) else {
                return Ok(None);
            };
            let mut next_msg: Option<MqMessage> = None;
            while let Some(candidate) = queue.pop_front() {
                if !is_message_expired(&candidate, now_ms) {
                    next_msg = Some(candidate);
                    break;
                }
            }
            let Some(mut msg) = next_msg else {
                return Ok(None);
            };
            msg.set_attempts(msg.get_attempts().saturating_add(1));
            let receipt_id: u64 = inner.next_receipt_id.fetch_add(1, Ordering::Relaxed);
            let visible_after_ms: u64 =
                now_ms.saturating_add(inner.config.get_visibility_timeout_ms());
            state.in_flight.insert(
                receipt_id,
                InFlightEntry {
                    group: group.to_owned(),
                    message: msg.clone(),
                    visible_after_ms,
                },
            );
            Ok(Some(MqDelivery {
                message: msg,
                receipt_id,
                group: group.to_owned(),
            }))
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
            let Self { inner } = self;
            let Ok(mut state) = inner.state.lock() else {
                return Err(MqError::BrokerClosed);
            };
            let Some(entry) = state.in_flight.remove(&receipt_id) else {
                return Err(MqError::UnknownDelivery);
            };
            let key: (String, String) = (entry.message.get_topic().clone(), entry.group);
            let acked: &mut u64 = state.acked_totals.entry(key).or_insert(0);
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
            let now_ms: u64 = now_epoch_ms();
            let Ok(mut state) = inner.state.lock() else {
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
                let key: (String, String) = (entry.message.get_topic().clone(), entry.group);
                state
                    .queues
                    .entry(key)
                    .or_default()
                    .push_back(entry.message);
            } else {
                let dlq_name: String = dlq_topic_name(entry.message.get_topic(), &inner.config);
                state
                    .dlqs
                    .entry(dlq_name)
                    .or_default()
                    .push_back(entry.message);
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
            let now_ms: u64 = now_epoch_ms();
            let Ok(mut state) = inner.state.lock() else {
                return Err(MqError::BrokerClosed);
            };
            Self::reclaim_expired_in_flight(&mut state, &inner.config, now_ms);
            let key: (String, String) = (topic.to_owned(), group.to_owned());
            let queued: usize = state.queues.get(&key).map_or(0, VecDeque::len);
            let in_flight: usize = state
                .in_flight
                .values()
                .filter(|e: &&InFlightEntry| e.message.get_topic() == topic && e.group == group)
                .count();
            let dlq_name: String = dlq_topic_name(topic, &inner.config);
            let dead_lettered: usize = state.dlqs.get(&dlq_name).map_or(0, VecDeque::len);
            let published_total: u64 = state.published_totals.get(topic).copied().unwrap_or(0);
            let acked_total: u64 = state.acked_totals.get(&key).copied().unwrap_or(0);
            Ok(MqStats {
                queued,
                in_flight,
                dead_lettered,
                published_total,
                acked_total,
            })
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
}
