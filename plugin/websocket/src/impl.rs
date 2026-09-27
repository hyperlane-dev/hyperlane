use super::*;

/// Allows `String` to be used as a broadcast identifier.
impl BroadcastTypeTrait for String {}

/// Allows string slices to be used as broadcast identifiers.
impl BroadcastTypeTrait for &str {}

/// Allows `char` to be used as a broadcast identifier.
impl BroadcastTypeTrait for char {}

/// Allows `bool` to be used as a broadcast identifier.
impl BroadcastTypeTrait for bool {}

/// Allows `i8` to be used as a broadcast identifier.
impl BroadcastTypeTrait for i8 {}

/// Allows `i16` to be used as a broadcast identifier.
impl BroadcastTypeTrait for i16 {}

/// Allows `i32` to be used as a broadcast identifier.
impl BroadcastTypeTrait for i32 {}

/// Allows `i64` to be used as a broadcast identifier.
impl BroadcastTypeTrait for i64 {}

/// Allows `i128` to be used as a broadcast identifier.
impl BroadcastTypeTrait for i128 {}

/// Allows `isize` to be used as a broadcast identifier.
impl BroadcastTypeTrait for isize {}

/// Allows `u8` to be used as a broadcast identifier.
impl BroadcastTypeTrait for u8 {}

/// Allows `u16` to be used as a broadcast identifier.
impl BroadcastTypeTrait for u16 {}

/// Allows `u32` to be used as a broadcast identifier.
impl BroadcastTypeTrait for u32 {}

/// Allows `u64` to be used as a broadcast identifier.
impl BroadcastTypeTrait for u64 {}

/// Allows `u128` to be used as a broadcast identifier.
impl BroadcastTypeTrait for u128 {}

/// Allows `usize` to be used as a broadcast identifier.
impl BroadcastTypeTrait for usize {}

/// Allows `f32` to be used as a broadcast identifier.
impl BroadcastTypeTrait for f32 {}

/// Allows `f64` to be used as a broadcast identifier.
impl BroadcastTypeTrait for f64 {}

/// Allows `IpAddr` to be used as a broadcast identifier.
impl BroadcastTypeTrait for IpAddr {}

/// Allows `Ipv4Addr` to be used as a broadcast identifier.
impl BroadcastTypeTrait for Ipv4Addr {}

/// Allows `Ipv6Addr` to be used as a broadcast identifier.
impl BroadcastTypeTrait for Ipv6Addr {}

/// Allows `SocketAddr` to be used as a broadcast identifier.
impl BroadcastTypeTrait for SocketAddr {}

/// Allows `NonZeroU8` to be used as a broadcast identifier.
impl BroadcastTypeTrait for NonZeroU8 {}

/// Allows `NonZeroU16` to be used as a broadcast identifier.
impl BroadcastTypeTrait for NonZeroU16 {}

/// Allows `NonZeroU32` to be used as a broadcast identifier.
impl BroadcastTypeTrait for NonZeroU32 {}

/// Allows `NonZeroU64` to be used as a broadcast identifier.
impl BroadcastTypeTrait for NonZeroU64 {}

/// Allows `NonZeroU128` to be used as a broadcast identifier.
impl BroadcastTypeTrait for NonZeroU128 {}

/// Allows `NonZeroUsize` to be used as a broadcast identifier.
impl BroadcastTypeTrait for NonZeroUsize {}

/// Allows `NonZeroI8` to be used as a broadcast identifier.
impl BroadcastTypeTrait for NonZeroI8 {}

/// Allows `NonZeroI16` to be used as a broadcast identifier.
impl BroadcastTypeTrait for NonZeroI16 {}

/// Allows `NonZeroI32` to be used as a broadcast identifier.
impl BroadcastTypeTrait for NonZeroI32 {}

/// Allows `NonZeroI64` to be used as a broadcast identifier.
impl BroadcastTypeTrait for NonZeroI64 {}

