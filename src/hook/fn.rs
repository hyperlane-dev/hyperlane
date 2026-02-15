use crate::*;

/// Creates a new `ServerHookHandler` from a trait object.
///
/// # Arguments
///
/// - `ServerHook` - The trait object implementing `ServerHook`.
///
/// # Returns
///
/// - `ServerHookHandler` - A new `ServerHookHandler` instance.
#[inline(always)]
pub fn server_hook_factory<R>() -> ServerHookHandler
where
    R: ServerHook,
{
    Arc::new(move |ctx: &mut Context| -> SendableAsyncTask<Context> {
        let mut ctx: Context = take(ctx);
        Box::pin(async move {
            R::new(&mut ctx).await.handle(&mut ctx).await;
            ctx
        })
    })
}

/// Verifies that hooks with the same type and execution priority are unique.
///
/// This function validates that no two hooks of the same type have identical
/// execution priorities (orders). Only hooks that define an explicit priority
/// (non-None order) are checked for uniqueness. Hooks without a priority are
/// ignored in duplicate detection.
///
/// # Arguments
///
/// - `Vec<HookType>` - A vector of `HookType` instances to validate for uniqueness.
///
/// # Panics
///
/// - Panics if duplicate hooks are detected with the same type and priority,
///   displaying the hook type and order in the error message.
#[inline(always)]
pub fn assert_hook_unique_order(list: Vec<HookType>) {
    let mut seen: HashSet<(HookType, isize)> = HashSet::new();
    list.iter().for_each(|hook| {
        if let Some(order) = hook.try_get_order()
            && !seen.insert((*hook, order))
        {
            panic!("Duplicate hook detected: {} with order {}", hook, order);
        }
    });
}
