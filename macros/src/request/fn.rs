use super::*;

/// Gets raw request body and assigns to specified variable.
/// Supports both single and multiple variable extraction.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with body extraction.
pub(crate) fn request_body_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_body: MultiRequestBodyData = parse_macro_input!(attr as MultiRequestBodyData);
    inject(position, item, |context: &Ident, _: &Ident| {
        let new_context: proc_macro2::TokenStream = leak_context(false, context);
        let statements = multi_body.variables.iter().map(|variable: &syn::Ident| {
            quote! {
                let #variable: &::hyperlane_core::RequestBody = #new_context.get_request().get_body();
            }
        });
        quote! {
            #(#statements)*
        }
    })
}

/// Parses request body as JSON and assigns to specified variable.
/// Supports both single and multiple variable-type pair extraction.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with JSON parsing.
pub(crate) fn request_body_json_result_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_body_json: MultiRequestBodyJsonData =
        parse_macro_input!(attr as MultiRequestBodyJsonData);
    inject(position, item, |context: &Ident, _: &Ident| {
        let statements = multi_body_json.params.iter().map(|(variable, type_name): &(syn::Ident, syn::Type)| {
            quote! {
                let #variable: Result<#type_name, ::hyperlane_core::serde_json::Error> = #context.get_request().try_get_body_json::<#type_name>();
            }
        });
        quote! {
            #(#statements)*
        }
    })
}

/// Parses request body as JSON and assigns to specified variable.
/// Supports both single and multiple variable-type pair extraction.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with JSON parsing.
pub(crate) fn request_body_json_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_body_json: MultiRequestBodyJsonData =
        parse_macro_input!(attr as MultiRequestBodyJsonData);
    inject(position, item, |context: &Ident, _: &Ident| {
        let statements = multi_body_json.params.iter().map(|(variable, type_name): &(syn::Ident, syn::Type)| {
            quote! {
                let #variable: #type_name = #context.get_request().get_body_json::<#type_name>();
            }
        });
        quote! {
            #(#statements)*
        }
    })
}

/// Gets request attribute by key and assigns to specified variable.
/// Supports both single and multiple attribute extraction.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with attribute extraction.
pub(crate) fn try_get_attribute_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_attr: MultiAttributeData = parse_macro_input!(attr as MultiAttributeData);
    inject(position, item, |context: &Ident, _: &Ident| {
        let statements = multi_attr.params.iter().map(
            |(key_name, variable, type_name): &(syn::Expr, syn::Ident, syn::Type)| {
                quote! {
                    let #variable: Option<#type_name> = #context.try_get_attribute(&#key_name);
                }
            },
        );
        quote! {
            #(#statements)*
        }
    })
}

/// Gets request attribute by key and assigns to specified variable.
/// Supports both single and multiple attribute extraction.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with attribute extraction.
pub(crate) fn attribute_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_attr: MultiAttributeData = parse_macro_input!(attr as MultiAttributeData);
    inject(position, item, |context: &Ident, _: &Ident| {
        let statements = multi_attr.params.iter().map(
            |(key_name, variable, type_name): &(syn::Expr, syn::Ident, syn::Type)| {
                quote! {
                    let #variable: #type_name = #context.get_attribute(&#key_name);
                }
            },
        );
        quote! {
            #(#statements)*
        }
    })
}

/// Gets all request attributes and assigns to specified variable.
/// Supports both single and multiple variable extraction.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with attributes extraction.
pub(crate) fn attributes_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_attrs: MultiAttributesData = parse_macro_input!(attr as MultiAttributesData);
    inject(position, item, |context: &Ident, _: &Ident| {
        let new_context: proc_macro2::TokenStream = leak_context(false, context);
        let statements = multi_attrs.variables.iter().map(|variable: &syn::Ident| {
            quote! {
                let #variable: &::hyperlane_core::ThreadSafeAttributeStore = #new_context.get_attributes();
            }
        });
        quote! {
            #(#statements)*
        }
    })
}

/// Gets panic data and assigns to specified variable wrapped in Option.
/// Supports both single and multiple variable extraction.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with panic data extraction.
pub(crate) fn try_get_task_panic_data_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_task_panic_data: MultiPanicData = parse_macro_input!(attr as MultiPanicData);
    inject(position, item, |context: &Ident, _: &Ident| {
        let statements = multi_task_panic_data.variables.iter().map(|variable: &syn::Ident| {
            quote! {
                let #variable: Option<::hyperlane_core::PanicData> = #context.try_get_task_panic_data();
            }
        });
        quote! {
            #(#statements)*
        }
    })
}

/// Gets panic data and assigns to specified variable with panic on missing value.
/// Supports both single and multiple variable extraction.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with panic data extraction.
pub(crate) fn task_panic_data_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_task_panic_data: MultiPanicData = parse_macro_input!(attr as MultiPanicData);
    inject(position, item, |context: &Ident, _: &Ident| {
        let statements = multi_task_panic_data
            .variables
            .iter()
            .map(|variable: &syn::Ident| {
                quote! {
                    let #variable: ::hyperlane_core::PanicData = #context.get_task_panic_data();
                }
            });
        quote! {
            #(#statements)*
        }
    })
}

