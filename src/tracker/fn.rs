use crate::*;

pub async fn with_context<F, R>(context: Context, future: F) -> R
where
    F: Future<Output = R>,
{
    REQUEST_CONTEXT.scope(context, future).await
}
