use super::*;

/// Implementation of Parse trait for MultiRefererData.
///
/// Parses referer value expressions from input stream.
/// Supports both single and multiple referer values.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<MultiRefererData>` - Parsed MultiRefererData or error.
impl Parse for MultiRefererData {
    /// Parses the input token stream into a MultiRefererData structure.
    ///
    /// # Arguments
    ///
    /// - `ParseStream` - The token stream to parse.
    ///
    /// # Returns
    ///
    /// - `syn::Result<Self>` - The parsed MultiRefererData, or an error for invalid input.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut referer_values: Vec<Expr> = Vec::new();
        loop {
            let referer_value: Expr = input.parse()?;
            referer_values.push(referer_value);
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
        }
        Ok(MultiRefererData { referer_values })
    }
}
