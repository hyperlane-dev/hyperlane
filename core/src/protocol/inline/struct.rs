use super::*;

/// Unregisters a [`Hook::factory`] inline initializer when its handler `Arc` is dropped.
pub(crate) struct InlineHookGuard {
    /// Pointer address of the enclosing [`ServerHookHandler`] `Arc` payload.
    pub(crate) key: Arc<AtomicUsize>,
}

/// Stack-pinned future that runs a [`ServerHook`] inline without heap allocation when it fits.
pub(crate) struct InlineHookCall<'a> {
    /// The stream passed to the hook.
    pub(super) stream: &'a mut Stream,
    /// The request context passed to the hook.
    pub(super) ctx: &'a mut Context,
    /// Monomorphized initializer for the concrete hook type.
    pub(super) init: InlineInitFn,
    /// Current lifecycle state of the future in `storage`.
    pub(super) state: InlinePollState,
    /// Aligned inline storage for the pinned hook future.
    pub(super) storage: MaybeUninit<[usize; INLINE_FUTURE_WORDS]>,
    /// Prevents moving the future after pinning.
    pub(super) _pinned: PhantomPinned,
}

/// Immutable per-server dispatch table compiled once when [`Server::run`] starts.
#[derive(Clone)]
pub(crate) struct CompiledPipeline {
    /// Static exact-match routes.
    pub(super) static_routes: CompiledHookMap,
    /// Dynamic parameter routes grouped by segment count.
    pub(super) dynamic_routes: CompiledPatternRoutes,
    /// Regex routes grouped by segment count.
    pub(super) regex_routes: CompiledPatternRoutes,
    /// Request middleware chain in registration order.
    pub(super) request_middleware: Vec<CompiledHook>,
    /// Response middleware chain in registration order.
    pub(super) response_middleware: Vec<CompiledHook>,
}
