/// Trait for types that can be converted to a `'static` reference.
///
/// This trait provides a way to obtain a `'static` reference or mutable reference from
/// a reference to `Self`, enabling safe lifetime extension for
/// certain use cases where the object is known to live for the entire
/// program duration.
pub trait Lifetime {
    /// Converts a reference to `Self` into a `'static` reference.
    ///
    /// # Returns
    ///
    /// - `&'static Self`: A reference to the instance with a `'static` lifetime.
    ///
    /// # Safety
    ///
    /// - The address is guaranteed to be a valid `Self` instance
    ///   that was previously converted from a reference and is managed by the runtime.
    fn leak(&self) -> &'static Self;

    /// Converts a reference to `Self` into a `'static` mutable reference.
    ///
    /// # Returns
    ///
    /// - `&'static mut Self`: A mutable reference to the instance with a `'static` lifetime.
    ///
    /// # Safety
    ///
    /// - The address is guaranteed to be a valid `Self` instance
    ///   that was previously converted from a reference and is managed by the runtime.
    fn leak_mut(&self) -> &'static mut Self;
}
