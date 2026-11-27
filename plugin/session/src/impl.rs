use super::*;

/// Default construction for [`SessionConfig`].
impl Default for SessionConfig {
    /// Constructs a secure default [`SessionConfig`] (`HttpOnly`, `SameSite=Lax`, 1 hour TTL).
    fn default() -> Self {
        Self {
            cookie_name: DEFAULT_SESSION_COOKIE_NAME.to_owned(),
            cookie_path: DEFAULT_SESSION_COOKIE_PATH.to_owned(),
            cookie_domain: None,
            same_site: DEFAULT_SESSION_SAME_SITE.to_owned(),
            http_only: true,
            secure: true,
            ttl_secs: DEFAULT_SESSION_TTL_SECS,
            max_sessions: DEFAULT_MAX_SESSIONS,
            signing_secret: None,
        }
    }
}

/// Inherent fluent builders for [`SessionConfig`].
impl SessionConfig {
    /// Sets the session cookie name.
    ///
    /// # Arguments
    ///
    /// - `N` - Cookie name convertible into `String`.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_cookie_name<N>(mut self, name: N) -> Self
    where
        N: Into<String>,
    {
        self.set_cookie_name(name.into());
        self
    }

    /// Sets the HMAC-SHA256 signing secret used to tamper-proof session cookies.
    ///
    /// # Arguments
    ///
    /// - `S` - Secret bytes convertible into `Vec<u8>`.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_signing_secret<S>(mut self, secret: S) -> Self
    where
        S: Into<Vec<u8>>,
    {
        self.set_signing_secret(Some(secret.into()));
        self
    }

    /// Sets the session TTL in seconds.
    ///
    /// # Arguments
    ///
    /// - `u64` - Session TTL in seconds.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_ttl_secs(mut self, ttl_secs: u64) -> Self {
        self.set_ttl_secs(ttl_secs);
        self
    }

    /// Sets whether the `Secure` attribute is emitted on session cookies.
    ///
    /// # Arguments
    ///
    /// - `bool` - `true` for HTTPS-only cookies.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_secure(mut self, secure: bool) -> Self {
        self.set_secure(secure);
        self
    }
}

/// Inherent implementation of [`SessionHandle`].
impl SessionHandle {
    /// Constructs a [`SessionHandle`] wrapping an existing or newly created session.
    ///
    /// # Arguments
    ///
    /// - `String` - Active session identifier.
    /// - `HashMap<String, String>` - Loaded session values.
    /// - `bool` - Whether this session already existed in the store.
    ///
    /// # Returns
    ///
    /// - `Self` - Ready session handle.
    pub(crate) fn new(id: String, values: HashMap<String, String>, existed: bool) -> Self {
        Self {
            state: Arc::new(Mutex::new(SessionState {
                id,
                previous_id: None,
                values,
                existed,
                dirty: false,
                destroyed: false,
            })),
        }
    }

