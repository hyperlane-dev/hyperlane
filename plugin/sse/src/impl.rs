use super::*;

/// Formatting / debug-printing for [`SseError`].
impl Display for SseError {
    /// Formats the [`SseError`] via the supplied formatter.
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
            Self::TopicLimitReached => f.write_str(ERR_TOPIC_LIMIT),
            Self::ChannelClosed => f.write_str(ERR_CHANNEL_CLOSED),
            Self::Lagged(skipped) => write!(f, "SSE subscriber lagged by {skipped} events"),
            Self::Transport(msg) => f.write_str(msg),
        }
    }
}

/// Implements [`Error`] for [`SseError`].
impl Error for SseError {}

/// Default construction for [`SseConfig`].
impl Default for SseConfig {
    /// Constructs a default [`SseConfig`] value.
    fn default() -> Self {
        Self {
            heartbeat_ms: DEFAULT_SSE_HEARTBEAT_MS,
            retry_ms: DEFAULT_SSE_RETRY_MS,
            replay_capacity: DEFAULT_SSE_REPLAY_CAPACITY,
            channel_capacity: DEFAULT_SSE_CHANNEL_CAPACITY,
            max_topics: DEFAULT_SSE_MAX_TOPICS,
        }
    }
}

/// Inherent implementation of [`SseConfig`].
impl SseConfig {
    /// Sets the heartbeat comment interval in milliseconds (`0` disables heartbeats).
    ///
    /// # Arguments
    ///
    /// - `u64` - Heartbeat interval in milliseconds.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_heartbeat_ms(mut self, heartbeat_ms: u64) -> Self {
        self.set_heartbeat_ms(heartbeat_ms);
        self
    }

    /// Sets the client reconnect retry hint in milliseconds (`0` omits `retry:`).
    ///
    /// # Arguments
    ///
    /// - `u64` - Reconnect hint in milliseconds.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_retry_ms(mut self, retry_ms: u64) -> Self {
        self.set_retry_ms(retry_ms);
        self
    }

    /// Sets the bounded per-topic replay buffer capacity for `Last-Event-ID`.
    ///
    /// # Arguments
    ///
    /// - `usize` - Number of recent events retained per topic (`0` disables replay).
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_replay_capacity(mut self, replay_capacity: usize) -> Self {
        self.set_replay_capacity(replay_capacity);
        self
    }

    /// Sets the per-topic broadcast channel buffer capacity.
    ///
    /// # Arguments
    ///
    /// - `usize` - Broadcast ring size per topic.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_channel_capacity(mut self, channel_capacity: usize) -> Self {
        self.set_channel_capacity(channel_capacity);
        self
    }

    /// Sets the maximum number of concurrent distinct topics tracked by an [`SseHub`].
    ///
    /// # Arguments
    ///
    /// - `usize` - Maximum topic cap.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_max_topics(mut self, max_topics: usize) -> Self {
        self.set_max_topics(max_topics);
        self
    }
}

/// Inherent implementation of [`SseEvent`].
impl SseEvent {
    /// Creates a new SSE event carrying `data`.
    ///
    /// # Arguments
    ///
    /// - `D` - Payload text convertible into `String`.
    ///
    /// # Returns
    ///
    /// - `Self` - Event with empty metadata and the provided payload.
    pub fn new<D>(data: D) -> Self
    where
        D: Into<String>,
    {
        Self {
            id: None,
            event: None,
            data: data.into(),
            retry: None,
            comment: None,
        }
    }

    /// Creates a comment-only SSE keep-alive frame (`: <comment>\n\n`).
    ///
    /// # Arguments
    ///
    /// - `C` - Comment text convertible into `String`.
    ///
    /// # Returns
    ///
    /// - `Self` - Comment-only SSE event.
    pub fn comment_only<C>(comment: C) -> Self
    where
        C: Into<String>,
    {
        Self {
            id: None,
            event: None,
            data: String::new(),
            retry: None,
            comment: Some(comment.into()),
        }
    }

