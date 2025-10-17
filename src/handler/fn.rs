use crate::*;

/// Creates a type-erased route handler from a concrete ServerHook implementation.
///
/// This function takes a type that implements the `ServerHook` trait and converts it
/// into a `ArcPinBoxFutureSendSync` that can be stored alongside other route handlers
/// of different concrete types.
///
/// # Type Parameters
///
/// - `ServerHook` - The concrete route handler type that implements `ServerHook`.
///
/// # Returns
///
/// - `ArcPinBoxFutureSendSync` that wraps the route handler's instantiation and execution logic.
#[inline]
pub(crate) fn create_route_handler<R>() -> ArcPinBoxFutureSendSync
where
    R: ServerHook,
{
    Arc::new(move |ctx: &Context| -> PinBoxFutureSend<()> {
        let ctx: Context = ctx.clone();
        Box::pin(async move {
            R::new(&ctx).await.handle(&ctx).await;
        })
    })
}

/// Creates a type-erased middleware handler from a concrete ServerHook implementation.
///
/// This function takes a type that implements the `ServerHook` trait and converts it
/// into a `ArcPinBoxFutureSendSync` that can be stored alongside other middleware handlers
/// of different concrete types.
///
/// # Type Parameters
///
/// - `ServerHook` - The concrete middleware type that implements `ServerHook`.
///
/// # Returns
///
/// - `ArcPinBoxFutureSendSync` that wraps the middleware's instantiation and execution logic.
#[inline]
pub(crate) fn create_middleware_handler<M>() -> ArcPinBoxFutureSendSync
where
    M: ServerHook,
{
    Arc::new(move |ctx: &Context| -> PinBoxFutureSend<()> {
        let ctx: Context = ctx.clone();
        Box::pin(async move {
            M::new(&ctx).await.handle(&ctx).await;
        })
    })
}

/// Creates a type-erased panic hook handler from a concrete ServerHook implementation.
///
/// This function takes a type that implements the `ServerHook` trait and converts it
/// into a `ArcPinBoxFutureSendSync` that can be stored alongside other panic hook handlers
/// of different concrete types.
///
/// # Type Parameters
///
/// - `ServerHook` - The concrete panic hook type that implements `ServerHook`.
///
/// # Returns
///
/// - `ArcPinBoxFutureSendSync` that wraps the panic hook's instantiation and execution logic.
#[inline]
pub(crate) fn create_panic_hook_handler<P>() -> ArcPinBoxFutureSendSync
where
    P: ServerHook,
{
    Arc::new(move |ctx: &Context| -> PinBoxFutureSend<()> {
        let ctx: Context = ctx.clone();
        Box::pin(async move {
            P::new(&ctx).await.handle(&ctx).await;
        })
    })
}
