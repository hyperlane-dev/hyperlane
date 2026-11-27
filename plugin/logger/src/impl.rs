use super::*;

/// Formatting / debug-printing for [`LoggerConfig`].
impl Debug for LoggerConfig {
    /// Formats the [`LoggerConfig`] without requiring [`LogSink`] to implement [`Debug`].
    ///
    /// # Arguments
    ///
    /// - `&mut Formatter<'_>` - The formatter receiving the formatted output.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - Formatting status.
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct(DEBUG_LOGGER_CONFIG)
            .field(DEBUG_FIELD_LEVEL, &self.level)
            .field(DEBUG_FIELD_FILE, &self.file)
            .field(DEBUG_FIELD_CONSOLE, &self.console)
            .field(DEBUG_FIELD_FORMAT, &self.format)
            .field(DEBUG_FIELD_CHANNEL_CAPACITY, &self.channel_capacity)
            .field(DEBUG_FIELD_HAS_SINK, &self.sink.is_some())
            .finish()
    }
}

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
            console: true,
            format: LogFormat::Plain,
            channel_capacity: DEFAULT_LOGGER_CHANNEL_CAPACITY,
            sink: None,
        }
    }
}

/// Inherent implementation of [`LoggerConfig`].
impl LoggerConfig {
    /// Creates a new [`LoggerConfig`] with the specified maximum log level.
    ///
    /// # Arguments
    ///
    /// - `LevelFilter` - Maximum severity level written by the logger.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed configuration.
    pub fn new(level: LevelFilter) -> Self {
        Self {
            level,
            ..Self::default()
        }
    }

    /// Returns the configured maximum [`LevelFilter`].
    ///
    /// # Returns
    ///
    /// - `LevelFilter` - Active maximum log level.
    pub fn get_level(&self) -> LevelFilter {
        self.level
    }

    /// Returns the optional log file path.
    ///
    /// # Returns
    ///
    /// - `Option<&str>` - Configured log file path, if any.
    pub fn get_file(&self) -> Option<&str> {
        self.file.as_deref()
    }

    /// Returns whether standard output console logging is enabled.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when console logging is enabled.
    pub fn get_console(&self) -> bool {
        self.console
    }

    /// Returns the active [`LogFormat`].
    ///
    /// # Returns
    ///
    /// - `LogFormat` - Plain or JSON output format.
    pub fn get_format(&self) -> LogFormat {
        self.format
    }

    /// Returns the bounded non-blocking background writer channel capacity.
    ///
    /// # Returns
    ///
    /// - `usize` - Maximum queued log records before dropping.
    pub fn get_channel_capacity(&self) -> usize {
        self.channel_capacity
    }

    /// Sets the maximum [`LevelFilter`] in place.
    ///
    /// # Arguments
    ///
    /// - `LevelFilter` - Desired maximum severity level.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Mutable reference for fluent chaining.
    pub fn set_level(&mut self, level: LevelFilter) -> &mut Self {
        self.level = level;
        self
    }

    /// Sets the optional log file path in place.
    ///
    /// # Arguments
    ///
    /// - `Option<String>` - Optional destination file path.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Mutable reference for fluent chaining.
    pub fn set_file(&mut self, file: Option<String>) -> &mut Self {
        self.file = file;
        self
    }

    /// Enables or disables standard output console printing in place.
    ///
    /// # Arguments
    ///
    /// - `bool` - `true` to write to stdout, `false` to skip console output.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Mutable reference for fluent chaining.
    pub fn set_console(&mut self, enabled: bool) -> &mut Self {
        self.console = enabled;
        self
    }

    /// Selects the [`LogFormat`] in place.
    ///
    /// # Arguments
    ///
    /// - `LogFormat` - Desired record wire format.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Mutable reference for fluent chaining.
    pub fn set_format(&mut self, format: LogFormat) -> &mut Self {
        self.format = format;
        self
    }

    /// Sets the bounded background channel capacity in place.
    ///
    /// # Arguments
    ///
    /// - `usize` - Channel capacity (clamped to `>= 1`).
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Mutable reference for fluent chaining.
    pub fn set_channel_capacity(&mut self, capacity: usize) -> &mut Self {
        self.channel_capacity = capacity.max(1);
        self
    }

    /// Sets the custom extensible [`LogSink`] in place.
    ///
    /// # Arguments
    ///
    /// - `Option<SharedLogSink>` - Optional custom sink implementation.
    ///
    /// # Returns
    ///
    /// - `&mut Self` - Mutable reference for fluent chaining.
    pub fn set_sink(&mut self, sink: Option<SharedLogSink>) -> &mut Self {
        self.sink = sink;
        self
    }

