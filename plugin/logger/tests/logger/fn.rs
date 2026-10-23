use super::*;

#[test]
fn installs_the_configuration_it_was_given() {
    let before: Option<&'static LoggerConfig> = LoggerPlugin::get();
    assert!(before.is_none());
    let config: LoggerConfig = LoggerConfig {
        level: LevelFilter::Debug,
        file: None,
    };
    assert!(LoggerPlugin::init(config).is_ok());
    let stored: Option<&'static LoggerConfig> = LoggerPlugin::get();
    let level: LevelFilter = match stored {
        Some(config) => config.level,
        None => LevelFilter::Off,
    };
    assert_eq!(level, LevelFilter::Debug);
}

#[test]
fn writes_the_info_level_by_default() {
    let config: LoggerConfig = LoggerConfig::default();
    assert_eq!(config.level, LevelFilter::Info);
    assert!(config.file.is_none());
}

#[test]
fn is_a_unit_type() {
    let plugin: LoggerPlugin = LoggerPlugin;
    let copied: LoggerPlugin = plugin;
    assert_eq!(format!("{copied:?}"), format!("{plugin:?}"));
}
