use super::*;

/// Implementation of the `log::Log` trait for colored console output.
impl log::Log for Logger {
    /// Determines whether a log record with the specified metadata would be logged.
    ///
    /// # Arguments
    ///
    /// - `&log::Metadata` - The metadata of the log record.
    ///
    /// # Returns
    ///
    /// - `bool` - Whether the record's level is within the maximum log level.
    fn enabled(&self, metadata: &log::Metadata) -> bool {
        metadata.level() <= log::max_level()
    }

    /// Logs a record with colored console output.
    ///
    /// # Arguments
    ///
    /// - `&log::Record` - The log record to output.
    fn log(&self, record: &log::Record) {
        if !self.enabled(record.metadata()) {
            return;
        }
        let now_time: String = color_output::time();
        let level: log::Level = record.level();
        let args: &std::fmt::Arguments<'_> = record.args();
        let file: Option<&str> = record.file();
        let module_path: Option<&str> = record.module_path();
        let target: &str = record.target();
        let line: u32 = record.line().unwrap_or_default();
        let location: &str = file.unwrap_or(module_path.unwrap_or(target));
        let time_text: String = format!("{LOG_SPACE}{now_time}{LOG_SPACE}");
        let level_text: String = format!("{LOG_SPACE}{level}{LOG_SPACE}");
        let args_text: String = format!("{args}{LOG_SPACE}");
        let location_text: String = format!("{LOG_SPACE}{location}{LOG_COLON}{line}{LOG_SPACE}");
        let color: ColorType = match record.level() {
            log::Level::Trace => ColorType::Use(Color::Magenta),
            log::Level::Debug => ColorType::Use(Color::Cyan),
            log::Level::Info => ColorType::Use(Color::Green),
            log::Level::Warn => ColorType::Use(Color::Yellow),
            log::Level::Error => ColorType::Use(Color::Red),
        };
        let mut time_output_builder: ColorOutputBuilder<'_> = ColorOutputBuilder::new();
        let mut level_output_builder: ColorOutputBuilder<'_> = ColorOutputBuilder::new();
        let mut location_output_builder: ColorOutputBuilder<'_> = ColorOutputBuilder::new();
        let mut args_output_builder: ColorOutputBuilder<'_> = ColorOutputBuilder::new();
        let time_output: ColorOutput<'_> = time_output_builder
            .text(&time_text)
            .bold(true)
            .color(ColorType::Use(Color::White))
            .bg_color(ColorType::Use(Color::Black))
            .build();
        let level_output: ColorOutput<'_> = level_output_builder
            .text(&level_text)
            .bold(true)
            .color(ColorType::Use(Color::White))
            .bg_color(color)
            .build();
        let location_output: ColorOutput<'_> = location_output_builder
            .text(&location_text)
            .bold(true)
            .color(color)
            .build();
        let args_output: ColorOutput<'_> = args_output_builder
            .text(&args_text)
            .bold(true)
            .color(color)
            .endl(true)
            .build();
        ColorOutputListBuilder::new()
            .add(time_output)
            .add(level_output)
            .add(location_output)
            .add(args_output)
            .run();
    }

    /// Flushes any buffered log output.
    fn flush(&self) {}
}

/// Implementation of associated functions for `Logger`.
impl Logger {
    /// Initializes the global logger with the specified maximum log level.
    ///
    /// # Arguments
    ///
    /// - `LevelFilter` - The maximum log level to enable.
    pub fn init(level_filter: log::LevelFilter) {
        let _: Result<(), SetLoggerError> = log::set_logger(&LOGGER);
        log::set_max_level(level_filter);
    }
}
