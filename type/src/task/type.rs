use super::*;

/// A type alias for an asynchronous task.
///
/// This is a common return type for asynchronous handlers, providing a type-erased
/// future that can be easily managed by the async runtime.
pub type AsyncTask = Pin<Box<dyn Future<Output = ()> + Send + 'static>>;
