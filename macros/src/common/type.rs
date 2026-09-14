use super::*;

/// A type alias for a simple macro handler function.
///
/// This handler takes a single `TokenStream` as input and returns a `TokenStream`.
pub(crate) type MacroHandlerPosition = fn(TokenStream, Position) -> TokenStream;

/// A type alias for a macro handler function that accepts attributes.
///
/// This handler takes two `TokenStream`s as input (one for attributes, one for the item)
/// and returns a `TokenStream`.
pub(crate) type MacroHandlerWithAttr = fn(TokenStream, TokenStream) -> TokenStream;

/// A type alias for a macro handler function that accepts attributes and a position.
///
/// This handler takes two `TokenStream`s as input (one for attributes, one for the item),
/// a `Position` enum, and returns a `TokenStream`.
pub(crate) type MacroHandlerWithAttrPosition =
    fn(TokenStream, TokenStream, Position) -> TokenStream;
