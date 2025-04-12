use crate::*;

pub trait Func: Fn(Context) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync {}

pub trait FuncWithoutPin<Fut>: Fn(Context) -> Fut + Send + Sync + 'static
where
    Fut: Future<Output = ()> + Send,
{
}