/// Allows `NonZeroI128` to be used as a broadcast identifier.
impl BroadcastTypeTrait for NonZeroI128 {}

/// Allows `NonZeroIsize` to be used as a broadcast identifier.
impl BroadcastTypeTrait for NonZeroIsize {}

/// Allows `Infallible` to be used as a broadcast identifier.
impl BroadcastTypeTrait for Infallible {}

/// Allows references to `String` to be used as broadcast identifiers.
impl BroadcastTypeTrait for &String {}

/// Allows double references to string slices to be used as broadcast identifiers.
impl BroadcastTypeTrait for &&str {}

/// Allows references to `char` to be used as broadcast identifiers.
impl BroadcastTypeTrait for &char {}

/// Allows references to `bool` to be used as broadcast identifiers.
impl BroadcastTypeTrait for &bool {}

/// Allows references to `i8` to be used as broadcast identifiers.
impl BroadcastTypeTrait for &i8 {}

/// Allows references to `i16` to be used as broadcast identifiers.
impl BroadcastTypeTrait for &i16 {}

/// Allows references to `i32` to be used as broadcast identifiers.
impl BroadcastTypeTrait for &i32 {}

/// Allows references to `i64` to be used as broadcast identifiers.
impl BroadcastTypeTrait for &i64 {}

/// Allows references to `i128` to be used as broadcast identifiers.
impl BroadcastTypeTrait for &i128 {}

/// Allows references to `isize` to be used as broadcast identifiers.
impl BroadcastTypeTrait for &isize {}

/// Allows references to `u8` to be used as broadcast identifiers.
impl BroadcastTypeTrait for &u8 {}

/// Allows references to `u16` to be used as broadcast identifiers.
impl BroadcastTypeTrait for &u16 {}

/// Allows references to `u32` to be used as broadcast identifiers.
impl BroadcastTypeTrait for &u32 {}

/// Allows references to `u64` to be used as
/// Implements `BroadcastTypeTrait` for `&u128`.
///
/// This allows references to `u128` to be used as a broadcast identifier.
impl BroadcastTypeTrait for &u128 {}

/// Implements `BroadcastTypeTrait` for `&usize`.
///
/// This allows references to `usize` to be used as a broadcast identifier.
impl BroadcastTypeTrait for &usize {}

/// Implements `BroadcastTypeTrait` for `&f32`.
///
/// This allows references to `f32` to be used as a broadcast identifier.
impl BroadcastTypeTrait for &f32 {}

/// Implements `BroadcastTypeTrait` for `&f64`.
///
/// This allows references to `f64` to be used as a broadcast identifier.
impl BroadcastTypeTrait for &f64 {}

/// Implements `BroadcastTypeTrait` for `&IpAddr`.
///
/// This allows references to `IpAddr` to be used as a broadcast identifier.
impl BroadcastTypeTrait for &IpAddr {}

/// Implements `BroadcastTypeTrait` for `&Ipv4Addr`.
///
/// This allows references to `Ipv4Addr` to be used as a broadcast identifier.
impl BroadcastTypeTrait for &Ipv4Addr {}

/// Implements `BroadcastTypeTrait` for `&Ipv6Addr`.
///
/// This allows references to `Ipv6Addr` to be used as a broadcast identifier.
impl BroadcastTypeTrait for &Ipv6Addr {}

/// Implements `BroadcastTypeTrait` for `&SocketAddr`.
///
/// This allows references to `SocketAddr` to be used as a broadcast identifier.
impl BroadcastTypeTrait for &SocketAddr {}

/// Implements `BroadcastTypeTrait` for `&NonZeroU8`.
///
/// This allows references to `NonZeroU8` to be used as a broadcast identifier.
impl BroadcastTypeTrait for &NonZeroU8 {}

