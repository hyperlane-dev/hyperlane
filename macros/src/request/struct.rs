use super::*;

/// Container for HTTP methods data.
///
/// Used to store parsed HTTP methods from macro input.
pub(crate) struct RequestMethods {
    /// The parsed HTTP methods as punctuated identifiers.
    pub(crate) methods: Punctuated<Ident, Token![,]>,
}

/// Container for request body data.
///
/// Used to store parsed request body variables from macro input.
/// Supports both single and multiple variables.
pub(crate) struct MultiRequestBodyData {
    /// Vector of request body variables.
    pub(crate) variables: Vec<Ident>,
}

/// Container for JSON request body data.
///
/// Used to store parsed JSON request body variable-type pairs from macro input.
/// Supports both single and multiple variable-type pairs.
pub(crate) struct MultiRequestBodyJsonData {
    /// Vector of JSON request body variable-type pairs.
    pub(crate) params: Vec<(Ident, Type)>,
}

/// Container for request attributes data.
///
/// Used to store parsed attribute key-variable-type tuples from macro input.
/// Supports both single and multiple tuples.
pub(crate) struct MultiAttributeData {
    /// Vector of attribute key-variable-type tuples.
    pub(crate) params: Vec<(Expr, Ident, Type)>,
}

/// Container for request attributes collection data.
///
/// Used to store parsed attributes variables from macro input.
/// Supports both single and multiple variables.
pub(crate) struct MultiAttributesData {
    /// Vector of attributes variables.
    pub(crate) variables: Vec<Ident>,
}

/// Container for route parameters data.
///
/// Used to store parsed route parameter key-variable pairs from macro input.
/// Supports both single and multiple pairs.
pub(crate) struct MultiRouteParamData {
    /// Vector of route parameter key-variable pairs.
    pub(crate) params: Vec<(Expr, Ident)>,
}

/// Container for route parameters collection data.
///
/// Used to store parsed route parameters variables from macro input.
/// Supports both single and multiple variables.
pub(crate) struct MultiRouteParamsData {
    /// Vector of route parameters variables.
    pub(crate) variables: Vec<Ident>,
}

/// Container for query parameters data.
///
/// Used to store parsed query parameter key-variable pairs from macro input.
/// Supports both single and multiple pairs.
pub(crate) struct MultiQueryData {
    /// Vector of query parameter key-variable pairs.
    pub(crate) params: Vec<(Expr, Ident)>,
}

/// Container for query parameters collection data.
///
/// Used to store parsed query parameters variables from macro input.
/// Supports both single and multiple variables.
pub(crate) struct MultiQuerysData {
    /// Vector of query parameters variables.
    pub(crate) variables: Vec<Ident>,
}

/// Container for request headers data.
///
/// Used to store parsed header key-variable pairs from macro input.
/// Supports both single and multiple pairs.
pub(crate) struct MultiHeaderData {
    /// Vector of header key-variable pairs.
    pub(crate) params: Vec<(Expr, Ident)>,
}

/// Container for request headers collection data.
///
/// Used to store parsed headers variables from macro input.
/// Supports both single and multiple variables.
pub(crate) struct MultiHeadersData {
    /// Vector of headers variables.
    pub(crate) variables: Vec<Ident>,
}

/// Container for request cookies data.
///
/// Used to store parsed cookie key-variable pairs from macro input.
/// Supports both single and multiple pairs.
pub(crate) struct MultiCookieData {
    /// Vector of cookie key-variable pairs.
    pub(crate) params: Vec<(Expr, Ident)>,
}

/// Container for request cookies collection data.
///
/// Used to store parsed cookies variables from macro input.
/// Supports both single and multiple variables.
pub(crate) struct MultiCookiesData {
    /// Vector of cookies variables.
    pub(crate) variables: Vec<Ident>,
}

/// Container for request version data.
///
/// Used to store parsed request version variables from macro input.
/// Supports both single and multiple variables.
pub(crate) struct MultiRequestVersionData {
    /// Vector of request version variables.
    pub(crate) variables: Vec<Ident>,
}

/// Container for request path data.
///
/// Used to store parsed request path variables from macro input.
/// Supports both single and multiple variables.
pub(crate) struct MultiRequestPathData {
    /// Vector of request path variables.
    pub(crate) variables: Vec<Ident>,
}

/// Container for panic data.
///
/// Used to store parsed panic data variable from macro input.
/// Supports both single and multiple variables.
pub(crate) struct MultiPanicData {
    /// Vector of panic data variables.
    pub(crate) variables: Vec<Ident>,
}

/// Container for request error data.
///
/// Used to store parsed request error data variable from macro input.
/// Supports both single and multiple variables.
pub(crate) struct MultiRequestErrorData {
    /// Vector of request error data variables.
    pub(crate) variables: Vec<Ident>,
}
