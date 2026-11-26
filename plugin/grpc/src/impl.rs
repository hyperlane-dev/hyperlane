use super::*;

/// Inherent implementation of [`GrpcStatus`].
impl GrpcStatus {
    /// Returns the canonical numeric gRPC status code (`0..=16`).
    ///
    /// # Returns
    ///
    /// - `u8` - Numeric status code.
    pub fn code(self) -> u8 {
        self as u8
    }

    /// Converts a numeric code (`0..=16`) back into a [`GrpcStatus`].
    ///
    /// # Arguments
    ///
    /// - `u8` - Numeric gRPC status code.
    ///
    /// # Returns
    ///
    /// - `Self` - Matching status variant, or [`GrpcStatus::Unknown`] for unrecognised codes.
    pub fn from_code(code: u8) -> Self {
        match code {
            0 => Self::Ok,
            1 => Self::Cancelled,
            2 => Self::Unknown,
            3 => Self::InvalidArgument,
            4 => Self::DeadlineExceeded,
            5 => Self::NotFound,
            6 => Self::AlreadyExists,
            7 => Self::PermissionDenied,
            8 => Self::ResourceExhausted,
            9 => Self::FailedPrecondition,
            10 => Self::Aborted,
            11 => Self::OutOfRange,
            12 => Self::Unimplemented,
            13 => Self::Internal,
            14 => Self::Unavailable,
            15 => Self::DataLoss,
            16 => Self::Unauthenticated,
            _ => Self::Unknown,
        }
    }
}

/// Formatting / debug-printing for [`GrpcError`].
impl Display for GrpcError {
    /// Formats the [`GrpcError`] via the supplied formatter.
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
            "gRPC status {}: {}",
            self.get_status().code(),
            self.get_message()
        )
    }
}

/// Implements [`Error`] for [`GrpcError`].
impl Error for GrpcError {}

/// Inherent implementation of [`GrpcError`].
impl GrpcError {
    /// Creates a new [`GrpcError`] with the supplied status code and message.
    ///
    /// # Arguments
    ///
    /// - `GrpcStatus` - Canonical gRPC status code.
    /// - `M` - Error message convertible into `String`.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed gRPC error.
    pub fn new<M>(status: GrpcStatus, message: M) -> Self
    where
        M: Into<String>,
    {
        Self {
            status,
            message: message.into(),
        }
    }

    /// Returns the percent-encoded `grpc-message` header value.
    ///
    /// # Returns
    ///
    /// - `String` - Wire-safe percent-encoded message.
    pub fn encoded_message(&self) -> String {
        percent_encode_grpc_message(self.get_message())
    }
}

/// Default construction for [`GrpcConfig`].
impl Default for GrpcConfig {
    /// Constructs a default [`GrpcConfig`] value.
    fn default() -> Self {
        Self {
            max_message_size: DEFAULT_GRPC_MAX_MESSAGE_SIZE,
            compression: Compress::Unknown,
        }
    }
}

/// Inherent implementation of [`GrpcFrame`].
impl GrpcFrame {
    /// Creates an uncompressed [`GrpcFrame`] carrying `payload`.
    ///
    /// # Arguments
    ///
    /// - `Vec<u8>` - Raw message payload bytes.
    ///
    /// # Returns
    ///
    /// - `Self` - Uncompressed frame.
    pub fn new(payload: Vec<u8>) -> Self {
        Self {
            compressed: false,
            payload,
        }
    }

    /// Encodes this frame into the 5-byte gRPC Length-Prefixed-Message wire format.
    ///
    /// # Arguments
    ///
    /// - `Compress` - Compression coding to apply (`Compress::Unknown` sends uncompressed).
    ///
    /// # Returns
    ///
    /// - `Result<Vec<u8>, GrpcError>` - Wire bytes, or [`GrpcStatus::ResourceExhausted`] on length overflow.
    pub fn encode(&self, encoding: Compress) -> Result<Vec<u8>, GrpcError> {
        let (flag, body): (u8, Vec<u8>) = if matches!(encoding, Compress::Unknown) {
            (0u8, self.get_payload().clone())
        } else {
            let compressed: Vec<u8> = encoding
                .encode(self.get_payload(), GRPC_DECOMPRESS_BUF_SIZE)
                .into_owned();
            (1u8, compressed)
        };
        let len_u32: u32 = u32::try_from(body.len())
            .map_err(|_| GrpcError::new(GrpcStatus::ResourceExhausted, ERR_GRPC_MSG_TOO_LARGE))?;
        let mut wire: Vec<u8> = Vec::with_capacity(GRPC_FRAME_HEADER_LEN + body.len());
        wire.push(flag);
        wire.extend_from_slice(&len_u32.to_be_bytes());
        wire.extend_from_slice(&body);
        Ok(wire)
    }

