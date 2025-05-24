use crate::*;

impl<T> ErrorHandle for T where T: Fn(String) {}

impl<F> Func for F where F: Fn(Context) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync {}

impl<F, Fut> FuncWithoutPin<Fut> for F
where
    F: Fn(Context) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send,
{
}
