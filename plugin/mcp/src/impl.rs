use super::*;

/// Formatting / debug-printing for [`McpError`].
impl Display for McpError {
    /// Formats the [`McpError`] via the supplied formatter.
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
            Self::InvalidParams(msg) | Self::NotFound(msg) | Self::Internal(msg) => {
                f.write_str(msg)
            }
        }
    }
}

/// Implements [`Error`] for [`McpError`].
impl Error for McpError {}

/// Default construction for [`McpServerConfig`].
impl Default for McpServerConfig {
    /// Constructs a default [`McpServerConfig`] value.
    fn default() -> Self {
        Self {
            name: DEFAULT_MCP_SERVER_NAME.to_owned(),
            version: DEFAULT_MCP_SERVER_VERSION.to_owned(),
            protocol_version: DEFAULT_MCP_PROTOCOL_VERSION.to_owned(),
            endpoint: DEFAULT_MCP_ENDPOINT.to_owned(),
            allowed_origins: Vec::new(),
            max_sessions: DEFAULT_MCP_MAX_SESSIONS,
            max_body_size: DEFAULT_MCP_MAX_BODY_SIZE,
            session_ttl_ms: DEFAULT_MCP_SESSION_TTL_MS,
        }
    }
}

/// Inherent fluent builders for [`McpServerConfig`].
impl McpServerConfig {
    /// Returns a configuration with the given advertised server name.
    ///
    /// # Arguments
    ///
    /// - `N` - Server name convertible into `String`.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_name<N>(mut self, name: N) -> Self
    where
        N: Into<String>,
    {
        self.set_name(name.into());
        self
    }

    /// Returns a configuration with the given advertised server version.
    ///
    /// # Arguments
    ///
    /// - `V` - Version string convertible into `String`.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_version<V>(mut self, version: V) -> Self
    where
        V: Into<String>,
    {
        self.set_version(version.into());
        self
    }

    /// Returns a configuration with the given MCP endpoint path.
    ///
    /// # Arguments
    ///
    /// - `E` - Endpoint path convertible into `String`.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_endpoint<E>(mut self, endpoint: E) -> Self
    where
        E: Into<String>,
    {
        self.set_endpoint(endpoint.into());
        self
    }

    /// Returns a configuration with the given allowed `Origin` allowlist.
    ///
    /// # Arguments
    ///
    /// - `Vec<String>` - Allowed `Origin` header values.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_allowed_origins(mut self, allowed_origins: Vec<String>) -> Self {
        self.set_allowed_origins(allowed_origins);
        self
    }

    /// Returns a configuration with the given maximum concurrent session count.
    ///
    /// # Arguments
    ///
    /// - `usize` - Maximum tracked concurrent sessions.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_max_sessions(mut self, max_sessions: usize) -> Self {
        self.set_max_sessions(max_sessions);
        self
    }

    /// Returns a configuration with the given maximum incoming body size in bytes.
    ///
    /// # Arguments
    ///
    /// - `usize` - Maximum request body bytes.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_max_body_size(mut self, max_body_size: usize) -> Self {
        self.set_max_body_size(max_body_size);
        self
    }

    /// Returns a configuration with the given session idle TTL in milliseconds (`0` disables expiration).
    ///
    /// # Arguments
    ///
    /// - `u64` - Session TTL in milliseconds.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_session_ttl_ms(mut self, session_ttl_ms: u64) -> Self {
        self.set_session_ttl_ms(session_ttl_ms);
        self
    }
}

/// Inherent implementation of [`McpToolResult`].
impl McpToolResult {
    /// Creates a plain-text MCP tool result block.
    ///
    /// # Arguments
    ///
    /// - `T` - Text content convertible into `String`.
    ///
    /// # Returns
    ///
    /// - `Self` - Non-error text tool result.
    pub fn text<T>(text: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            content: vec![mcp_text_content(text.into())],
            is_error: false,
        }
    }

    /// Creates an error-flagged text MCP tool result block.
    ///
    /// # Arguments
    ///
    /// - `T` - Error message convertible into `String`.
    ///
    /// # Returns
    ///
    /// - `Self` - Tool result with `is_error == true`.
    pub fn error_text<T>(text: T) -> Self
    where
        T: Into<String>,
    {
        Self {
            content: vec![mcp_text_content(text.into())],
            is_error: true,
        }
    }
}

