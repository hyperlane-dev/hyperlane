use super::*;

/// Implementation of Parse trait for RequestMethods.
///
/// Parses HTTP methods from input stream.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<RequestMethods>` - Parsed RequestMethods or error.
impl Parse for RequestMethods {
    /// Parses the input token stream into a RequestMethods structure.
    ///
    /// # Arguments
    ///
    /// - `ParseStream` - The token stream to parse.
    ///
    /// # Returns
    ///
    /// - `syn::Result<Self>` - The parsed RequestMethods, or an error for invalid input.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(RequestMethods {
            methods: Punctuated::parse_separated_nonempty(input)?,
        })
    }
}

/// Implementation of Parse trait for MultiRequestBodyData.
///
/// Parses request body variables from input stream.
/// Supports both single and multiple variables.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<MultiRequestBodyData>` - Parsed MultiRequestBodyData or error.
impl Parse for MultiRequestBodyData {
    /// Parses the input token stream into a MultiRequestBodyData structure.
    ///
    /// # Arguments
    ///
    /// - `ParseStream` - The token stream to parse.
    ///
    /// # Returns
    ///
    /// - `syn::Result<Self>` - The parsed MultiRequestBodyData, or an error for invalid input.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut variables: Vec<Ident> = Vec::new();
        loop {
            let variable: Ident = input.parse()?;
            variables.push(variable);
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
        }
        Ok(MultiRequestBodyData { variables })
    }
}

/// Implementation of Parse trait for MultiRequestBodyJsonData.
///
/// Parses request body JSON variable-type pairs from input stream.
/// Supports both single and multiple pairs.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<MultiRequestBodyJsonData>` - Parsed MultiRequestBodyJsonData or error.
impl Parse for MultiRequestBodyJsonData {
    /// Parses the input token stream into a MultiRequestBodyJsonData structure.
    ///
    /// # Arguments
    ///
    /// - `ParseStream` - The token stream to parse.
    ///
    /// # Returns
    ///
    /// - `syn::Result<Self>` - The parsed MultiRequestBodyJsonData, or an error for invalid input.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut params: Vec<(Ident, Type)> = Vec::new();
        loop {
            let variable: Ident = input.parse()?;
            input.parse::<Token![:]>()?;
            let type_name: Type = input.parse()?;
            params.push((variable, type_name));
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
        }
        Ok(MultiRequestBodyJsonData { params })
    }
}

/// Implementation of Parse trait for MultiAttributeData.
///
/// Parses attribute key-variable-type tuples from input stream.
/// Supports both single and multiple tuples.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<MultiAttributeData>` - Parsed MultiAttributeData or error.
impl Parse for MultiAttributeData {
    /// Parses the input token stream into a MultiAttributeData structure.
    ///
    /// # Arguments
    ///
    /// - `ParseStream` - The token stream to parse.
    ///
    /// # Returns
    ///
    /// - `syn::Result<Self>` - The parsed MultiAttributeData, or an error for invalid input.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut params: Vec<(Expr, Ident, Type)> = Vec::new();
        loop {
            let key_name: Expr = input.parse()?;
            input.parse::<Token![=>]>()?;
            let variable: Ident = input.parse()?;
            input.parse::<Token![:]>()?;
            let type_name: Type = input.parse()?;
            params.push((key_name, variable, type_name));
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
        }
        Ok(MultiAttributeData { params })
    }
}

/// Implementation of Parse trait for MultiAttributesData.
///
/// Parses attributes variables from input stream.
/// Supports both single and multiple variables.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<MultiAttributesData>` - Parsed MultiAttributesData or error.
impl Parse for MultiAttributesData {
    /// Parses the input token stream into a MultiAttributesData structure.
    ///
    /// # Arguments
    ///
    /// - `ParseStream` - The token stream to parse.
    ///
    /// # Returns
    ///
    /// - `syn::Result<Self>` - The parsed MultiAttributesData, or an error for invalid input.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut variables: Vec<Ident> = Vec::new();
        loop {
            let variable: Ident = input.parse()?;
            variables.push(variable);
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
        }
        Ok(MultiAttributesData { variables })
    }
}

/// Implementation of Parse trait for MultiRouteParamData.
///
/// Parses route parameter key-variable pairs from input stream.
/// Supports both single and multiple pairs.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<MultiRouteParamData>` - Parsed MultiRouteParamData or error.
impl Parse for MultiRouteParamData {
    /// Parses the input token stream into a MultiRouteParamData structure.
    ///
    /// # Arguments
    ///
    /// - `ParseStream` - The token stream to parse.
    ///
    /// # Returns
    ///
    /// - `syn::Result<Self>` - The parsed MultiRouteParamData, or an error for invalid input.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut params: Vec<(Expr, Ident)> = Vec::new();
        loop {
            let key_name: Expr = input.parse()?;
            input.parse::<Token![=>]>()?;
            let variable: Ident = input.parse()?;
            params.push((key_name, variable));
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
        }
        Ok(MultiRouteParamData { params })
    }
}

