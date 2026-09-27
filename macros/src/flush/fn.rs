use super::*;

/// Expands macro to generate async try_flush call on stream.
///
/// # Arguments
///
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with try_flush call.
pub(crate) fn try_flush_macro(item: TokenStream, position: Position) -> TokenStream {
    inject(position, item, |_: &Ident, stream: &Ident| {
        quote! {
            let _: ::std::result::Result<(), ::hyperlane::ResponseError> = #stream.try_flush().await;
        }
    })
}

/// Expands macro to generate async flush call on stream.
///
/// # Arguments
///
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with flush call.
pub(crate) fn flush_macro(item: TokenStream, position: Position) -> TokenStream {
    inject(position, item, |_: &Ident, stream: &Ident| {
        quote! {
            #stream.flush().await;
        }
    })
}
