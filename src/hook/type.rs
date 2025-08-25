use crate::*;

/// A type alias for a thread-safe, shareable, pinned, boxed, sendable, synchronous function.
///
/// This type is used for storing handlers in a shared context, allowing multiple
/// parts of the application to safely access and execute the same handler.
pub type ArcFnContextPinBoxSendSync<T> = Arc<dyn FnContextPinBoxSendSync<T>>;
/// An optional, thread-safe, shareable handler function.
///
/// This is used when a handler may or may not be present, such as for optional
/// middleware or hooks.
pub type OptionArcFnContextPinBoxSendSync<T> = Option<ArcFnContextPinBoxSendSync<T>>;
/// A vector of thread-safe, shareable handler functions.
///
/// This type is used to represent a chain of middleware or hooks that can be
/// executed sequentially.
pub type VecArcFnContextPinBoxSendSync<T> = Vec<ArcFnContextPinBoxSendSync<T>>;
/// A type alias for a pinned, boxed, sendable, static future.
///
/// This is a common return type for asynchronous handlers, providing a type-erased
/// future that can be easily managed by the async runtime.
pub type PinBoxFutureSendStatic = Pin<Box<(dyn Future<Output = ()> + Send + 'static)>>;
/// A type alias for a pinned, boxed, `Send`-able future with a generic output.
///
/// This is often used to represent an asynchronous task that can be sent across threads.
pub type PinBoxFutureSend<T> = Pin<Box<dyn Future<Output = T> + Send>>;
/// A type alias for a thread-safe, reference-counted closure that produces a `FnPinBoxFutureSend`.
///
/// This is useful for creating and sharing asynchronous task factories.
pub type ArcFnPinBoxFutureSend<T> = Arc<dyn FnPinBoxFutureSend<T>>;