    /// Attaches an `id:` field to this event.
    ///
    /// # Arguments
    ///
    /// - `I` - Identifier text convertible into `String`.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated event.
    pub fn with_id<I>(mut self, id: I) -> Self
    where
        I: Into<String>,
    {
        self.set_id(Some(id.into()));
        self
    }

    /// Attaches an `event:` type field to this event.
    ///
    /// # Arguments
    ///
    /// - `E` - Event type convertible into `String`.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated event.
    pub fn with_event<E>(mut self, event: E) -> Self
    where
        E: Into<String>,
    {
        self.set_event(Some(event.into()));
        self
    }

    /// Attaches a `retry:` millisecond reconnection hint to this event.
    ///
    /// # Arguments
    ///
    /// - `u64` - Reconnect delay in milliseconds.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated event.
    pub fn with_retry(mut self, retry_ms: u64) -> Self {
        self.set_retry(Some(retry_ms));
        self
    }

    /// Encodes this event directly into `out` with zero intermediate heap allocations.
    ///
    /// # Arguments
    ///
    /// - `&mut Vec<u8>` - Reusable wire buffer (cleared before writing).
    pub fn encode_into(&self, out: &mut Vec<u8>) {
        let Self {
            id,
            event,
            data,
            retry,
            comment,
        } = self;
        out.clear();
        out.reserve(data.len() + 64);
        if let Some(comment_text) = comment.as_deref() {
            out.extend_from_slice(SSE_COMMENT_PREFIX.as_bytes());
            write_sanitized_sse_bytes(comment_text, out);
            out.push(b'\n');
        }
        if let Some(id_text) = id.as_deref() {
            out.extend_from_slice(SSE_FIELD_ID.as_bytes());
            write_sanitized_sse_bytes(id_text, out);
            out.push(b'\n');
        }
        if let Some(event_text) = event.as_deref() {
            out.extend_from_slice(SSE_FIELD_EVENT.as_bytes());
            write_sanitized_sse_bytes(event_text, out);
            out.push(b'\n');
        }
        if let Some(retry_ms) = *retry {
            out.extend_from_slice(SSE_FIELD_RETRY.as_bytes());
            write_u64_decimal(retry_ms, out);
            out.push(b'\n');
        }
        let has_data: bool = !data.is_empty() || comment.is_none();
        if has_data {
            let bytes: &[u8] = data.as_bytes();
            let mut start: usize = 0;
            let mut idx: usize = 0;
            while idx < bytes.len() {
                if bytes[idx] == b'\r' || bytes[idx] == b'\n' {
                    out.extend_from_slice(SSE_FIELD_DATA.as_bytes());
                    out.extend_from_slice(&bytes[start..idx]);
                    out.push(b'\n');
                    if bytes[idx] == b'\r' && idx + 1 < bytes.len() && bytes[idx + 1] == b'\n' {
                        idx += 1;
                    }
                    start = idx + 1;
                }
                idx += 1;
            }
            out.extend_from_slice(SSE_FIELD_DATA.as_bytes());
            out.extend_from_slice(&bytes[start..]);
            out.push(b'\n');
        }
        out.push(b'\n');
    }

    /// Encodes this event into WHATWG EventSource wire bytes with CRLF injection sanitization.
    ///
    /// # Returns
    ///
    /// - `Vec<u8>` - Complete SSE frame terminated by `\n\n`.
    pub fn encode(&self) -> Vec<u8> {
        let mut out: Vec<u8> = Vec::with_capacity(self.get_data().len() + 64);
        self.encode_into(&mut out);
        out
    }
}

/// Inherent implementation of [`SseHub`].
impl SseHub {
    /// Creates a new 64-shard [`SseHub`] governed by `config`.
    ///
    /// # Arguments
    ///
    /// - `SseConfig` - Hub capacity and replay configuration.
    ///
    /// # Returns
    ///
    /// - `Self` - Ready multi-topic SSE hub.
    pub fn new(config: SseConfig) -> Self {
        let shards: SseShardArray =
            Box::new(from_fn(|_: usize| RwLock::new(hash_map_xx_hash3_64())));
        Self {
            inner: Arc::new(SseHubInner {
                config,
                next_seq: AtomicU64::new(1),
                active_topics: AtomicUsize::new(0),
                shards,
            }),
        }
    }

