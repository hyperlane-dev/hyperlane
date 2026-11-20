use super::*;

/// Type-erased poll entry point for an inline stack-allocated hook future.
pub(crate) type InlinePollFn = unsafe fn(*mut u8, &mut task::Context<'_>) -> Poll<Status>;

/// Type-erased destructor for an inline stack-allocated hook future.
pub(crate) type InlineDropFn = unsafe fn(*mut u8);

/// Initializer that constructs a concrete [`ServerHook`] future in place or falls back to boxing.
pub(crate) type InlineInitFn = fn(usize, usize, *mut u8, usize) -> InlinePollState;

/// Map of static route paths to compiled hook entries.
pub(crate) type CompiledHookMap = HashMapXxHash3_64<String, CompiledHook>;

/// Map of segment counts to compiled pattern route entries.
pub(crate) type CompiledPatternRoutes = HashMapXxHash3_64<usize, Vec<(RoutePattern, CompiledHook)>>;
