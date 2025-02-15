use crate::*;

impl<F> Func for F where F: Fn(ArcRwLock<ControllerData>) + Send + Sync + 'static {}

impl<F> AsyncFunc for F where
    F: Fn(ArcRwLock<ControllerData>) -> Pin<Box<dyn Future<Output = ()> + Send + Sync + 'static>>
        + Send
        + Sync
        + 'static
{
}

impl<F, Fut> AsyncFuncWithoutPin<Fut> for F
where
    F: Fn(ArcRwLock<ControllerData>) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send + Sync + 'static,
{
}
