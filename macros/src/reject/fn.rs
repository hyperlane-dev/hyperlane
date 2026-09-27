use super::*;

/// Rejects requests based on a boolean condition.
///
/// The function returns early if the provided code block returns `true`.
///
/// # Arguments
///
/// - `TokenStream` - A code block that returns a boolean value.
/// - `TokenStream` - The function to which the attribute is applied.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The modified function wrapped with a conditional check;
///   if the condition evaluates to `true`, the function returns early,
///   otherwise the original function body is executed.
pub(crate) fn reject_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let condition: Expr = parse_macro_input!(attr as Expr);
    inject(position, item, |_: &Ident, _: &Ident| {
        quote! {
            if #condition {
                return ::hyperlane::Status::Continue;
            }
        }
    })
}