/// Default construction for [`McpRegistry`].
impl Default for McpRegistry {
    /// Constructs a default [`McpRegistry`] value.
    fn default() -> Self {
        Self::new()
    }
}

/// Inherent implementation of [`McpRegistry`].
impl McpRegistry {
    /// Creates an empty [`McpRegistry`] with a sharded session table.
    ///
    /// # Returns
    ///
    /// - `Self` - Ready MCP registry.
    pub fn new() -> Self {
        let mut shards: Vec<RwLock<HashMapXxHash3_64<String, Instant>>> =
            Vec::with_capacity(MCP_SESSION_SHARD_COUNT);
        for _ in 0..MCP_SESSION_SHARD_COUNT {
            shards.push(RwLock::new(hash_map_xx_hash3_64()));
        }
        Self {
            tools: hash_map_xx_hash3_64(),
            resources: hash_map_xx_hash3_64(),
            prompts: hash_map_xx_hash3_64(),
            session_shards: Arc::new(shards),
            session_count: Arc::new(AtomicUsize::new(0)),
            next_session_seq: Arc::new(AtomicU64::new(1)),
            session_seed: mcp_random_seed(),
        }
    }

    /// Registers an MCP tool and its async handler.
    ///
    /// # Arguments
    ///
    /// - `McpTool` - Tool metadata and JSON input schema.
    /// - `F` - Async handler invoked on `tools/call`.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - This registry for fluent chaining.
    pub fn register_tool<F, Fut>(&mut self, meta: McpTool, handler: F) -> &mut Self
    where
        F: Fn(Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<McpToolResult, McpError>> + Send + 'static,
    {
        let Self { tools, .. } = self;
        let key: String = meta.get_name().clone();
        let boxed: McpToolHandler = Arc::new(move |args: Value| Box::pin(handler(args)));
        tools.insert(
            key,
            RegisteredTool {
                meta,
                handler: boxed,
            },
        );
        self
    }

    /// Registers an MCP resource and its async content reader.
    ///
    /// # Arguments
    ///
    /// - `McpResource` - Resource metadata.
    /// - `F` - Async handler invoked on `resources/read`.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - This registry for fluent chaining.
    pub fn register_resource<F, Fut>(&mut self, meta: McpResource, handler: F) -> &mut Self
    where
        F: Fn(String) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<McpResourceContent, McpError>> + Send + 'static,
    {
        let Self { resources, .. } = self;
        let key: String = meta.get_uri().clone();
        let boxed: McpResourceHandler = Arc::new(move |uri: String| Box::pin(handler(uri)));
        resources.insert(
            key,
            RegisteredResource {
                meta,
                handler: boxed,
            },
        );
        self
    }

    /// Registers an MCP prompt template and its async renderer.
    ///
    /// # Arguments
    ///
    /// - `McpPrompt` - Prompt metadata and declared arguments.
    /// - `F` - Async handler invoked on `prompts/get`.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - This registry for fluent chaining.
    pub fn register_prompt<F, Fut>(&mut self, meta: McpPrompt, handler: F) -> &mut Self
    where
        F: Fn(Value) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Value, McpError>> + Send + 'static,
    {
        let Self { prompts, .. } = self;
        let key: String = meta.get_name().clone();
        let boxed: McpPromptHandler = Arc::new(move |args: Value| Box::pin(handler(args)));
        prompts.insert(
            key,
            RegisteredPrompt {
                meta,
                handler: boxed,
            },
        );
        self
    }

    /// Removes an active session identifier on `DELETE` teardown.
    ///
    /// # Arguments
    ///
    /// - `&str` - Session identifier from `Mcp-Session-Id`.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the session existed and was terminated.
    pub fn close_session(&self, session_id: &str) -> bool {
        let Self {
            session_shards,
            session_count,
            ..
        } = self;
        let shard_idx: usize = mcp_session_shard_index(session_id);
        let removed: bool = session_shards[shard_idx]
            .write()
            .map(
                |mut map: RwLockWriteGuard<'_, HashMapXxHash3_64<String, Instant>>| {
                    map.remove(session_id).is_some()
                },
            )
            .unwrap_or(false);
        if removed {
            session_count.fetch_sub(1, Ordering::Relaxed);
        }
        removed
    }