    /// Updates the maximum [`LevelFilter`].
    ///
    /// # Arguments
    ///
    /// - `LevelFilter` - Desired maximum severity level.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_level(mut self, level: LevelFilter) -> Self {
        self.set_level(level);
        self
    }

    /// Configures a file path for non-blocking buffered append logging.
    ///
    /// # Arguments
    ///
    /// - `P` - Destination file path convertible into `String`.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_file<P>(mut self, path: P) -> Self
    where
        P: Into<String>,
    {
        self.set_file(Some(path.into()));
        self
    }

    /// Enables or disables standard output console printing.
    ///
    /// # Arguments
    ///
    /// - `bool` - `true` to write to stdout, `false` to skip console output.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_console(mut self, enabled: bool) -> Self {
        self.set_console(enabled);
        self
    }

    /// Selects the [`LogFormat`] emitted by the logger.
    ///
    /// # Arguments
    ///
    /// - `LogFormat` - Desired record wire format.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_format(mut self, format: LogFormat) -> Self {
        self.set_format(format);
        self
    }

    /// Sets the bounded background channel capacity for non-blocking file I/O.
    ///
    /// # Arguments
    ///
    /// - `usize` - Channel capacity (clamped to `>= 1`).
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_channel_capacity(mut self, capacity: usize) -> Self {
        self.set_channel_capacity(capacity);
        self
    }

    /// Attaches a custom extensible [`LogSink`] to receive every formatted log record.
    ///
    /// # Arguments
    ///
    /// - `SharedLogSink` - Custom sink implementation.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_sink(mut self, sink: SharedLogSink) -> Self {
        self.set_sink(Some(sink));
        self
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
            .is_none_or(|config: &LoggerConfig| metadata.level() <= config.get_level())
    }

    /// Writes one sanitized record without blocking request threads on file I/O.
    ///
    /// # Arguments
    ///
    /// - `&Record<'_>` - The record to write.
    fn log(&self, record: &Record<'_>) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let config_opt: Option<&LoggerConfig> = CONFIG.get();
        let format: LogFormat = config_opt.map_or(LogFormat::Plain, LoggerConfig::get_format);
        let console: bool = config_opt.is_none_or(LoggerConfig::get_console);
        let line: String = format_log_record(record, format);
        if let Some(config) = config_opt
            && let Some(sink) = config.sink.as_ref()
        {
            sink.write_record(record.level().to_level_filter(), &line);
        }
        if let Some(Some(tx)) = WRITER_TX.get()
            && tx.try_send(WriterCommand::Line(line.clone())).is_err()
        {
            DROPPED_RECORDS.fetch_add(1, Ordering::Relaxed);
        }
        if console {
            let mut out: io::StdoutLock<'static> = stdout().lock();
            let _: io::Result<()> = out.write_all(line.as_bytes());
            let _: io::Result<()> = out.write_all(b"\n");
        }
    }

    /// Flushes the background file writer and any attached custom sink.
    fn flush(&self) {
        if let Some(Some(tx)) = WRITER_TX.get() {
            let (ack_tx, ack_rx): (SyncSender<()>, Receiver<()>) = sync_channel(1);
            if tx.send(WriterCommand::Flush(ack_tx)).is_ok() {
                let _: Result<(), RecvError> = ack_rx.recv();
            }
        }
        if let Some(config) = CONFIG.get()
            && let Some(sink) = config.sink.as_ref()
        {
            sink.flush();
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
    pub fn sanitize_log_text(input: &str) -> String {
        let mut out: String = String::with_capacity(input.len());
        push_sanitized_plain(input, &mut out);
        out
    }

    /// Installs the logger with non-blocking buffered file output when configured.
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
        if WRITER_TX.get().is_some() {
            return Err(ERROR_FILE_ALREADY_SET.to_owned());
        }
        let writer_tx: Option<SyncSender<WriterCommand>> = match config.get_file() {
            Some(path) => {
                let file: File = OpenOptions::new()
                    .create(true)
                    .append(true)
                    .open(path)
                    .map_err(|error: io::Error| {
                        format!("the log file could not be opened: {error}")
                    })?;
                Some(spawn_file_writer(file, config.get_channel_capacity())?)
            }
            None => None,
        };
        let level: LevelFilter = config.get_level();
        set_logger(&LoggerPlugin)
            .map_err(|_: SetLoggerError| ERROR_LOGGER_ALREADY_INSTALLED.to_owned())?;
        WRITER_TX
            .set(writer_tx)
            .map_err(|_: Option<SyncSender<WriterCommand>>| ERROR_FILE_ALREADY_SET.to_owned())?;
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

    /// Returns the total number of log records dropped due to background writer backpressure.
    ///
    /// # Returns
    ///
    /// - `u64` - Cumulative dropped record count.
    pub fn dropped_count() -> u64 {
        DROPPED_RECORDS.load(Ordering::Relaxed)
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
        if CONFIG.get().is_none() || !log_enabled!(Level::Info) {
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
        if CONFIG.get().is_none() || !log_enabled!(Level::Info) {
            return Status::Continue;
        }
        let started_at: Option<Instant> = ctx.try_get_attribute(ATTRIBUTE_STARTED_AT);
        let elapsed: u128 = match started_at {
            Some(started_at) => started_at.elapsed().as_millis(),
            None => 0,
        };
        let method: &RequestMethod = ctx.get_request().get_method();
        let path: &str = ctx.get_request().get_path().as_str();
        let status: usize = ctx.get_response().get_status_code();
        let method_text: String = method.to_string();
        log::info!(
            "{} {} {} {}ms",
            if method_text.is_empty() {
                METHOD_UNKNOWN
            } else {
                method_text.as_str()
            },
            if path.is_empty() { PATH_UNKNOWN } else { path },
            status,
            elapsed
        );
        Status::Continue
    }
}
