use crate::*;

pub trait AsyncFunc:
    Fn(ArcRwLock<ControllerData>) -> Pin<Box<dyn Future<Output = ()> + Send + Sync + 'static>>
    + Send
    + Sync
    + 'static
{
}

pub trait AsyncFuncWithoutPin<Fut>:
    Fn(ArcRwLock<ControllerData>) -> Fut + Send + Sync + 'static
where
    Fut: Future<Output = ()> + Send + 'static,
{
}

pub trait Func: Fn(ArcRwLock<ControllerData>) + Send + Sync + 'static {}
