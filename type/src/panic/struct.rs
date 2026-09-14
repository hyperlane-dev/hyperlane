use super::*;

/// Represents detailed information about a panic that has occurred within the server.
///
/// This struct captures essential details about a panic, such as the message,
/// source code location, and payload. It is used by the server's panic handling
/// mechanism and passed to the configured panic hook for custom processing.
#[derive(
    Clone, CustomDebug, Default, Deserialize, DisplayDebug, Eq, Getter, PartialEq, Serialize, New,
)]
pub struct PanicData {
    /// The message associated with the panic.
    /// This is `None` if the panic payload is not a string.
    pub(super) message: Option<String>,
    /// The source code location where the panic occurred.
    pub(super) location: Option<String>,
    /// The payload of the panic, often a string literal.
    /// The hook attempts to downcast it to a `&str` or `String`.
    pub(super) payload: Option<String>,
}
