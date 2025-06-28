use crate::*;

impl<T> ErrorHandler for T where T: Fn(String) {}

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

impl<T> FutureSendStatic for T where T: Future<Output = ()> + Send + 'static {}
