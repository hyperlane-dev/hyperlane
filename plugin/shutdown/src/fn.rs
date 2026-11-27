use super::*;

/// Waits for Unix `SIGTERM` when enabled, or remains pending on non-Unix platforms or when disabled.
///
/// # Arguments
///
/// - `bool` - Whether `SIGTERM` handling is enabled.
pub(crate) async fn wait_sigterm(enabled: bool) {
    if !enabled {
        pending::<()>().await;
    }
    #[cfg(unix)]
    {
        if let Ok(mut sig) = signal(SignalKind::terminate()) {
            let _: Option<()> = sig.recv().await;
            return;
        }
    }
    pending::<()>().await;
}
