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
    /// - `Result<(), String>` - Nothing, or the reason the watcher could not be
    ///   started.
    pub fn watch(hook: &ServerControlHook) -> Result<(), String> {
        let config: ShutdownConfig = match CONFIG.get() {
            Some(config) => config.clone(),
            None => ShutdownConfig::default(),
        };
        let hook: ServerControlHook = hook.clone();
        spawn_watcher(
            config.watch_terminate,
            move |signal: TerminationSignal| async move {
                ShutdownPlugin::stop(&signal, &config, &hook).await;
            },
        );
        Ok(())
    }

    /// The message that names the signal.
    ///
    /// # Arguments
    ///
    /// - `&TerminationSignal` - The signal that asked the process to stop.
    ///
    /// # Returns
    ///
    /// - `&'static str` - The message of the signal.
    fn message(signal: &TerminationSignal) -> &'static str {
        match signal {
            TerminationSignal::Interrupted => MESSAGE_INTERRUPTED,
            TerminationSignal::Terminated => MESSAGE_TERMINATED,
        }
    }

    /// Runs the handler and stops the server.
    ///
    /// # Arguments
    ///
    /// - `&TerminationSignal` - The signal that asked the process to stop.
    /// - `&ShutdownConfig` - The configuration to work from.
    /// - `&ServerControlHook` - The hook that owns the server.
    async fn stop(signal: &TerminationSignal, config: &ShutdownConfig, hook: &ServerControlHook) {
        match HANDLER.get() {
            Some(handler) => handler().await,
            None => eprintln!("{}: {}", config.message, Self::message(signal)),
        }
        hook.shutdown().await;
    }
}
