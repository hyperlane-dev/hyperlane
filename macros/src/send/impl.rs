use super::*;

/// Implementation of Parse trait for SendData.
///     
/// Parses data to send from input stream.
impl Parse for SendData {
    /// Parses the input token stream into a SendData structure.
    ///
    /// # Arguments
    ///
    /// - `ParseStream` - The token stream to parse.
    ///
    /// # Returns
    ///
    /// - `syn::Result<Self>` - The parsed SendData, or an error for invalid input.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let data: Expr = input.parse()?;
        Ok(SendData { data })
    }
}
