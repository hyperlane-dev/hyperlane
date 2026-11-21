use super::*;

/// Returns the global inline hook registry.
///
/// # Returns
///
/// - `&'static StdRwLock<HashMap<usize, InlineInitFn>>` - The registry lock.
fn registry() -> &'static StdRwLock<HashMap<usize, InlineInitFn>> {
    INLINE_HOOK_REGISTRY.get_or_init(|| StdRwLock::new(HashMap::new()))
}

/// Registers an inline initializer for a [`Hook::factory`] handler address.
///
/// # Arguments
///
/// - `usize` - The raw `Arc` pointer address of the handler.
/// - `InlineInitFn` - The monomorphized inline initializer.
pub(crate) fn register_inline_hook(key: usize, init: InlineInitFn) {
    if let Ok(mut map) = registry().write() {
        map.insert(key, init);
    }
}

/// Removes an inline initializer when its handler `Arc` is dropped.
///
/// # Arguments
///
/// - `usize` - The raw `Arc` pointer address of the handler.
pub(crate) fn unregister_inline_hook(key: usize) {
    if key == 0 {
        return;
    }
    if let Ok(mut map) = registry().write() {
        map.remove(&key);
    }
}

/// Resolves a [`ServerHookHandler`] into a compiled inline or dynamic hook.
///
/// # Arguments
///
/// - `&ServerHookHandler` - The registered server hook handler.
///
/// # Returns
///
/// - `CompiledHook` - An inline hook when registered by [`Hook::factory`], or dynamic fallback.
pub(crate) fn lookup_inline_hook(handler: &ServerHookHandler) -> CompiledHook {
    let key: usize = Arc::as_ptr(handler) as *const () as usize;
    if let Ok(map) = registry().read()
        && let Some(&init) = map.get(&key)
    {
        return CompiledHook::Inline(init);
    }
    CompiledHook::Dynamic(Arc::clone(handler))
}

/// Executes a concrete [`ServerHook`] given raw stream and context addresses.
///
/// # Arguments
///
/// - `usize` - The memory address of the active `&mut Stream`.
/// - `usize` - The memory address of the active `&mut Context`.
///
/// # Returns
///
/// - `Status` - The hook lifecycle decision.
async fn execute_server_hook<R>(stream_addr: usize, ctx_addr: usize) -> Status
where
    R: ServerHook,
{
    let stream: &mut Stream = stream_addr.into();
    let ctx: &mut Context = ctx_addr.into();
    R::new(stream, ctx).await.handle(stream, ctx).await
}

/// Polls an inline future stored at `ptr`.
///
/// # Arguments
///
/// - `*mut u8` - Pointer to the pinned concrete future `F`.
/// - `&mut task::Context<'_>` - The current task waker context.
///
/// # Returns
///
/// - `Poll<Status>` - The poll result of the concrete future.
///
/// # Safety
///
/// - `ptr` must point to a valid, pinned instance of `F` with proper alignment.
unsafe fn poll_inline_future<F>(ptr: *mut u8, cx: &mut task::Context<'_>) -> Poll<Status>
where
    F: Future<Output = Status> + Send,
{
    // SAFETY: Caller guarantees `ptr` points to a pinned, initialized `F`.
    let future: Pin<&mut F> = unsafe { Pin::new_unchecked(&mut *ptr.cast::<F>()) };
    future.poll(cx)
}

/// Drops an inline future stored at `ptr`.
///
/// # Arguments
///
/// - `*mut u8` - Pointer to the initialized concrete future `F`.
///
/// # Safety
///
/// - `ptr` must point to a valid initialized `F` that will not be used again.
unsafe fn drop_inline_future<F>(ptr: *mut u8)
where
    F: Future<Output = Status> + Send,
{
    // SAFETY: Caller guarantees `ptr` points to a valid initialized `F` dropped at most once.
    unsafe {
        ptr::drop_in_place(ptr.cast::<F>());
    }
}

/// Places a concrete sendable future into the stack slot when size and alignment fit.
///
/// # Arguments
///
/// - `F` - The concrete future to store.
/// - `*mut u8` - The destination inline storage pointer.
/// - `usize` - The size in bytes of the destination inline storage.
///
/// # Returns
///
/// - `InlinePollState` - Inline vtable state or boxed fallback.
#[inline(always)]
fn place_inline_future<F>(future: F, slot: *mut u8, slot_len: usize) -> InlinePollState
where
    F: Future<Output = Status> + Send + 'static,
{
    if mem::size_of::<F>() <= slot_len && mem::align_of::<F>() <= mem::align_of::<usize>() {
        // SAFETY: `slot` is `usize`-aligned with at least `slot_len >= size_of::<F>()` bytes.
        unsafe {
            ptr::write(slot.cast::<F>(), future);
        }
        InlinePollState::Inline {
            poll_fn: poll_inline_future::<F>,
            drop_fn: drop_inline_future::<F>,
        }
    } else {
        InlinePollState::Boxed(Box::pin(future))
    }
}

/// Monomorphized initializer registered by [`Hook::factory`] for concrete hook `R`.
///
/// # Arguments
///
/// - `usize` - The memory address of the active `&mut Stream`.
/// - `usize` - The memory address of the active `&mut Context`.
/// - `*mut u8` - The destination inline storage pointer.
/// - `usize` - The byte capacity of the inline storage.
///
/// # Returns
///
/// - `InlinePollState` - Inline vtable state or boxed fallback.
pub(crate) fn inline_init_hook<R>(
    stream_addr: usize,
    ctx_addr: usize,
    slot: *mut u8,
    slot_len: usize,
) -> InlinePollState
where
    R: ServerHook,
{
    place_inline_future(
        execute_server_hook::<R>(stream_addr, ctx_addr),
        slot,
        slot_len,
    )
}
