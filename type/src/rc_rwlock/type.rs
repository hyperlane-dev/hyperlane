use super::*;

/// A type alias for a reference-counted, read-write locked value.
///
/// This alias simplifies the use of `Rc<RwLock<T>>`, providing a convenient way to manage shared mutable data across multiple owners.
pub type RcRwLock<T> = Rc<RwLock<T>>;
