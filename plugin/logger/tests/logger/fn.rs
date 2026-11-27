use super::*;

#[test]
fn installs_the_configuration_it_was_given() {
    let before: Option<&'static LoggerConfig> = LoggerPlugin::get();
    assert!(before.is_none());
    let config: LoggerConfig = LoggerConfig::new(LevelFilter::Debug).with_console(false);
    assert!(LoggerPlugin::init(config).is_ok());
    let stored: Option<&'static LoggerConfig> = LoggerPlugin::get();
    let level: LevelFilter = match stored {
        Some(config) => config.get_level(),
        None => LevelFilter::Off,
    };
    assert_eq!(level, LevelFilter::Debug);
    assert_eq!(LoggerPlugin::dropped_count(), 0u64);
}

#[test]
fn writes_the_info_level_by_default() {
    let config: LoggerConfig = LoggerConfig::default();
    assert_eq!(config.get_level(), LevelFilter::Info);
    assert!(config.get_file().is_none());
    assert!(config.get_console());
    assert_eq!(config.get_format(), LogFormat::Plain);
    assert_eq!(
        config.get_channel_capacity(),
        DEFAULT_LOGGER_CHANNEL_CAPACITY
    );
}

#[test]
fn is_a_unit_type() {
    let plugin: LoggerPlugin = LoggerPlugin;
    let copied: LoggerPlugin = plugin;
    assert_eq!(format!("{copied:?}"), format!("{plugin:?}"));
}

#[test]
fn test_security_log_sanitization_strips_crlf_and_ansi_escapes() {
    let forged: &str = "user=alice\r\nERROR injected\x1b[31mred\x00end";
    let safe: String = LoggerPlugin::sanitize_log_text(forged);
    assert!(!safe.contains('\r'));
    assert!(!safe.contains('\n'));
    assert!(!safe.contains('\x1b'));
    assert!(!safe.contains('\0'));
    assert!(safe.contains("\\r\\n"));
    assert!(safe.contains("\\x1b"));
}

#[test]
fn test_json_format_escapes_quotes_and_control_bytes() {
    let mut escaped: String = String::new();
    push_sanitized_json("say \"hi\"\\\r\n\t\x1b", &mut escaped);
    assert_eq!(escaped, "say \\\"hi\\\"\\\\\\r\\n\\t\\x1b");
}
