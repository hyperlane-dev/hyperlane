use super::*;

impl Default for ShutdownConfig {
    /// Builds the configuration the shutdown plugin uses when none is given.
    ///
    /// # Returns
    ///
    /// - `Self` - The default configuration.
    fn default() -> Self {
        Self {
            watch_terminate: true,
            message: MESSAGE_DEFAULT.to_owned(),
        }
    }
}

impl ShutdownPlugin {
    /// Stores the configuration the plugin works from.
    ///
    /// # Arguments
    ///
    /// - `ShutdownConfig` - The configuration to store.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>` - Nothing, or the reason the configuration was
    ///   refused.
    pub fn init(config: ShutdownConfig) -> Result<(), String> {
        CONFIG
            .set(config)
            .map_err(|_: ShutdownConfig| "the shutdown configuration was already set".to_owned())
    }

    /// The configuration the plugin works from.
    ///
    /// # Returns
    ///
    /// - `Option<&'static ShutdownConfig>` - The configuration, when it was
    ///   set.
    pub fn get() -> Option<&'static ShutdownConfig> {
        CONFIG.get()
    }

    /// Sets the handler that runs when the server stops.
    ///
    /// # Arguments
    ///
    /// - `&ServerControlHookHandler<()>` - The handler to run.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>` - Nothing, or the reason the handler was refused.
    pub fn on_shutdown(handler: &ServerControlHookHandler<()>) -> Result<(), String> {
        HANDLER
            .set(handler.clone())
            .map_err(|_: ServerControlHookHandler<()>| {
                "the shutdown handler was already set".to_owned()
            })
    }

    /// Stops the server when the process is asked to.
    ///
    /// The watcher runs in the background, so the call returns as soon as the
    /// task is started.
    ///
    /// # Arguments
    ///
    /// - `&ServerControlHook` - The hook that owns the server.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>` - Nothing, or the reason the watcher could not
    ///   be started.
    pub fn watch(hook: &ServerControlHook) -> Result<(), String> {
        let config: ShutdownConfig = match CONFIG.get() {
            Some(config) => config.clone(),
            None => ShutdownConfig::default(),
        };
        let hook: ServerControlHook = hook.clone();
        tokio::spawn(async move {
            let _: Option<String> = Self::wait(&config, &hook).await;
        });
        Ok(())
    }

    /// Waits for a termination signal and stops the server.
    ///
    /// # Arguments
    ///
    /// - `&ShutdownConfig` - The configuration to work from.
    /// - `&ServerControlHook` - The hook that owns the server.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - The signal that stopped the server, when one did.
    async fn wait(config: &ShutdownConfig, hook: &ServerControlHook) -> Option<String> {
        let signal: String = Self::wait_for_signal(config.watch_terminate).await?;
        Self::stop(&signal, config, hook).await;
        Some(signal)
    }

    /// Waits for the signal that asks the process to stop.
    ///
    /// # Arguments
    ///
    /// - `bool` - Whether the terminate signal also stops the server.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - The signal that arrived, when one did.
    async fn wait_for_signal(watch_terminate: bool) -> Option<String> {
        #[cfg(unix)]
        {
            if watch_terminate {
                return Self::wait_for_terminate().await;
            }
        }
        let _: bool = watch_terminate;
        Self::wait_for_interrupt().await
    }

    /// Waits for the interrupt signal.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - The signal that arrived, when one did.
    async fn wait_for_interrupt() -> Option<String> {
        let interrupt: Result<(), Error> = tokio::signal::ctrl_c().await;
        if interrupt.is_err() {
            return None;
        }
        Some(MESSAGE_INTERRUPTED.to_owned())
    }

    /// Waits for the terminate signal.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - The signal that arrived, when one did.
    #[cfg(unix)]
    async fn wait_for_terminate() -> Option<String> {
        match tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()) {
            Ok(mut signal) => {
                signal.recv().await;
                Some(MESSAGE_TERMINATED.to_owned())
            }
            Err(_) => None,
        }
    }

    /// Runs the handler and stops the server.
    ///
    /// # Arguments
    ///
    /// - `&str` - The signal that asked the process to stop.
    /// - `&ShutdownConfig` - The configuration to work from.
    /// - `&ServerControlHook` - The hook that owns the server.
    async fn stop(signal: &str, config: &ShutdownConfig, hook: &ServerControlHook) {
        match HANDLER.get() {
            Some(handler) => handler().await,
            None => eprintln!("{}: {signal}", config.message),
        }
        hook.shutdown().await;
    }
}