/// Implements `BroadcastTypeTrait` for `&NonZeroU16`.
///
/// This allows references to `NonZeroU16` to be used as a broadcast identifier.
impl BroadcastTypeTrait for &NonZeroU16 {}

/// Implements `BroadcastTypeTrait` for `&NonZeroU32`.
///
/// This allows references to `NonZeroU32` to be used as a broadcast identifier.
impl BroadcastTypeTrait for &NonZeroU32 {}

/// Implements `BroadcastTypeTrait` for `&NonZeroU64`.
///
/// This allows references to `NonZeroU64` to be used as a broadcast identifier.
impl BroadcastTypeTrait for &NonZeroU64 {}

/// Implements `BroadcastTypeTrait` for `&NonZeroU128`.
///
/// This allows references to `NonZeroU128` to be used as a broadcast identifier.
impl BroadcastTypeTrait for &NonZeroU128 {}

/// Implements `BroadcastTypeTrait` for `&NonZeroUsize`.
///
/// This allows references to `NonZeroUsize` to be used as a broadcast identifier.
impl BroadcastTypeTrait for &NonZeroUsize {}

/// Implements `BroadcastTypeTrait` for `&NonZeroI8`.
///
/// This allows references to `NonZeroI8` to be used as a broadcast identifier.
impl BroadcastTypeTrait for &NonZeroI8 {}

/// Implements `BroadcastTypeTrait` for `&NonZeroI16`.
///
/// This allows references to `NonZeroI16` to be used as a broadcast identifier.
impl BroadcastTypeTrait for &NonZeroI16 {}

/// Implements `BroadcastTypeTrait` for `&NonZeroI32`.
///
/// This allows references to `NonZeroI32` to be used as a broadcast identifier.
impl BroadcastTypeTrait for &NonZeroI32 {}

/// Implements `BroadcastTypeTrait` for `&NonZeroI64`.
///
/// This allows references to `NonZeroI64` to be used as a broadcast identifier.
impl BroadcastTypeTrait for &NonZeroI64 {}

/// Implements `BroadcastTypeTrait` for `&NonZeroI128`.
///
/// This allows references to `NonZeroI128` to be used as a broadcast identifier.
impl BroadcastTypeTrait for &NonZeroI128 {}

/// Implements `BroadcastTypeTrait` for `&NonZeroIsize`.
///
/// This allows references to `NonZeroIsize` to be used as a broadcast identifier.
impl BroadcastTypeTrait for &NonZeroIsize {}

/// Implements `BroadcastTypeTrait` for `&Infallible`.
///
/// This allows references to `Infallible` to be used as a broadcast identifier.
impl BroadcastTypeTrait for &Infallible {}

/// Implements the `Default` trait for `BroadcastType`.
///
/// The default value is `BroadcastType::Unknown`.
///
/// # Type Parameters
///
/// - `BroadcastTypeTrait`: The type parameter for `BroadcastType`, which must implement `BroadcastTypeTrait`.
impl<B> Default for BroadcastType<B>
where
    B: BroadcastTypeTrait,
{
    #[inline(always)]
    fn default() -> Self {
        BroadcastType::Unknown
    }
}

impl<B> BroadcastType<B>
where
    B: BroadcastTypeTrait,
{
    /// Generates a unique key string for a given broadcast type.
    ///
    /// For point-to-point types, the keys are sorted to ensure consistent key generation
    /// regardless of the order of the input keys.
    ///
    /// # Arguments
    ///
    /// - `BroadcastType<B>` - The broadcast type for which to generate the key.
    ///
    /// # Returns
    ///
    /// - `String` - The unique key string for the broadcast type.
    #[inline(always)]
    pub fn get_key(broadcast_type: BroadcastType<B>) -> String {
        match broadcast_type {
            BroadcastType::PointToPoint(key1, key2) => {
                let (first_key, second_key) = if key1 <= key2 {
                    (key1, key2)
                } else {
                    (key2, key1)
                };
                format!(
                    "{}-{}-{}",
                    POINT_TO_POINT_KEY,
                    first_key.to_string(),
                    second_key.to_string()
                )
            }
            BroadcastType::PointToGroup(key) => {
                format!("{}-{}", POINT_TO_GROUP_KEY, key.to_string())
            }
            BroadcastType::Unknown => String::new(),
        }
    }
}