    /// Returns a reference to the hub configuration.
    ///
    /// # Returns
    ///
    /// - `&SseConfig` - Active hub configuration.
    pub fn get_config(&self) -> &SseConfig {
        &self.inner.config
    }

    /// Looks up or inserts the [`TopicState`] for `topic_name` using read-fast sharding.
    ///
    /// # Arguments
    ///
    /// - `&str` - Topic name.
    ///
    /// # Returns
    ///
    /// - `Result<Arc<TopicState>, SseError>` - Shared topic state or [`SseError::TopicLimitReached`].
    fn get_or_create_topic(&self, topic_name: &str) -> Result<Arc<TopicState>, SseError> {
        let Self { inner } = self;
        let shard_idx: usize = sse_shard_index(topic_name);
        let shard: &RwLock<HashMapXxHash3_64<String, Arc<TopicState>>> = &inner.shards[shard_idx];
        if let Ok(read_map) = shard.read()
            && let Some(existing) = read_map.get(topic_name)
        {
            return Ok(Arc::clone(existing));
        }
        let Ok(mut write_map) = shard.write() else {
            return Err(SseError::ChannelClosed);
        };
        if let Some(existing) = write_map.get(topic_name) {
            return Ok(Arc::clone(existing));
        }
        let max_topics: usize = inner.config.get_max_topics();
        let mut current: usize = inner.active_topics.load(Ordering::Acquire);
        loop {
            if current >= max_topics {
                return Err(SseError::TopicLimitReached);
            }
            match inner.active_topics.compare_exchange_weak(
                current,
                current + 1,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => break,
                Err(actual) => current = actual,
            }
        }
        let chan_cap: usize = inner.config.get_channel_capacity().max(1);
        let replay_cap: usize = inner.config.get_replay_capacity();
        let (tx, _): (Sender<SseEvent>, Receiver<SseEvent>) = broadcast::channel(chan_cap);
        let state: Arc<TopicState> = Arc::new(TopicState {
            sender: tx,
            replay: Mutex::new(VecDeque::with_capacity(replay_cap)),
        });
        write_map.insert(topic_name.to_owned(), Arc::clone(&state));
        Ok(state)
    }

    /// Publishes an event to `topic`, assigning a monotonic id if absent and appending to replay history.
    ///
    /// # Arguments
    ///
    /// - `T` - Topic name convertible into `String`.
    /// - `SseEvent` - Event to fan out to active subscribers.
    ///
    /// # Returns
    ///
    /// - `Result<u64, SseError>` - Assigned sequence identifier, or [`SseError::TopicLimitReached`].
    pub fn publish<T>(&self, topic: T, mut event: SseEvent) -> Result<u64, SseError>
    where
        T: Into<String>,
    {
        let Self { inner } = self;
        let topic_name: String = topic.into();
        let state: Arc<TopicState> = self.get_or_create_topic(&topic_name)?;
        let seq: u64 = inner.next_seq.fetch_add(1, Ordering::Relaxed);
        if event.get_id().is_none() {
            event.set_id(Some(seq.to_string()));
        }
        let replay_cap: usize = inner.config.get_replay_capacity();
        if replay_cap > 0
            && let Ok(mut replay) = state.replay.lock()
        {
            if replay.len() >= replay_cap {
                let _: Option<(u64, SseEvent)> = replay.pop_front();
            }
            replay.push_back((seq, event.clone()));
        }
        let _: Result<usize, broadcast::error::SendError<SseEvent>> = state.sender.send(event);
        Ok(seq)
    }

    /// Publishes a batch of events to `topic` in order, returning the last assigned sequence id.
    ///
    /// # Arguments
    ///
    /// - `T` - Topic name convertible into `String`.
    /// - `I` - Iterator yielding [`SseEvent`] items.
    ///
    /// # Returns
    ///
    /// - `Result<Option<u64>, SseError>` - Last assigned sequence id, or `None` when empty.
    pub fn publish_many<T, I>(&self, topic: T, events: I) -> Result<Option<u64>, SseError>
    where
        T: Into<String>,
        I: IntoIterator<Item = SseEvent>,
    {
        let topic_name: String = topic.into();
        let mut last_seq: Option<u64> = None;
        for event in events {
            last_seq = Some(self.publish(topic_name.as_str(), event)?);
        }
        Ok(last_seq)
    }

