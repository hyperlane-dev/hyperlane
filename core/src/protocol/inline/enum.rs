use super::*;

/// State machine for a single inline-or-boxed [`ServerHook`] invocation.
pub(crate) enum InlinePollState {
    /// The concrete future has not yet been constructed in pinned storage.
    Uninit,
    /// The concrete future lives directly in `InlineHookCall::storage`.
    Inline {
        /// Polls the in-place future stored at the storage base address.
        poll_fn: InlinePollFn,
        /// Drops the in-place future stored at the storage base address.
        drop_fn: InlineDropFn,
    },
    /// The concrete future exceeded the inline slot size and was boxed.
    Boxed(FutureBox<Status>),
    /// The future completed or was dropped.
    Done,
}

/// Dispatch representation of a registered hook resolved once at server startup.
#[derive(Clone)]
pub(crate) enum CompiledHook {
    /// Hook created via [`Hook::factory`] that can be constructed inline without `Box::pin`.
    Inline(InlineInitFn),
    /// Arbitrary user-supplied [`ServerHookHandler`] closure.
    Dynamic(ServerHookHandler),
}
