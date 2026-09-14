use super::*;

/// Parses the attributes for the `OrderAttr` macro.
///
/// This implementation of the `Parse` trait allows `syn` to parse
/// an optional `order` from the macro's attribute tokens.
/// If no order is provided, it defaults to `0`.
impl Parse for OrderAttr {
    /// Parses the input stream into an `OrderAttr` struct.
    ///
    /// # Arguments
    ///
    /// - `input` - The token stream to parse.
    ///
    /// # Returns
    ///
    /// A `Result` containing the parsed `OrderAttr` or an error.
    fn parse(input: ParseStream) -> Result<Self> {
        if input.is_empty() {
            return Ok(OrderAttr { order: None });
        }
        let expr: Expr = input.parse()?;
        Ok(OrderAttr { order: Some(expr) })
    }
}
