use crate::*;

pub trait ErrorHandler<Fut>: Fn(Context, PanicInfo) -> Fut + Send + Sync + 'static
where
    Fut: Future<Output = ()> + Send,
{
}

pub trait FnPinBoxSendSync:
    Fn(Context) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync
{
}

pub trait FnSendSyncStatic<Fut>: Fn(Context) -> Fut + Send + Sync + 'static
where
    Fut: Future<Output = ()> + Send,
{
}

pub trait FutureSendStatic: Future<Output = ()> + Send + 'static {}
