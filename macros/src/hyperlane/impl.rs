use super::*;

/// Implementation of the `Parse` trait for `MultiHyperlaneAttr`.
///
/// This implementation allows parsing multiple variable-type pairs from a token stream,
/// expecting the format `variable_name: TypeName, variable_name2: TypeName2, ...`.
/// Also supports single pair format for backward compatibility.
///
/// # Arguments
///
/// - `ParseStream` - The `ParseStream` to parse from.
///
/// # Returns
///
/// A `syn::Result` containing the parsed `MultiHyperlaneAttr` or an error.
impl Parse for MultiHyperlaneAttr {
    fn parse(input: ParseStream) -> Result<Self> {
        let mut params: Vec<(Ident, Ident)> = Vec::new();
        loop {
            let var_name: Ident = input.parse()?;
            input.parse::<Token![:]>()?;
            let type_name: Ident = input.parse()?;
            params.push((var_name, type_name));
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
        }
        Ok(MultiHyperlaneAttr { params })
    }
}
