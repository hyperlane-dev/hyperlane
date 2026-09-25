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
