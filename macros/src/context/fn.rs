use super::*;

/// Checks if a type is a mutable reference type.
///
/// Returns true if the type is `&mut T`, false otherwise (including `&T` and other types).
pub(crate) fn is_mutable_reference_type(ty: &Type) -> bool {
    if let Type::Reference(type_ref) = ty {
        type_ref.mutability.is_some()
    } else {
        false
    }
}

/// Implements the context! procedural macro.
///
/// This macro generates a let statement that converts a context pointer into a reference.
/// The generated code performs an unsafe conversion from a raw pointer to a reference
/// through the `Into` trait.
///
/// The mutability of the generated reference is determined by the optional type annotation:
/// - If the type annotation contains `&mut`, generates `leak_mut()` call
/// - Otherwise, generates `leak()` call (default behavior)
///
/// # Arguments
///
/// - `TokenStream` - The input token stream containing the variable name identifier
///   and an optional type annotation (e.g., `ctx` or `ctx: &mut Context`).
///
/// # Returns
///
/// - `TokenStream` - The generated let statement binding the variable to the converted context reference.
///
/// # Safety
///
/// - The address is guaranteed to be a valid `Self` instance
///   that was previously converted from a reference and is managed by the runtime.
pub(crate) fn context_macro(input: TokenStream) -> TokenStream {
    let context_input: ContextInput = match parse(input) {
        Ok(input) => input,
        Err(err) => return err.to_compile_error().into(),
    };
    let source_ctx: Ident = context_input.source_ctx;
    let is_mut: bool = context_input
        .ty
        .as_ref()
        .is_some_and(is_mutable_reference_type);
    if is_mut {
        leak_mut_context(true, &source_ctx).into()
    } else {
        leak_context(true, &source_ctx).into()
    }
}
