/// A trait for types that can be used as broadcast identifiers.
///
/// Types implementing this trait must be convertible to a string,
/// be partially orderable, and be cloneable.
pub trait BroadcastTypeTrait: ToString + PartialOrd + Clone {}
