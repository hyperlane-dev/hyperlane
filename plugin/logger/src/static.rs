use super::*;

/// The configuration the plugin was given, when it was given one.
pub(super) static CONFIG: OnceLock<LoggerConfig> = OnceLock::new();

/// Non-blocking channel feeding the background buffered file writer thread.
pub(super) static WRITER_TX: OnceLock<Option<SyncSender<WriterCommand>>> = OnceLock::new();

/// Lock-free count of records dropped when the background writer channel was saturated.
pub(super) static DROPPED_RECORDS: AtomicU64 = AtomicU64::new(0);
