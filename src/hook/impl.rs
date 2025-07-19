use crate::*;

impl<F, Fut> ErrorHandler<Fut> for F
where
    F: Fn(Context) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send,
{
}

impl<F> FnPinBoxSendSync for F where
    F: Fn(Context) -> Pin<Box<dyn Future<Output = ()> + Send>> + Send + Sync
{
}

impl<F, Fut> FnSendSyncStatic<Fut> for F
where
    F: Fn(Context) -> Fut + Send + Sync + 'static,
    Fut: Future<Output = ()> + Send,
{
}

impl<T, R> FutureSendStatic<R> for T where T: Future<Output = R> + Send + 'static {}
