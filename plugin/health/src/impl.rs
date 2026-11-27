use super::*;

/// Inherent constructors for [`CheckOutcome`].
impl CheckOutcome {
    /// Constructs a healthy [`HealthStatus::Up`] outcome.
    ///
    /// # Returns
    ///
    /// - `Self` - Healthy outcome without extra detail.
    pub fn up() -> Self {
        Self {
            status: HealthStatus::Up,
            detail: None,
        }
    }

    /// Constructs a [`HealthStatus::Degraded`] outcome with diagnostic `detail`.
    ///
    /// # Arguments
    ///
    /// - `D` - Detail string convertible into `String`.
    ///
    /// # Returns
    ///
    /// - `Self` - Degraded outcome.
    pub fn degraded<D>(detail: D) -> Self
    where
        D: Into<String>,
    {
        Self {
            status: HealthStatus::Degraded,
            detail: Some(detail.into()),
        }
    }

    /// Constructs an unhealthy [`HealthStatus::Down`] outcome with diagnostic `detail`.
    ///
    /// # Arguments
    ///
    /// - `D` - Detail string convertible into `String`.
    ///
    /// # Returns
    ///
    /// - `Self` - Down outcome.
    pub fn down<D>(detail: D) -> Self
    where
        D: Into<String>,
    {
        Self {
            status: HealthStatus::Down,
            detail: Some(detail.into()),
        }
    }
}

/// Default construction for [`HealthConfig`].
impl Default for HealthConfig {
    /// Constructs a default [`HealthConfig`] with standard Kubernetes probe paths.
    fn default() -> Self {
        Self {
            health_path: DEFAULT_HEALTH_PATH.to_owned(),
            live_path: DEFAULT_LIVE_PATH.to_owned(),
            ready_path: DEFAULT_READY_PATH.to_owned(),
            check_timeout_ms: DEFAULT_HEALTH_CHECK_TIMEOUT_MS,
            cache_ttl_ms: DEFAULT_HEALTH_CACHE_TTL_MS,
        }
    }
}

/// Inherent fluent builders for [`HealthConfig`].
impl HealthConfig {
    /// Sets the combined health endpoint path.
    ///
    /// # Arguments
    ///
    /// - `P` - Path string convertible into `String`.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_health_path<P>(mut self, path: P) -> Self
    where
        P: Into<String>,
    {
        self.set_health_path(path.into());
        self
    }

    /// Sets the liveness endpoint path.
    ///
    /// # Arguments
    ///
    /// - `P` - Path string convertible into `String`.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_live_path<P>(mut self, path: P) -> Self
    where
        P: Into<String>,
    {
        self.set_live_path(path.into());
        self
    }

    /// Sets the readiness endpoint path.
    ///
    /// # Arguments
    ///
    /// - `P` - Path string convertible into `String`.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_ready_path<P>(mut self, path: P) -> Self
    where
        P: Into<String>,
    {
        self.set_ready_path(path.into());
        self
    }

    /// Sets the per-probe timeout in milliseconds.
    ///
    /// # Arguments
    ///
    /// - `u64` - Timeout in milliseconds.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_check_timeout_ms(mut self, timeout_ms: u64) -> Self {
        self.set_check_timeout_ms(timeout_ms);
        self
    }

    /// Sets the cached probe snapshot TTL in milliseconds (`0` disables caching).
    ///
    /// # Arguments
    ///
    /// - `u64` - Cache TTL in milliseconds.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_cache_ttl_ms(mut self, ttl_ms: u64) -> Self {
        self.set_cache_ttl_ms(ttl_ms);
        self
    }
}

/// Default construction for [`HealthRegistry`].
impl Default for HealthRegistry {
    /// Constructs an empty [`HealthRegistry`] in ready (non-draining) state.
    fn default() -> Self {
        Self::new()
    }
}

/// Inherent implementation of [`HealthRegistry`].
impl HealthRegistry {
    /// Creates an empty [`HealthRegistry`].
    ///
    /// # Returns
    ///
    /// - `Self` - Ready registry with no registered dependency checks.
    pub fn new() -> Self {
        Self {
            checks: Arc::new(Vec::new()),
            draining: Arc::new(AtomicBool::new(false)),
            cache: Arc::new(RwLock::new(None)),
        }
    }