    /// Returns the current session identifier.
    ///
    /// # Returns
    ///
    /// - `String` - Session ID.
    pub fn id(&self) -> String {
        let Self { state } = self;
        state
            .lock()
            .map(|guard: MutexGuard<'_, SessionState>| guard.id.clone())
            .unwrap_or_default()
    }

    /// Reads one attribute from the session without marking it dirty.
    ///
    /// # Arguments
    ///
    /// - `&str` - Attribute key.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - Stored attribute value, if present.
    pub fn get(&self, key: &str) -> Option<String> {
        let Self { state } = self;
        state
            .lock()
            .ok()
            .and_then(|guard: MutexGuard<'_, SessionState>| guard.values.get(key).cloned())
    }

    /// Inserts or updates a session attribute and marks the session dirty for persistence.
    ///
    /// # Arguments
    ///
    /// - `K` - Attribute key convertible into `String`.
    /// - `V` - Attribute value convertible into `String`.
    pub fn insert<K, V>(&self, key: K, value: V)
    where
        K: Into<String>,
        V: Into<String>,
    {
        let Self { state } = self;
        if let Ok(mut guard) = state.lock() {
            guard.values.insert(key.into(), value.into());
            guard.dirty = true;
            guard.destroyed = false;
        }
    }

    /// Removes a session attribute and marks the session dirty when present.
    ///
    /// # Arguments
    ///
    /// - `&str` - Attribute key to remove.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - Removed value, if any.
    pub fn remove(&self, key: &str) -> Option<String> {
        let Self { state } = self;
        let mut guard: MutexGuard<'_, SessionState> = state.lock().ok()?;
        let removed: Option<String> = guard.values.remove(key);
        if removed.is_some() {
            guard.dirty = true;
        }
        removed
    }

    /// Mints a fresh unpredictable session ID (session-fixation defense) and marks the session dirty.
    ///
    /// # Returns
    ///
    /// - `String` - Newly generated session identifier.
    pub fn regenerate_id(&self) -> String {
        let Self { state } = self;
        let next_id: String = generate_session_id();
        if let Ok(mut guard) = state.lock() {
            if guard.previous_id.is_none() && guard.existed {
                guard.previous_id = Some(guard.id.clone());
            }
            guard.id = next_id.clone();
            guard.dirty = true;
            guard.destroyed = false;
        }
        next_id
    }

    /// Marks the session for deletion and clears its in-memory attributes.
    pub fn destroy(&self) {
        let Self { state } = self;
        if let Ok(mut guard) = state.lock() {
            guard.values.clear();
            guard.destroyed = true;
            guard.dirty = true;
        }
    }

    /// Reports whether this session has been modified during the current request.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when dirty.
    pub fn is_dirty(&self) -> bool {
        let Self { state } = self;
        state
            .lock()
            .map(|guard: MutexGuard<'_, SessionState>| guard.dirty)
            .unwrap_or(false)
    }

    /// Captures a snapshot of the internal [`SessionState`] for persistence in [`SessionResponseHook`].
    ///
    /// # Returns
    ///
    /// - `Option<SessionState>` - Cloned state snapshot.
    pub(crate) fn snapshot(&self) -> Option<SessionState> {
        let Self { state } = self;
        state
            .lock()
            .ok()
            .map(|guard: MutexGuard<'_, SessionState>| guard.clone())
    }
}

/// Default construction for [`InMemorySessionStore`].
impl Default for InMemorySessionStore {
    /// Constructs a default 64-shard [`InMemorySessionStore`].
    fn default() -> Self {
        Self::new(DEFAULT_MAX_SESSIONS)
    }
}

/// Inherent implementation of [`InMemorySessionStore`].
impl InMemorySessionStore {
    /// Creates a 64-way sharded bounded [`InMemorySessionStore`] capped at `max_sessions`.
    ///
    /// # Arguments
    ///
    /// - `usize` - Maximum concurrent tracked sessions.
    ///
    /// # Returns
    ///
    /// - `Self` - Ready sharded session store.
    pub fn new(max_sessions: usize) -> Self {
        let shards: SessionShardArray =
            Box::new(from_fn(|_: usize| RwLock::new(hash_map_xx_hash3_64())));
        Self {
            inner: Arc::new(InMemorySessionStoreInner {
                max_sessions: max_sessions.max(1),
                count: AtomicUsize::new(0),
                shards,
            }),
        }
    }

    /// Returns the number of tracked sessions across all shards in `O(1)` time.
    ///
    /// # Returns
    ///
    /// - `usize` - Active tracked session count.
    pub fn len(&self) -> usize {
        let Self { inner } = self;
        inner.count.load(Ordering::Relaxed)
    }

    /// Reports whether zero sessions are currently stored.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

/// Implements [`SessionStore`] for [`InMemorySessionStore`].
impl SessionStore for InMemorySessionStore {
    /// Loads an unexpired session from its single owning shard.
    ///
    /// # Arguments
    ///
    /// - `&'a self` - Shared reference to self.
    /// - `&'a str` - Session identifier.
    ///
    /// # Returns
    ///
    /// - `SessionFuture<'a, Option<HashMap<String, String>>>` - Attributes when live.
    fn load<'a>(
        &'a self,
        session_id: &'a str,
    ) -> SessionFuture<'a, Option<HashMap<String, String>>> {
        let Self { inner } = self;
        let idx: usize = session_shard_index(session_id);
        let now: u64 = session_now_secs();
        let loaded: Option<StoredSession> = inner.shards[idx].read().ok().and_then(
            |map: RwLockReadGuard<'_, HashMapXxHash3_64<String, StoredSession>>| {
                map.get(session_id).cloned()
            },
        );
        let outcome: Option<HashMap<String, String>> = match loaded {
            Some(entry) if now < entry.expires_at_secs => Some(entry.values),
            Some(_) => {
                if let Ok(mut map) = inner.shards[idx].write()
                    && map.remove(session_id).is_some()
                {
                    inner.count.fetch_sub(1, Ordering::Relaxed);
                }
                None
            }
            None => None,
        };
        Box::pin(async move { outcome })
    }

    /// Saves a session into its owning shard while enforcing `max_sessions`.
    ///
    /// # Arguments
    ///
    /// - `String` - Session identifier.
    /// - `HashMap<String, String>` - Session attributes.
    /// - `u64` - TTL in seconds.
    ///
    /// # Returns
    ///
    /// - `SessionFuture<'_, ()>` - Completion future.
    fn save(
        &self,
        session_id: String,
        values: HashMap<String, String>,
        ttl_secs: u64,
    ) -> SessionFuture<'_, ()> {
        let Self { inner } = self;
        let now: u64 = session_now_secs();
        if inner.count.load(Ordering::Relaxed) >= inner.max_sessions {
            let _: usize = self.purge_expired();
        }
        let idx: usize = session_shard_index(&session_id);
        if let Ok(mut map) = inner.shards[idx].write() {
            let existed: bool = map.contains_key(&session_id);
            if existed || inner.count.load(Ordering::Relaxed) < inner.max_sessions {
                map.insert(
                    session_id,
                    StoredSession {
                        values,
                        expires_at_secs: now.saturating_add(ttl_secs.max(1)),
                    },
                );
                if !existed {
                    inner.count.fetch_add(1, Ordering::Relaxed);
                }
            }
        }
        Box::pin(async {})
    }

    /// Deletes a session from its owning shard.
    ///
    /// # Arguments
    ///
    /// - `&'a self` - Shared reference to self.
    /// - `&'a str` - Session identifier.
    ///
    /// # Returns
    ///
    /// - `SessionFuture<'a, bool>` - `true` if deleted.
    fn delete<'a>(&'a self, session_id: &'a str) -> SessionFuture<'a, bool> {
        let Self { inner } = self;
        let idx: usize = session_shard_index(session_id);
        let removed: bool = inner.shards[idx]
            .write()
            .ok()
            .and_then(
                |mut map: RwLockWriteGuard<'_, HashMapXxHash3_64<String, StoredSession>>| {
                    map.remove(session_id)
                },
            )
            .is_some();
        if removed {
            inner.count.fetch_sub(1, Ordering::Relaxed);
        }
        Box::pin(async move { removed })
    }

