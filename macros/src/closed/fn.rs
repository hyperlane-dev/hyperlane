use super::*;

/// Expands the macro to generate an asynchronous closed call.
///
/// This macro takes a `TokenStream` as input, which typically represents
/// the stream of a function or block, and inserts a call to `.set_closed(true)`
/// on that stream. This is useful for ensuring that a component gracefully
/// handles being closed.
///
/// # Arguments
///
/// - `TokenStream` - The input `TokenStream` to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// Returns the expanded `TokenStream` with the closed call inserted.
pub(crate) fn closed_macro(item: TokenStream, position: Position) -> TokenStream {
    inject(position, item, |_: &Ident, stream: &Ident| {
        quote! {
            #stream.set_closed(true);
        }
    })
}
