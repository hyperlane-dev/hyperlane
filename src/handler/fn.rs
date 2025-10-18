use crate::*;

/// Creates a type-erased route handler from a concrete ServerHook implementation.
///
/// This function takes a type that implements the `ServerHook` trait and converts it
/// into a `ServerHookHandler` that can be stored alongside other route handlers
/// of different concrete types.
///
/// # Type Parameters
///
/// - `ServerHook` - The concrete route handler type that implements `ServerHook`.
///
/// # Returns
///
/// - `ServerHookHandler` - A `ServerHookHandler` that wraps the route handler's instantiation and execution logic.
#[inline]
pub(crate) fn create_route_hook<R>() -> ServerHookHandler
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

/// Creates a type-erased middleware handler from a concrete ServerHook implementation.
///
/// This function takes a type that implements the `ServerHook` trait and converts it
/// into a `ServerHookHandler` that can be stored alongside other middleware handlers
/// of different concrete types.
///
/// # Type Parameters
///
/// - `ServerHook` - The concrete middleware type that implements `ServerHook`.
///
/// # Returns
///
/// - `ServerHookHandler` - A `ServerHookHandler` that wraps the middleware's instantiation and execution logic.
#[inline]
pub(crate) fn create_middleware_hook<M>() -> ServerHookHandler
where
    M: ServerHook,
{
    Arc::new(move |ctx: &Context| -> SendableAsyncTask<()> {
        let ctx: Context = ctx.clone();
        Box::pin(async move {
            M::new(&ctx).await.handle(&ctx).await;
        })
    })
}

/// Creates a type-erased panic hook handler from a concrete ServerHook implementation.
///
/// This function takes a type that implements the `ServerHook` trait and converts it
/// into a `ServerHookHandler` that can be stored alongside other panic hook handlers
/// of different concrete types.
///
/// # Type Parameters
///
/// - `ServerHook` - The concrete panic hook type that implements `ServerHook`.
///
/// # Returns
///
/// - `ServerHookHandler` - A `ServerHookHandler` that wraps the panic hook's instantiation and execution logic.
#[inline]
pub(crate) fn create_panic_hook<P>() -> ServerHookHandler
where
    P: ServerHook,
{
    Arc::new(move |ctx: &Context| -> SendableAsyncTask<()> {
        let ctx: Context = ctx.clone();
        Box::pin(async move {
            P::new(&ctx).await.handle(&ctx).await;
        })
    })
}
