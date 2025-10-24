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
#[inline]
pub fn server_hook_factory<R>() -> ServerHookHandler
where
    R: ServerHook,
{
    Arc::new(move |ctx: &Context| -> SendableAsyncTask<()> {
        let ctx: Context = ctx.clone();
        Box::pin(async move {
            R::new(&ctx).await.handle(&ctx).await;
        })
    })
}

/// Verify that each `Hook` in the list with the same type and non-zero priority is unique.
///
/// This function iterates over all provided `Hook` items and ensures that no two
/// `Hook` items of the same type define the same non-zero `order`. If a duplicate
/// is found, the function will panic at runtime.
///
/// # Arguments
///
/// - `Vec<HookMacro>`- A vector of `HookMacro` instances to be checked.
///
/// # Panics
///
/// - Panics if two or more `Hook` items of the same type define the same non-zero `order`.
#[inline]
pub fn assert_hook_unique_order(list: Vec<HookMacro>) {
    let mut seen: HashSet<(HookType, isize)> = HashSet::new();
    list.iter()
        .filter_map(|hook| {
            hook.hook_type
                .try_get()
                .map(|order| (hook.hook_type, order))
        })
        .for_each(|(key, order)| {
            if !seen.insert((key, order)) {
                panic!("Duplicate hook detected: {} with order {}", key, order);
            }
        });
}
