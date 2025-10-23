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
