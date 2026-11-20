use super::*;

/// Removes the inline registry entry when the owning hook handler is dropped.
impl Drop for InlineHookGuard {
    /// Unregisters the associated handler address from the global inline registry.
    fn drop(&mut self) {
        let Self { key } = self;
        let value: usize = key.load(AtomicOrdering::Relaxed);
        unregister_inline_hook(value);
    }
}

/// SAFETY: `InlineHookCall` only stores `Send` references, function pointers,
/// and `Send` futures constructed via `place_inline_future`.
unsafe impl Send for InlineHookCall<'_> {}

/// Inherent implementation of [`InlineHookCall`].
impl<'a> InlineHookCall<'a> {
    /// Creates a new uninitialized inline hook call future.
    ///
    /// # Arguments
    ///
    /// - `&'a mut Stream` - The active stream.
    /// - `&'a mut Context` - The active context.
    /// - `InlineInitFn` - The monomorphized hook initializer.
    ///
    /// # Returns
    ///
    /// - `Self` - The future ready to be pinned and polled.
    #[inline(always)]
    pub(crate) fn new(stream: &'a mut Stream, ctx: &'a mut Context, init: InlineInitFn) -> Self {
        Self {
            stream,
            ctx,
            init,
            state: InlinePollState::Uninit,
            storage: MaybeUninit::uninit(),
            _pinned: PhantomPinned,
        }
    }
}

/// Drops any active inline future if cancelled or unwound before completion.
impl Drop for InlineHookCall<'_> {
    /// Drops the in-place future when active.
    fn drop(&mut self) {
        let Self { state, storage, .. } = self;
        let previous: InlinePollState = mem::replace(state, InlinePollState::Done);
        if let InlinePollState::Inline { drop_fn, .. } = previous {
            let ptr: *mut u8 = storage.as_mut_ptr().cast::<u8>();
            // SAFETY: `state` was `Inline`, so `storage` holds an initialized future dropped once.
            unsafe {
                drop_fn(ptr);
            }
        }
    }
}

/// Polls the inline-or-boxed [`ServerHook`] future directly in pinned stack storage.
impl Future for InlineHookCall<'_> {
    type Output = Status;

    /// Constructs the concrete future on first poll and drives it to completion.
    ///
    /// # Arguments
    ///
    /// - `Pin<&mut Self>` - The pinned inline call future.
    /// - `&mut task::Context<'_>` - The task waker context.
    ///
    /// # Returns
    ///
    /// - `Poll<Self::Output>` - The hook status once ready, or pending.
    #[inline(always)]
    fn poll(self: Pin<&mut Self>, cx: &mut task::Context<'_>) -> Poll<Self::Output> {
        // SAFETY: We never move `storage` once initialized; all field projections respect pinning.
        let Self {
            stream,
            ctx,
            init,
            state,
            storage,
            ..
        } = unsafe { self.get_unchecked_mut() };
        if matches!(state, InlinePollState::Uninit) {
            let stream_addr: usize = (&mut **stream).into();
            let ctx_addr: usize = (&mut **ctx).into();
            let slot: *mut u8 = storage.as_mut_ptr().cast::<u8>();
            let slot_len: usize = mem::size_of::<[usize; INLINE_FUTURE_WORDS]>();
            *state = (*init)(stream_addr, ctx_addr, slot, slot_len);
        }
        match state {
            InlinePollState::Inline { poll_fn, drop_fn } => {
                let poll_entry: InlinePollFn = *poll_fn;
                let drop_entry: InlineDropFn = *drop_fn;
                let ptr: *mut u8 = storage.as_mut_ptr().cast::<u8>();
                // SAFETY: `storage` holds the pinned concrete future initialized above.
                match unsafe { poll_entry(ptr, cx) } {
                    Poll::Ready(status) => {
                        *state = InlinePollState::Done;
                        // SAFETY: Completed future is dropped exactly once here.
                        unsafe {
                            drop_entry(ptr);
                        }
                        Poll::Ready(status)
                    }
                    Poll::Pending => Poll::Pending,
                }
            }
            InlinePollState::Boxed(future) => match future.as_mut().poll(cx) {
                Poll::Ready(status) => {
                    *state = InlinePollState::Done;
                    Poll::Ready(status)
                }
                Poll::Pending => Poll::Pending,
            },
            InlinePollState::Uninit | InlinePollState::Done => Poll::Ready(Status::default()),
        }
    }
}

/// Inherent implementation of [`CompiledHook`].
impl CompiledHook {
    /// Invokes the hook inline on the stack when possible, or dynamically otherwise.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The active stream.
    /// - `&mut Context` - The active request context.
    ///
    /// # Returns
    ///
    /// - `Status` - The hook lifecycle decision.
    #[inline(always)]
    pub(crate) async fn call(&self, stream: &mut Stream, ctx: &mut Context) -> Status {
        match self {
            Self::Inline(init) => InlineHookCall::new(stream, ctx, *init).await,
            Self::Dynamic(handler) => handler(stream, ctx).await,
        }
    }
}