    /// Returns the number of currently active MCP sessions in `O(1)` time without locking.
    ///
    /// # Returns
    ///
    /// - `usize` - Count of live sessions.
    pub fn session_count(&self) -> usize {
        let Self { session_count, .. } = self;
        session_count.load(Ordering::Relaxed)
    }

    /// Evicts all sessions whose elapsed lifetime exceeds `ttl_ms`.
    ///
    /// # Arguments
    ///
    /// - `u64` - Session TTL in milliseconds (`0` disables expiration).
    ///
    /// # Returns
    ///
    /// - `usize` - Number of expired sessions removed.
    pub fn purge_expired_sessions(&self, ttl_ms: u64) -> usize {
        if ttl_ms == 0 {
            return 0;
        }
        let Self {
            session_shards,
            session_count,
            ..
        } = self;
        let ttl: Duration = Duration::from_millis(ttl_ms);
        let now: Instant = Instant::now();
        let mut evicted: usize = 0;
        for shard in session_shards.iter() {
            if let Ok(mut map) = shard.write() {
                let before: usize = map.len();
                map.retain(|_: &String, seen: &mut Instant| now.duration_since(*seen) <= ttl);
                evicted += before.saturating_sub(map.len());
            }
        }
        if evicted > 0 {
            session_count.fetch_sub(evicted, Ordering::Relaxed);
        }
        evicted
    }

    /// Allocates and stores one new session id within `max_sessions`, reclaiming expired ones first.
    ///
    /// # Arguments
    ///
    /// - `usize` - Maximum allowed concurrent sessions.
    /// - `u64` - Session TTL in milliseconds (`0` disables expiration).
    ///
    /// # Returns
    ///
    /// - `Option<String>` - Newly created session identifier, or `None` when at capacity.
    fn create_session(&self, max_sessions: usize, ttl_ms: u64) -> Option<String> {
        let Self {
            session_shards,
            session_count,
            next_session_seq,
            session_seed,
            ..
        } = self;
        if session_count.load(Ordering::Relaxed) >= max_sessions && ttl_ms > 0 {
            let _: usize = self.purge_expired_sessions(ttl_ms);
        }
        let mut observed: usize = session_count.load(Ordering::Relaxed);
        loop {
            if observed >= max_sessions {
                return None;
            }
            match session_count.compare_exchange_weak(
                observed,
                observed + 1,
                Ordering::AcqRel,
                Ordering::Relaxed,
            ) {
                Ok(_) => break,
                Err(actual) => observed = actual,
            }
        }
        let seq: u64 = next_session_seq.fetch_add(1, Ordering::Relaxed);
        let id: String = mint_mcp_session_id(*session_seed, seq);
        let shard_idx: usize = mcp_session_shard_index(&id);
        let Ok(mut map) = session_shards[shard_idx].write() else {
            session_count.fetch_sub(1, Ordering::Relaxed);
            return None;
        };
        map.insert(id.clone(), Instant::now());
        Some(id)
    }

