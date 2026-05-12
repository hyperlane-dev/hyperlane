use crate::*;

/// Creates a default `ServerControlHook` instance with default no-op hooks.
///
/// The default `wait_hook` and `shutdown_hook` do nothing, allowing the server
/// to run without specific shutdown or wait logic unless configured otherwise.
///
/// # Returns
///
/// - `ServerControlHookHandler<()>` - A default `ServerControlHookHandler<()>` instance.
#[inline(always)]
pub fn default_server_control_hook_handler() -> ServerControlHookHandler<()> {
    Arc::new(|| Box::pin(async {}))
}

/// Creates a default `ServerHookHandler` from a trait object.
///
/// # Returns
///
/// - `ServerHookHandler` - A default `ServerHookHandler` instance.
#[inline(always)]
pub fn default_server_hook_handler() -> ServerHookHandler {
    Arc::new(|_: &mut Stream, _: &mut Context| -> FutureBox<Status> {
        Box::pin(async move { Status::default() })
    })
}

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
    Arc::new(
        move |stream: &mut Stream, ctx: &mut Context| -> FutureBox<Status> {
            let ctx_address: usize = ctx.into();
            let stream_address: usize = stream.into();
            Box::pin(async move {
                let ctx: &mut Context = ctx_address.into();
                let stream: &mut Stream = stream_address.into();
                R::new(stream, ctx).await.handle(stream, ctx).await
            })
        },
    )
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
