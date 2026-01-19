use crate::*;

/// Represents detailed information about a panic that has occurred within the server.
///
/// This struct captures essential details about a panic, such as the message,
/// source code location, and payload. It is used by the server's panic handling
/// mechanism and passed to the configured panic hook for custom processing.
#[derive(Clone, CustomDebug, Data, Default, New, PartialEq, Eq, DisplayDebug)]
pub struct PanicData {
    /// The message associated with the panic.
    /// This is `None` if the panic payload is not a string.
    #[get(pub)]
    #[set(pub(crate))]
    pub(super) message: Option<String>,
    /// The source code location where the panic occurred.
    #[get(pub)]
    #[set(pub(crate))]
    pub(super) location: Option<String>,
    /// The payload of the panic, often a string literal.
    /// The hook attempts to downcast it to a `&str` or `String`.
    #[get(pub)]
    #[set(pub(crate))]
    pub(super) payload: Option<String>,
}