    /// Checks whether `session_id` is active and unexpired in this registry.
    ///
    /// # Arguments
    ///
    /// - `&str` - Session identifier to verify.
    /// - `u64` - Session TTL in milliseconds (`0` disables expiration).
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when active and unexpired.
    fn has_session(&self, session_id: &str, ttl_ms: u64) -> bool {
        let Self {
            session_shards,
            session_count,
            ..
        } = self;
        let shard_idx: usize = mcp_session_shard_index(session_id);
        let seen_opt: Option<Instant> = session_shards[shard_idx].read().ok().and_then(
            |map: RwLockReadGuard<'_, HashMapXxHash3_64<String, Instant>>| {
                map.get(session_id).copied()
            },
        );
        let Some(seen) = seen_opt else {
            return false;
        };
        if ttl_ms == 0 {
            return true;
        }
        let ttl: Duration = Duration::from_millis(ttl_ms);
        if Instant::now().duration_since(seen) <= ttl {
            return true;
        }
        if let Ok(mut map) = session_shards[shard_idx].write()
            && map.remove(session_id).is_some()
        {
            session_count.fetch_sub(1, Ordering::Relaxed);
        }
        false
    }

    /// Dispatches one raw MCP JSON-RPC 2.0 request body with session enforcement.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - Incoming JSON request bytes.
    /// - `Option<&str>` - Optional `Mcp-Session-Id` header value.
    /// - `&McpServerConfig` - Server metadata and session caps.
    ///
    /// # Returns
    ///
    /// - `McpDispatchOutcome` - Serialized JSON response, `202 Accepted`, or session error.
    pub async fn handle_json_bytes(
        &self,
        raw: &[u8],
        session_id: Option<&str>,
        config: &McpServerConfig,
    ) -> McpDispatchOutcome {
        let Self {
            tools,
            resources,
            prompts,
            ..
        } = self;
        let parsed: Value = match from_slice(raw) {
            Ok(val) => val,
            Err(_) => {
                return McpDispatchOutcome::Response {
                    body: mcp_json_error(Value::Null, MCP_ERR_PARSE, MSG_MCP_PARSE_ERROR),
                    new_session_id: None,
                };
            }
        };
        let Some(obj) = parsed.as_object() else {
            return McpDispatchOutcome::Response {
                body: mcp_json_error(
                    Value::Null,
                    MCP_ERR_INVALID_REQUEST,
                    MSG_MCP_INVALID_REQUEST,
                ),
                new_session_id: None,
            };
        };
        let version_ok: bool = obj
            .get(KEY_JSONRPC)
            .and_then(Value::as_str)
            .is_some_and(|v: &str| v == MCP_JSONRPC_VERSION);
        let Some(method) = obj.get(KEY_METHOD).and_then(Value::as_str) else {
            let id: Value = obj.get(KEY_ID).cloned().unwrap_or(Value::Null);
            return McpDispatchOutcome::Response {
                body: mcp_json_error(id, MCP_ERR_INVALID_REQUEST, MSG_MCP_INVALID_REQUEST),
                new_session_id: None,
            };
        };
        if !version_ok {
            let id: Value = obj.get(KEY_ID).cloned().unwrap_or(Value::Null);
            return McpDispatchOutcome::Response {
                body: mcp_json_error(id, MCP_ERR_INVALID_REQUEST, MSG_MCP_INVALID_REQUEST),
                new_session_id: None,
            };
        }
        let id_opt: Option<Value> = obj.get(KEY_ID).cloned();
        let params: Value = obj.get(KEY_PARAMS).cloned().unwrap_or_else(|| json!({}));
        if method != METHOD_INITIALIZE
            && let Some(sid) = session_id
            && !self.has_session(sid, config.get_session_ttl_ms())
        {
            return McpDispatchOutcome::UnknownSession;
        }
        if method == METHOD_NOTIF_INITIALIZED || id_opt.is_none() {
            return McpDispatchOutcome::Accepted;
        }
        let id: Value = id_opt.unwrap_or(Value::Null);
        match method {
            METHOD_INITIALIZE => {
                let Some(new_sid) =
                    self.create_session(config.get_max_sessions(), config.get_session_ttl_ms())
                else {
                    return McpDispatchOutcome::SessionLimitReached;
                };
                let result: Value = mcp_initialize_result(config);
                McpDispatchOutcome::Response {
                    body: mcp_json_result(id, result),
                    new_session_id: Some(new_sid),
                }
            }
            METHOD_PING => McpDispatchOutcome::Response {
                body: mcp_json_result(id, json!({})),
                new_session_id: None,
            },
            METHOD_TOOLS_LIST => {
                let list: Vec<&McpTool> = tools
                    .values()
                    .map(|item: &RegisteredTool| &item.meta)
                    .collect();
                let list_val: Value = to_value(&list).unwrap_or(Value::Array(Vec::new()));
                McpDispatchOutcome::Response {
                    body: mcp_json_result(id, mcp_single_field_object(KEY_TOOLS, list_val)),
                    new_session_id: None,
                }
            }
            METHOD_TOOLS_CALL => {
                let Some(name) = params.get(KEY_NAME).and_then(Value::as_str) else {
                    return McpDispatchOutcome::Response {
                        body: mcp_json_error(id, MCP_ERR_INVALID_PARAMS, MSG_MCP_INVALID_PARAMS),
                        new_session_id: None,
                    };
                };
                let Some(entry) = tools.get(name) else {
                    return McpDispatchOutcome::Response {
                        body: mcp_json_error(id, MCP_ERR_METHOD_NOT_FOUND, MSG_MCP_UNKNOWN_TOOL),
                        new_session_id: None,
                    };
                };
                let args: Value = params
                    .get(KEY_ARGUMENTS)
                    .cloned()
                    .unwrap_or_else(|| json!({}));
                let body: Vec<u8> = match (entry.handler)(args).await {
                    Ok(res) => mcp_json_result(id, json!(res)),
                    Err(McpError::InvalidParams(msg)) => {
                        mcp_json_error(id, MCP_ERR_INVALID_PARAMS, &msg)
                    }
                    Err(McpError::NotFound(msg)) => {
                        mcp_json_error(id, MCP_ERR_METHOD_NOT_FOUND, &msg)
                    }
                    Err(McpError::Internal(msg)) => mcp_json_error(id, MCP_ERR_INTERNAL, &msg),
                };
                McpDispatchOutcome::Response {
                    body,
                    new_session_id: None,
                }
            }
            METHOD_RESOURCES_LIST => {
                let list: Vec<&McpResource> = resources
                    .values()
                    .map(|item: &RegisteredResource| &item.meta)
                    .collect();
                let list_val: Value = to_value(&list).unwrap_or(Value::Array(Vec::new()));
                McpDispatchOutcome::Response {
                    body: mcp_json_result(id, mcp_single_field_object(KEY_RESOURCES, list_val)),
                    new_session_id: None,
                }
            }
            METHOD_RESOURCES_READ => {
                let Some(uri) = params.get(KEY_URI).and_then(Value::as_str) else {
                    return McpDispatchOutcome::Response {
                        body: mcp_json_error(id, MCP_ERR_INVALID_PARAMS, MSG_MCP_INVALID_PARAMS),
                        new_session_id: None,
                    };
                };
                let Some(entry) = resources.get(uri) else {
                    return McpDispatchOutcome::Response {
                        body: mcp_json_error(
                            id,
                            MCP_ERR_METHOD_NOT_FOUND,
                            MSG_MCP_UNKNOWN_RESOURCE,
                        ),
                        new_session_id: None,
                    };
                };
                let body: Vec<u8> = match (entry.handler)(uri.to_owned()).await {
                    Ok(content) => {
                        let contents_val: Value =
                            to_value([content]).unwrap_or(Value::Array(Vec::new()));
                        mcp_json_result(id, mcp_single_field_object(KEY_CONTENTS, contents_val))
                    }
                    Err(McpError::InvalidParams(msg)) => {
                        mcp_json_error(id, MCP_ERR_INVALID_PARAMS, &msg)
                    }
                    Err(McpError::NotFound(msg)) => {
                        mcp_json_error(id, MCP_ERR_METHOD_NOT_FOUND, &msg)
                    }
                    Err(McpError::Internal(msg)) => mcp_json_error(id, MCP_ERR_INTERNAL, &msg),
                };
                McpDispatchOutcome::Response {
                    body,
                    new_session_id: None,
                }
            }
            METHOD_PROMPTS_LIST => {
                let list: Vec<&McpPrompt> = prompts
                    .values()
                    .map(|item: &RegisteredPrompt| &item.meta)
                    .collect();
                let list_val: Value = to_value(&list).unwrap_or(Value::Array(Vec::new()));
                McpDispatchOutcome::Response {
                    body: mcp_json_result(id, mcp_single_field_object(KEY_PROMPTS, list_val)),
                    new_session_id: None,
                }
            }
            METHOD_PROMPTS_GET => {
                let Some(name) = params.get(KEY_NAME).and_then(Value::as_str) else {
                    return McpDispatchOutcome::Response {
                        body: mcp_json_error(id, MCP_ERR_INVALID_PARAMS, MSG_MCP_INVALID_PARAMS),
                        new_session_id: None,
                    };
                };
                let Some(entry) = prompts.get(name) else {
                    return McpDispatchOutcome::Response {
                        body: mcp_json_error(id, MCP_ERR_METHOD_NOT_FOUND, MSG_MCP_UNKNOWN_PROMPT),
                        new_session_id: None,
                    };
                };
                let args: Value = params
                    .get(KEY_ARGUMENTS)
                    .cloned()
                    .unwrap_or_else(|| json!({}));
                let body: Vec<u8> = match (entry.handler)(args).await {
                    Ok(rendered) => mcp_json_result(id, rendered),
                    Err(McpError::InvalidParams(msg)) => {
                        mcp_json_error(id, MCP_ERR_INVALID_PARAMS, &msg)
                    }
                    Err(McpError::NotFound(msg)) => {
                        mcp_json_error(id, MCP_ERR_METHOD_NOT_FOUND, &msg)
                    }
                    Err(McpError::Internal(msg)) => mcp_json_error(id, MCP_ERR_INTERNAL, &msg),
                };
                McpDispatchOutcome::Response {
                    body,
                    new_session_id: None,
                }
            }
            _ => McpDispatchOutcome::Response {
                body: mcp_json_error(id, MCP_ERR_METHOD_NOT_FOUND, MSG_MCP_METHOD_NOT_FOUND),
                new_session_id: None,
            },
        }
    }
}