impl<'a, B> WebSocketConfig<'a, B>
where
    B: BroadcastTypeTrait,
{
    /// Creates a new WebSocket configuration with the given context.
    ///
    /// # Arguments
    ///
    /// - `&mut Context` - The context object to associate with the WebSocket.
    ///
    /// # Returns
    ///
    /// - `WebSocketConfig<B>` - A new WebSocket configuration instance.
    #[inline(always)]
    pub fn new(stream: &'a mut Stream, context: &'a mut Context) -> Self {
        Self {
            stream,
            context,
            capacity: DEFAULT_BROADCAST_SENDER_CAPACITY,
            broadcast_type: BroadcastType::default(),
            connected_hook: Hook::default_handler(),
            request_hook: Hook::default_handler(),
            sended_hook: Hook::default_handler(),
            closed_hook: Hook::default_handler(),
        }
    }
}

impl<'a, B> WebSocketConfig<'a, B>
where
    B: BroadcastTypeTrait,
{
    /// Sets the capacity for the broadcast sender.
    ///
    /// # Arguments
    ///
    /// - `Capacity` - The desired capacity.
    ///
    /// # Returns
    ///
    /// - `WebSocketConfig<B>` - The modified WebSocket configuration instance.
    #[inline(always)]
    pub fn set_capacity(mut self, capacity: Capacity) -> Self {
        self.capacity = capacity;
        self
    }

    /// Sets the context for the WebSocket connection.
    ///
    /// # Arguments
    ///
    /// - `&mut Context` - The context object to associate with the WebSocket.
    ///
    /// # Returns
    ///
    /// - `WebSocketConfig<B>` - The modified WebSocket configuration instance.
    #[inline(always)]
    pub fn set_context(mut self, context: &'a mut Context) -> Self {
        self.context = context;
        self
    }

    /// Sets the broadcast type for the WebSocket connection.
    ///
    /// # Arguments
    ///
    /// - `BroadcastType<B>` - The broadcast type to use for this WebSocket.
    ///
    /// # Returns
    ///
    /// - `WebSocketConfig<B>` - The modified WebSocket configuration instance.
    #[inline(always)]
    pub fn set_broadcast_type(mut self, broadcast_type: BroadcastType<B>) -> Self {
        self.broadcast_type = broadcast_type;
        self
    }

    #[inline(always)]
    pub fn get_stream(&mut self) -> &mut Stream {
        self.stream
    }

    /// Retrieves a reference to the context associated with this configuration.
    ///
    /// # Returns
    ///
    /// - `&mut Context` - A reference to the context object.
    #[inline(always)]
    pub fn get_context(&mut self) -> &mut Context {
        self.context
    }

    /// Retrieves the capacity configured for the broadcast sender.
    ///
    /// # Returns
    ///
    /// - `Capacity` - The capacity.
    #[inline(always)]
    pub fn get_capacity(&self) -> Capacity {
        self.capacity
    }

    /// Retrieves a reference to the broadcast type configured for this WebSocket.
    ///
    /// # Returns
    ///
    /// - `&BroadcastType<B>` - A reference to the broadcast type object.
    #[inline(always)]
    pub fn get_broadcast_type(&self) -> &BroadcastType<B> {
        &self.broadcast_type
    }

    /// Sets the connected hook handler.
    ///
    /// This hook is executed when the WebSocket connection is established.
    ///
    /// # Type Parameters
    ///
    /// - `S`: The hook type, which must implement `ServerHook`.
    ///
    /// # Returns
    ///
    /// The modified `WebSocketConfig` instance.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// struct MyConnectedHook;
    /// impl ServerHook for MyConnectedHook {
    ///     async fn new(_ctx: &Context) -> Self { Self }
    ///     async fn handle(self, ctx: &Context) { /* ... */ }
    /// }
    ///
    /// let config = WebSocketConfig::new()
    ///     .set_connected_hook::<MyConnectedHook>();
    /// ```
    #[inline(always)]
    pub fn set_connected_hook<S>(mut self) -> Self
    where
        S: ServerHook,
    {
        self.connected_hook = Hook::factory::<S>();
        self
    }

    /// Sets the request hook handler.
    ///
    /// This hook is executed when a new request is received on the WebSocket.
    ///
    /// # Type Parameters
    ///
    /// - `S`: The hook type, which must implement `ServerHook`.
    ///
    /// # Returns
    ///
    /// The modified `WebSocketConfig` instance.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// struct MyRequestHook;
    /// impl ServerHook for MyRequestHook {
    ///     async fn new(_ctx: &Context) -> Self { Self }
    ///     async fn handle(self, ctx: &Context) { /* ... */ }
    /// }
    ///
    /// let config = WebSocketConfig::new()
    ///     .set_request_hook::<MyRequestHook>();
    /// ```
    #[inline(always)]
    pub fn set_request_hook<S>(mut self) -> Self
    where
        S: ServerHook,
    {
        self.request_hook = Hook::factory::<S>();
        self
    }

    /// Sets the sended hook handler.
    ///
    /// This hook is executed after a message has been successfully sent over the WebSocket.
    ///
    /// # Type Parameters
    ///
    /// - `S`: The hook type, which must implement `ServerHook`.
    ///
    /// # Returns
    ///
    /// The modified `WebSocketConfig` instance.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// struct MySendedHook;
    /// impl ServerHook for MySendedHook {
    ///     async fn new(_ctx: &Context) -> Self { Self }
    ///     async fn handle(self, ctx: &Context) { /* ... */ }
    /// }
    ///
    /// let config = WebSocketConfig::new()
    ///     .set_sended_hook::<MySendedHook>();
    /// ```
    #[inline(always)]
    pub fn set_sended_hook<S>(mut self) -> Self
    where
        S: ServerHook,
    {
        self.sended_hook = Hook::factory::<S>();
        self
    }

    /// Sets the closed hook handler.
    ///
    /// This hook is executed when the WebSocket connection is closed.
    ///
    /// # Type Parameters
    ///
    /// - `S`: The hook type, which must implement `ServerHook`.
    ///
    /// # Returns
    ///
    /// The modified `WebSocketConfig` instance.
    ///
    /// # Examples
    ///
    /// ```rust,ignore
    /// struct MyClosedHook;
    /// impl ServerHook for MyClosedHook {
    ///     async fn new(_ctx: &Context) -> Self { Self }
    ///     async fn handle(self, ctx: &Context) { /* ... */ }
    /// }
    ///
    /// let config = WebSocketConfig::new()
    ///     .set_closed_hook::<MyClosedHook>();
    /// ```
    #[inline(always)]
    pub fn set_closed_hook<S>(mut self) -> Self
    where
        S: ServerHook,
    {
        self.closed_hook = Hook::factory::<S>();
        self
    }

    /// Retrieves a reference to the connected hook handler.
    ///
    /// # Returns
    ///
    /// - `&ServerHookHandler` - A reference to the connected hook handler.
    #[inline(always)]
    pub fn get_connected_hook(&self) -> &ServerHookHandler {
        &self.connected_hook
    }

    /// Retrieves a reference to the request hook handler.
    ///
    /// # Returns
    ///
    /// - `&ServerHookHandler` - A reference to the request hook handler.
    #[inline(always)]
    pub fn get_request_hook(&self) -> &ServerHookHandler {
        &self.request_hook
    }

    /// Retrieves a reference to the sended hook handler.
    ///
    /// # Returns
    ///
    /// - `&ServerHookHandler` - A reference to the sended hook handler.
    #[inline(always)]
    pub fn get_sended_hook(&self) -> &ServerHookHandler {
        &self.sended_hook
    }

    /// Retrieves a reference to the closed hook handler.
    ///
    /// # Returns
    ///
    /// - `&ServerHookHandler` - A reference to the closed hook handler.
    #[inline(always)]
    pub fn get_closed_hook(&self) -> &ServerHookHandler {
        &self.closed_hook
    }
}

