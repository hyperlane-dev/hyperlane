use super::*;

/// Implementation of Parse trait for ContextInput.
///
/// Parses an identifier followed by an optional type annotation from the input stream.
/// The identifier is the source context variable. The type annotation is used to
/// determine whether to generate a mutable or immutable reference.
///
/// Expected format: `ctx` or `ctx: &mut Context` or `ctx: &Context`
impl Parse for ContextInput {
    fn parse(input: ParseStream) -> Result<Self> {
        let source_ctx: Ident = input.parse()?;
        let ty: Option<Type> = if input.peek(Token![:]) {
            input.parse::<Token![:]>()?;
            Some(input.parse()?)
        } else {
            None
        };
        Ok(ContextInput { source_ctx, ty })
    }
}
