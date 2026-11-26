use super::*;

/// Formatting / debug-printing for [`RpcError`].
impl Display for RpcError {
    /// Formats the [`RpcError`] via the supplied formatter.
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter<'_>` - The formatter receiving the formatted output.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - A `fmt::Result` value.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "JSON-RPC error {}: {}",
            self.get_code(),
            self.get_message()
        )
    }
}

/// Implements [`Error`] for [`RpcError`].
impl Error for RpcError {}

/// Inherent implementation of [`RpcError`].
impl RpcError {
    /// Builds a JSON-RPC 2.0 Parse Error (`-32700`).
    ///
    /// # Returns
    ///
    /// - `Self` - Parse error object.
    pub fn parse_error() -> Self {
        Self::custom(RPC_PARSE_ERROR, MSG_PARSE_ERROR, None)
    }

    /// Builds a JSON-RPC 2.0 Invalid Request error (`-32600`).
    ///
    /// # Returns
    ///
    /// - `Self` - Invalid request error object.
    pub fn invalid_request() -> Self {
        Self::custom(RPC_INVALID_REQUEST, MSG_INVALID_REQUEST, None)
    }

    /// Builds a JSON-RPC 2.0 Method Not Found error (`-32601`).
    ///
    /// # Returns
    ///
    /// - `Self` - Method not found error object.
    pub fn method_not_found() -> Self {
        Self::custom(RPC_METHOD_NOT_FOUND, MSG_METHOD_NOT_FOUND, None)
    }

    /// Builds a JSON-RPC 2.0 Invalid Params error (`-32602`).
    ///
    /// # Arguments
    ///
    /// - `Option<Value>` - Optional detail describing the parameter validation failure.
    ///
    /// # Returns
    ///
    /// - `Self` - Invalid params error object.
    pub fn invalid_params(data: Option<Value>) -> Self {
        Self::custom(RPC_INVALID_PARAMS, MSG_INVALID_PARAMS, data)
    }

    /// Builds a JSON-RPC 2.0 Internal Error (`-32603`).
    ///
    /// # Arguments
    ///
    /// - `Option<Value>` - Optional detail payload.
    ///
    /// # Returns
    ///
    /// - `Self` - Internal error object.
    pub fn internal_error(data: Option<Value>) -> Self {
        Self::custom(RPC_INTERNAL_ERROR, MSG_INTERNAL_ERROR, data)
    }

    /// Builds a custom JSON-RPC error with explicit code, message, and optional data.
    ///
    /// # Arguments
    ///
    /// - `i64` - Numeric error code.
    /// - `M` - Error message convertible into `String`.
    /// - `Option<Value>` - Optional structured error payload.
    ///
    /// # Returns
    ///
    /// - `Self` - Custom error object.
    pub fn custom<M>(code: i64, message: M, data: Option<Value>) -> Self
    where
        M: Into<String>,
    {
        Self {
            code,
            message: message.into(),
            data,
        }
    }
}

/// Inherent implementation of [`RpcResponse`].
impl RpcResponse {
    /// Creates a success [`RpcResponse`] carrying `result`.
    ///
    /// # Arguments
    ///
    /// - `RpcId` - Request correlation identifier.
    /// - `Value` - Method return value.
    ///
    /// # Returns
    ///
    /// - `Self` - JSON-RPC 2.0 success response.
    pub fn success(id: RpcId, result: Value) -> Self {
        Self {
            jsonrpc: JSONRPC_VERSION.to_owned(),
            result: Some(result),
            error: None,
            id,
        }
    }

    /// Creates an error [`RpcResponse`] carrying `error`.
    ///
    /// # Arguments
    ///
    /// - `RpcId` - Request correlation identifier (`RpcId::Null` when unparseable).
    /// - `RpcError` - Structured error object.
    ///
    /// # Returns
    ///
    /// - `Self` - JSON-RPC 2.0 error response.
    pub fn failure(id: RpcId, error: RpcError) -> Self {
        Self {
            jsonrpc: JSONRPC_VERSION.to_owned(),
            result: None,
            error: Some(error),
            id,
        }
    }
}

