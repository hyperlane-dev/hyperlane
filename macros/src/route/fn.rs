use super::*;

/// Internal implementation for the `route` attribute macro.
///
/// This function processes the route attribute and generates code to register
/// the decorated struct as a route handler in the inventory system.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream containing route parameters (path)
/// - `TokenStream` - The struct token stream being decorated
///
/// # Returns
///
/// A `TokenStream` containing the original struct and inventory registration code
///
/// # Generated Code
///
/// The macro generates:
/// - The original struct unchanged
/// - An `inventory::submit!` block that registers a `HookType` instance
/// - A handler factory that creates boxed handlers for the struct
pub(crate) fn route_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let route_attr: RouteAttr = parse_macro_input!(attr as RouteAttr);
    let path: &Expr = &route_attr.path;
    let input_struct: ItemStruct = parse_macro_input!(item as ItemStruct);
    let struct_name: &Ident = &input_struct.ident;
    let gen_code: proc_macro2::TokenStream = quote! {
        #input_struct
        ::hyperlane::inventory::submit! {
            ::hyperlane::HookType::Route(#path, || ::hyperlane::Hook::factory::<#struct_name>())
        }
    };
    gen_code.into()
}
