use super::*;

/// Appends sanitized plain-text characters from `input` into `out` without extra allocations.
///
/// # Arguments
///
/// - `&str` - Raw input text.
/// - `&mut String` - Destination buffer receiving sanitized text.
pub(crate) fn push_sanitized_plain(input: &str, out: &mut String) {
    let needs_escape: bool = input
        .bytes()
        .any(|b: u8| b < 0x20 && b != b'\t' || b == 0x7f);
    if !needs_escape {
        out.push_str(input);
        return;
    }
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
}

/// Appends JSON-escaped and control-sanitized text from `input` into `out`.
///
/// # Arguments
///
/// - `&str` - Raw text to embed inside a JSON string literal.
/// - `&mut String` - Destination buffer receiving escaped characters.
pub(crate) fn push_sanitized_json(input: &str, out: &mut String) {
    for ch in input.chars() {
        match ch {
            '"' => out.push_str(ESCAPED_QUOTE),
            '\\' => out.push_str(ESCAPED_BACKSLASH),
            '\r' => out.push_str(ESCAPED_CR),
            '\n' => out.push_str(ESCAPED_LF),
            '\t' => out.push_str(ESCAPED_TAB),
            '\x1b' => out.push_str(ESCAPED_ESC),
            c if c.is_control() => out.push(' '),
            c => out.push(c),
        }
    }
}

/// Formats one [`Record`] according to `format` using a single output allocation.
///
/// # Arguments
///
/// - `&Record<'_>` - Incoming log record.
/// - `LogFormat` - Output wire format.
///
/// # Returns
///
/// - `String` - Formatted single-line record ready for console, file, or sink output.
pub(crate) fn format_log_record(record: &Record<'_>, format: LogFormat) -> String {
    let raw_message: String = match record.args().as_str() {
        Some(literal) => literal.to_owned(),
        None => {
            let mut buf: String = String::with_capacity(64);
            let _: fmt::Result = fmt::write(&mut buf, *record.args());
            buf
        }
    };
    match format {
        LogFormat::Plain => {
            let level_str: &str = record.level().as_str();
            let mut line: String = String::with_capacity(level_str.len() + 1 + raw_message.len());
            line.push_str(level_str);
            line.push(' ');
            push_sanitized_plain(&raw_message, &mut line);
            line
        }
        LogFormat::Json => {
            let level_str: &str = record.level().as_str();
            let target_str: &str = record.target();
            let mut line: String =
                String::with_capacity(64 + level_str.len() + target_str.len() + raw_message.len());
            line.push_str(JSON_PREFIX_LEVEL);
            line.push_str(level_str);
            line.push_str(JSON_SEP_TARGET);
            push_sanitized_json(target_str, &mut line);
            line.push_str(JSON_SEP_MESSAGE);
            push_sanitized_json(&raw_message, &mut line);
            line.push_str(JSON_SUFFIX);
            line
        }
    }
}

/// Spawns the dedicated background buffered file writer thread.
///
/// # Arguments
///
/// - `File` - Opened append-mode log file handle.
/// - `usize` - Bounded channel capacity before backpressure drops records.
///
/// # Returns
///
/// - `Result<SyncSender<WriterCommand>, String>` - Non-blocking sender handle or spawn error.
pub(crate) fn spawn_file_writer(
    file: File,
    capacity: usize,
) -> Result<SyncSender<WriterCommand>, String> {
    let (tx, rx): (SyncSender<WriterCommand>, Receiver<WriterCommand>) =
        sync_channel(capacity.max(1));
    let _: JoinHandle<()> = Builder::new()
        .name(LOGGER_WRITER_THREAD_NAME.to_owned())
        .spawn(move || {
            let mut writer: BufWriter<File> =
                BufWriter::with_capacity(LOGGER_FILE_BUFFER_BYTES, file);
            while let Ok(cmd) = rx.recv() {
                match cmd {
                    WriterCommand::Line(line) => {
                        let _: io::Result<()> = writer.write_all(line.as_bytes());
                        let _: io::Result<()> = writer.write_all(b"\n");
                        while let Ok(next_cmd) = rx.try_recv() {
                            match next_cmd {
                                WriterCommand::Line(next_line) => {
                                    let _: io::Result<()> = writer.write_all(next_line.as_bytes());
                                    let _: io::Result<()> = writer.write_all(b"\n");
                                }
                                WriterCommand::Flush(ack) => {
                                    let _: io::Result<()> = writer.flush();
                                    let _: Result<(), TrySendError<()>> = ack.try_send(());
                                }
                            }
                        }
                        let _: io::Result<()> = writer.flush();
                    }
                    WriterCommand::Flush(ack) => {
                        let _: io::Result<()> = writer.flush();
                        let _: Result<(), TrySendError<()>> = ack.try_send(());
                    }
                }
            }
            let _: io::Result<()> = writer.flush();
        })
        .map_err(|err: io::Error| format!("failed to spawn logger writer thread: {err}"))?;
    Ok(tx)
}