    /// Subscribes to `topic`, replaying retained events newer than `last_event_id` when supplied.
    ///
    /// # Arguments
    ///
    /// - `T` - Topic name convertible into `String`.
    /// - `Option<&str>` - Optional `Last-Event-ID` header value for catch-up replay.
    ///
    /// # Returns
    ///
    /// - `Result<SseSubscription, SseError>` - Active subscription or [`SseError::TopicLimitReached`].
    pub fn subscribe<T>(
        &self,
        topic: T,
        last_event_id: Option<&str>,
    ) -> Result<SseSubscription, SseError>
    where
        T: Into<String>,
    {
        let Self { inner } = self;
        let topic_name: String = topic.into();
        let after_seq: Option<u64> = parse_last_event_id(last_event_id);
        let state: Arc<TopicState> = self.get_or_create_topic(&topic_name)?;
        let receiver: Receiver<SseEvent> = state.sender.subscribe();
        let mut replayed: VecDeque<SseEvent> = VecDeque::new();
        if let Some(cursor) = after_seq
            && let Ok(replay) = state.replay.lock()
        {
            for (seq, ev) in replay.iter() {
                if *seq > cursor {
                    replayed.push_back(ev.clone());
                }
            }
        }
        Ok(SseSubscription {
            topic: topic_name,
            hub: Arc::clone(inner),
            replayed,
            receiver,
        })
    }

    /// Explicitly removes `topic` from the hub registry.
    ///
    /// # Arguments
    ///
    /// - `&str` - Topic name to evict.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the topic existed and was removed.
    pub fn remove_topic(&self, topic: &str) -> bool {
        let Self { inner } = self;
        let shard_idx: usize = sse_shard_index(topic);
        if let Ok(mut write_map) = inner.shards[shard_idx].write()
            && write_map.remove(topic).is_some()
        {
            inner.active_topics.fetch_sub(1, Ordering::AcqRel);
            return true;
        }
        false
    }

    /// Returns all currently active topic names across all shards.
    ///
    /// # Returns
    ///
    /// - `Vec<String>` - Snapshot of active topic names.
    pub fn topic_names(&self) -> Vec<String> {
        let Self { inner } = self;
        let mut names: Vec<String> = Vec::with_capacity(self.topic_count());
        for shard in inner.shards.iter() {
            if let Ok(read_map) = shard.read() {
                names.extend(read_map.keys().cloned());
            }
        }
        names
    }

    /// Returns the number of active topics currently tracked by this hub in `O(1)`.
    ///
    /// # Returns
    ///
    /// - `usize` - Count of tracked topics.
    pub fn topic_count(&self) -> usize {
        let Self { inner } = self;
        inner.active_topics.load(Ordering::Acquire)
    }

    /// Returns the number of active subscribers currently listening to `topic`.
    ///
    /// # Arguments
    ///
    /// - `&str` - Topic name to inspect.
    ///
    /// # Returns
    ///
    /// - `usize` - Active subscriber count (`0` when topic does not exist).
    pub fn subscriber_count(&self, topic: &str) -> usize {
        let Self { inner } = self;
        let shard_idx: usize = sse_shard_index(topic);
        inner.shards[shard_idx]
            .read()
            .ok()
            .and_then(
                |map: RwLockReadGuard<'_, HashMapXxHash3_64<String, Arc<TopicState>>>| {
                    map.get(topic)
                        .map(|state: &Arc<TopicState>| state.sender.receiver_count())
                },
            )
            .unwrap_or(0)
    }
}

/// Default construction for [`SseHub`].
impl Default for SseHub {
    /// Constructs a default [`SseHub`] value.
    fn default() -> Self {
        Self::new(SseConfig::default())
    }
}