    /// Appends a [`HealthCheck`] probe to this registry.
    ///
    /// # Arguments
    ///
    /// - `C` - Concrete [`HealthCheck`] implementation.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated registry containing the probe.
    pub fn with_check<C>(self, check: C) -> Self
    where
        C: HealthCheck,
    {
        let Self {
            checks,
            draining,
            cache,
        } = self;
        let mut next: Vec<SharedHealthCheck> = (*checks).clone();
        next.push(Arc::new(check));
        Self {
            checks: Arc::new(next),
            draining,
            cache,
        }
    }

    /// Marks the server as draining so readiness and health probes immediately report `Down` (`503`).
    pub fn mark_draining(&self) {
        let Self {
            draining, cache, ..
        } = self;
        draining.store(true, Ordering::Release);
        if let Ok(mut guard) = cache.write() {
            *guard = None;
        }
    }

    /// Clears the draining flag so readiness probes resume normal evaluation.
    pub fn clear_draining(&self) {
        let Self {
            draining, cache, ..
        } = self;
        draining.store(false, Ordering::Release);
        if let Ok(mut guard) = cache.write() {
            *guard = None;
        }
    }

    /// Reports whether the server is currently marked draining.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when draining.
    pub fn is_draining(&self) -> bool {
        let Self { draining, .. } = self;
        draining.load(Ordering::Acquire)
    }

    /// Evaluates liveness (fast lock-free check that never fails due to draining).
    ///
    /// # Returns
    ///
    /// - `HealthReport` - Liveness report with `HealthStatus::Up`.
    pub fn liveness_report(&self) -> HealthReport {
        HealthReport {
            status: HealthStatus::Up,
            draining: self.is_draining(),
            checks: Vec::new(),
        }
    }

    /// Evaluates all registered checks concurrently with `check_timeout_ms` and optional snapshot cache.
    ///
    /// # Arguments
    ///
    /// - `u64` - Per-probe timeout in milliseconds.
    /// - `u64` - Snapshot cache TTL in milliseconds (`0` disables caching).
    ///
    /// # Returns
    ///
    /// - `HealthReport` - Aggregate health report.
    pub async fn evaluate(&self, check_timeout_ms: u64, cache_ttl_ms: u64) -> HealthReport {
        let Self { checks, cache, .. } = self;
        if self.is_draining() {
            return HealthReport {
                status: HealthStatus::Down,
                draining: true,
                checks: Vec::new(),
            };
        }
        if cache_ttl_ms > 0
            && let Ok(guard) = cache.read()
            && let Some((captured_at, report)) = guard.as_ref()
            && captured_at.elapsed() <= Duration::from_millis(cache_ttl_ms)
        {
            return report.clone();
        }
        let timeout_dur: Duration = Duration::from_millis(check_timeout_ms.max(1));
        let mut handles: Vec<(String, JoinHandle<CheckReport>)> = Vec::with_capacity(checks.len());
        for check in checks.iter() {
            let probe: SharedHealthCheck = Arc::clone(check);
            let name: String = probe.name().to_owned();
            let task_name: String = name.clone();
            let handle: JoinHandle<CheckReport> = spawn(async move {
                let started: Instant = Instant::now();
                let outcome: CheckOutcome = match timeout(timeout_dur, probe.check()).await {
                    Ok(res) => res,
                    Err(_) => CheckOutcome::down(DETAIL_PROBE_TIMEOUT),
                };
                CheckReport {
                    name: task_name,
                    status: outcome.get_status(),
                    detail: outcome.get_detail(),
                    latency_ms: started.elapsed().as_millis() as u64,
                }
            });
            handles.push((name, handle));
        }
        let mut reports: Vec<CheckReport> = Vec::with_capacity(handles.len());
        let mut aggregate: HealthStatus = HealthStatus::Up;
        for (name, handle) in handles {
            let entry: CheckReport = match handle.await {
                Ok(report) => report,
                Err(_) => CheckReport {
                    name,
                    status: HealthStatus::Down,
                    detail: Some(DETAIL_PROBE_PANICKED.to_owned()),
                    latency_ms: 0,
                },
            };
            match entry.get_status() {
                HealthStatus::Down => aggregate = HealthStatus::Down,
                HealthStatus::Degraded => {
                    if aggregate == HealthStatus::Up {
                        aggregate = HealthStatus::Degraded;
                    }
                }
                HealthStatus::Up => {}
            }
            reports.push(entry);
        }
        let report: HealthReport = HealthReport {
            status: aggregate,
            draining: self.is_draining(),
            checks: reports,
        };
        if cache_ttl_ms > 0
            && let Ok(mut guard) = cache.write()
        {
            *guard = Some((Instant::now(), report.clone()));
        }
        report
    }
}

