/// Creates a new boxed value and leaks it, returning a static mutable reference.
///
/// # Arguments
///
/// - `T` - The data type to be boxed and leaked.
///
/// # Returns
///
/// - `&'static mut T` - A static mutable reference to the leaked value.
#[inline(always)]
pub fn box_leak_new<T>(data: T) -> &'static mut T {
    Box::leak(Box::new(data))
}
