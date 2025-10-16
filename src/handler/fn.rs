use crate::*;

/// Creates a type-erased route handler from a concrete Route implementation.
///
/// This function takes a type that implements the `Route` trait and converts it
/// into a `ArcPinBoxFutureSendSync` that can be stored alongside other route handlers
/// of different concrete types.
///
/// # Type Parameters
///
/// - `Route` - The concrete route handler type that implements `Route`.
///
/// # Returns
///
/// - `ArcPinBoxFutureSendSync` that wraps the route handler's instantiation and execution logic.
///
/// # Note
///
/// At runtime, a `DefaultInitialHook` is created and its `Context` is extracted
/// and passed directly to the route's `new` and `handle` methods.
pub(crate) fn create_route_handler<R>() -> ArcPinBoxFutureSendSync
where
    R: Route,
{
    Arc::new(move |initial: DefaultInitialHook| -> PinBoxFutureSend<()> {
        Box::pin(async move {
            let ctx = initial.context.clone();
            R::new(ctx.clone()).await.handle(ctx).await;
        })
    })
}

/// Creates a type-erased middleware handler from a concrete Middleware implementation.
///
/// This function takes a type that implements the `Middleware` trait and converts it
/// into a `ArcPinBoxFutureSendSync` that can be stored alongside other middleware handlers
/// of different concrete types.
///
/// # Type Parameters
///
/// - `Middleware` - The concrete middleware type that implements `Middleware`.
///
/// # Returns
///
/// - `ArcPinBoxFutureSendSync` that wraps the middleware's instantiation and execution logic.
///
/// # Note
///
/// At runtime, a `DefaultInitialHook` is created and its `Context` is extracted
/// and passed directly to the middleware's `new` and `handle` methods.
pub(crate) fn create_middleware_handler<M>() -> ArcPinBoxFutureSendSync
where
    M: Middleware,
{
    Arc::new(move |initial: DefaultInitialHook| -> PinBoxFutureSend<()> {
        Box::pin(async move {
            let ctx = initial.context.clone();
            M::new(ctx.clone()).await.handle(ctx).await;
        })
    })
}

/// Creates a type-erased panic hook handler from a concrete PanicHook implementation.
///
/// This function takes a type that implements the `PanicHook` trait and converts it
/// into a `ArcPinBoxFutureSendSync` that can be stored alongside other panic hook handlers
/// of different concrete types.
///
/// # Type Parameters
///
/// - `PanicHook` - The concrete panic hook type that implements `PanicHook`.
///
/// # Returns
///
/// - `ArcPinBoxFutureSendSync` that wraps the panic hook's instantiation and execution logic.
///
/// # Note
///
/// At runtime, a `DefaultInitialHook` is created and its `Context` is extracted
/// and passed directly to the panic hook's `new` and `handle` methods.
pub(crate) fn create_panic_hook_handler<P>() -> ArcPinBoxFutureSendSync
where
    P: PanicHook,
{
    Arc::new(move |initial: DefaultInitialHook| -> PinBoxFutureSend<()> {
        Box::pin(async move {
            let ctx = initial.context.clone();
            P::new(ctx.clone()).await.handle(ctx).await;
        })
    })
}