/// Inherent implementation of [`McpPlugin`].
impl McpPlugin {
    /// Initializes the global [`McpPlugin`] once with `config` and `registry`.
    ///
    /// # Arguments
    ///
    /// - `McpServerConfig` - MCP server settings and allowed origins.
    /// - `McpRegistry` - Tool, resource, and prompt registry.
    ///
    /// # Returns
    ///
    /// - `&'static Self` - Reference to the initialized global plugin.
    pub fn init(config: McpServerConfig, registry: McpRegistry) -> &'static Self {
        MCP_PLUGIN.get_or_init(|| Self { config, registry })
    }

    /// Returns the initialized global [`McpPlugin`], if [`McpPlugin::init`] was called.
    ///
    /// # Returns
    ///
    /// - `Option<&'static Self>` - Reference to the global plugin, or `None`.
    pub fn get() -> Option<&'static Self> {
        MCP_PLUGIN.get()
    }

    /// Returns the active [`McpServerConfig`].
    ///
    /// # Returns
    ///
    /// - `&McpServerConfig` - Active configuration.
    pub fn get_config(&self) -> &McpServerConfig {
        &self.config
    }

    /// Returns the active [`McpRegistry`].
    ///
    /// # Returns
    ///
    /// - `&McpRegistry` - Registered tools, resources, prompts, and sessions.
    pub fn get_registry(&self) -> &McpRegistry {
        &self.registry
    }
}

