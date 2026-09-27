use super::*;

/// Sets response status code from macro input.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with status code setting.
pub(crate) fn response_status_code_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let value: Expr = match parse(attr) {
        Ok(v) => v,
        Err(err) => return err.to_compile_error().into(),
    };
    inject(position, item, |context: &Ident, _: &Ident| {
        let new_context: proc_macro2::TokenStream = leak_mut_context(false, context);
        quote! {
            #new_context.get_mut_response().set_status_code(::hyperlane::ResponseStatusCode::from(#value as usize));
        }
    })
}

/// Sets response reason phrase from macro input.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with reason phrase setting.
pub(crate) fn response_reason_phrase_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let value: Expr = match parse(attr) {
        Ok(v) => v,
        Err(err) => return err.to_compile_error().into(),
    };
    inject(position, item, |context: &Ident, _: &Ident| {
        let new_context: proc_macro2::TokenStream = leak_mut_context(false, context);
        quote! {
            #new_context.get_mut_response().set_reason_phrase(&#value);
        }
    })
}

/// Sets or replaces response header from macro input.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with header operation.
pub(crate) fn response_header_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let header_data: ResponseHeaderData = parse_macro_input!(attr as ResponseHeaderData);
    let key: Expr = header_data.key;
    let value: Expr = header_data.value;
    let operation: HeaderOperation = header_data.operation;
    inject(position, item, |context: &Ident, _: &Ident| {
        let new_context: proc_macro2::TokenStream = leak_mut_context(false, context);
        match operation {
            HeaderOperation::Add => {
                quote! {
                    #new_context.get_mut_response().add_header(&#key, &#value);
                }
            }
            HeaderOperation::Set => {
                quote! {
                    #new_context.get_mut_response().set_header(&#key, &#value);
                }
            }
        }
    })
}

/// Sets response body from macro input.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with body setting.
pub(crate) fn response_body_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let body_data: ResponseBodyData = parse_macro_input!(attr as ResponseBodyData);
    let body: Expr = body_data.body;
    inject(position, item, |context: &Ident, _: &Ident| {
        let new_context: proc_macro2::TokenStream = leak_mut_context(false, context);
        quote! {
            #new_context.get_mut_response().set_body(&#body);
        }
    })
}

/// Clears all response headers from macro input.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with header operation.
pub(crate) fn clear_response_headers_macro(item: TokenStream, position: Position) -> TokenStream {
    inject(position, item, |context: &Ident, _: &Ident| {
        let new_context: proc_macro2::TokenStream = leak_mut_context(false, context);
        quote! {
            #new_context.get_mut_response().clear_headers();
        }
    })
}

/// Sets response version from macro input.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with version setting.
pub(crate) fn response_version_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let value: Expr = match parse(attr) {
        Ok(v) => v,
        Err(err) => return err.to_compile_error().into(),
    };
    inject(position, item, |context: &Ident, _: &Ident| {
        let new_context: proc_macro2::TokenStream = leak_mut_context(false, context);
        quote! {
            #new_context.get_mut_response().set_version(#value);
        }
    })
}