/// Inherent implementation of [`SseSubscription`].
impl SseSubscription {
    /// Returns the topic name associated with this subscription.
    ///
    /// # Returns
    ///
    /// - `&str` - Subscribed topic name.
    pub fn topic(&self) -> &str {
        let Self { topic, .. } = self;
        topic
    }

    /// Receives the next event (draining `Last-Event-ID` replay entries before live broadcasts).
    ///
    /// # Returns
    ///
    /// - `Result<SseEvent, SseError>` - Next SSE event or channel error.
    pub async fn recv(&mut self) -> Result<SseEvent, SseError> {
        let Self {
            replayed, receiver, ..
        } = self;
        if let Some(event) = replayed.pop_front() {
            return Ok(event);
        }
        match receiver.recv().await {
            Ok(event) => Ok(event),
            Err(RecvError::Lagged(skipped)) => Err(SseError::Lagged(skipped)),
            Err(RecvError::Closed) => Err(SseError::ChannelClosed),
        }
    }
}

/// Implements [`Drop`] for [`SseSubscription`].
impl Drop for SseSubscription {
    /// Evicts empty ephemeral topics when the last subscriber disconnects.
    fn drop(&mut self) {
        let Self { hub, topic, .. } = self;
        let shard_idx: usize = sse_shard_index(topic);
        if let Ok(mut write_map) = hub.shards[shard_idx].write()
            && let Some(state) = write_map.get(topic)
            && state.sender.receiver_count() <= 1
            && state
                .replay
                .lock()
                .is_ok_and(|replay: MutexGuard<'_, VecDeque<(u64, SseEvent)>>| replay.is_empty())
        {
            write_map.remove(topic);
            hub.active_topics.fetch_sub(1, Ordering::AcqRel);
        }
    }
}

/// Inherent implementation of [`SseStream`].
impl SseStream {
    /// Configures standard SSE response headers on `ctx` without closing the stream.
    ///
    /// # Arguments
    ///
    /// - `&mut Context` - Active Hyperlane request context.
    pub fn prepare(ctx: &mut Context) {
        ctx.get_mut_response()
            .set_header(CONTENT_TYPE, SSE_CONTENT_TYPE)
            .set_header(CACHE_CONTROL, SSE_CACHE_CONTROL)
            .set_header(SSE_ACCEL_BUFFERING_HEADER, SSE_ACCEL_BUFFERING_VALUE);
    }

    /// Writes one [`SseEvent`] frame directly onto `stream`.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - Active Hyperlane client stream.
    /// - `&SseEvent` - Event to encode and flush.
    ///
    /// # Returns
    ///
    /// - `Result<(), SseError>` - `Ok(())` on successful write, or [`SseError::Transport`].
    pub async fn send_event(stream: &mut Stream, event: &SseEvent) -> Result<(), SseError> {
        let bytes: Vec<u8> = event.encode();
        stream
            .try_send(&bytes)
            .await
            .map_err(|err: ResponseError| SseError::Transport(err.to_string()))
    }

    /// Encodes `event` into a reusable `scratch` buffer and writes it onto `stream`.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - Active Hyperlane client stream.
    /// - `&SseEvent` - Event to encode and flush.
    /// - `&mut Vec<u8>` - Reusable frame encoding buffer.
    ///
    /// # Returns
    ///
    /// - `Result<(), SseError>` - `Ok(())` on successful write, or [`SseError::Transport`].
    pub async fn send_event_buffered(
        stream: &mut Stream,
        event: &SseEvent,
        scratch: &mut Vec<u8>,
    ) -> Result<(), SseError> {
        event.encode_into(scratch);
        stream
            .try_send(scratch)
            .await
            .map_err(|err: ResponseError| SseError::Transport(err.to_string()))
    }

    /// Writes a single SSE comment / heartbeat line directly onto `stream`.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - Active Hyperlane client stream.
    /// - `&str` - Comment text.
    ///
    /// # Returns
    ///
    /// - `Result<(), SseError>` - `Ok(())` on successful write, or [`SseError::Transport`].
    pub async fn send_comment(stream: &mut Stream, comment: &str) -> Result<(), SseError> {
        let frame: SseEvent = SseEvent::comment_only(comment);
        Self::send_event(stream, &frame).await
    }

