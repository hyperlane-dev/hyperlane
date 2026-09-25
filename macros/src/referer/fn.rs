use super::*;

/// Filters requests matching the specified Referer header.
/// Supports both single and multiple referer value checks.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with Referer filter.
pub(crate) fn referer_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_referer: MultiRefererData = parse_macro_input!(attr as MultiRefererData);
    inject(position, item, |context: &Ident, _: &Ident| {
        let statements = multi_referer.referer_values.iter().map(|referer_value| {
            quote! {
                if #context.get_request().try_get_header_back(::hyperlane_core::REFERER).map_or(true, |referer_header| referer_header != #referer_value) {
                    return ::hyperlane_core::Status::Continue;
                }
            }
        });
        quote! {
            #(#statements)*
        }
    })
}

/// Rejects requests matching the specified Referer header.
/// Supports both single and multiple referer value checks.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with Referer rejection filter.
pub(crate) fn reject_referer_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_referer: MultiRefererData = parse_macro_input!(attr as MultiRefererData);
    inject(position, item, |context: &Ident, _: &Ident| {
        let statements = multi_referer.referer_values.iter().map(|referer_value| {
            quote! {
                if #context.get_request().try_get_header_back(::hyperlane_core::REFERER).map_or(false, |referer_header| referer_header == #referer_value) {
                    return ::hyperlane_core::Status::Continue;
                }
            }
        });
        quote! {
            #(#statements)*
        }
    })
}
