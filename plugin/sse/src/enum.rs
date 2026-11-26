/// Errors returned by [`crate::SseHub`], [`crate::SseSubscription`], and [`crate::SseStream`] operations.
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum SseError {
    /// Creating a new topic would exceed `SseConfig::max_topics`.
    TopicLimitReached,
    /// The underlying topic broadcast channel has closed.
    ChannelClosed,
    /// The subscriber lagged behind the broadcast ring by `u64` messages.
    Lagged(u64),
    /// Writing an SSE frame to the HTTP response stream failed.
    Transport(String),
}
