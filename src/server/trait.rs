use crate::*;

/// A trait for `Send`-able futures with no output.
pub trait FutureSend: Future<Output = ()> + Send {}

/// A trait for thread-safe, reference-counted closures that produce a `PinBoxFutureSend`.
pub trait FnPinBoxFutureSendSync: Fn() -> PinBoxFutureSend + Send + Sync {}
