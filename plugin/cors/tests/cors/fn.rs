use super::*;

#[test]
fn allows_every_origin_by_default() {
    let config: CorsConfig = CorsConfig::default();
    let allowed: Option<String> = config.allowed_origin("https://example.com");
    assert_eq!(allowed, Some(String::from("*")));
}

#[test]
fn answers_with_the_origin_that_was_asked_for() {
    let config: CorsConfig = CorsConfig {
        allowed_origins: vec![String::from("https://example.com")],
        ..CorsConfig::default()
    };
    let allowed: Option<String> = config.allowed_origin("https://example.com");
    assert_eq!(allowed, Some(String::from("https://example.com")));
}

#[test]
fn refuses_an_origin_that_is_not_allowed() {
    let config: CorsConfig = CorsConfig {
        allowed_origins: vec![String::from("https://example.com")],
        ..CorsConfig::default()
    };
    let allowed: Option<String> = config.allowed_origin("https://other.example.com");
    assert!(allowed.is_none());
}

#[test]
fn writes_no_credentials_by_default() {
    let config: CorsConfig = CorsConfig::default();
    assert!(config.max_age.is_none());
    assert!(!config.allow_credentials);
}

#[test]
fn stores_the_configuration_it_was_given() {
    let before: Option<&'static CorsConfig> = CorsPlugin::get();
    assert!(before.is_none());
    let config: CorsConfig = CorsConfig {
        allowed_origins: vec![String::from("https://example.com")],
        allow_credentials: true,
        ..CorsConfig::default()
    };
    assert!(CorsPlugin::init(config).is_ok());
    let stored: Option<&'static CorsConfig> = CorsPlugin::get();
    let credentials: bool = match stored {
        Some(config) => config.allow_credentials,
        None => false,
    };
    assert!(credentials);
}

#[test]
fn is_a_unit_type() {
    let plugin: CorsPlugin = CorsPlugin;
    let copied: CorsPlugin = plugin;
    assert_eq!(format!("{copied:?}"), format!("{plugin:?}"));
}
