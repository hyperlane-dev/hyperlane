pub trait AsyncFuncWithoutPin<Fut>: Fn(ArcRwLock<ControllerData>) -> Fut + Send + Sync + 'static
where
    Fut: Future<Output = ()> + Send + 'static,
{
}