/// Implementation of Parse trait for MultiRouteParamsData.
///
/// Parses route parameters variables from input stream.
/// Supports both single and multiple variables.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<MultiRouteParamsData>` - Parsed MultiRouteParamsData or error.
impl Parse for MultiRouteParamsData {
    /// Parses the input token stream into a MultiRouteParamsData structure.
    ///
    /// # Arguments
    ///
    /// - `ParseStream` - The token stream to parse.
    ///
    /// # Returns
    ///
    /// - `syn::Result<Self>` - The parsed MultiRouteParamsData, or an error for invalid input.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut variables: Vec<Ident> = Vec::new();
        loop {
            let variable: Ident = input.parse()?;
            variables.push(variable);
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
        }
        Ok(MultiRouteParamsData { variables })
    }
}

/// Implementation of Parse trait for MultiQueryData.
///
/// Parses query parameter key-variable pairs from input stream.
/// Supports both single and multiple pairs.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<MultiQueryData>` - Parsed MultiQueryData or error.
impl Parse for MultiQueryData {
    /// Parses the input token stream into a MultiQueryData structure.
    ///
    /// # Arguments
    ///
    /// - `ParseStream` - The token stream to parse.
    ///
    /// # Returns
    ///
    /// - `syn::Result<Self>` - The parsed MultiQueryData, or an error for invalid input.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut params: Vec<(Expr, Ident)> = Vec::new();
        loop {
            let key_name: Expr = input.parse()?;
            input.parse::<Token![=>]>()?;
            let variable: Ident = input.parse()?;
            params.push((key_name, variable));
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
        }
        Ok(MultiQueryData { params })
    }
}

/// Implementation of Parse trait for MultiQuerysData.
///
/// Parses query parameters variables from input stream.
/// Supports both single and multiple variables.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<MultiQuerysData>` - Parsed MultiQuerysData or error.
impl Parse for MultiQuerysData {
    /// Parses the input token stream into a MultiQuerysData structure.
    ///
    /// # Arguments
    ///
    /// - `ParseStream` - The token stream to parse.
    ///
    /// # Returns
    ///
    /// - `syn::Result<Self>` - The parsed MultiQuerysData, or an error for invalid input.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut variables: Vec<Ident> = Vec::new();
        loop {
            let variable: Ident = input.parse()?;
            variables.push(variable);
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
        }
        Ok(MultiQuerysData { variables })
    }
}

/// Implementation of Parse trait for MultiHeaderData.
///
/// Parses header key-variable pairs from input stream.
/// Supports both single and multiple pairs.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<MultiHeaderData>` - Parsed MultiHeaderData or error.
impl Parse for MultiHeaderData {
    /// Parses the input token stream into a MultiHeaderData structure.
    ///
    /// # Arguments
    ///
    /// - `ParseStream` - The token stream to parse.
    ///
    /// # Returns
    ///
    /// - `syn::Result<Self>` - The parsed MultiHeaderData, or an error for invalid input.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut params: Vec<(Expr, Ident)> = Vec::new();
        loop {
            let key_name: Expr = input.parse()?;
            input.parse::<Token![=>]>()?;
            let variable: Ident = input.parse()?;
            params.push((key_name, variable));
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
        }
        Ok(MultiHeaderData { params })
    }
}

/// Implementation of Parse trait for MultiHeadersData.
///
/// Parses headers variables from input stream.
/// Supports both single and multiple variables.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<MultiHeadersData>` - Parsed MultiHeadersData or error.
impl Parse for MultiHeadersData {
    /// Parses the input token stream into a MultiHeadersData structure.
    ///
    /// # Arguments
    ///
    /// - `ParseStream` - The token stream to parse.
    ///
    /// # Returns
    ///
    /// - `syn::Result<Self>` - The parsed MultiHeadersData, or an error for invalid input.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut variables: Vec<Ident> = Vec::new();
        loop {
            let variable: Ident = input.parse()?;
            variables.push(variable);
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
        }
        Ok(MultiHeadersData { variables })
    }
}