/// Default construction for [`RpcConfig`].
impl Default for RpcConfig {
    /// Constructs a default [`RpcConfig`] value.
    fn default() -> Self {
        Self {
            endpoint: DEFAULT_RPC_ENDPOINT.to_owned(),
            max_body_size: DEFAULT_RPC_MAX_BODY_SIZE,
            max_batch_size: DEFAULT_RPC_MAX_BATCH_SIZE,
        }
    }
}

/// Inherent implementation of [`RpcRouter`].
impl RpcRouter {
    /// Creates an empty [`RpcRouter`].
    ///
    /// # Returns
    ///
    /// - `Self` - Empty router.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers an async method handler under `name`.
    ///
    /// # Arguments
    ///
    /// - `N` - Method name convertible into `String`.
    /// - `F` - Async callback accepting `Option<Value>` and returning `Fut`.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - This router for fluent chaining.
    pub fn register<N, F, Fut>(&mut self, name: N, handler: F) -> &mut Self
    where
        N: Into<String>,
        F: Fn(Option<Value>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Value, RpcError>> + Send + 'static,
    {
        let Self { methods } = self;
        let boxed: RpcHandler = Arc::new(move |params: Option<Value>| Box::pin(handler(params)));
        methods.insert(name.into(), boxed);
        self
    }

    /// Reports whether `name` is registered on this router.
    ///
    /// # Arguments
    ///
    /// - `&str` - Method name to check.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when registered.
    pub fn has_method(&self, name: &str) -> bool {
        let Self { methods } = self;
        methods.contains_key(name)
    }

    /// Returns a list of all registered method names.
    ///
    /// # Returns
    ///
    /// - `Vec<String>` - Registered method names.
    pub fn method_names(&self) -> Vec<String> {
        let Self { methods } = self;
        methods.keys().cloned().collect()
    }

    /// Dispatches one parsed JSON value representing a single JSON-RPC 2.0 call or notification.
    ///
    /// # Arguments
    ///
    /// - `Value` - Raw JSON object value.
    ///
    /// # Returns
    ///
    /// - `Option<RpcResponse>` - `None` for valid notifications, otherwise the response envelope.
    pub async fn dispatch_value(&self, raw: Value) -> Option<RpcResponse> {
        let Self { methods } = self;
        let req: RpcRequest = match from_value(raw) {
            Ok(parsed) => parsed,
            Err(_) => {
                return Some(RpcResponse::failure(
                    RpcId::Null,
                    RpcError::invalid_request(),
                ));
            }
        };
        if req.get_jsonrpc() != JSONRPC_VERSION || req.get_method().is_empty() {
            let id: RpcId = req.get_id().unwrap_or(RpcId::Null);
            return Some(RpcResponse::failure(id, RpcError::invalid_request()));
        }
        let Some(handler) = methods.get(req.get_method()) else {
            return req
                .get_id()
                .map(|id: RpcId| RpcResponse::failure(id, RpcError::method_not_found()));
        };
        let outcome: Result<Value, RpcError> = handler(req.get_params()).await;
        let id: RpcId = req.get_id()?;
        match outcome {
            Ok(value) => Some(RpcResponse::success(id, value)),
            Err(err) => Some(RpcResponse::failure(id, err)),
        }
    }

    /// Parses and dispatches raw JSON-RPC 2.0 request bytes (single call, notification, or batch).
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - Raw HTTP request payload bytes.
    /// - `&RpcConfig` - Batch and payload safety bounds.
    ///
    /// # Returns
    ///
    /// - `Option<Vec<u8>>` - Serialized JSON response bytes, or `None` when every call was a notification.
    pub async fn dispatch_bytes(&self, raw: &[u8], config: &RpcConfig) -> Option<Vec<u8>> {
        let parsed: Value = match from_slice(raw) {
            Ok(val) => val,
            Err(_) => {
                let err_resp: RpcResponse =
                    RpcResponse::failure(RpcId::Null, RpcError::parse_error());
                return Some(encode_rpc_response(&err_resp));
            }
        };
        match parsed {
            Value::Array(items) => {
                if items.is_empty() {
                    let err_resp: RpcResponse =
                        RpcResponse::failure(RpcId::Null, RpcError::invalid_request());
                    return Some(encode_rpc_response(&err_resp));
                }
                if items.len() > config.get_max_batch_size() {
                    let err_resp: RpcResponse = RpcResponse::failure(
                        RpcId::Null,
                        RpcError::custom(RPC_INVALID_REQUEST, MSG_BATCH_TOO_LARGE, None),
                    );
                    return Some(encode_rpc_response(&err_resp));
                }
                let mut responses: Vec<RpcResponse> = Vec::with_capacity(items.len());
                for item in items {
                    if let Some(resp) = self.dispatch_value(item).await {
                        responses.push(resp);
                    }
                }
                if responses.is_empty() {
                    None
                } else {
                    Some(to_vec(&responses).unwrap_or_default())
                }
            }
            single => self
                .dispatch_value(single)
                .await
                .map(|resp: RpcResponse| encode_rpc_response(&resp)),
        }
    }
}

/// Inherent implementation of [`RpcPlugin`].
impl RpcPlugin {
    /// Initializes the global [`RpcPlugin`] once with `config` and `router`.
    ///
    /// # Arguments
    ///
    /// - `RpcConfig` - JSON-RPC server configuration.
    /// - `RpcRouter` - Registered method router.
    ///
    /// # Returns
    ///
    /// - `&'static Self` - Reference to the initialized global plugin.
    pub fn init(config: RpcConfig, router: RpcRouter) -> &'static Self {
        RPC_PLUGIN.get_or_init(|| Self { config, router })
    }

