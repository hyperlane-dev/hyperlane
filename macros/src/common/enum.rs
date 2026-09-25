use super::*;

/// Defines the type of macro handler.
///
/// This enum distinguishes between simple macros that do not accept attributes
/// and more complex macros that can process attribute inputs. It is used to route
/// macro invocations to the appropriate expansion logic based on their expected syntax.
pub(crate) enum Handler {
    /// A macro handler for macros that accept attribute arguments.
    ///
    /// This variant is used for macros that support syntax like `#[my_macro(...)]`,
    /// where the content inside the parentheses is parsed and processed as input.
    /// The `MacroHandlerWithAttr` contains the logic to interpret and expand such macros.
    WithAttr(MacroHandlerWithAttr),
    /// A macro handler for simple macros that do not take any attributes.
    ///
    /// This variant is used for attribute-like macros that are invoked as `#[my_macro]`
    /// without any additional arguments. The associated `MacroHandlerPosition` is responsible
    /// for handling the macro at a specific location in the syntax tree.
    NoAttrPosition(MacroHandlerPosition),
    /// A macro handler for macros that accept attribute arguments and depend on position.
    ///
    /// This variant is used for macros with syntax like `#[my_macro(...)]`, similar to `WithAttr`.
    /// The difference is that `WithAttrPosition` also incorporates the syntactic position
    /// of the macro invocation into the expansion logic.
    /// The `MacroHandlerWithAttrPosition` handles both the attribute input and the positional context.
    WithAttrPosition(MacroHandlerWithAttrPosition),
}

/// Defines the position where code should be injected in a function.
pub(crate) enum Position {
    /// Injects code at the beginning of the function body.
    Prologue,
    /// Injects code at the end of the function body.
    Epilogue,
}
