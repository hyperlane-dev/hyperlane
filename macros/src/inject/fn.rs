use super::*;

/// Applies a macro to a token stream.
///
/// This function takes a macro's metadata and a token stream, finds the corresponding
/// registered macro, and applies it.
///
/// # Arguments
///
/// - `&Meta` - The metadata of the macro to apply.
/// - `TokenStream` - The token stream to apply the macro to.
/// - `Position` - The position to inject the code.
///
/// # Returns
///
/// The resulting token stream after applying the macro.
///
/// # Panics
///
/// This function will panic if the macro is not found, if the macro format is unsupported,
/// or if a simple macro is given attributes.
fn apply_macro(macro_meta: &Meta, item_stream: TokenStream, position: Position) -> TokenStream {
    let (macro_name, macro_attr) = match macro_meta {
        Meta::Path(path) => (
            path.get_ident()
                .expect("Macro path should have an identifier")
                .to_string(),
            TokenStream::new(),
        ),
        Meta::List(meta_list) => (
            meta_list
                .path
                .get_ident()
                .expect("Macro path should have an identifier")
                .to_string(),
            meta_list.tokens.clone().into(),
        ),
        _ => panic!("Unsupported macro format in inject macro"),
    };
    for injectable_macro in INJECTABLE_MACROS {
        if injectable_macro.name == macro_name {
            return match injectable_macro.handler {
                Handler::WithAttr(handler) => handler(macro_attr, item_stream),
                Handler::NoAttrPosition(handler) => {
                    if !macro_attr.is_empty() {
                        panic!("Macro {macro_name} does not take attributes");
                    }
                    handler(item_stream, position)
                }
                Handler::WithAttrPosition(handler) => handler(macro_attr, item_stream, position),
            };
        }
    }
    panic!("Unsupported macro: {macro_name}");
}

/// Injects a list of macros before the decorated function.
///
/// The macros are applied in head-insertion order, meaning the first macro in the list
/// is the outermost macro.
///
/// # Arguments
///
/// - `TokenStream` - The token stream representing the attributes of the macro.
/// - `TokenStream` - The token stream representing the item to which the macro is applied.
///
/// # Returns
///
/// The resulting token stream after applying all the prologue hooks.
pub(crate) fn prologue_macros_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let metas: Punctuated<Meta, Comma> = Punctuated::<Meta, Token![,]>::parse_terminated
        .parse(attr)
        .expect("Failed to parse macro attributes");
    let mut current_stream: TokenStream = item;
    for meta in metas.iter().rev() {
        current_stream = apply_macro(meta, current_stream, Position::Prologue);
    }
    current_stream
}

/// Injects a list of macros after the decorated function.
///
/// The macros are applied in tail-insertion order, meaning the last macro in the list
/// is the outermost macro.
///
/// # Arguments
///
/// - `TokenStream` - The token stream representing the attributes of the macro.
/// - `TokenStream` - The token stream representing the item to which the macro is applied.
///
/// # Returns
///
/// The resulting token stream after applying all the epilogue hooks.
pub(crate) fn epilogue_macros_macro(attr: TokenStream, item: TokenStream) -> TokenStream {
    let metas: Punctuated<Meta, Comma> = Punctuated::<Meta, Token![,]>::parse_terminated
        .parse(attr)
        .expect("Failed to parse macro attributes");
    let mut current_stream: TokenStream = item;
    for meta in metas.iter() {
        current_stream = apply_macro(meta, current_stream, Position::Epilogue);
    }
    current_stream
}
