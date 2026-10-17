use super::*;

/// Injects an anonymous import at the top of the decorated function body.
///
/// This function parses the attribute as a path and prepends `use <path> as _;`
/// to the function body, bringing trait methods (or other importable items)
/// into scope without binding a name.
///
/// # Arguments
///
/// - `TokenStream` - The path to import anonymously.
/// - `TokenStream` - The function item to be modified.
///
/// # Returns
///
/// Returns the expanded `TokenStream` with the anonymous import injected.
pub(crate) fn import_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let path: Path = match parse(attr) {
        Ok(path) => path,
        Err(error) => return error.to_compile_error().into(),
    };
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let attrs: &Vec<Attribute> = &input_fn.attrs;
    let vis: &Visibility = &input_fn.vis;
    let sig: &Signature = &input_fn.sig;
    let stmts: &Vec<Stmt> = &input_fn.block.stmts;
    let gen_code: proc_macro2::TokenStream = quote! {
        #(#attrs)*
        #vis #sig {
            #[allow(unused_imports)]
            use #path as _;
            #(#stmts)*
        }
    };
    gen_code.into()
}
