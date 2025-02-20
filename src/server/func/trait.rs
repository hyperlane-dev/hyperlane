use crate::*;
use std::{future::Future, pin::Pin};

pub trait Func:
    Fn(ArcRwLock<ControllerData>) -> Pin<Box<dyn Future<Output = ()> + Send + Sync + 'static>>
    + Send
    + Sync
    + 'static
{
}

pub trait FuncWithoutPin<Fut>:
    Fn(ArcRwLock<ControllerData>) -> Fut + Send + Sync + 'static
where
    Fut: Future<Output = ()> + Send + Sync + 'static,
{
}
