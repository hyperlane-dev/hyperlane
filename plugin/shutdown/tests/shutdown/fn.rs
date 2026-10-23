use super::*;

#[test]
fn stores_the_configuration_it_was_given() {
    let before: Option<&'static ShutdownConfig> = ShutdownPlugin::get();
    assert!(before.is_none());
    let config: ShutdownConfig = ShutdownConfig {
        watch_terminate: false,
        message: String::from("stopping"),
    };
    assert!(ShutdownPlugin::init(config).is_ok());
    let stored: Option<&'static ShutdownConfig> = ShutdownPlugin::get();
    let message: &str = match stored {
        Some(config) => config.message.as_str(),
        None => "",
    };
    assert_eq!(message, "stopping");
}

#[test]
fn watches_for_the_terminate_signal_by_default() {
    let config: ShutdownConfig = ShutdownConfig::default();
    assert!(config.watch_terminate);
}

#[test]
fn is_a_unit_type() {
    let plugin: ShutdownPlugin = ShutdownPlugin;
    let copied: ShutdownPlugin = plugin;
    assert_eq!(format!("{copied:?}"), format!("{plugin:?}"));
}
