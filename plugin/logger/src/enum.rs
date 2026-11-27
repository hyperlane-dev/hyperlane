use super::*;

/// Output wire format emitted by [`LoggerPlugin`].
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum LogFormat {
    /// Human-readable single-line `<LEVEL> <MESSAGE>` format.
    #[default]
    Plain,
    /// Structured single-line JSON object `{"level":"...","target":"...","message":"..."}`.
    Json,
}

/// Commands dispatched over the bounded channel to the background file writer thread.
pub(crate) enum WriterCommand {
    /// Appends one pre-formatted line terminated by `\n`.
    Line(String),
    /// Flushes the buffered writer and notifies the caller through the barrier channel.
    Flush(SyncSender<()>),
}