/// Implementation of Parse trait for MultiCookieData.
///
/// Parses cookie key-variable pairs from input stream.
/// Supports both single and multiple pairs.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<MultiCookieData>` - Parsed MultiCookieData or error.
impl Parse for MultiCookieData {
    /// Parses the input token stream into a MultiCookieData structure.
    ///
    /// # Arguments
    ///
    /// - `ParseStream` - The token stream to parse.
    ///
    /// # Returns
    ///
    /// - `syn::Result<Self>` - The parsed MultiCookieData, or an error for invalid input.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut params: Vec<(Expr, Ident)> = Vec::new();
        loop {
            let key_name: Expr = input.parse()?;
            input.parse::<Token![=>]>()?;
            let variable: Ident = input.parse()?;
            params.push((key_name, variable));
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
        }
        Ok(MultiCookieData { params })
    }
}

/// Implementation of Parse trait for MultiCookiesData.
///
/// Parses cookies variables from input stream.
/// Supports both single and multiple variables.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<MultiCookiesData>` - Parsed MultiCookiesData or error.
impl Parse for MultiCookiesData {
    /// Parses the input token stream into a MultiCookiesData structure.
    ///
    /// # Arguments
    ///
    /// - `ParseStream` - The token stream to parse.
    ///
    /// # Returns
    ///
    /// - `syn::Result<Self>` - The parsed MultiCookiesData, or an error for invalid input.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut variables: Vec<Ident> = Vec::new();
        loop {
            let variable: Ident = input.parse()?;
            variables.push(variable);
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
        }
        Ok(MultiCookiesData { variables })
    }
}

/// Implementation of Parse trait for MultiRequestVersionData.
///
/// Parses request version variables from input stream.
/// Supports both single and multiple variables.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<MultiRequestVersionData>` - Parsed MultiRequestVersionData or error.
impl Parse for MultiRequestVersionData {
    /// Parses the input token stream into a MultiRequestVersionData structure.
    ///
    /// # Arguments
    ///
    /// - `ParseStream` - The token stream to parse.
    ///
    /// # Returns
    ///
    /// - `syn::Result<Self>` - The parsed MultiRequestVersionData, or an error for invalid input.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut variables: Vec<Ident> = Vec::new();
        loop {
            let variable: Ident = input.parse()?;
            variables.push(variable);
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
        }
        Ok(MultiRequestVersionData { variables })
    }
}

/// Implementation of Parse trait for MultiRequestPathData.
///
/// Parses request path variables from input stream.
/// Supports both single and multiple variables.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<MultiRequestPathData>` - Parsed MultiRequestPathData or error.
impl Parse for MultiRequestPathData {
    /// Parses the input token stream into a MultiRequestPathData structure.
    ///
    /// # Arguments
    ///
    /// - `ParseStream` - The token stream to parse.
    ///
    /// # Returns
    ///
    /// - `syn::Result<Self>` - The parsed MultiRequestPathData, or an error for invalid input.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut variables: Vec<Ident> = Vec::new();
        loop {
            let variable: Ident = input.parse()?;
            variables.push(variable);
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
        }
        Ok(MultiRequestPathData { variables })
    }
}

/// Implementation of Parse trait for MultiPanicData.
///
/// Parses panic data variables from input stream.
/// Supports both single and multiple variables.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<MultiPanicData>` - Parsed MultiPanicData or error.
impl Parse for MultiPanicData {
    /// Parses the input token stream into a MultiPanicData structure.
    ///
    /// # Arguments
    ///
    /// - `ParseStream` - The token stream to parse.
    ///
    /// # Returns
    ///
    /// - `syn::Result<Self>` - The parsed MultiPanicData, or an error for invalid input.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut variables: Vec<Ident> = Vec::new();
        loop {
            let variable: Ident = input.parse()?;
            variables.push(variable);
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
        }
        Ok(MultiPanicData { variables })
    }
}

/// Implementation of Parse trait for MultiRequestErrorData.
///
/// Parses request error data variables from input stream.
/// Supports both single and multiple variables.
///
/// # Arguments
///
/// - `ParseStream` - The input parse stream.
///
/// # Returns
///
/// - `syn::Result<MultiRequestErrorData>` - Parsed MultiRequestErrorData or error.
impl Parse for MultiRequestErrorData {
    /// Parses the input token stream into a MultiRequestErrorData structure.
    ///
    /// # Arguments
    ///
    /// - `ParseStream` - The token stream to parse.
    ///
    /// # Returns
    ///
    /// - `syn::Result<Self>` - The parsed MultiRequestErrorData, or an error for invalid input.
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let mut variables: Vec<Ident> = Vec::new();
        loop {
            let variable: Ident = input.parse()?;
            variables.push(variable);
            if input.is_empty() {
                break;
            }
            input.parse::<Token![,]>()?;
            if input.is_empty() {
                break;
            }
        }
        Ok(MultiRequestErrorData { variables })
    }
}
