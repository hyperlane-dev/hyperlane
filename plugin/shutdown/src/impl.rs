use super::*;

/// Default construction for [`ShutdownConfig`].
impl Default for ShutdownConfig {
    /// Constructs a default [`ShutdownConfig`] with 30 s drain timeout and `SIGINT`/`SIGTERM` enabled.
    fn default() -> Self {
        Self {
            drain_timeout_ms: DEFAULT_SHUTDOWN_DRAIN_TIMEOUT_MS,
            callback_timeout_ms: DEFAULT_SHUTDOWN_CALLBACK_TIMEOUT_MS,
            handle_sigint: true,
            handle_sigterm: true,
        }
    }
}

/// Inherent builders for [`ShutdownConfig`].
impl ShutdownConfig {
    /// Sets the maximum in-flight drain timeout in milliseconds.
    ///
    /// # Arguments
    ///
    /// - `u64` - Drain timeout in milliseconds.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_drain_timeout_ms(mut self, drain_timeout_ms: u64) -> Self {
        self.set_drain_timeout_ms(drain_timeout_ms);
        self
    }

    /// Sets the maximum timeout for each [`ShutdownCallback`] in milliseconds.
    ///
    /// # Arguments
    ///
    /// - `u64` - Per-callback timeout in milliseconds.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_callback_timeout_ms(mut self, callback_timeout_ms: u64) -> Self {
        self.set_callback_timeout_ms(callback_timeout_ms);
        self
    }
}

/// Drop implementation for [`ShutdownGuard`].
impl Drop for ShutdownGuard {
    /// Decrements the lock-free in-flight counter and wakes drain waiters upon reaching zero.
    fn drop(&mut self) {
        let Self { inner } = self;
        let prev: usize = inner.in_flight.fetch_sub(1, Ordering::AcqRel);
        if prev == 1 && inner.shutting_down.load(Ordering::Acquire) {
            inner.drained_notify.notify_waiters();
        }
    }
}

/// Default construction for [`ShutdownCoordinator`].
impl Default for ShutdownCoordinator {
    /// Constructs a default [`ShutdownCoordinator`].
    fn default() -> Self {
        Self::new(ShutdownConfig::default())
    }
}

/// Inherent implementation of [`ShutdownCoordinator`].
impl ShutdownCoordinator {
    /// Creates a new [`ShutdownCoordinator`] governed by `config`.
    ///
    /// # Arguments
    ///
    /// - `ShutdownConfig` - Drain timeout and signal handling settings.
    ///
    /// # Returns
    ///
    /// - `Self` - Ready shutdown coordinator.
    pub fn new(config: ShutdownConfig) -> Self {
        Self {
            inner: Arc::new(ShutdownCoordinatorInner {
                config,
                shutting_down: AtomicBool::new(false),
                in_flight: AtomicUsize::new(0),
                shutdown_notify: Notify::new(),
                drained_notify: Notify::new(),
                callbacks: Vec::new(),
            }),
        }
    }

    /// Registers an async [`ShutdownCallback`] executed in LIFO order during shutdown.
    ///
    /// # Arguments
    ///
    /// - `C` - Concrete [`ShutdownCallback`] implementation.
    ///
    /// # Returns
    ///
    /// - `Self` - New coordinator handle including the registered callback.
    pub fn with_callback<C>(self, callback: C) -> Self
    where
        C: ShutdownCallback,
    {
        let Self { inner } = self;
        let mut callbacks: Vec<SharedShutdownCallback> = inner.callbacks.clone();
        callbacks.push(Arc::new(callback));
        Self {
            inner: Arc::new(ShutdownCoordinatorInner {
                config: inner.config.clone(),
                shutting_down: AtomicBool::new(inner.shutting_down.load(Ordering::Acquire)),
                in_flight: AtomicUsize::new(inner.in_flight.load(Ordering::Acquire)),
                shutdown_notify: Notify::new(),
                drained_notify: Notify::new(),
                callbacks,
            }),
        }
    }

    /// Returns the active [`ShutdownConfig`].
    ///
    /// # Returns
    ///
    /// - `&ShutdownConfig` - Active configuration.
    pub fn get_config(&self) -> &ShutdownConfig {
        let Self { inner } = self;
        &inner.config
    }

    /// Reports whether shutdown has been triggered.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` once shutdown has started.
    pub fn is_shutting_down(&self) -> bool {
        let Self { inner } = self;
        inner.shutting_down.load(Ordering::Acquire)
    }

    /// Returns the current number of active [`ShutdownGuard`] instances.
    ///
    /// # Returns
    ///
    /// - `usize` - In-flight guard count.
    pub fn in_flight(&self) -> usize {
        let Self { inner } = self;
        inner.in_flight.load(Ordering::Acquire)
    }

    /// Acquires a lock-free RAII [`ShutdownGuard`] if shutdown has not yet started.
    ///
    /// # Returns
    ///
    /// - `Option<ShutdownGuard>` - Active guard, or `None` once shutdown has been triggered.
    pub fn enter(&self) -> Option<ShutdownGuard> {
        let Self { inner } = self;
        if inner.shutting_down.load(Ordering::Acquire) {
            return None;
        }
        inner.in_flight.fetch_add(1, Ordering::AcqRel);
        if inner.shutting_down.load(Ordering::Acquire) {
            let prev: usize = inner.in_flight.fetch_sub(1, Ordering::AcqRel);
            if prev == 1 {
                inner.drained_notify.notify_waiters();
            }
            return None;
        }
        Some(ShutdownGuard {
            inner: Arc::clone(inner),
        })
    }