impl WebSocket {
    /// Creates a new WebSocket instance.
    ///
    /// Initializes with a default broadcast map.
    ///
    /// # Returns
    ///
    /// - `WebSocket` - A new WebSocket instance.
    #[inline(always)]
    pub fn new() -> Self {
        Self::default()
    }

    /// Subscribes to a broadcast type or inserts a new one if it doesn't exist.
    ///
    /// # Type Parameters
    ///
    /// - `BroadcastTypeTrait`: The type implementing `BroadcastTypeTrait`.
    ///
    /// # Arguments
    ///
    /// - `BroadcastType<B>` - The broadcast type to subscribe to.
    /// - `Capacity` - The capacity for the broadcast sender if a new one is inserted.
    ///
    /// # Returns
    ///
    /// - `BroadcastMapReceiver<Vec<u8>>` - A broadcast map receiver for the specified broadcast type.
    #[inline(always)]
    fn subscribe_unwrap_or_insert<B>(
        &self,
        broadcast_type: BroadcastType<B>,
        capacity: Capacity,
    ) -> BroadcastMapReceiver<Vec<u8>>
    where
        B: BroadcastTypeTrait,
    {
        let key: String = BroadcastType::get_key(broadcast_type);
        self.broadcast_map.subscribe_or_insert(&key, capacity)
    }

