use super::*;

/// Implementation of Parse trait for SendData.
///     
/// Parses data to send from input stream.
impl Parse for SendData {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let data: Expr = input.parse()?;
        Ok(SendData { data })
    }
}