    /// Evicts expired sessions across all 64 shards.
    ///
    /// # Returns
    ///
    /// - `usize` - Number of expired sessions purged.
    fn purge_expired(&self) -> usize {
        let Self { inner } = self;
        let now: u64 = session_now_secs();
        let mut evicted: usize = 0;
        for shard in inner.shards.iter() {
            if let Ok(mut map) = shard.write() {
                let before: usize = map.len();
                map.retain(|_: &String, entry: &mut StoredSession| now < entry.expires_at_secs);
                evicted += before.saturating_sub(map.len());
            }
        }
        if evicted > 0 {
            inner.count.fetch_sub(evicted, Ordering::Relaxed);
        }
        evicted
    }
}

/// Inherent implementation of [`SessionPlugin`].
impl SessionPlugin {
    /// Initializes the global [`SessionPlugin`] once with `config` and the default [`InMemorySessionStore`].
    ///
    /// # Arguments
    ///
    /// - `SessionConfig` - Cookie and session policy.
    ///
    /// # Returns
    ///
    /// - `&'static Self` - Reference to the initialized global plugin.
    pub fn init(config: SessionConfig) -> &'static Self {
        let store: SharedSessionStore =
            Arc::new(InMemorySessionStore::new(config.get_max_sessions()));
        Self::init_with_store(config, store)
    }

    /// Initializes the global [`SessionPlugin`] once with `config` and a custom [`SessionStore`].
    ///
    /// # Arguments
    ///
    /// - `SessionConfig` - Cookie and session policy.
    /// - `SharedSessionStore` - Pluggable session store backend.
    ///
    /// # Returns
    ///
    /// - `&'static Self` - Reference to the initialized global plugin.
    pub fn init_with_store(config: SessionConfig, store: SharedSessionStore) -> &'static Self {
        SESSION_PLUGIN.get_or_init(|| Self { config, store })
    }

    /// Returns the initialized global [`SessionPlugin`], if initialized.
    ///
    /// # Returns
    ///
    /// - `Option<&'static Self>` - Reference to the global plugin, or `None`.
    pub fn get() -> Option<&'static Self> {
        SESSION_PLUGIN.get()
    }

    /// Returns the active [`SessionConfig`].
    ///
    /// # Returns
    ///
    /// - `&SessionConfig` - Active session configuration.
    pub fn get_config(&self) -> &SessionConfig {
        &self.config
    }

    /// Returns the active [`SharedSessionStore`].
    ///
    /// # Returns
    ///
    /// - `&SharedSessionStore` - Active backing store.
    pub fn get_store(&self) -> &SharedSessionStore {
        &self.store
    }

    /// Retrieves the active [`SessionHandle`] stored in `ctx`.
    ///
    /// # Arguments
    ///
    /// - `&Context` - Request context.
    ///
    /// # Returns
    ///
    /// - `Option<SessionHandle>` - Per-request session handle, if session middleware ran.
    pub fn current(ctx: &Context) -> Option<SessionHandle> {
        ctx.try_get_attribute(ATTR_SESSION_HANDLE)
    }

    /// Formats a `Set-Cookie` header value for `session_id` (or an expiration cookie when `clear` is `true`).
    ///
    /// # Arguments
    ///
    /// - `&str` - Session identifier.
    /// - `bool` - When `true`, emits `Max-Age=0` with empty value to clear the client cookie.
    ///
    /// # Returns
    ///
    /// - `String` - Formatted `Set-Cookie` header value.
    pub fn build_set_cookie(&self, session_id: &str, clear: bool) -> String {
        let config: &SessionConfig = self.get_config();
        let value: String = if clear {
            String::new()
        } else if let Some(secret) = config.get_signing_secret() {
            sign_session_cookie(session_id, &secret)
        } else {
            session_id.to_owned()
        };
        let mut builder: CookieBuilder = CookieBuilder::new(config.get_cookie_name(), value);
        builder
            .set_path(config.get_cookie_path())
            .set_same_site(config.get_same_site());
        if let Some(domain) = config.get_cookie_domain() {
            builder.set_domain(domain);
        }
        if config.get_http_only() {
            builder.http_only();
        }
        if config.get_secure() {
            builder.secure();
        }
        if clear {
            builder.set_max_age(0i64);
        } else {
            builder.set_max_age(config.get_ttl_secs() as i64);
        }
        builder.build()
    }

    /// Loads or initializes the [`SessionHandle`] on `ctx` from the incoming `Cookie` header.
    ///
    /// # Arguments
    ///
    /// - `&mut Context` - Incoming request context.
    ///
    /// # Returns
    ///
    /// - `SessionHandle` - Active session handle attached to `ctx`.
    pub async fn load_into_context(&self, ctx: &mut Context) -> SessionHandle {
        let config: &SessionConfig = self.get_config();
        let candidate_id: Option<String> = ctx
            .get_request()
            .try_get_header_back(COOKIE)
            .and_then(|raw: String| {
                let cookies: Cookies = Cookie::parse(raw);
                cookies.get(config.get_cookie_name()).cloned()
            })
            .and_then(|raw_val: String| match config.get_signing_secret() {
                Some(secret) => verify_session_cookie(&raw_val, &secret),
                None => {
                    if raw_val.is_empty() {
                        None
                    } else {
                        Some(raw_val)
                    }
                }
            });
        let handle: SessionHandle = if let Some(sid) = candidate_id {
            if let Some(values) = self.get_store().load(&sid).await {
                SessionHandle::new(sid, values, true)
            } else {
                SessionHandle::new(generate_session_id(), HashMap::new(), false)
            }
        } else {
            SessionHandle::new(generate_session_id(), HashMap::new(), false)
        };
        ctx.set_attribute(ATTR_SESSION_HANDLE, handle.clone());
        handle
    }

    /// Persists dirty, regenerated, or destroyed session state and emits `Set-Cookie` on `ctx`.
    ///
    /// # Arguments
    ///
    /// - `&mut Context` - Outgoing request/response context.
    pub async fn persist_from_context(&self, ctx: &mut Context) {
        let Some(handle) = Self::current(ctx) else {
            return;
        };
        let Some(state) = handle.snapshot() else {
            return;
        };
        if !state.dirty {
            return;
        }
        if let Some(old_id) = state.previous_id {
            let _: bool = self.get_store().delete(&old_id).await;
        }
        if state.destroyed {
            let _: bool = self.get_store().delete(&state.id).await;
            let clear_cookie: String = self.build_set_cookie(&state.id, true);
            ctx.get_mut_response().add_header(SET_COOKIE, clear_cookie);
            return;
        }
        self.get_store()
            .save(
                state.id.clone(),
                state.values,
                self.get_config().get_ttl_secs(),
            )
            .await;
        let set_cookie: String = self.build_set_cookie(&state.id, false);
        ctx.get_mut_response().add_header(SET_COOKIE, set_cookie);
    }
}

