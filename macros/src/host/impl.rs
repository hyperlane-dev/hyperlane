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
