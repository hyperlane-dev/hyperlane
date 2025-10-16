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
/// The `Prev` type parameter of the `Route` trait can be any type that is `Send`.
/// At runtime, a `DefaultInitialHook` is created and converted to the route's `Prev` type,
/// then passed by reference to the `new` method. The `Prev` type must implement
/// `From<DefaultInitialHook>` to enable this conversion.
pub(crate) fn create_route_handler<R>() -> ArcPinBoxFutureSendSync
where
    R: Route,
    R::Prev: From<DefaultInitialHook>,
{
    Arc::new(move |initial: DefaultInitialHook| -> PinBoxFutureSend<()> {
        Box::pin(async move {
            R::new(&R::Prev::from(initial)).await.handle().await;
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
/// The `Prev` type parameter of the `Middleware` trait can be any type that is `Send`.
/// At runtime, a `DefaultInitialHook` is created and converted to the middleware's `Prev` type,
/// then passed by reference to the `new` method. The `Prev` type must implement
/// `From<DefaultInitialHook>` to enable this conversion.
pub(crate) fn create_middleware_handler<M>() -> ArcPinBoxFutureSendSync
where
    M: Middleware,
    M::Prev: From<DefaultInitialHook>,
{
    Arc::new(move |initial: DefaultInitialHook| -> PinBoxFutureSend<()> {
        Box::pin(async move {
            M::new(&M::Prev::from(initial)).await.handle().await;
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
/// The `Prev` type parameter of the `PanicHook` trait can be any type that is `Send`.
/// At runtime, a `DefaultInitialHook` is created and converted to the panic hook's `Prev` type,
/// then passed by reference to the `new` method. The `Prev` type must implement
/// `From<DefaultInitialHook>` to enable this conversion.
pub(crate) fn create_panic_hook_handler<P>() -> ArcPinBoxFutureSendSync
where
    P: PanicHook,
    P::Prev: From<DefaultInitialHook>,
{
    Arc::new(move |initial: DefaultInitialHook| -> PinBoxFutureSend<()> {
        Box::pin(async move {
            P::new(&P::Prev::from(initial)).await.handle().await;
        })
    })
}