    /// Decodes one 5-byte length-prefixed gRPC wire frame enforcing `max_message_size` and decompression caps.
    ///
    /// # Arguments
    ///
    /// - `&[u8]` - Raw HTTP/2 request body bytes.
    /// - `Compress` - Negotiated `grpc-encoding` coding (`Compress::Unknown` for identity).
    /// - `usize` - Maximum allowed encoded and decoded payload length.
    ///
    /// # Returns
    ///
    /// - `Result<Self, GrpcError>` - Decoded frame or canonical [`GrpcError`].
    pub fn decode(
        raw: &[u8],
        encoding: Compress,
        max_message_size: usize,
    ) -> Result<Self, GrpcError> {
        if raw.len() < GRPC_FRAME_HEADER_LEN {
            return Err(GrpcError::new(
                GrpcStatus::Internal,
                ERR_GRPC_TRUNCATED_HEADER,
            ));
        }
        let flag: u8 = raw[0];
        if flag > 1 {
            return Err(GrpcError::new(GrpcStatus::Internal, ERR_GRPC_INVALID_FLAG));
        }
        let declared_len: usize = u32::from_be_bytes([raw[1], raw[2], raw[3], raw[4]]) as usize;
        if declared_len > max_message_size {
            return Err(GrpcError::new(
                GrpcStatus::ResourceExhausted,
                ERR_GRPC_MSG_TOO_LARGE,
            ));
        }
        let body_end: usize = GRPC_FRAME_HEADER_LEN.saturating_add(declared_len);
        if raw.len() < body_end {
            return Err(GrpcError::new(
                GrpcStatus::Internal,
                ERR_GRPC_TRUNCATED_BODY,
            ));
        }
        let frame_slice: &[u8] = &raw[GRPC_FRAME_HEADER_LEN..body_end];
        if flag == 0 {
            return Ok(Self {
                compressed: false,
                payload: frame_slice.to_vec(),
            });
        }
        if matches!(encoding, Compress::Unknown) {
            return Err(GrpcError::new(
                GrpcStatus::Unimplemented,
                ERR_GRPC_UNSUPPORTED_ENCODING,
            ));
        }
        let decoded: Vec<u8> = encoding
            .decode(frame_slice, GRPC_DECOMPRESS_BUF_SIZE)
            .into_owned();
        if decoded.len() > max_message_size {
            return Err(GrpcError::new(
                GrpcStatus::ResourceExhausted,
                ERR_GRPC_MSG_TOO_LARGE,
            ));
        }
        Ok(Self {
            compressed: true,
            payload: decoded,
        })
    }
}

/// Inherent implementation of [`GrpcRouter`].
impl GrpcRouter {
    /// Creates an empty [`GrpcRouter`].
    ///
    /// # Returns
    ///
    /// - `Self` - Empty service router.
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a unary async gRPC method handler under full path `/package.Service/Method`.
    ///
    /// # Arguments
    ///
    /// - `P` - Full gRPC method path convertible into `String`.
    /// - `F` - Async handler receiving decoded request bytes and returning response bytes.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - This router for fluent chaining.
    pub fn register<P, F, Fut>(&mut self, path: P, handler: F) -> &mut Self
    where
        P: Into<String>,
        F: Fn(Vec<u8>) -> Fut + Send + Sync + 'static,
        Fut: Future<Output = Result<Vec<u8>, GrpcError>> + Send + 'static,
    {
        let Self { methods } = self;
        let boxed: GrpcHandler = Arc::new(move |payload: Vec<u8>| Box::pin(handler(payload)));
        methods.insert(path.into(), boxed);
        self
    }

    /// Reports whether `path` is registered on this router.
    ///
    /// # Arguments
    ///
    /// - `&str` - Full gRPC path.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when registered.
    pub fn has_method(&self, path: &str) -> bool {
        let Self { methods } = self;
        methods.contains_key(path)
    }

    /// Dispatches one gRPC unary call with frame decoding, optional timeout, and response framing.
    ///
    /// # Arguments
    ///
    /// - `&str` - Full gRPC method path.
    /// - `&[u8]` - Raw HTTP/2 request body bytes.
    /// - `Compress` - Incoming `grpc-encoding` coding.
    /// - `Option<Duration>` - Optional client `grpc-timeout` budget.
    /// - `&GrpcConfig` - Max message size and outgoing compression config.
    ///
    /// # Returns
    ///
    /// - `(GrpcStatus, String, Vec<u8>)` - `(status, percent_encoded_message, framed_response_body)`.
    pub async fn dispatch(
        &self,
        path: &str,
        raw_body: &[u8],
        req_encoding: Compress,
        deadline: Option<Duration>,
        config: &GrpcConfig,
    ) -> (GrpcStatus, String, Vec<u8>) {
        let Self { methods } = self;
        let Some(handler) = methods.get(path) else {
            return (
                GrpcStatus::Unimplemented,
                percent_encode_grpc_message(ERR_GRPC_UNIMPLEMENTED),
                Vec::new(),
            );
        };
        let frame: GrpcFrame =
            match GrpcFrame::decode(raw_body, req_encoding, config.get_max_message_size()) {
                Ok(decoded) => decoded,
                Err(err) => return (err.get_status(), err.encoded_message(), Vec::new()),
            };
        let outcome: Result<Vec<u8>, GrpcError> = match deadline {
            Some(limit) => match timeout(limit, handler(frame.payload)).await {
                Ok(res) => res,
                Err(_) => Err(GrpcError::new(
                    GrpcStatus::DeadlineExceeded,
                    ERR_GRPC_DEADLINE_EXCEEDED,
                )),
            },
            None => handler(frame.payload).await,
        };
        match outcome {
            Ok(resp_payload) => {
                if resp_payload.len() > config.get_max_message_size() {
                    return (
                        GrpcStatus::ResourceExhausted,
                        percent_encode_grpc_message(ERR_GRPC_MSG_TOO_LARGE),
                        Vec::new(),
                    );
                }
                match GrpcFrame::new(resp_payload).encode(config.get_compression()) {
                    Ok(wire) => (GrpcStatus::Ok, String::new(), wire),
                    Err(err) => (err.get_status(), err.encoded_message(), Vec::new()),
                }
            }
            Err(err) => (err.get_status(), err.encoded_message(), Vec::new()),
        }
    }
}

