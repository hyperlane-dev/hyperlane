use super::*;

/// Registers a response middleware.
///
/// This macro takes a struct as input and registers it as a response middleware.
/// The registered struct will be used to create handlers that are called after the main request handler but before the response is sent.
///
/// # Arguments
///
/// - `TokenStream` - The attribute `TokenStream`, which can optionally specify an `order`.
/// - `TokenStream` - The input token stream representing the struct to be registered as a middleware.
///
/// # Note
///
/// If an order parameter is not specified, the hook will have a higher priority than hooks with a specified order.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with the middleware registration.
pub(crate) fn response_middleware_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let attr_args: OrderAttr = parse_macro_input!(attr as OrderAttr);
    let order: proc_macro2::TokenStream = expr_to_isize(&attr_args.order);
    let input_struct: ItemStruct = parse_macro_input!(item as ItemStruct);
    let struct_name: &Ident = &input_struct.ident;
    let gen_code: proc_macro2::TokenStream = quote! {
        #input_struct
        ::hyperlane::inventory::submit! {
            ::hyperlane::HookType::ResponseMiddleware(#order, || ::hyperlane::Hook::factory::<#struct_name>())
        }
    };
    gen_code.into()
}
