use super::*;

/// Creates a method check function for HTTP request validation.
///
/// # Arguments
///
/// - `&proc_macro2::Ident` - The HTTP method name as an ident.
///
/// # Returns
///
/// Returns a closure that generates the method check code.
pub(crate) fn create_method_check(
    method: &proc_macro2::Ident,
) -> impl FnOnce(&Ident, &Ident) -> proc_macro2::TokenStream {
    let method_str: String = method.to_string();
    move |context: &Ident, _: &Ident| {
        let check_fn: proc_macro2::Ident = Ident::new(&format!("is_{method_str}"), context.span());
        quote! {
            if !#context.get_request().get_method().#check_fn() {
                return ::hyperlane_core::Status::Continue;
            }
        }
    }
}

/// Handles HTTP requests for multiple method types.
///
/// This macro allows a handler to respond to multiple HTTP methods.
/// It generates code that checks if the request method matches any of the specified methods.
///
/// # Arguments
///
/// - `TokenStream` - The attribute `TokenStream` containing the list of allowed HTTP methods.
/// - `TokenStream` - The input `TokenStream` representing the handler function.
/// - `Position` - The position at which to inject the method check code.
///
/// # Returns
///
/// Returns the expanded `TokenStream` with the methods check code injected.
pub(crate) fn methods_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let methods: RequestMethods = parse_macro_input!(attr as RequestMethods);
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let sig: &Signature = &input_fn.sig;
    match parse_context_from_signature(sig) {
        Ok(context) => {
            let method_checks = methods.methods.iter().map(|method| {
                let method_str: String = method.to_string();
                let check_fn: proc_macro2::Ident =
                    Ident::new(&format!("is_{method_str}"), method.span());
                quote! {
                    #context.get_request().get_method().#check_fn()
                }
            });
            inject(
                position,
                TokenStream::from(quote! { #input_fn }),
                |_: &Ident, _: &Ident| {
                    quote! {
                        if !(#(#method_checks)||*) {
                            return ::hyperlane_core::Status::Continue;
                        }
                    }
                },
            )
        }
        Err(err) => err.to_compile_error().into(),
    }
}

/// Implements an HTTP method macro.
///
/// This macro generates a handler function for a specific HTTP method (e.g., GET, POST).
/// It expands to a check that aborts the request if the HTTP method does not match.
///
/// # Arguments
///
/// - `$name` - The name of the generated handler function.
/// - `$submit_name` - The string name for macro registration as an ident.
/// - `$method` - The HTTP method as an ident (e.g., get, post).
///
/// # Returns
///
/// Returns a macro that generates a handler function for the specified HTTP method.
macro_rules! impl_http_method_macro {
    ($name:ident, $submit_name:ident, $method:ident) => {
        pub(crate) fn $name(item: TokenStream, position: Position) -> TokenStream {
            inject(
                position,
                item,
                create_method_check(&proc_macro2::Ident::new(
                    stringify!($method),
                    Span::call_site(),
                )),
            )
        }
    };
}

impl_http_method_macro!(is_get_method_handler, is_get_method, get);
impl_http_method_macro!(is_post_method_handler, is_post_method, post);
impl_http_method_macro!(is_put_method_handler, is_put_method, put);
impl_http_method_macro!(is_delete_method_handler, is_delete_method, delete);
impl_http_method_macro!(is_patch_method_handler, is_patch_method, patch);
impl_http_method_macro!(is_head_method_handler, is_head_method, head);
impl_http_method_macro!(is_options_method_handler, is_options_method, options);
impl_http_method_macro!(is_connect_method_handler, is_connect_method, connect);
impl_http_method_macro!(is_trace_method_handler, is_trace_method, trace);
impl_http_method_macro!(is_unknown_method_handler, is_unknown_method, unknown);