/// Inherent implementation of [`GrpcPlugin`].
impl GrpcPlugin {
    /// Initializes the global [`GrpcPlugin`] once with `config` and `router`.
    ///
    /// # Arguments
    ///
    /// - `GrpcConfig` - gRPC plugin configuration.
    /// - `GrpcRouter` - Registered service router.
    ///
    /// # Returns
    ///
    /// - `&'static Self` - Reference to the initialized global plugin.
    pub fn init(config: GrpcConfig, router: GrpcRouter) -> &'static Self {
        GRPC_PLUGIN.get_or_init(|| Self { config, router })
    }

    /// Returns the initialized global [`GrpcPlugin`], if [`GrpcPlugin::init`] was called.
    ///
    /// # Returns
    ///
    /// - `Option<&'static Self>` - Reference to the global plugin, or `None`.
    pub fn get() -> Option<&'static Self> {
        GRPC_PLUGIN.get()
    }

    /// Returns the active [`GrpcConfig`].
    ///
    /// # Returns
    ///
    /// - `&GrpcConfig` - Active configuration.
    pub fn get_config(&self) -> &GrpcConfig {
        &self.config
    }

    /// Returns the active [`GrpcRouter`].
    ///
    /// # Returns
    ///
    /// - `&GrpcRouter` - Registered service router.
    pub fn get_router(&self) -> &GrpcRouter {
        &self.router
    }
}

/// Implements [`ServerHook`] for [`GrpcHook`].
impl ServerHook for GrpcHook {
    /// Constructs a per-request [`GrpcHook`].
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

    /// Dispatches incoming gRPC requests carrying `Content-Type: application/grpc*`.
    ///
    /// # Arguments
    ///
    /// - `Self` - Hook instance.
    /// - `&mut Stream` - Connection stream.
    /// - `&mut Context` - Request context.
    ///
    /// # Returns
    ///
    /// - `Status` - `Status::Continue` when unconfigured or non-gRPC; `Status::Stop` once answered.
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        let Some(plugin) = GrpcPlugin::get() else {
            return Status::Continue;
        };
        let is_grpc: bool = ctx
            .get_request()
            .try_get_header_back(CONTENT_TYPE)
            .is_some_and(|val: String| is_grpc_content_type(&val));
        if !is_grpc {
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
        let path: String = ctx.get_request().get_path().clone();
        let req_encoding: Compress = parse_grpc_encoding(
            ctx.get_request()
                .try_get_header_back(GRPC_ENCODING_HEADER)
                .as_deref(),
        );
        let deadline: Option<Duration> = ctx
            .get_request()
            .try_get_header_back(GRPC_TIMEOUT_HEADER)
            .and_then(|val: String| parse_grpc_timeout(&val));
        let raw_body: Vec<u8> = ctx.get_request().get_body().clone();
        let config: &GrpcConfig = plugin.get_config();
        let (status, encoded_msg, wire_body): (GrpcStatus, String, Vec<u8>) = plugin
            .get_router()
            .dispatch(&path, &raw_body, req_encoding, deadline, config)
            .await;
        let response: &mut Response = ctx.get_mut_response();
        response
            .set_status_code(HttpStatus::Ok.code())
            .set_header(CONTENT_TYPE, GRPC_CONTENT_TYPE)
            .set_header(GRPC_STATUS_HEADER, status.code().to_string())
            .set_body(wire_body);
        if !encoded_msg.is_empty() {
            response.set_header(GRPC_MESSAGE_HEADER, encoded_msg);
        }
        if !matches!(config.get_compression(), Compress::Unknown) && status == GrpcStatus::Ok {
            response.set_header(GRPC_ENCODING_HEADER, config.get_compression().as_str());
        }
        let wire: ResponseData = ctx.get_mut_response().build();
        let _: Result<(), ResponseError> = stream.try_send(&wire).await;
        Status::Reject
    }
}