    /// Prepares headers, sends the HTTP head, and pumps `subscription` with heartbeats until disconnect.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - Active Hyperlane client stream.
    /// - `&mut Context` - Active Hyperlane request context.
    /// - `SseSubscription` - Topic subscription to stream.
    /// - `&SseConfig` - Heartbeat and reconnect configuration.
    ///
    /// # Returns
    ///
    /// - `Result<(), SseError>` - Terminates cleanly when the channel closes or client disconnects.
    pub async fn serve_subscription(
        stream: &mut Stream,
        ctx: &mut Context,
        mut subscription: SseSubscription,
        config: &SseConfig,
    ) -> Result<(), SseError> {
        Self::prepare(ctx);
        let head: ResponseData = ctx.get_mut_response().build_head();
        stream
            .try_send(&head)
            .await
            .map_err(|err: ResponseError| SseError::Transport(err.to_string()))?;
        let mut scratch: Vec<u8> = Vec::with_capacity(256);
        if config.get_retry_ms() > 0 {
            let hint: SseEvent =
                SseEvent::comment_only(SSE_HEARTBEAT_COMMENT).with_retry(config.get_retry_ms());
            Self::send_event_buffered(stream, &hint, &mut scratch).await?;
        }
        let heartbeat_ms: u64 = config.get_heartbeat_ms();
        if heartbeat_ms == 0 {
            loop {
                match subscription.recv().await {
                    Ok(event) => Self::send_event_buffered(stream, &event, &mut scratch).await?,
                    Err(SseError::Lagged(_)) => continue,
                    Err(SseError::ChannelClosed) => {
                        stream.set_closed(true);
                        return Ok(());
                    }
                    Err(err) => return Err(err),
                }
            }
        }
        let heartbeat_frame: Vec<u8> = SseEvent::comment_only(SSE_HEARTBEAT_COMMENT).encode();
        let mut ticker: Interval = interval(Duration::from_millis(heartbeat_ms));
        ticker.set_missed_tick_behavior(MissedTickBehavior::Delay);
        let _: Instant = ticker.tick().await;
        loop {
            select! {
                _ = ticker.tick() => {
                    stream
                        .try_send(&heartbeat_frame)
                        .await
                        .map_err(|err: ResponseError| SseError::Transport(err.to_string()))?;
                }
                next = subscription.recv() => {
                    match next {
                        Ok(event) => Self::send_event_buffered(stream, &event, &mut scratch).await?,
                        Err(SseError::Lagged(_)) => continue,
                        Err(SseError::ChannelClosed) => {
                            stream.set_closed(true);
                            return Ok(());
                        }
                        Err(err) => return Err(err),
                    }
                }
            }
        }
    }
}

/// Inherent implementation of [`SsePlugin`].
impl SsePlugin {
    /// Initializes the global [`SsePlugin`] once with `config`.
    ///
    /// # Arguments
    ///
    /// - `SseConfig` - SSE plugin configuration.
    ///
    /// # Returns
    ///
    /// - `&'static Self` - Reference to the initialized global plugin instance.
    pub fn init(config: SseConfig) -> &'static Self {
        SSE_PLUGIN.get_or_init(|| Self {
            hub: SseHub::new(config.clone()),
            config,
        })
    }

    /// Returns the initialized global [`SsePlugin`], if [`SsePlugin::init`] was called.
    ///
    /// # Returns
    ///
    /// - `Option<&'static Self>` - Reference to the global plugin, or `None`.
    pub fn get() -> Option<&'static Self> {
        SSE_PLUGIN.get()
    }

    /// Returns the plugin's [`SseConfig`].
    ///
    /// # Returns
    ///
    /// - `&SseConfig` - Active configuration.
    pub fn get_config(&self) -> &SseConfig {
        &self.config
    }

    /// Returns the plugin's shared [`SseHub`].
    ///
    /// # Returns
    ///
    /// - `&SseHub` - Process-wide SSE topic hub.
    pub fn get_hub(&self) -> &SseHub {
        &self.hub
    }
}
