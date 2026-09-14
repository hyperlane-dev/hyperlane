use super::*;

/// Implements the `Parse` trait for `RouteAttr`.
///
/// This implementation defines how to parse a `TokenStream` into a `RouteAttr` struct,
/// extracting the path expression from the input.
impl Parse for RouteAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let first_expr: Expr = input.parse()?;
        Ok(RouteAttr { path: first_expr })
    }
}
