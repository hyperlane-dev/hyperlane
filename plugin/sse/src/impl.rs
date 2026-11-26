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

    /// Encodes this event into WHATWG EventSource wire bytes with CRLF injection sanitization.
    ///
    /// # Returns
    ///
    /// - `Vec<u8>` - Complete SSE frame terminated by `\n\n`.
    pub fn encode(&self) -> Vec<u8> {
        let mut out: String = String::with_capacity(self.get_data().len() + 64);
        if let Some(comment) = self.get_comment() {
            let safe_comment: String = sanitize_sse_line(&comment);
            out.push_str(SSE_COMMENT_PREFIX);
            out.push_str(&safe_comment);
            out.push('\n');
        }
        if let Some(id) = self.get_id() {
            let safe_id: String = sanitize_sse_line(&id);
            out.push_str(SSE_FIELD_ID);
            out.push_str(&safe_id);
            out.push('\n');
        }
        if let Some(event) = self.get_event() {
            let safe_event: String = sanitize_sse_line(&event);
            out.push_str(SSE_FIELD_EVENT);
            out.push_str(&safe_event);
            out.push('\n');
        }
        if let Some(retry) = self.get_retry() {
            out.push_str(SSE_FIELD_RETRY);
            out.push_str(&retry.to_string());
            out.push('\n');
        }
        let has_data: bool = !self.get_data().is_empty() || self.get_comment().is_none();
        if has_data {
            let normalized: String = self.get_data().replace("\r\n", "\n").replace('\r', "\n");
            for line in normalized.split('\n') {
                out.push_str(SSE_FIELD_DATA);
                out.push_str(line);
                out.push('\n');
            }
        }
        out.push('\n');
        out.into_bytes()
    }
}

/// Inherent implementation of [`SseHub`].
impl SseHub {
    /// Creates a new [`SseHub`] governed by `config`.
    ///
    /// # Arguments
    ///
    /// - `SseConfig` - Hub capacity and replay configuration.
    ///
    /// # Returns
    ///
    /// - `Self` - Ready multi-topic SSE hub.
    pub fn new(config: SseConfig) -> Self {
        Self {
            inner: Arc::new(SseHubInner {
                config,
                next_seq: AtomicU64::new(1),
                topics: Mutex::new(HashMap::new()),
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
        let seq: u64 = inner.next_seq.fetch_add(1, Ordering::Relaxed);
        if event.get_id().is_none() {
            event.set_id(Some(seq.to_string()));
        }
        let Ok(mut map) = inner.topics.lock() else {
            return Err(SseError::ChannelClosed);
        };
        if !map.contains_key(&topic_name) && map.len() >= inner.config.get_max_topics() {
            return Err(SseError::TopicLimitReached);
        }
        let chan_cap: usize = inner.config.get_channel_capacity().max(1);
        let replay_cap: usize = inner.config.get_replay_capacity();
        let entry: &mut TopicState = map.entry(topic_name).or_insert_with(|| {
            let (tx, _): (Sender<SseEvent>, Receiver<SseEvent>) = broadcast::channel(chan_cap);
            TopicState {
                sender: tx,
                replay: VecDeque::with_capacity(replay_cap),
            }
        });
        if replay_cap > 0 {
            if entry.replay.len() >= replay_cap {
                let _: Option<(u64, SseEvent)> = entry.replay.pop_front();
            }
            entry.replay.push_back((seq, event.clone()));
        }
        let _: Result<usize, broadcast::error::SendError<SseEvent>> = entry.sender.send(event);
        Ok(seq)
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
        let Ok(mut map) = inner.topics.lock() else {
            return Err(SseError::ChannelClosed);
        };
        if !map.contains_key(&topic_name) && map.len() >= inner.config.get_max_topics() {
            return Err(SseError::TopicLimitReached);
        }
        let chan_cap: usize = inner.config.get_channel_capacity().max(1);
        let replay_cap: usize = inner.config.get_replay_capacity();
        let entry: &mut TopicState = map.entry(topic_name.clone()).or_insert_with(|| {
            let (tx, _): (Sender<SseEvent>, Receiver<SseEvent>) = broadcast::channel(chan_cap);
            TopicState {
                sender: tx,
                replay: VecDeque::with_capacity(replay_cap),
            }
        });
        let mut replayed: VecDeque<SseEvent> = VecDeque::new();
        if let Some(cursor) = after_seq {
            for (seq, ev) in &entry.replay {
                if *seq > cursor {
                    replayed.push_back(ev.clone());
                }
            }
        }
        let receiver: Receiver<SseEvent> = entry.sender.subscribe();
        Ok(SseSubscription {
            topic: topic_name,
            hub: Arc::clone(inner),
            replayed,
            receiver,
        })
    }

    /// Returns the number of active topics currently tracked by this hub.
    ///
    /// # Returns
    ///
    /// - `usize` - Count of tracked topics.
    pub fn topic_count(&self) -> usize {
        let Self { inner } = self;
        inner
            .topics
            .lock()
            .map(|map: MutexGuard<'_, HashMap<String, TopicState>>| map.len())
            .unwrap_or(0)
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
        inner
            .topics
            .lock()
            .ok()
            .and_then(|map: MutexGuard<'_, HashMap<String, TopicState>>| {
                map.get(topic)
                    .map(|state: &TopicState| state.sender.receiver_count())
            })
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
    /// Implements the `drop` operation on [`SseSubscription`].
    fn drop(&mut self) {
        let Self { hub, topic, .. } = self;
        if let Ok(mut map) = hub.topics.lock()
            && let Some(state) = map.get(topic)
            && state.sender.receiver_count() <= 1
            && state.replay.is_empty()
        {
            map.remove(topic);
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
        if config.get_retry_ms() > 0 {
            let hint: SseEvent =
                SseEvent::comment_only(SSE_HEARTBEAT_COMMENT).with_retry(config.get_retry_ms());
            Self::send_event(stream, &hint).await?;
        }
        let heartbeat_ms: u64 = config.get_heartbeat_ms();
        if heartbeat_ms == 0 {
            loop {
                match subscription.recv().await {
                    Ok(event) => Self::send_event(stream, &event).await?,
                    Err(SseError::Lagged(_)) => continue,
                    Err(SseError::ChannelClosed) => {
                        stream.set_closed(true);
                        return Ok(());
                    }
                    Err(err) => return Err(err),
                }
            }
        }
        let mut ticker: Interval = interval(Duration::from_millis(heartbeat_ms));
        ticker.set_missed_tick_behavior(MissedTickBehavior::Delay);
        let _: Instant = ticker.tick().await;
        loop {
            select! {
                _ = ticker.tick() => {
                    Self::send_comment(stream, SSE_HEARTBEAT_COMMENT).await?;
                }
                next = subscription.recv() => {
                    match next {
                        Ok(event) => Self::send_event(stream, &event).await?,
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
