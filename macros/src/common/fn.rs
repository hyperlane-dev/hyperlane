use super::*;

/// Expands macro with code inserted before method body, providing both context and stream identifiers.
///
/// # Arguments
///
/// - `TokenStream` - The input token stream to process.
/// - `FnOnce(&Ident, &Ident) -> TokenStream2` - Function to generate code inserted before, receiving context and stream idents.
///
/// # Returns
///
/// - `TokenStream` - The expanded token stream with inserted code.
fn inject_at_start(
    input: TokenStream,
    before_fn: impl FnOnce(&Ident, &Ident) -> proc_macro2::TokenStream,
) -> TokenStream {
    let input_fn: ItemFn = parse_macro_input!(input as ItemFn);
    let vis: &Visibility = &input_fn.vis;
    let sig: &Signature = &input_fn.sig;
    let block: &Block = &input_fn.block;
    let attrs: &Vec<Attribute> = &input_fn.attrs;
    match parse_context_from_signature(sig) {
        Ok(context) => match parse_stream_from_signature(sig) {
            Ok(stream) => {
                let before_code: proc_macro2::TokenStream = before_fn(&context, &stream);
                let stmts: &Vec<Stmt> = &block.stmts;
                let gen_code: proc_macro2::TokenStream = quote! {
                    #(#attrs)*
                    #vis #sig {
                        #before_code
                        #(#stmts)*
                    }
                };
                gen_code.into()
            }
            Err(err) => err.to_compile_error().into(),
        },
        Err(err) => err.to_compile_error().into(),
    }
}

