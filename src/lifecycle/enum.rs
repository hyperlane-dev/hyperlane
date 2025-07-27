/// Represents the control flow state of a request's lifecycle.
///
/// This enum is used internally to manage whether the request processing pipeline
/// should proceed to the next stage or be terminated prematurely. It also tracks
/// whether the underlying connection should be kept alive for subsequent requests.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum Lifecycle {
    /// Indicates that the request processing should be aborted.
    /// The boolean value specifies whether the connection should be kept alive (`true`) or closed (`false`).
    Abort(bool),
    /// Indicates that the request processing should continue to the next stage.
    /// The boolean value specifies whether the connection should be kept alive (`true`) or closed (`false`).
    Continue(bool),
}