    /// Subscribes to a point-to-point broadcast.
    ///
    /// # Type Parameters
    ///
    /// - `BroadcastTypeTrait`: The type implementing `BroadcastTypeTrait`.
    ///
    /// # Arguments
    ///
    /// - `&BroadcastTypeTrait` - The first identifier for the point-to-point communication.
    /// - `&BroadcastTypeTrait` - The second identifier for the point-to-point communication.
    /// - `Capacity` - The capacity for the broadcast sender.
    ///
    /// # Returns
    ///
    /// - `BroadcastMapReceiver<Vec<u8>>` - A broadcast map receiver for the point-to-point broadcast.
    #[inline(always)]
    fn point_to_point<B>(
        &self,
        key1: &B,
        key2: &B,
        capacity: Capacity,
    ) -> BroadcastMapReceiver<Vec<u8>>
    where
        B: BroadcastTypeTrait,
    {
        self.subscribe_unwrap_or_insert(
            BroadcastType::PointToPoint(key1.clone(), key2.clone()),
            capacity,
        )
    }

    /// Subscribes to a point-to-group broadcast.
    ///
    /// # Type Parameters
    ///
    /// - `BroadcastTypeTrait`: The type implementing `BroadcastTypeTrait`.
    ///
    /// # Arguments
    ///
    /// - `&BroadcastTypeTrait` - The identifier for the group.
    /// - `Capacity` - The capacity for the broadcast sender.
    ///
    /// # Returns
    ///
    /// - `BroadcastMapReceiver<Vec<u8>>` - A broadcast map receiver for the point-to-group broadcast.
    #[inline(always)]
    fn point_to_group<B>(&self, key: &B, capacity: Capacity) -> BroadcastMapReceiver<Vec<u8>>
    where
        B: BroadcastTypeTrait,
    {
        self.subscribe_unwrap_or_insert(BroadcastType::PointToGroup(key.clone()), capacity)
    }

