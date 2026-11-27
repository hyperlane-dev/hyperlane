use super::*;

/// Configuration controlling cookie attributes, signing, TTL, and capacity for [`SessionPlugin`].
#[derive(Clone, Data, Debug, Eq, PartialEq)]
pub struct SessionConfig {
    /// Cookie name carrying the session token (default `"hyperlane_sid"`).
    pub(crate) cookie_name: String,
    /// Cookie `Path` attribute (default `"/"`).
    pub(crate) cookie_path: String,
    /// Optional cookie `Domain` attribute.
    #[get(type(clone))]
    pub(crate) cookie_domain: Option<String>,
    /// Cookie `SameSite` policy (`"Lax"`, `"Strict"`, `"None"`).
    pub(crate) same_site: String,
    /// Sets `HttpOnly` flag on session cookies when `true`.
    #[get(type(copy))]
    pub(crate) http_only: bool,
    /// Sets `Secure` flag on session cookies when `true`.
    #[get(type(copy))]
    pub(crate) secure: bool,
    /// Session idle TTL in seconds (also emitted as cookie `Max-Age`).
    #[get(type(copy))]
    pub(crate) ttl_secs: u64,
    /// Maximum active sessions allowed in the built-in [`InMemorySessionStore`].
    #[get(type(copy))]
    pub(crate) max_sessions: usize,
    /// Optional HMAC-SHA256 secret signing cookie values (`<sid>.<mac>`).
    #[get(type(clone))]
    pub(crate) signing_secret: Option<Vec<u8>>,
}

/// Mutable per-request session state tracking dirty writes, regeneration, and destruction.
#[derive(Clone, Debug)]
pub(crate) struct SessionState {
    /// Current session identifier.
    pub(crate) id: String,
    /// Previous session identifier deleted from the store after [`SessionHandle::regenerate_id`].
    pub(crate) previous_id: Option<String>,
    /// Key-value session attributes.
    pub(crate) values: HashMap<String, String>,
    /// `true` when loaded from an existing persisted session.
    pub(crate) existed: bool,
    /// `true` when modified during this request (drives zero-write read optimization).
    pub(crate) dirty: bool,
    /// `true` when [`SessionHandle::destroy`] was called.
    pub(crate) destroyed: bool,
}

/// Thread-safe handle stored in Hyperlane [`Context`] allowing handlers to read and mutate the active session.
#[derive(Clone, Debug)]
pub struct SessionHandle {
    /// Protected per-request session state.
    pub(crate) state: Arc<Mutex<SessionState>>,
}

/// Internal record stored inside each shard of [`InMemorySessionStore`].
#[derive(Clone)]
pub(crate) struct StoredSession {
    /// Persisted key-value map.
    pub(crate) values: HashMap<String, String>,
    /// Expiration Unix timestamp in seconds.
    pub(crate) expires_at_secs: u64,
}

/// Shared interior state of [`InMemorySessionStore`].
pub(crate) struct InMemorySessionStoreInner {
    /// Maximum tracked concurrent sessions across all shards.
    pub(crate) max_sessions: usize,
    /// Lock-free active session counter.
    pub(crate) count: AtomicUsize,
    /// 64-way lock-striped session tables.
    pub(crate) shards: SessionShardArray,
}

/// High-concurrency 64-way sharded bounded in-memory [`SessionStore`].
#[derive(Clone)]
pub struct InMemorySessionStore {
    /// Shared sharded store state.
    pub(crate) inner: Arc<InMemorySessionStoreInner>,
}

/// Process-wide [`SessionPlugin`] singleton storing [`SessionConfig`] and a pluggable [`SessionStore`].
#[derive(Clone)]
pub struct SessionPlugin {
    /// Cookie, signing, and TTL configuration.
    pub(crate) config: SessionConfig,
    /// Backing session store.
    pub(crate) store: SharedSessionStore,
}

/// Opt-in request middleware loading the incoming session cookie into [`SessionHandle`].
#[derive(Clone, Copy, Debug, Default)]
#[hyperlane::request_middleware(-85)]
pub struct SessionRequestHook;

/// Opt-in response middleware persisting modified/regenerated/destroyed sessions and emitting `Set-Cookie`.
#[derive(Clone, Copy, Debug, Default)]
#[hyperlane::response_middleware(-70)]
pub struct SessionResponseHook;
