use super::*;

/// Global mapping from [`Hook::factory`] handler pointers to their inline initializers.
pub(crate) static INLINE_HOOK_REGISTRY: OnceLock<StdRwLock<HashMap<usize, InlineInitFn>>> =
    OnceLock::new();