/// Implements [`ServerHook`] for [`SessionRequestHook`].
impl ServerHook for SessionRequestHook {
    /// Constructs a per-request [`SessionRequestHook`].
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - Connection stream.
    /// - `&mut Context` - Request context.
    ///
    /// # Returns
    ///
    /// - `Self` - Hook instance.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    /// Loads or initializes the per-request [`SessionHandle`] into `ctx`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Hook instance.
    /// - `&mut Stream` - Connection stream.
    /// - `&mut Context` - Request context.
    ///
    /// # Returns
    ///
    /// - `Status` - Always `Status::Continue`.
    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        let Some(plugin) = SessionPlugin::get() else {
            return Status::Continue;
        };
        let _: SessionHandle = plugin.load_into_context(ctx).await;
        Status::Continue
    }
}

/// Implements [`ServerHook`] for [`SessionResponseHook`].
impl ServerHook for SessionResponseHook {
    /// Constructs a per-response [`SessionResponseHook`].
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - Connection stream.
    /// - `&mut Context` - Request context.
    ///
    /// # Returns
    ///
    /// - `Self` - Hook instance.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    /// Persists modified session state and attaches `Set-Cookie` only when dirty.
    ///
    /// # Arguments
    ///
    /// - `Self` - Hook instance.
    /// - `&mut Stream` - Connection stream.
    /// - `&mut Context` - Request context.
    ///
    /// # Returns
    ///
    /// - `Status` - Always `Status::Continue`.
    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        let Some(plugin) = SessionPlugin::get() else {
            return Status::Continue;
        };
        plugin.persist_from_context(ctx).await;
        Status::Continue
    }
}
