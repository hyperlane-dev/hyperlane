use super::*;

/// Implementation of Parse trait for FromStreamData.
///
/// This implementation handles parsing of macro attributes that specify stream processing parameters.
/// It supports an optional variable name parameter.
///
/// # Arguments
/// - `input`: The parse stream containing the token stream to be parsed
///
/// # Returns
/// Returns a `syn::Result<Self>` containing the parsed FromStreamData on success,
/// or a syn::Error with appropriate error message on failure
///
/// # Errors
/// This function returns an error when:
/// - More than one parameter is provided
impl Parse for FromStreamData {
    /// Parses the input token stream into a FromStreamData structure.
    ///
    /// This method implements the core parsing logic for the FromStream macro attribute.
    /// It handles two possible parameter configurations:
    /// 1. Single parameter: interpreted as variable name
    /// 2. No parameters: variable_name will be None
    ///
    /// # Arguments
    /// - `ParseStream`: The ParseStream containing the token stream to be parsed
    ///
    /// # Returns
    /// Returns `syn::Result<Self>` where:
    /// - Ok(FromStreamData) contains the successfully parsed data with variable name
    /// - Err(syn::Error) contains an appropriate error message for invalid input
    ///
    /// # Errors
    /// The function returns errors in the following cases:
    /// - More than one parameter is provided
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let variable_name: Option<Expr> = if input.is_empty() {
            None
        } else {
            let expr: Expr = input.parse()?;
            if !input.is_empty() {
                return Err(syn::Error::new(
                    input.span(),
                    "expected at most one parameter",
                ));
            }
            Some(expr)
        };
        Ok(FromStreamData { variable_name })
    }
}
