use super::*;

/// Generates stream processing loop for HTTP stream.
///
/// Creates a `while let Ok(request)` loop that reads HTTP requests from the stream,
/// sets the request on the context, and executes the function body statements.
///
/// # Arguments
///
/// - `&Ident` - The stream identifier to use for stream access.
/// - `&Ident` - The context identifier to use for setting the request.
/// - `&FromStreamData` - The FromStreamData containing variable name.
/// - `&[Stmt]` - The statements to execute when data is successfully read.
///
/// # Returns
///
/// - `TokenStream2` - The generated loop code as a token stream.
pub(crate) fn generate_http_stream(
    stream: &Ident,
    context: &Ident,
    data: &FromStreamData,
    stmts: &[Stmt],
) -> proc_macro2::TokenStream {
    let method_ident: Ident = Ident::new("try_get_http_request", Span::call_site());
    match data.variable_name.clone() {
        Some(variable_name) => {
            quote! {
                while let Ok(#variable_name) = #stream.#method_ident().await {
                    #context.set_request(#variable_name.clone());
                    #(#stmts)*
                }
                ::hyperlane::Status::Continue
            }
        }
        None => {
            quote! {
                while let Ok(_request) = #stream.#method_ident().await {
                    #context.set_request(_request);
                    #(#stmts)*
                }
                ::hyperlane::Status::Continue
            }
        }
    }
}

/// Generates stream processing loop for WebSocket stream.
///
/// Creates a `while let Ok(body)` loop that reads WebSocket data from the stream,
/// sets the body on the context's request, and executes the function body statements.
///
/// # Arguments
///
/// - `&Ident` - The stream identifier to use for stream access.
/// - `&Ident` - The context identifier to use for setting the request body.
/// - `&FromStreamData` - The FromStreamData containing variable name.
/// - `&[Stmt]` - The statements to execute when data is successfully read.
///
/// # Returns
///
/// - `TokenStream2` - The generated loop code as a token stream.
pub(crate) fn generate_websocket_stream(
    stream: &Ident,
    context: &Ident,
    data: &FromStreamData,
    stmts: &[Stmt],
) -> proc_macro2::TokenStream {
    let method_ident: Ident = Ident::new("try_get_websocket_request", Span::call_site());
    match data.variable_name.clone() {
        Some(variable_name) => {
            quote! {
                while let Ok(#variable_name) = #stream.#method_ident().await {
                    #context.get_mut_request().set_body(#variable_name.clone());
                    #(#stmts)*
                }
                ::hyperlane::Status::Continue
            }
        }
        None => {
            quote! {
                while let Ok(_body) = #stream.#method_ident().await {
                    #context.get_mut_request().set_body(_body);
                    #(#stmts)*
                }
                ::hyperlane::Status::Continue
            }
        }
    }
}

/// Wraps function body with HTTP stream processing.
///
/// This macro generates code that wraps the function body with a check to see if
/// data can be read from an HTTP stream. The function body is only executed
/// if data is successfully read from the stream. The read request is automatically
/// set on the context via `ctx.set_request()`.
///
/// # Arguments
///
/// - `TokenStream` - The attribute containing the variable name.
/// - `TokenStream` - The input token stream to process.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with HTTP stream processing.
pub(crate) fn try_get_http_request_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let data: FromStreamData = parse_macro_input!(attr as FromStreamData);
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let vis: &Visibility = &input_fn.vis;
    let sig: &Signature = &input_fn.sig;
    let block: &Block = &input_fn.block;
    let attrs: &Vec<Attribute> = &input_fn.attrs;
    match parse_stream_from_signature(sig) {
        Ok(stream) => match parse_context_from_signature(sig) {
            Ok(context) => {
                let stmts: &Vec<Stmt> = &block.stmts;
                let loop_stream: proc_macro2::TokenStream =
                    generate_http_stream(&stream, &context, &data, stmts);
                quote! {
                    #(#attrs)*
                    #vis #sig {
                        #loop_stream
                    }
                }
                .into()
            }
            Err(err) => err.to_compile_error().into(),
        },
        Err(err) => err.to_compile_error().into(),
    }
}

/// Wraps function body with WebSocket stream processing.
///
/// This macro generates code that wraps the function body with a check to see if
/// data can be read from a WebSocket stream. The function body is only executed
/// if data is successfully read from the stream. The read body data is automatically
/// set on the context's request via `ctx.get_mut_request().set_body()`.
///
/// # Arguments
///
/// - `attr` - The attribute containing the variable name.
/// - `item` - The input token stream to process.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with WebSocket stream processing.
pub(crate) fn try_get_websocket_request_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let data: FromStreamData = parse_macro_input!(attr as FromStreamData);
    let input_fn: ItemFn = parse_macro_input!(item as ItemFn);
    let vis: &Visibility = &input_fn.vis;
    let sig: &Signature = &input_fn.sig;
    let block: &Block = &input_fn.block;
    let attrs: &Vec<Attribute> = &input_fn.attrs;
    match parse_stream_from_signature(sig) {
        Ok(stream) => match parse_context_from_signature(sig) {
            Ok(context) => {
                let stmts: &Vec<Stmt> = &block.stmts;
                let loop_stream: proc_macro2::TokenStream =
                    generate_websocket_stream(&stream, &context, &data, stmts);
                quote! {
                    #(#attrs)*
                    #vis #sig {
                        #loop_stream
                    }
                }
                .into()
            }
            Err(err) => err.to_compile_error().into(),
        },
        Err(err) => err.to_compile_error().into(),
    }
}
