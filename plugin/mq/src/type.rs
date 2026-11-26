use super::*;

/// Boxed `Send` future returned by async [`MessageBroker`] trait methods.
pub type BrokerFuture<'a, T> = Pin<Box<dyn Future<Output = Result<T, MqError>> + Send + 'a>>;
