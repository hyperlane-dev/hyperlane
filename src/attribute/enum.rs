use crate::*;

/// Represents the key for an attribute.
///
/// Attributes can be either external, defined by a user-provided string,
/// or internal, representing framework-specific functionality.
#[derive(Clone, CustomDebug, Deserialize, DisplayDebug, Eq, Hash, PartialEq, Serialize)]
pub(crate) enum Attribute {
    /// An external attribute identified by a string.
    External(String),
    /// An internal attribute with a predefined key.
    Internal(InternalAttribute),
}

/// Defines keys for internal attributes used by the framework.
///
/// These keys correspond to specific, built-in functionalities.
#[derive(Clone, Copy, CustomDebug, Deserialize, DisplayDebug, Eq, Hash, PartialEq, Serialize)]
pub(crate) enum InternalAttribute {
    /// The attribute key for task panic handling with detailed error information.
    TaskPanicData,
    /// The attribute key for request error handling.
    RequestErrorData,
}
