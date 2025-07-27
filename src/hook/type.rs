use crate::*;

/// A type alias for a thread-safe, shareable, pinned, boxed, sendable, synchronous function.
///
/// This type is used for storing handlers in a shared context, allowing multiple
/// parts of the application to safely access and execute the same handler.
pub type ArcFnPinBoxSendSync = Arc<dyn FnPinBoxSendSync>;

/// An optional, thread-safe, shareable handler function.
///
/// This is used when a handler may or may not be present, such as for optional
/// middleware or hooks.
pub type OptionArcFnPinBoxSendSync = Option<ArcFnPinBoxSendSync>;

/// A vector of thread-safe, shareable handler functions.
///
/// This type is used to represent a chain of middleware or hooks that can be
/// executed sequentially.
pub type VecArcFnPinBoxSendSync = Vec<ArcFnPinBoxSendSync>;

/// A type alias for a thread-safe, shareable error handler function.
///
/// This type allows for an error handler to be stored and shared across threads,
/// making it suitable for global or application-level error handling.
pub type ArcErrorHandlerSendSync =
    Arc<dyn Fn(Context) -> PinBoxFutureSendStatic + Send + Sync + 'static>;

/// A type alias for a pinned, boxed, sendable, static future.
///
/// This is a common return type for asynchronous handlers, providing a type-erased
/// future that can be easily managed by the async runtime.
pub type PinBoxFutureSendStatic = Pin<Box<(dyn Future<Output = ()> + Send + 'static)>>;