/// Inherent implementation of [`CompiledPipeline`].
impl CompiledPipeline {
    /// Compiles a server's routes and middleware once before accepting connections.
    ///
    /// # Arguments
    ///
    /// - `&Server` - The configured server instance.
    ///
    /// # Returns
    ///
    /// - `Self` - The compiled zero-lock dispatch table.
    pub(crate) fn from_server(server: &Server) -> Self {
        let matcher: &RouteMatcher = server.get_route_matcher();
        let mut static_routes: CompiledHookMap = hash_map_xx_hash3_64();
        for (path, handler) in matcher.get_static_route().iter() {
            static_routes.insert(path.clone(), lookup_inline_hook(handler));
        }
        let mut dynamic_routes: CompiledPatternRoutes = hash_map_xx_hash3_64();
        for (&count, list) in matcher.get_dynamic_route().iter() {
            let compiled: Vec<(RoutePattern, CompiledHook)> = list
                .iter()
                .map(|(pattern, handler): &(RoutePattern, ServerHookHandler)| {
                    (pattern.clone(), lookup_inline_hook(handler))
                })
                .collect();
            dynamic_routes.insert(count, compiled);
        }
        let mut regex_routes: CompiledPatternRoutes = hash_map_xx_hash3_64();
        for (&count, list) in matcher.get_regex_route().iter() {
            let compiled: Vec<(RoutePattern, CompiledHook)> = list
                .iter()
                .map(|(pattern, handler): &(RoutePattern, ServerHookHandler)| {
                    (pattern.clone(), lookup_inline_hook(handler))
                })
                .collect();
            regex_routes.insert(count, compiled);
        }
        let request_middleware: Vec<CompiledHook> = server
            .get_request_middleware()
            .iter()
            .map(lookup_inline_hook)
            .collect();
        let response_middleware: Vec<CompiledHook> = server
            .get_response_middleware()
            .iter()
            .map(lookup_inline_hook)
            .collect();
        Self {
            static_routes,
            dynamic_routes,
            regex_routes,
            request_middleware,
            response_middleware,
        }
    }

    /// Resolves a route path into a compiled hook, filling `params` in place for dynamic/regex routes.
    ///
    /// # Arguments
    ///
    /// - `&mut RouteParams` - The context route parameter map to fill.
    /// - `&str` - The request path to match.
    ///
    /// # Returns
    ///
    /// - `Option<&CompiledHook>` - The matched compiled hook, if any.
    #[inline(always)]
    pub(crate) fn resolve_route(
        &self,
        params: &mut RouteParams,
        path: &str,
    ) -> Option<&CompiledHook> {
        let Self {
            static_routes,
            dynamic_routes,
            regex_routes,
            ..
        } = self;
        if let Some(hook) = static_routes.get(path) {
            return Some(hook);
        }
        if dynamic_routes.is_empty() && regex_routes.is_empty() {
            return None;
        }
        let trimmed: &str = path.trim_start_matches(DEFAULT_HTTP_PATH);
        let segment_count: usize = if trimmed.is_empty() {
            0
        } else {
            trimmed.split(DEFAULT_HTTP_PATH).count()
        };
        if let Some(routes) = dynamic_routes.get(&segment_count) {
            for (pattern, hook) in routes.iter() {
                if pattern.try_match_path_into(path, params) {
                    return Some(hook);
                }
            }
        }
        if !regex_routes.is_empty() {
            if let Some(routes) = regex_routes.get(&segment_count) {
                for (pattern, hook) in routes.iter() {
                    if pattern.try_match_path_into(path, params) {
                        return Some(hook);
                    }
                }
            }
            for (&pattern_seg_count, routes) in regex_routes.iter() {
                if pattern_seg_count != segment_count {
                    for (pattern, hook) in routes.iter() {
                        if pattern.has_tail_regex() && pattern.try_match_path_into(path, params) {
                            return Some(hook);
                        }
                    }
                }
            }
        }
        None
    }

    /// Executes the full request pipeline inline without per-hook `Box::pin` allocations.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The active stream.
    /// - `&mut Context` - The active request context.
    /// - `Option<String>` - The original request target supplied by multiplexed protocols.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the connection should remain open for keep-alive.
    #[inline(always)]
    pub(crate) async fn run_request(
        &self,
        stream: &mut Stream,
        ctx: &mut Context,
        target: Option<String>,
    ) -> bool {
        let Self {
            request_middleware,
            response_middleware,
            ..
        } = self;
        let keep_alive: bool = ctx.get_request().is_enable_keep_alive();
        let version: RequestVersion = ctx.get_request().get_version().clone();
        ctx.set_original_head(Some(ctx.get_request().get_method().is_head()));
        ctx.get_mut_response().reset().set_version(version);
        ctx.clear_route_params();
        ctx.reset_request_attributes();
        match target {
            Some(target) => {
                ctx.set_request_target(target);
            }
            None => {
                if let Some(target) = stream.get_line_buffer().split_ascii_whitespace().nth(1) {
                    ctx.set_request_target(target);
                } else {
                    ctx.clear_attribute();
                }
            }
        }
        stream.set_closed(false);
        for hook in request_middleware.iter() {
            if hook.call(stream, ctx).await.is_reject() {
                return stream.is_keep_alive(keep_alive);
            }
        }
        let matched_hook: Option<&CompiledHook> = {
            let path: &str = ctx.request.get_path().as_str();
            self.resolve_route(&mut ctx.route_params, path)
        };
        if let Some(hook) = matched_hook
            && hook.call(stream, ctx).await.is_reject()
        {
            return stream.is_keep_alive(keep_alive);
        }
        for hook in response_middleware.iter() {
            if hook.call(stream, ctx).await.is_reject() {
                return stream.is_keep_alive(keep_alive);
            }
        }
        stream.is_keep_alive(keep_alive)
    }
}