/// Gets request error data and assigns to specified variable wrapped in Option.
/// Supports both single and multiple variable extraction.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with request error data extraction.
pub(crate) fn try_get_request_error_data_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_error_data: MultiRequestErrorData = parse_macro_input!(attr as MultiRequestErrorData);
    inject(position, item, |context: &Ident, _: &Ident| {
        let statements = multi_error_data.variables.iter().map(|variable: &syn::Ident| {
            quote! {
                let #variable: Option<::hyperlane_core::RequestError> = #context.try_get_request_error_data();
            }
        });
        quote! {
            #(#statements)*
        }
    })
}

/// Gets request error data and assigns to specified variable with panic on missing value.
/// Supports both single and multiple variable extraction.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with request error data extraction.
pub(crate) fn request_error_data_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_error_data: MultiRequestErrorData = parse_macro_input!(attr as MultiRequestErrorData);
    inject(position, item, |context: &Ident, _: &Ident| {
        let statements = multi_error_data.variables.iter().map(|variable: &syn::Ident| {
            quote! {
                let #variable: ::hyperlane_core::RequestError = #context.get_request_error_data();
            }
        });
        quote! {
            #(#statements)*
        }
    })
}

/// Gets route parameter by key and assigns to specified variable.
/// Supports both single and multiple route parameter extraction.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with route param extraction.
pub(crate) fn try_get_route_param_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_param: MultiRouteParamData = parse_macro_input!(attr as MultiRouteParamData);
    inject(position, item, |context: &Ident, _: &Ident| {
        let statements = multi_param.params.iter().map(|(key_name, variable): &(syn::Expr, syn::Ident)| {
            quote! {
                let #variable: Option<std::string::String> = #context.try_get_route_param(#key_name);
            }
        });
        quote! {
            #(#statements)*
        }
    })
}

/// Gets route parameter by key and assigns to specified variable.
/// Supports both single and multiple route parameter extraction.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with route param extraction.
pub(crate) fn route_param_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_param: MultiRouteParamData = parse_macro_input!(attr as MultiRouteParamData);
    inject(position, item, |context: &Ident, _: &Ident| {
        let statements =
            multi_param
                .params
                .iter()
                .map(|(key_name, variable): &(syn::Expr, syn::Ident)| {
                    quote! {
                        let #variable: std::string::String = #context.get_route_param(#key_name);
                    }
                });
        quote! {
            #(#statements)*
        }
    })
}

/// Gets all route parameters and assigns to specified variable.
/// Supports both single and multiple variable extraction.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with route params extraction.
pub(crate) fn route_params_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_route_params: MultiRouteParamsData = parse_macro_input!(attr as MultiRouteParamsData);
    inject(position, item, |context: &Ident, _: &Ident| {
        let new_context: proc_macro2::TokenStream = leak_context(false, context);
        let statements = multi_route_params
            .variables
            .iter()
            .map(|variable: &syn::Ident| {
                quote! {
                    let #variable: &::hyperlane_core::RouteParams = #new_context.get_route_params();
                }
            });
        quote! {
            #(#statements)*
        }
    })
}

/// Gets request query parameter by key and assigns to specified variable.
/// Supports both single and multiple parameter extraction.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with query param extraction.
pub(crate) fn try_get_request_query_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_query: MultiQueryData = parse_macro_input!(attr as MultiQueryData);
    inject(position, item, |context: &Ident, _: &Ident| {
        let statements = multi_query.params.iter().map(|(key_name, variable): &(syn::Expr, syn::Ident)| {
            quote! {
                let #variable: Option<::hyperlane_core::RequestQuerysValue> = #context.get_request().try_get_query(#key_name);
            }
        });
        quote! {
            #(#statements)*
        }
    })
}

/// Gets request query parameter by key and assigns to specified variable.
/// Supports both single and multiple parameter extraction.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with query param extraction.
pub(crate) fn request_query_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_query: MultiQueryData = parse_macro_input!(attr as MultiQueryData);
    inject(position, item, |context: &Ident, _: &Ident| {
        let statements = multi_query.params.iter().map(|(key_name, variable): &(syn::Expr, syn::Ident)| {
            quote! {
                let #variable: ::hyperlane_core::RequestQuerysValue = #context.get_request().get_query(#key_name);
            }
        });
        quote! {
            #(#statements)*
        }
    })
}

/// Gets all request query parameters and assigns to specified variable.
/// Supports both single and multiple variable extraction.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with query params extraction.
pub(crate) fn request_querys_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_querys: MultiQuerysData = parse_macro_input!(attr as MultiQuerysData);
    inject(position, item, |context: &Ident, _: &Ident| {
        let new_context: proc_macro2::TokenStream = leak_context(false, context);
        let statements = multi_querys.variables.iter().map(|variable: &syn::Ident| {
            quote! {
                let #variable: &::hyperlane_core::RequestQuerys = #new_context.get_request().get_querys();
            }
        });
        quote! {
            #(#statements)*
        }
    })
}

