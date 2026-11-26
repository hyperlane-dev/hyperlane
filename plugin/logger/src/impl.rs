use super::*;

/// Default construction for [`LoggerConfig`].
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

/// Implements `impl log::Log for LoggerPlugin`.
impl log::Log for LoggerPlugin {
    /// Reports whether the level is written.
    ///
    /// # Arguments
    ///
    /// - `&Metadata<'_>` - The metadata of the record.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the record is written.
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        CONFIG
            .get()
            .is_none_or(|config: &LoggerConfig| metadata.level() <= config.level)
    }

    /// Writes one sanitized record without allowing CRLF or terminal control injection.
    ///
    /// # Arguments
    ///
    /// - `&Record<'_>` - The record to write.
    fn log(&self, record: &Record<'_>) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let raw_message: String = format!("{}", record.args());
        let safe_message: String = Self::sanitize_log_text(&raw_message);
        let line: String = format!("{} {}", record.level(), safe_message);
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

/// Inherent implementation of [`LoggerPlugin`].
impl LoggerPlugin {
    /// Escapes CR, LF, ESC, and control characters to prevent log forging and terminal injection.
    ///
    /// # Arguments
    ///
    /// - `&str` - Raw log text.
    ///
    /// # Returns
    ///
    /// - `String` - Single-line sanitized log text.
    pub(crate) fn sanitize_log_text(input: &str) -> String {
        let mut out: String = String::with_capacity(input.len());
        for ch in input.chars() {
            match ch {
                '\r' => out.push_str(ESCAPED_CR),
                '\n' => out.push_str(ESCAPED_LF),
                '\x1b' => out.push_str(ESCAPED_ESC),
                '\t' => out.push('\t'),
                c if c.is_control() => out.push(' '),
                c => out.push(c),
            }
        }
        out
    }

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
        if CONFIG.get().is_some() {
            return Err(ERROR_CONFIG_ALREADY_SET.to_owned());
        }
        if FILE.get().is_some() {
            return Err(ERROR_FILE_ALREADY_SET.to_owned());
        }
        let file: Option<File> = match config.file.as_ref() {
            Some(path) => match OpenOptions::new().create(true).append(true).open(path) {
                Ok(file) => Some(file),
                Err(error) => return Err(format!("the log file could not be opened: {error}")),
            },
            None => None,
        };
        let level: LevelFilter = config.level;
        set_logger(&LoggerPlugin)
            .map_err(|_: SetLoggerError| ERROR_LOGGER_ALREADY_INSTALLED.to_owned())?;
        FILE.set(Mutex::new(file))
            .map_err(|_: Mutex<Option<File>>| ERROR_FILE_ALREADY_SET.to_owned())?;
        CONFIG
            .set(config)
            .map_err(|_: LoggerConfig| ERROR_CONFIG_ALREADY_SET.to_owned())?;
        set_max_level(level);
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

/// Implements [`ServerHook`] for [`RequestLogger`].
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

/// Implements [`ServerHook`] for [`ResponseLogger`].
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
