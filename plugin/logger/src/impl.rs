use super::*;

impl Default for LoggerConfig {
    /// Builds the configuration the logger uses when none is given.
    ///
    /// # Returns
    ///
    /// - `Self` - The default configuration.
    fn default() -> Self {
        Self {
            level: LevelFilter::Info,
            file: None,
        }
    }
}

impl log::Log for LoggerPlugin {
    /// Reports whether the level is written.
    ///
    /// # Arguments
    ///
    /// - `&log::Metadata<'_>` - The metadata of the record.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the record is written.
    fn enabled(&self, _: &log::Metadata<'_>) -> bool {
        true
    }

    /// Writes one record.
    ///
    /// # Arguments
    ///
    /// - `&log::Record<'_>` - The record to write.
    fn log(&self, record: &log::Record<'_>) {
        let line: String = format!("{} {}", record.level(), record.args());
        if let Some(file) = FILE.get()
            && let Ok(mut file) = file.lock()
            && let Some(file) = file.as_mut()
        {
            let _: Result<(), Error> = writeln!(file, "{line}");
        }
        println!("{line}");
    }

    /// Flushes the file the log is appended to.
    fn flush(&self) {
        if let Some(file) = FILE.get()
            && let Ok(mut file) = file.lock()
            && let Some(file) = file.as_mut()
        {
            let _: Result<(), Error> = file.flush();
        }
    }
}

impl LoggerPlugin {
    /// Installs the logger.
    ///
    /// # Arguments
    ///
    /// - `LoggerConfig` - The configuration to work from.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>` - Nothing, or the reason the logger was refused.
    pub fn init(config: LoggerConfig) -> Result<(), String> {
        let file: Option<File> = match config.file.as_ref() {
            Some(path) => match OpenOptions::new().create(true).append(true).open(path) {
                Ok(file) => Some(file),
                Err(error) => return Err(format!("the log file could not be opened: {error}")),
            },
            None => None,
        };
        CONFIG
            .set(config)
            .map_err(|_: LoggerConfig| "the logger configuration was already set".to_owned())?;
        FILE.set(Mutex::new(file))
            .map_err(|_: Mutex<Option<File>>| "the log file was already set".to_owned())?;
        log::set_logger(&LoggerPlugin)
            .map_err(|_: log::SetLoggerError| "a logger was already installed".to_owned())?;
        log::set_max_level(
            CONFIG
                .get()
                .map(|config: &LoggerConfig| config.level)
                .unwrap_or(LevelFilter::Info),
        );
        Ok(())
    }

    /// The configuration the logger works from.
    ///
    /// # Returns
    ///
    /// - `Option<&'static LoggerConfig>` - The configuration, when it was set.
    pub fn get() -> Option<&'static LoggerConfig> {
        CONFIG.get()
    }

    /// Writes a line at the info level.
    ///
    /// # Arguments
    ///
    /// - `T` - The line to write.
    pub fn log_info<T>(data: T)
    where
        T: AsRef<str>,
    {
        log::info!("{}", data.as_ref());
    }

    /// Writes a line at the error level.
    ///
    /// # Arguments
    ///
    /// - `T` - The line to write.
    pub fn log_error<T>(data: T)
    where
        T: AsRef<str>,
    {
        log::error!("{}", data.as_ref());
    }

    /// Writes a line at the warn level.
    ///
    /// # Arguments
    ///
    /// - `T` - The line to write.
    pub fn log_warn<T>(data: T)
    where
        T: AsRef<str>,
    {
        log::warn!("{}", data.as_ref());
    }

    /// Writes a line at the debug level.
    ///
    /// # Arguments
    ///
    /// - `T` - The line to write.
    pub fn log_debug<T>(data: T)
    where
        T: AsRef<str>,
    {
        log::debug!("{}", data.as_ref());
    }

    /// Writes a line at the trace level.
    ///
    /// # Arguments
    ///
    /// - `T` - The line to write.
    pub fn log_trace<T>(data: T)
    where
        T: AsRef<str>,
    {
        log::trace!("{}", data.as_ref());
    }
}

impl ServerHook for RequestLogger {
    /// Builds the middleware.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The stream the request arrived on.
    /// - `&mut Context` - The context of the request.
    ///
    /// # Returns
    ///
    /// - `Self` - The middleware.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    /// Notes when the request arrived.
    ///
    /// # Arguments
    ///
    /// - `Self` - The middleware.
    /// - `&mut Stream` - The stream the request arrived on.
    /// - `&mut Context` - The context of the request.
    ///
    /// # Returns
    ///
    /// - `Status` - Whether the pipeline continues.
    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        if CONFIG.get().is_none() {
            return Status::Continue;
        }
        ctx.set_attribute(ATTRIBUTE_STARTED_AT, Instant::now());
        Status::Continue
    }
}

impl ServerHook for ResponseLogger {
    /// Builds the middleware.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The stream the request arrived on.
    /// - `&mut Context` - The context of the request.
    ///
    /// # Returns
    ///
    /// - `Self` - The middleware.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    /// Writes the line of the answered request.
    ///
    /// # Arguments
    ///
    /// - `Self` - The middleware.
    /// - `&mut Stream` - The stream the request arrived on.
    /// - `&mut Context` - The context of the request.
    ///
    /// # Returns
    ///
    /// - `Status` - Whether the pipeline continues.
    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        if CONFIG.get().is_none() {
            return Status::Continue;
        }
        let started_at: Option<Instant> = ctx.try_get_attribute(ATTRIBUTE_STARTED_AT);
        let elapsed: u128 = match started_at {
            Some(started_at) => started_at.elapsed().as_millis(),
            None => 0,
        };
        let method: String = ctx.get_request().get_method().to_string();
        let path: String = ctx.get_request().get_path().to_owned();
        let status: usize = ctx.get_response().get_status_code();
        log::info!(
            "{} {} {} {}ms",
            if method.is_empty() {
                METHOD_UNKNOWN
            } else {
                method.as_str()
            },
            if path.is_empty() {
                PATH_UNKNOWN
            } else {
                path.as_str()
            },
            status,
            elapsed
        );
        Status::Continue
    }
}