/// Gets request header by key and assigns to specified variable.
/// Supports both single and multiple header extraction.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with header extraction.
pub(crate) fn try_get_request_header_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_header: MultiHeaderData = parse_macro_input!(attr as MultiHeaderData);
    inject(position, item, |context: &Ident, _: &Ident| {
        let statements = multi_header.params.iter().map(|(key_name, variable): &(syn::Expr, syn::Ident)| {
            quote! {
                let #variable: Option<::hyperlane_core::RequestHeadersValueItem> = #context.get_request().try_get_header_back(#key_name);
            }
        });
        quote! {
            #(#statements)*
        }
    })
}

/// Gets request header by key and assigns to specified variable.
/// Supports both single and multiple header extraction.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with header extraction.
pub(crate) fn request_header_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_header: MultiHeaderData = parse_macro_input!(attr as MultiHeaderData);
    inject(position, item, |context: &Ident, _: &Ident| {
        let statements = multi_header.params.iter().map(|(key_name, variable): &(syn::Expr, syn::Ident)| {
            quote! {
                let #variable: ::hyperlane_core::RequestHeadersValueItem = #context.get_request().get_header_back(#key_name);
            }
        });
        quote! {
            #(#statements)*
        }
    })
}

/// Gets all request headers and assigns to specified variable.
/// Supports both single and multiple variable extraction.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with headers extraction.
pub(crate) fn request_headers_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_headers: MultiHeadersData = parse_macro_input!(attr as MultiHeadersData);
    inject(position, item, |context: &Ident, _: &Ident| {
        let new_context: proc_macro2::TokenStream = leak_context(false, context);
        let statements = multi_headers.variables.iter().map(|variable: &syn::Ident| {
            quote! {
                let #variable: &::hyperlane_core::RequestHeaders = #new_context.get_request().get_headers();
            }
        });
        quote! {
            #(#statements)*
        }
    })
}

/// Gets request cookie by key and assigns to specified variable.
/// Supports both single and multiple cookie extraction.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with cookie extraction.
pub(crate) fn try_get_request_cookie_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_cookie: MultiCookieData = parse_macro_input!(attr as MultiCookieData);
    inject(position, item, |context: &Ident, _: &Ident| {
        let statements = multi_cookie.params.iter().map(|(key_name, variable): &(syn::Expr, syn::Ident)| {
            quote! {
                let #variable: Option<::hyperlane_core::CookieValue> = #context.get_request().try_get_cookie(#key_name);
            }
        });
        quote! {
            #(#statements)*
        }
    })
}

/// Gets request cookie by key and assigns to specified variable.
/// Supports both single and multiple cookie extraction.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with cookie extraction.
pub(crate) fn request_cookie_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_cookie: MultiCookieData = parse_macro_input!(attr as MultiCookieData);
    inject(position, item, |context: &Ident, _: &Ident| {
        let statements = multi_cookie.params.iter().map(|(key_name, variable): &(syn::Expr, syn::Ident)| {
            quote! {
                let #variable: ::hyperlane_core::CookieValue = #context.get_request().get_cookie(#key_name);
            }
        });
        quote! {
            #(#statements)*
        }
    })
}

/// Gets all request cookies and assigns to specified variable.
/// Supports both single and multiple variable extraction.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with cookies extraction.
pub(crate) fn request_cookies_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_cookies: MultiCookiesData = parse_macro_input!(attr as MultiCookiesData);
    inject(position, item, |context: &Ident, _: &Ident| {
        let statements = multi_cookies.variables.iter().map(|variable: &syn::Ident| {
            quote! {
                let #variable: ::hyperlane_core::Cookies = #context.get_request().get_cookies();
            }
        });
        quote! {
            #(#statements)*
        }
    })
}

/// Gets request version and assigns to specified variable.
/// Supports both single and multiple variable extraction.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with version extraction.
pub(crate) fn request_version_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_version: MultiRequestVersionData =
        parse_macro_input!(attr as MultiRequestVersionData);
    inject(position, item, |context: &Ident, _: &Ident| {
        let new_context: proc_macro2::TokenStream = leak_context(false, context);
        let statements = multi_version.variables.iter().map(|variable: &syn::Ident| {
            quote! {
                let #variable: &::hyperlane_core::RequestVersion = #new_context.get_request().get_version();
            }
        });
        quote! {
            #(#statements)*
        }
    })
}

/// Gets request path and assigns to specified variable.
/// Supports both single and multiple variable extraction.
///
/// # Arguments
///
/// - `TokenStream` - The attribute token stream.
/// - `TokenStream` - The input token stream to process.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with path extraction.
pub(crate) fn request_path_macro(
    attr: TokenStream,
    item: TokenStream,
    position: Position,
) -> TokenStream {
    let multi_path: MultiRequestPathData = parse_macro_input!(attr as MultiRequestPathData);
    inject(position, item, |context: &Ident, _: &Ident| {
        let new_context: proc_macro2::TokenStream = leak_context(false, context);
        let statements = multi_path.variables.iter().map(|variable: &syn::Ident| {
            quote! {
                let #variable: &::hyperlane_core::RequestPath = #new_context.get_request().get_path();
            }
        });
        quote! {
            #(#statements)*
        }
    })
}
