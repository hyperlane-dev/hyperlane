use super::*;

/// A type alias for a boxed dynamic trait object implementing the `ResponseTrait` trait.
///
/// This alias defines a `ResponseTrait` as a `Box` containing any type that implements the
/// `ResponseTrait` trait, with associated types `OutputText` set to `HttpResponseText`
/// and `OutputBinary` set to `HttpResponseBinary`. It allows for flexible handling of
/// HTTP responses that can be either in text or binary format.
pub type BoxResponseTrait =
    Box<dyn ResponseTrait<OutputText = HttpResponseText, OutputBinary = HttpResponseBinary>>;
