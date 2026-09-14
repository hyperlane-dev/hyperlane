use super::*;

/// Tries to send data via stream after function execution.
///
/// If an attribute argument is provided, it is used as the data to send.
/// If no argument is provided, the response is built from the context automatically.
///
/// # Arguments
///
/// - `TokenStream` - Optional attribute containing the data expression to send.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with try send operation.
pub(crate) fn try_send_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let data_expr: Option<Expr> = if attr.is_empty() {
        None
    } else {
        let data: SendData = parse_macro_input!(attr as SendData);
        Some(data.data)
    };
    inject(position, item, |context, stream| match data_expr {
        Some(expr) => {
            quote! {
                let _: ::std::result::Result<(), ::hyperlane_core::ResponseError> = #stream.try_send(#expr).await;
            }
        }
        None => {
            quote! {
                let _: ::std::result::Result<(), ::hyperlane_core::ResponseError> = #stream.try_send(#context.get_mut_response().build()).await;
            }
        }
    })
}

/// Sends data via stream after function execution, panics on failure.
///
/// If an attribute argument is provided, it is used as the data to send.
/// If no argument is provided, the response is built from the context automatically.
///
/// # Arguments
///
/// - `TokenStream` - Optional attribute containing the data expression to send.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with send operation that panics on failure.
pub(crate) fn send_macro(attr: TokenStream, item: TokenStream, position: Position) -> TokenStream {
    let data_expr: Option<Expr> = if attr.is_empty() {
        None
    } else {
        let data: SendData = parse_macro_input!(attr as SendData);
        Some(data.data)
    };
    inject(position, item, |context, stream| match data_expr {
        Some(expr) => {
            quote! {
                #stream.send(#expr).await;
            }
        }
        None => {
            quote! {
                #stream.send(#context.get_mut_response().build()).await;
            }
        }
    })
}