/// Inherent implementation of [`HealthPlugin`].
impl HealthPlugin {
    /// Initializes the global [`HealthPlugin`] once with `config` and `registry`.
    ///
    /// # Arguments
    ///
    /// - `HealthConfig` - Endpoint paths, timeouts, and caching settings.
    /// - `HealthRegistry` - Registered health probes and drain state.
    ///
    /// # Returns
    ///
    /// - `&'static Self` - Reference to the initialized global plugin.
    pub fn init(config: HealthConfig, registry: HealthRegistry) -> &'static Self {
        HEALTH_PLUGIN.get_or_init(|| Self { config, registry })
    }

    /// Returns the initialized global [`HealthPlugin`], if [`HealthPlugin::init`] was called.
    ///
    /// # Returns
    ///
    /// - `Option<&'static Self>` - Reference to the global plugin, or `None`.
    pub fn get() -> Option<&'static Self> {
        HEALTH_PLUGIN.get()
    }

    /// Returns the active [`HealthConfig`].
    ///
    /// # Returns
    ///
    /// - `&HealthConfig` - Probe configuration.
    pub fn get_config(&self) -> &HealthConfig {
        &self.config
    }

    /// Returns the active [`HealthRegistry`].
    ///
    /// # Returns
    ///
    /// - `&HealthRegistry` - Health check registry and drain controller.
    pub fn get_registry(&self) -> &HealthRegistry {
        &self.registry
    }
}

/// Implements [`ServerHook`] for [`HealthHook`].
impl ServerHook for HealthHook {
    /// Constructs a per-request [`HealthHook`].
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - Connection stream.
    /// - `&mut Context` - Request context.
    ///
    /// # Returns
    ///
    /// - `Self` - Hook instance.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    /// Intercepts `/healthz`, `/livez`, and `/readyz` probes and returns JSON status payloads.
    ///
    /// # Arguments
    ///
    /// - `Self` - Hook instance.
    /// - `&mut Stream` - Connection stream.
    /// - `&mut Context` - Request context.
    ///
    /// # Returns
    ///
    /// - `Status` - `Status::Reject` once a probe endpoint was served; `Status::Continue` otherwise.
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        let Some(plugin) = HealthPlugin::get() else {
            return Status::Continue;
        };
        let config: &HealthConfig = plugin.get_config();
        let path: &str = ctx.get_request().get_path();
        let is_live: bool = path == config.get_live_path();
        let is_ready_or_health: bool =
            path == config.get_ready_path() || path == config.get_health_path();
        if !is_live && !is_ready_or_health {
            return Status::Continue;
        }
        let report: HealthReport = if is_live {
            plugin.get_registry().liveness_report()
        } else {
            plugin
                .get_registry()
                .evaluate(config.get_check_timeout_ms(), config.get_cache_ttl_ms())
                .await
        };
        let status_code: usize = if report.get_status() == HealthStatus::Down {
            HttpStatus::ServiceUnavailable.code()
        } else {
            HttpStatus::Ok.code()
        };
        let body: Vec<u8> = if ctx.get_request().get_method().is_head() {
            Vec::new()
        } else {
            to_vec(&report).unwrap_or_default()
        };
        ctx.get_mut_response()
            .set_status_code(status_code)
            .set_header(CONTENT_TYPE, APPLICATION_JSON)
            .set_header(CACHE_CONTROL, CACHE_CONTROL_NO_STORE)
            .set_body(body);
        let wire: ResponseData = ctx.get_mut_response().build();
        let _: Result<(), ResponseError> = stream.try_send(&wire).await;
        Status::Reject
    }
}
