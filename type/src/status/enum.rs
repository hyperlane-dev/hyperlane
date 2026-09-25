use super::*;

/// Represents the execution status of a server hook.
///
/// This enum is used to control the request processing pipeline flow.
/// When a hook returns `Reject`, the pipeline stops processing further hooks
/// and the connection lifecycle is aborted. When a hook returns `Continue`,
/// the pipeline proceeds to the next hook or stage.
#[derive(Clone, Copy, Debug, Default, Deserialize, Eq, PartialEq, Serialize)]
pub enum Status {
    /// Indicates that the request processing should continue to the next hook or stage.
    Continue,
    /// Indicates that the request processing should be aborted and no further hooks should be executed.
    #[default]
    Reject,
}
