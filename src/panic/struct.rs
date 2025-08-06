use crate::*;

/// Represents detailed information about a panic that has occurred within the server.
///
/// This struct captures essential details about a panic, such as the message,
/// source code location, and payload. It is used by the server's panic handling
/// mechanism and passed to the configured panic hook for custom processing.
#[derive(CustomDebug, Default, PartialEq, Eq, Clone, Getter, DisplayDebug)]
pub struct Panic {
    /// The message associated with the panic.
    /// This is `None` if the panic payload is not a string.
    #[get(pub)]
    pub(super) message: OptionString,
    /// The source code location where the panic occurred.
    #[get(pub)]
    pub(super) location: OptionString,
    /// The payload of the panic, often a string literal.
    /// The handler attempts to downcast it to a `&str` or `String`.
    #[get(pub)]
    pub(super) payload: OptionString,
}
