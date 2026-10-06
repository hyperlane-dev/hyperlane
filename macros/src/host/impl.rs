use super::*;

/// Implementation of Parse trait for MultiHostData.
///
/// Parses host value expressions from input stream.
/// Supports both single and multiple host values.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<MultiHostData>` - Parsed MultiHostData or error.
impl Parse for MultiHostData {
    /// Parses the input token stream into a MultiHostData structure.
    ///
    /// # Arguments
    ///
    /// - `ParseStream` - The token stream to parse.
    ///
    /// # Returns
    ///
    /// - `syn::Result<Self>` - The parsed MultiHostData, or an error for invalid input.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut host_values: Vec<Expr> = Vec::new();
        loop {
            let host_value: Expr = input.parse()?;
            host_values.push(host_value);
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
        }
        Ok(MultiHostData { host_values })
    }
}