    /// Retrieves the current receiver count for a given broadcast type.
    ///
    /// # Type Parameters
    ///
    /// - `BroadcastTypeTrait`: The type implementing `BroadcastTypeTrait`.
    ///
    /// # Arguments
    ///
    /// - `BroadcastType<B>` - The broadcast type for which to get the receiver count.
    ///
    /// # Returns
    ///
    /// - `ReceiverCount` - The number of active receivers for the broadcast type, or 0 if not found.
    #[inline(always)]
    pub fn receiver_count<B>(&self, broadcast_type: BroadcastType<B>) -> ReceiverCount
    where
        B: BroadcastTypeTrait,
    {
        let key: String = BroadcastType::get_key(broadcast_type);
        self.broadcast_map.receiver_count(&key).unwrap_or(0)
    }

    /// Calculates the receiver count before a connection is established.
    ///
    /// Ensures the count does not exceed the maximum allowed value minus one.
    ///
    /// # Type Parameters
    ///
    /// - `BroadcastTypeTrait`: The type implementing `BroadcastTypeTrait`.
    ///
    /// # Arguments
    ///
    /// - `BroadcastType<B>` - The broadcast type for which to get the receiver count.
    ///
    /// # Returns
    ///
    /// - `ReceiverCount` - The receiver count after the connection is established.
    #[inline(always)]
    pub fn receiver_count_before_connected<B>(
        &self,
        broadcast_type: BroadcastType<B>,
    ) -> ReceiverCount
    where
        B: BroadcastTypeTrait,
    {
        let count: ReceiverCount = self.receiver_count(broadcast_type);
        count.clamp(0, ReceiverCount::MAX - 1) + 1
    }

    /// Calculates the receiver count after a connection is closed.
    ///
    /// Ensures the count does not go below 0.
    ///
    /// # Type Parameters
    ///
    /// - `BroadcastTypeTrait`: The type implementing `BroadcastTypeTrait`.
    ///
    /// # Arguments
    ///
    /// - `BroadcastType<BroadcastTypeTrait>` - The broadcast type for which to get the receiver count.
    ///
    /// # Returns
    ///
    /// - `ReceiverCount` - The receiver count after the connection is closed.
    #[inline(always)]
    pub fn receiver_count_after_closed<B>(&self, broadcast_type: BroadcastType<B>) -> ReceiverCount
    where
        B: BroadcastTypeTrait,
    {
        let count: ReceiverCount = self.receiver_count(broadcast_type);
        count.clamp(1, ReceiverCount::MAX) - 1
    }

    /// Sends data to all active receivers for a given broadcast type.
    ///
    /// # Type Parameters
    ///
    /// - `Into<Vec<u8>>`: The type of data to send, which must be convertible to `Vec<u8>`.
    /// - `BroadcastTypeTrait`: The type implementing `BroadcastTypeTrait`.
    ///
    /// # Arguments
    ///
    /// - `BroadcastType<BroadcastTypeTrait>` - The broadcast type to which to send the data.
    /// - `Into<Vec<u8>>` - The data to send.
    ///
    /// # Returns
    ///
    /// - `Result<Option<ReceiverCount>, SendError<Vec<u8>>>` - A result indicating the success or failure of the send operation.
    #[inline(always)]
    pub fn try_send<T, B>(
        &self,
        broadcast_type: BroadcastType<B>,
        data: T,
    ) -> Result<Option<ReceiverCount>, SendError<Vec<u8>>>
    where
        T: Into<Vec<u8>>,
        B: BroadcastTypeTrait,
    {
        let key: String = BroadcastType::get_key(broadcast_type);
        self.broadcast_map.try_send(&key, data.into())
    }

