use crate::*;

/// Default initial hook that wraps the Context at the start of the processing pipeline.
///
/// This struct serves as the entry point for the trait-based handler chain.
/// It is created by the framework at the beginning of request processing and
/// contains the initial `Context`. All subsequent handlers in the chain receive
/// this or a derived handler through their `new` method.
#[derive(Clone, Default, CustomDebug, DisplayDebug, Getter)]
pub struct DefaultInitialHook {
    /// The request context containing all request/response data.
    pub context: Context,
}
