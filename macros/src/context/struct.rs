use super::*;

/// Parses the input for context macro.
///
/// This struct represents the source context identifier and optional type annotation
/// parsed from the macro input. The type annotation is used to determine whether
/// to generate a mutable or immutable reference.
pub(crate) struct ContextInput {
    /// The source context variable identifier.
    pub(crate) source_ctx: Ident,
    /// The optional type annotation (e.g., `: &mut Context` or `: &Context`).
    pub(crate) ty: Option<Type>,
}