    /// Triggers shutdown immediately and wakes all tasks awaiting [`ShutdownCoordinator::wait_triggered`].
    pub fn trigger(&self) {
        let Self { inner } = self;
        if !inner.shutting_down.swap(true, Ordering::AcqRel) {
            inner.shutdown_notify.notify_waiters();
            if inner.in_flight.load(Ordering::Acquire) == 0 {
                inner.drained_notify.notify_waiters();
            }
        }
    }

    /// Waits asynchronously until [`ShutdownCoordinator::trigger`] is invoked.
    pub async fn wait_triggered(&self) {
        let Self { inner } = self;
        if inner.shutting_down.load(Ordering::Acquire) {
            return;
        }
        let notified: Notified<'_> = inner.shutdown_notify.notified();
        if inner.shutting_down.load(Ordering::Acquire) {
            return;
        }
        notified.await;
    }

    /// Executes full graceful drain, runs registered callbacks in LIFO order, and optionally stops `control`.
    ///
    /// # Arguments
    ///
    /// - `Option<ServerControlHook>` - Optional Hyperlane server control hook to shut down cleanly.
    ///
    /// # Returns
    ///
    /// - `ShutdownReport` - Detailed drain and callback outcome summary.
    pub async fn execute_shutdown(&self, control: Option<ServerControlHook>) -> ShutdownReport {
        let Self { inner } = self;
        let started: Instant = Instant::now();
        self.trigger();
        let drain_budget: Duration = Duration::from_millis(inner.config.get_drain_timeout_ms());
        let drained_cleanly: bool = if inner.in_flight.load(Ordering::Acquire) == 0 {
            true
        } else {
            timeout(drain_budget, async {
                loop {
                    let notified: Notified<'_> = inner.drained_notify.notified();
                    if inner.in_flight.load(Ordering::Acquire) == 0 {
                        break;
                    }
                    notified.await;
                }
            })
            .await
            .is_ok()
        };
        let remaining_in_flight: usize = inner.in_flight.load(Ordering::Acquire);
        let cb_budget: Duration =
            Duration::from_millis(inner.config.get_callback_timeout_ms().max(1));
        let mut completed_callbacks: Vec<String> = Vec::with_capacity(inner.callbacks.len());
        let mut timed_out_callbacks: Vec<String> = Vec::new();
        for callback in inner.callbacks.iter().rev() {
            let name: String = callback.name().to_owned();
            if timeout(cb_budget, callback.on_shutdown()).await.is_ok() {
                completed_callbacks.push(name);
            } else {
                timed_out_callbacks.push(name);
            }
        }
        if let Some(hook) = control {
            hook.shutdown().await;
        }
        ShutdownReport {
            drained_cleanly,
            remaining_in_flight,
            completed_callbacks,
            timed_out_callbacks,
            elapsed_ms: started.elapsed().as_millis() as u64,
        }
    }

    /// Arms a background Tokio task waiting for OS signals or manual [`ShutdownCoordinator::trigger`] to stop `control`.
    ///
    /// # Arguments
    ///
    /// - `ServerControlHook` - Hyperlane server control hook returned by `Server::run`.
    ///
    /// # Returns
    ///
    /// - `JoinHandle<ShutdownReport>` - Join handle resolving to the [`ShutdownReport`] once shutdown completes.
    pub fn arm_signals(&self, control: ServerControlHook) -> JoinHandle<ShutdownReport> {
        let coordinator: Self = self.clone();
        spawn(async move {
            let handle_sigint: bool = coordinator.get_config().get_handle_sigint();
            let handle_sigterm: bool = coordinator.get_config().get_handle_sigterm();
            select! {
                _ = coordinator.wait_triggered() => {}
                _ = async {
                    if handle_sigint {
                        let _: io::Result<()> = ctrl_c().await;
                    } else {
                        pending::<()>().await;
                    }
                } => {}
                _ = wait_sigterm(handle_sigterm) => {}
            }
            coordinator.execute_shutdown(Some(control)).await
        })
    }
}

/// Inherent implementation of [`ShutdownPlugin`].
impl ShutdownPlugin {
    /// Initializes the global [`ShutdownPlugin`] once with `coordinator`.
    ///
    /// # Arguments
    ///
    /// - `ShutdownCoordinator` - Configured graceful shutdown coordinator.
    ///
    /// # Returns
    ///
    /// - `&'static Self` - Reference to the initialized global plugin.
    pub fn init(coordinator: ShutdownCoordinator) -> &'static Self {
        SHUTDOWN_PLUGIN.get_or_init(|| Self { coordinator })
    }

    /// Returns the initialized global [`ShutdownPlugin`], if initialized.
    ///
    /// # Returns
    ///
    /// - `Option<&'static Self>` - Reference to the global plugin, or `None`.
    pub fn get() -> Option<&'static Self> {
        SHUTDOWN_PLUGIN.get()
    }

    /// Returns the shared [`ShutdownCoordinator`].
    ///
    /// # Returns
    ///
    /// - `&ShutdownCoordinator` - Active shutdown coordinator.
    pub fn get_coordinator(&self) -> &ShutdownCoordinator {
        &self.coordinator
    }
}
