use super::*;

/// Main macro for creating and configuring a Hyperlane server instance.
/// Supports both single and multiple variable-type pair initialization.
///
/// This macro expects an attribute in the format `#[hyperlane(variable_name: TypeName)]`
/// or `#[hyperlane(var1: Type1, var2: Type2, ...)]`.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream, containing the variable and type name.
/// - `TokenStream` - The input token stream to process, typically an `async fn`.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with server initialization.
pub(crate) fn hyperlane_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let multi_hyperlane: MultiHyperlaneAttr = parse_macro_input!(attr as MultiHyperlaneAttr);
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let vis: &Visibility = &input_fn.vis;
    let sig: &Signature = &input_fn.sig;
    let block: &Block = &input_fn.block;
    let attrs: &Vec<Attribute> = &input_fn.attrs;
    let stmts: &Vec<Stmt> = &block.stmts;
    let mut init_statements: Vec<proc_macro2::TokenStream> = Vec::new();
    for (var_name, type_name) in &multi_hyperlane.params {
        init_statements.push(quote! {
            let mut #var_name: #type_name = #type_name::default();
        });
        if type_name == SERVER_TYPE_KEY {
            init_statements.push(quote! {
                let mut hooks: Vec<::hyperlane_core::HookType> = ::hyperlane_core::inventory::iter().cloned().collect();
                ::hyperlane_core::HookType::assert_unique_order(hooks.clone());
                hooks.sort_by_key(|hook| hook.try_get_order());
                for hook in hooks {
                    #var_name.handle_hook(hook.clone());
                }
            });
        }
    }
    let gen_code: proc_macro2::TokenStream = quote! {
        #(#attrs)*
        #vis #sig {
            #(#init_statements)*
            #(#stmts)*
        }
    };
    gen_code.into()
}
