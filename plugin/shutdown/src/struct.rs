use super::*;

/// Configuration controlling graceful drain and OS signal handling for [`ShutdownCoordinator`].
#[derive(Clone, Data, Debug, Eq, PartialEq)]
pub struct ShutdownConfig {
    /// Maximum duration in milliseconds to wait for active in-flight guards to drain.
    #[get(type(copy))]
    pub(crate) drain_timeout_ms: u64,
    /// Maximum duration in milliseconds allowed for each individual [`ShutdownCallback`].
    #[get(type(copy))]
    pub(crate) callback_timeout_ms: u64,
    /// Whether `arm_signals` listens for `SIGINT` (`Ctrl-C`).
    #[get(type(copy))]
    pub(crate) handle_sigint: bool,
    /// Whether `arm_signals` listens for Unix `SIGTERM`.
    #[get(type(copy))]
    pub(crate) handle_sigterm: bool,
}

/// Summary report returned once a graceful shutdown sequence finishes.
#[derive(Clone, Data, Debug, Eq, PartialEq)]
pub struct ShutdownReport {
    /// `true` when all tracked in-flight work drained before `drain_timeout_ms` elapsed.
    #[get(type(copy))]
    pub(crate) drained_cleanly: bool,
    /// Number of still-active guards remaining if the drain timed out (`0` when clean).
    #[get(type(copy))]
    pub(crate) remaining_in_flight: usize,
    /// Names of callbacks that completed within their timeout in execution order.
    pub(crate) completed_callbacks: Vec<String>,
    /// Names of callbacks that timed out.
    pub(crate) timed_out_callbacks: Vec<String>,
    /// Total elapsed shutdown duration in milliseconds.
    #[get(type(copy))]
    pub(crate) elapsed_ms: u64,
}

/// Shared lock-free interior state of [`ShutdownCoordinator`].
pub(crate) struct ShutdownCoordinatorInner {
    /// Active timeout and signal configuration.
    pub(crate) config: ShutdownConfig,
    /// Lock-free shutdown flag (`false` = running, `true` = draining/stopped).
    pub(crate) shutting_down: AtomicBool,
    /// Lock-free count of currently held [`ShutdownGuard`] handles.
    pub(crate) in_flight: AtomicUsize,
    /// Notifier woken whenever shutdown is triggered.
    pub(crate) shutdown_notify: Notify,
    /// Notifier woken when `in_flight` transitions down to `0` during shutdown.
    pub(crate) drained_notify: Notify,
    /// Ordered list of cleanup callbacks (executed in reverse LIFO order).
    pub(crate) callbacks: Vec<SharedShutdownCallback>,
}

/// Thread-safe lock-free graceful shutdown and request-drain coordinator.
#[derive(Clone)]
pub struct ShutdownCoordinator {
    /// Shared coordinator state.
    pub(crate) inner: Arc<ShutdownCoordinatorInner>,
}

/// RAII guard representing one active unit of in-flight work tracked by [`ShutdownCoordinator`].
pub struct ShutdownGuard {
    /// Reference to coordinator state decremented on drop.
    pub(crate) inner: Arc<ShutdownCoordinatorInner>,
}

/// Process-wide [`ShutdownPlugin`] singleton wrapping [`ShutdownCoordinator`].
#[derive(Clone)]
pub struct ShutdownPlugin {
    /// Shared process-wide shutdown coordinator.
    pub(crate) coordinator: ShutdownCoordinator,
}