    /// Returns the initialized global [`RpcPlugin`], if [`RpcPlugin::init`] was called.
    ///
    /// # Returns
    ///
    /// - `Option<&'static Self>` - Reference to the global plugin, or `None`.
    pub fn get() -> Option<&'static Self> {
        RPC_PLUGIN.get()
    }

    /// Returns the active [`RpcConfig`].
    ///
    /// # Returns
    ///
    /// - `&RpcConfig` - Active configuration.
    pub fn get_config(&self) -> &RpcConfig {
        &self.config
    }

    /// Returns the active [`RpcRouter`].
    ///
    /// # Returns
    ///
    /// - `&RpcRouter` - Registered method router.
    pub fn get_router(&self) -> &RpcRouter {
        &self.router
    }
}

/// Implements [`ServerHook`] for [`RpcHook`].
impl ServerHook for RpcHook {
    /// Constructs a per-request [`RpcHook`].
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

    /// Handles JSON-RPC 2.0 `POST` requests matching the configured endpoint.
    ///
    /// # Arguments
    ///
    /// - `Self` - Hook instance.
    /// - `&mut Stream` - Connection stream.
    /// - `&mut Context` - Request context.
    ///
    /// # Returns
    ///
    /// - `Status` - `Status::Continue` when unconfigured or path does not match; `Status::Stop` once answered.
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        let Some(plugin) = RpcPlugin::get() else {
            return Status::Continue;
        };
        let config: &RpcConfig = plugin.get_config();
        if ctx.get_request().get_path() != config.get_endpoint() {
            return Status::Continue;
        }
        if !ctx.get_request().get_method().is_post() {
            ctx.get_mut_response()
                .set_status_code(HttpStatus::MethodNotAllowed.code())
                .set_body(Vec::new());
            let wire: ResponseData = ctx.get_mut_response().build();
            let _: Result<(), ResponseError> = stream.try_send(&wire).await;
            return Status::Reject;
        }
        let body: &[u8] = ctx.get_request().get_body();
        if body.len() > config.get_max_body_size() {
            ctx.get_mut_response()
                .set_status_code(HttpStatus::PayloadTooLarge.code())
                .set_body(Vec::new());
            let wire: ResponseData = ctx.get_mut_response().build();
            let _: Result<(), ResponseError> = stream.try_send(&wire).await;
            return Status::Reject;
        }
        match plugin.get_router().dispatch_bytes(body, config).await {
            Some(payload) => {
                ctx.get_mut_response()
                    .set_status_code(HttpStatus::Ok.code())
                    .set_header(CONTENT_TYPE, APPLICATION_JSON)
                    .set_body(payload);
            }
            None => {
                ctx.get_mut_response()
                    .set_status_code(HttpStatus::NoContent.code())
                    .set_body(Vec::new());
            }
        }
        let wire: ResponseData = ctx.get_mut_response().build();
        let _: Result<(), ResponseError> = stream.try_send(&wire).await;
        Status::Reject
    }
}