/// Expands macro with code inserted after method body, providing both context and stream identifiers.
///
/// # Arguments
///
/// - `TokenStream` - The input `TokenStream` to process.
/// - `FnOnce(&Ident, &Ident) -> TokenStream2` - A closure that takes context and stream identifiers and returns a `TokenStream` to be inserted at the end of the method.
fn inject_at_end(
    input: TokenStream,
    after_fn: impl FnOnce(&Ident, &Ident) -> proc_macro2::TokenStream,
) -> TokenStream {
    let input_fn: ItemFn = parse_macro_input!(input as ItemFn);
    let vis: &Visibility = &input_fn.vis;
    let sig: &Signature = &input_fn.sig;
    let block: &Block = &input_fn.block;
    let attrs: &Vec<Attribute> = &input_fn.attrs;
    match parse_context_from_signature(sig) {
        Ok(context) => match parse_stream_from_signature(sig) {
            Ok(stream) => {
                let after_code: proc_macro2::TokenStream = after_fn(&context, &stream);
                let stmts: &Vec<Stmt> = &block.stmts;
                let (leading_stmts, tail_expr) = if let Some((last, leading)) = stmts.split_last() {
                    match last {
                        Stmt::Expr(expr, None) => (leading, Some(quote! { #expr })),
                        _ => (stmts.as_slice(), None),
                    }
                } else {
                    (stmts.as_slice(), None)
                };
                let normalized_leading: Vec<proc_macro2::TokenStream> = leading_stmts
                    .iter()
                    .map(|stmt: &syn::Stmt| match stmt {
                        Stmt::Expr(expr, None) => quote! { #expr; },
                        _ => quote! { #stmt },
                    })
                    .collect();
                let gen_code: proc_macro2::TokenStream = match tail_expr {
                    Some(expr) => quote! {
                        #(#attrs)*
                        #vis #sig {
                            #(#normalized_leading)*
                            #after_code
                            #expr
                        }
                    },
                    None => quote! {
                        #(#attrs)*
                        #vis #sig {
                            #(#normalized_leading)*
                            #after_code
                        }
                    },
                };
                gen_code.into()
            }
            Err(err) => err.to_compile_error().into(),
        },
        Err(err) => err.to_compile_error().into(),
    }
}

/// Injects code into a method at a specified position, providing both context and stream identifiers.
///
/// # Arguments
///
/// - `Position` - The position at which to inject the code (`Prologue` or `Epilogue`).
/// - `TokenStream` - The input `TokenStream` of the method to modify.
/// - `FnOnce(&Ident, &Ident) -> TokenStream2` - A closure that generates the code to be injected, based on the method's context and stream identifiers.
///
/// # Returns
///
/// - `TokenStream` - Returns the modified `TokenStream` with the injected code.
pub(crate) fn inject(
    position: Position,
    input: TokenStream,
    hook: impl FnOnce(&Ident, &Ident) -> proc_macro2::TokenStream,
) -> TokenStream {
    match position {
        Position::Prologue => inject_at_start(input, hook),
        Position::Epilogue => inject_at_end(input, hook),
    }
}

/// Checks if a type matches `::hyperlane_core::Context`.
///
/// This function checks if the given type is a reference to `::hyperlane_core::Context`.
///
/// # Arguments
///
/// - `&Type` - The type to check.
///
/// # Returns
///
/// - `bool` - Returns `true` if the type is `&::hyperlane_core::Context` or `&mut Context`, `false` otherwise.
fn is_context_type(ty: &Type) -> bool {
    if let Type::Reference(type_ref) = ty
        && let Type::Path(type_path) = &*type_ref.elem
    {
        let path: &Path = &type_path.path;
        if path.segments.len() >= 2 {
            let segments: Vec<&syn::PathSegment> = path.segments.iter().collect();
            if segments.len() >= 2 {
                let last_two: &[&PathSegment] = &segments[segments.len() - 2..];
                if last_two[0].ident == "hyperlane" && last_two[1].ident == "Context" {
                    return true;
                }
            }
        }
        if path.segments.len() == 1 && path.segments[0].ident == "Context" {
            return true;
        }
    }
    false
}

/// Checks if a type matches `::hyperlane_core::Stream`.
///
/// This function checks if the given type is a reference to `::hyperlane_core::Stream`.
///
/// # Arguments
///
/// - `&Type` - The type to check.
///
/// # Returns
///
/// - `bool` - Returns `true` if the type is `&::hyperlane_core::Stream` or `&mut Stream`, `false` otherwise.
fn is_stream_type(ty: &Type) -> bool {
    if let Type::Reference(type_ref) = ty
        && let Type::Path(type_path) = &*type_ref.elem
    {
        let path: &Path = &type_path.path;
        if path.segments.len() >= 2 {
            let segments: Vec<&syn::PathSegment> = path.segments.iter().collect();
            if segments.len() >= 2 {
                let last_two: &[&PathSegment] = &segments[segments.len() - 2..];
                if last_two[0].ident == "hyperlane" && last_two[1].ident == "Stream" {
                    return true;
                }
            }
        }
        if path.segments.len() == 1 && path.segments[0].ident == "Stream" {
            return true;
        }
    }
    false
}

/// Parses context identifier from function signature by searching all parameters.
///
/// This function iterates through all function parameters and returns the first one
/// that has type `::hyperlane_core::Context`. It supports:
/// 1. Methods with self: Searches from the second parameter onwards
/// 2. Functions without self: Searches from the first parameter onwards
/// 3. Context parameter can be at any position
/// 4. Anonymous `_` parameters are returned as-is
///
/// # Arguments
///
/// - `&Signature` - The function signature to parse.
///
/// # Returns
///
/// - `syn::Result<Ident>` - Returns the context identifier or an error.
pub(crate) fn parse_context_from_signature(sig: &Signature) -> syn::Result<Ident> {
    for arg in sig.inputs.iter() {
        if let FnArg::Typed(pat_type) = arg
            && is_context_type(&pat_type.ty)
        {
            let ident: Ident = match &*pat_type.pat {
                Pat::Ident(pat_ident) => pat_ident.ident.clone(),
                Pat::Wild(_) => Ident::new("_", Span::call_site()),
                _ => {
                    return Err(syn::Error::new_spanned(
                        &pat_type.pat,
                        "expected identifier for context parameter",
                    ));
                }
            };
            return Ok(ident);
        }
    }
    Err(syn::Error::new_spanned(
        &sig.inputs,
        "expected at least one parameter of type &::hyperlane_core::Context",
    ))
}

/// Parses stream identifier from function signature by searching all parameters.
///
/// This function iterates through all function parameters and returns the first one
/// that has type `::hyperlane_core::Stream`. It supports:
/// 1. Methods with self: Searches from the second parameter onwards
/// 2. Functions without self: Searches from the first parameter onwards
/// 3. Stream parameter can be at any position
/// 4. Anonymous `_` parameters are returned as-is
///
/// # Arguments
///
/// - `&Signature` - The function signature to parse.
///
/// # Returns
///
/// - `syn::Result<Ident>` - Returns the stream identifier or an error.
pub(crate) fn parse_stream_from_signature(sig: &Signature) -> syn::Result<Ident> {
    for arg in sig.inputs.iter() {
        if let FnArg::Typed(pat_type) = arg
            && is_stream_type(&pat_type.ty)
        {
            let ident: Ident = match &*pat_type.pat {
                Pat::Ident(pat_ident) => pat_ident.ident.clone(),
                Pat::Wild(_) => Ident::new("_", Span::call_site()),
                _ => {
                    return Err(syn::Error::new_spanned(
                        &pat_type.pat,
                        "expected identifier for stream parameter",
                    ));
                }
            };
            return Ok(ident);
        }
    }
    Err(syn::Error::new_spanned(
        &sig.inputs,
        "expected at least one parameter of type &::hyperlane_core::Stream",
    ))
}

/// Convert an optional expression into an `Option<isize>` token stream.
///
/// This function supports integer and string literals only:
/// - Integer literals are parsed and converted into `Some(isize)`.
/// - String literals are parsed into `isize` and wrapped in `Some(...)`.
/// - Any other expression types will result in `None`.
/// - If `opt_expr` is `None`, the result is also `None`.
///
/// # Arguments
///
/// - `&Option<Expr>` - An optional reference to the expression to convert.
///
/// # Returns
///
/// - `TokenStream` - A `TokenStream2` representing `Some(isize)` for supported literals, or `None` otherwise.
pub(crate) fn expr_to_isize(opt_expr: &Option<Expr>) -> proc_macro2::TokenStream {
    match opt_expr {
        Some(expr) => match expr {
            Expr::Lit(ExprLit {
                lit: Lit::Int(lit_int),
                ..
            }) => {
                let value: isize = lit_int.base10_parse::<isize>().unwrap();
                quote! { Some(#value) }
            }
            Expr::Lit(ExprLit {
                lit: Lit::Str(lit_str),
                ..
            }) => {
                let value: isize = lit_str.value().parse().expect("Cannot parse to isize");
                quote! { Some(#value) }
            }
            _ => quote! { None },
        },
        None => quote! { None },
    }
}

/// Generates a token stream that calls `leak_mut()` method on the context identifier.
///
/// # Arguments
///
/// - `bool` - Whether to use `unsafe` or not.
/// - `&Ident` - The context variable identifier.
///
/// # Returns
///
/// - `TokenStream2` - The token stream calling `#context.leak_mut()`.
///
/// # Safety
///
/// - The address is guaranteed to be a valid `Self` instance
///   that was previously converted from a reference and is managed by the runtime.
pub(crate) fn leak_mut_context(is_unsafe_error: bool, context: &Ident) -> proc_macro2::TokenStream {
    if is_unsafe_error {
        quote! {
          #context.leak_mut()
        }
    } else {
        quote! {
            unsafe { #context.leak_mut() }
        }
    }
}

/// Generates a token stream that calls `leak()` method on the context identifier.
///
/// # Arguments
///
/// - `&Ident` - The context variable identifier.
///
/// # Returns
///
/// - `TokenStream2` - The token stream calling `#context.leak()`.
///
/// # Safety
///
/// - The address is guaranteed to be a valid `Self` instance
///   that was previously converted from a reference and is managed by the runtime.
pub(crate) fn leak_context(is_unsafe_error: bool, context: &Ident) -> proc_macro2::TokenStream {
    if is_unsafe_error {
        quote! {
          #context.leak()
        }
    } else {
        quote! {
            unsafe { #context.leak() }
        }
    }
}
