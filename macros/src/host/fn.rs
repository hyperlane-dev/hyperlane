use super::*;

/// Filters requests matching the specified host.
/// Supports both single and multiple host value checks.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with host filter.
pub(crate) fn host_macro(attr: TokenStream, item: TokenStream, position: Position) -> TokenStream {
    let multi_host: MultiHostData = parse_macro_input!(attr as MultiHostData);
    inject(position, item, |context: &Ident, _: &Ident| {
        let statements = multi_host.host_values.iter().map(|host_value: &syn::Expr| {
            quote! {
                if #context.get_request().get_host() != #host_value {
                    return ::hyperlane_core::Status::Continue;
                }
            }
        });
        quote! {
            #(#statements)*
        }
    })
}

/// Rejects requests matching the specified host.
/// Supports both single and multiple host value checks.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with host rejection filter.
pub(crate) fn reject_host_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_host: MultiHostData = parse_macro_input!(attr as MultiHostData);
    inject(position, item, |context: &Ident, _: &Ident| {
        let statements = multi_host.host_values.iter().map(|host_value: &syn::Expr| {
            quote! {
                if #context.get_request().get_host() == #host_value {
                    return ::hyperlane_core::Status::Continue;
                }
            }
        });
        quote! {
            #(#statements)*
        }
    })
}