/// Implements [`ServerHook`] for [`McpHook`].
impl ServerHook for McpHook {
    /// Constructs a per-request [`McpHook`].
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

    /// Handles MCP Streamable HTTP (`POST` and `DELETE`) requests on the configured endpoint.
    ///
    /// # Arguments
    ///
    /// - `Self` - Hook instance.
    /// - `&mut Stream` - Connection stream.
    /// - `&mut Context` - Request context.
    ///
    /// # Returns
    ///
    /// - `Status` - `Status::Continue` when unconfigured or path does not match; `Status::Reject` once answered.
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        let Some(plugin) = McpPlugin::get() else {
            return Status::Continue;
        };
        let config: &McpServerConfig = plugin.get_config();
        if ctx.get_request().get_path() != config.get_endpoint() {
            return Status::Continue;
        }
        let origin: Option<String> = ctx.get_request().try_get_header_back(ORIGIN);
        if !is_origin_allowed(origin.as_deref(), config.get_allowed_origins()) {
            ctx.get_mut_response()
                .set_status_code(HttpStatus::Forbidden.code())
                .set_body(Vec::new());
            let wire: ResponseData = ctx.get_mut_response().build();
            let _: Result<(), ResponseError> = stream.try_send(&wire).await;
            return Status::Reject;
        }
        let session_id: Option<String> =
            ctx.get_request().try_get_header_back(MCP_SESSION_ID_HEADER);
        if ctx.get_request().get_method().is_delete() {
            let closed: bool = session_id
                .as_deref()
                .is_some_and(|sid: &str| plugin.get_registry().close_session(sid));
            let code: usize = if closed {
                HttpStatus::NoContent.code()
            } else {
                HttpStatus::NotFound.code()
            };
            ctx.get_mut_response()
                .set_status_code(code)
                .set_body(Vec::new());
            let wire: ResponseData = ctx.get_mut_response().build();
            let _: Result<(), ResponseError> = stream.try_send(&wire).await;
            return Status::Reject;
        }
        if !ctx.get_request().get_method().is_post() {
            ctx.get_mut_response()
                .set_status_code(HttpStatus::MethodNotAllowed.code())
                .set_body(Vec::new());
            let wire: ResponseData = ctx.get_mut_response().build();
            let _: Result<(), ResponseError> = stream.try_send(&wire).await;
            return Status::Reject;
        }
        if ctx.get_request().get_body().len() > config.get_max_body_size() {
            ctx.get_mut_response()
                .set_status_code(HttpStatus::PayloadTooLarge.code())
                .set_body(Vec::new());
            let wire: ResponseData = ctx.get_mut_response().build();
            let _: Result<(), ResponseError> = stream.try_send(&wire).await;
            return Status::Reject;
        }
        let outcome: McpDispatchOutcome = plugin
            .get_registry()
            .handle_json_bytes(ctx.get_request().get_body(), session_id.as_deref(), config)
            .await;
        match outcome {
            McpDispatchOutcome::Response {
                body: resp_body,
                new_session_id,
            } => {
                ctx.get_mut_response()
                    .set_status_code(HttpStatus::Ok.code())
                    .set_header(CONTENT_TYPE, APPLICATION_JSON)
                    .set_body(resp_body);
                if let Some(sid) = new_session_id {
                    ctx.get_mut_response()
                        .set_header(MCP_SESSION_ID_HEADER, sid);
                }
            }
            McpDispatchOutcome::Accepted => {
                ctx.get_mut_response()
                    .set_status_code(HttpStatus::Accepted.code())
                    .set_body(Vec::new());
            }
            McpDispatchOutcome::UnknownSession => {
                ctx.get_mut_response()
                    .set_status_code(HttpStatus::NotFound.code())
                    .set_body(Vec::new());
            }
            McpDispatchOutcome::SessionLimitReached => {
                ctx.get_mut_response()
                    .set_status_code(HttpStatus::ServiceUnavailable.code())
                    .set_header(CONTENT_TYPE, APPLICATION_JSON)
                    .set_body(mcp_json_error(
                        Value::Null,
                        MCP_ERR_INTERNAL,
                        MSG_MCP_SESSION_LIMIT,
                    ));
            }
        }
        let wire: ResponseData = ctx.get_mut_response().build();
        let _: Result<(), ResponseError> = stream.try_send(&wire).await;
        Status::Reject
    }
}
