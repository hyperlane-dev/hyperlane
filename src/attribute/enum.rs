use crate::*;

/// Represents the key for an attribute.
///
/// Attributes can be either external, defined by a user-provided string,
/// or internal, representing framework-specific functionality.
#[derive(CustomDebug, Clone, PartialEq, Eq, Hash, DisplayDebug)]
pub(crate) enum AttributeKey {
    /// An external attribute identified by a string.
    External(String),
    /// An internal attribute with a predefined key.
    Internal(InternalAttributeKey),
}

/// Defines keys for internal attributes used by the framework.
///
/// These keys correspond to specific, built-in functionalities.
#[derive(CustomDebug, Clone, PartialEq, Eq, Hash, DisplayDebug)]
pub(crate) enum InternalAttributeKey {
    /// The attribute key for panic handling.
    Panic,
}