    /// Sends data to all active receivers for a given broadcast type.
    ///
    /// This method panics if the send operation fails.
    ///
    /// # Type Parameters
    ///
    /// - `Into<Vec<u8>>`: The type of data to send, which must be convertible to `Vec<u8>`.
    /// - `BroadcastTypeTrait`: The type implementing `BroadcastTypeTrait`.
    ///
    /// # Arguments
    ///
    /// - `BroadcastType<BroadcastTypeTrait>` - The broadcast type to which to send the data.
    /// - `Into<Vec<u8>>` - The data to send.
    ///
    /// # Returns
    ///
    /// - `Option<ReceiverCount>` - The receiver count if the send operation succeeds.
    ///
    /// # Panics
    ///
    /// Panics if the send operation fails.
    #[inline(always)]
    pub fn send<T, B>(&self, broadcast_type: BroadcastType<B>, data: T) -> Option<ReceiverCount>
    where
        T: Into<Vec<u8>>,
        B: BroadcastTypeTrait,
    {
        self.try_send(broadcast_type, data).unwrap()
    }

    /// Runs the WebSocket connection, handling incoming requests and outgoing messages.
    ///
    /// This asynchronous function continuously monitors for new WebSocket requests
    /// and incoming broadcast messages, processing them according to the configured hooks.
    ///
    /// # Type Parameters
    ///
    /// - `BroadcastTypeTrait`: The type implementing `BroadcastTypeTrait`.
    ///
    /// # Arguments
    ///
    /// - `WebSocketConfig<BroadcastTypeTrait>` - The WebSocket configuration containing the configuration for this WebSocket instance.
    ///
    /// # Panics
    ///
    /// Panics if the context in the WebSocket configuration is not set (i.e., it's the default context).
    /// Panics if the broadcast type in the WebSocket configuration is `BroadcastType::Unknown`.
    pub async fn run<B>(&self, websocket_config: WebSocketConfig<'_, B>)
    where
        B: BroadcastTypeTrait,
    {
        let capacity: Capacity = websocket_config.get_capacity();
        let broadcast_type: BroadcastType<B> = websocket_config.get_broadcast_type().clone();
        let connected_hook: ServerHookHandler = websocket_config.get_connected_hook().clone();
        let sended_hook: ServerHookHandler = websocket_config.get_sended_hook().clone();
        let request_hook: ServerHookHandler = websocket_config.get_request_hook().clone();
        let closed_hook: ServerHookHandler = websocket_config.get_closed_hook().clone();
        let WebSocketConfig {
            stream,
            context: ctx,
            ..
        } = websocket_config;
        let mut receiver: Receiver<Vec<u8>> = match &broadcast_type {
            BroadcastType::PointToPoint(key1, key2) => self.point_to_point(key1, key2, capacity),
            BroadcastType::PointToGroup(key) => self.point_to_group(key, capacity),
            BroadcastType::Unknown => panic!("BroadcastType must be PointToPoint or PointToGroup"),
        };
        let key: String = BroadcastType::get_key(broadcast_type);
        if connected_hook(stream, ctx).await.is_reject() {
            return;
        }
        let mut is_reject: bool;
        loop {
            tokio::select! {
                request_res = stream.try_get_websocket_request() => {
                    if let Ok(body) = request_res {
                        ctx.get_mut_request().set_body(body);
                        is_reject = request_hook(stream, ctx).await.is_reject();
                    } else {
                        is_reject = true;
                        closed_hook(stream, ctx).await;
                    }
                    let body: ResponseBody = ctx.get_response().get_body().clone();
                    let is_err: bool = self.broadcast_map.try_send(&key, body).is_err();
                    if is_err || sended_hook(stream, ctx).await.is_reject() || is_reject {
                        break;
                    }
                },
                msg_res = receiver.recv() => {
                    if let Ok(msg) = &msg_res {
                        if stream.try_send_list(&WebSocketFrame::create_frame_list(msg)).await.is_ok() {
                            continue;
                        } else {
                            break;
                        }
                    }
                    break;
                }
            }
        }
        stream.set_closed(true);
    }
}
