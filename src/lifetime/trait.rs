/// Trait for types that can be converted to a `'static` mutable reference.
///
/// This trait provides a way to obtain a `'static` mutable reference from
/// a mutable reference to `Self`, enabling safe lifetime extension for
/// certain use cases where the object is known to live for the entire
/// program duration.
pub trait Lifetime {
    /// Converts a mutable reference to `Self` into a `'static` mutable reference.
    ///
    /// # Returns
    ///
    /// - `&'static mut Self`: A mutable reference to the instance with a `'static` lifetime.
    fn leak_mut(&self) -> &'static mut Self;
}
