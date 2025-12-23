use crate::*;

/// A type alias for optional panic information.
///
/// This is used in contexts where a panic might not have occurred, allowing for
/// graceful handling of both panic and non-panic scenarios.
pub type OptionPanic = Option<Panic>;
/// A type alias for an optional reference to a panic location.
///
/// The lifetimes `'a` and `'b` are tied to the `PanicHookInfo` from which the
/// location information is sourced. This ensures that the reference does not
/// outlive the panic information itself, preventing dangling pointers.
pub type OptionalPanicLocation<'a, 'b> = Option<&'a Location<'b>>;
