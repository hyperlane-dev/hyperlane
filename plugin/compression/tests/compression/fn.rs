use super::*;

#[test]
fn picks_the_encoding_the_client_asked_for() {
    let config: CompressionConfig = CompressionConfig::default();
    let encoding: Option<Compress> = config.pick("gzip");
    assert_eq!(encoding, Some(Compress::Gzip));
}

#[test]
fn prefers_the_offered_order_over_the_order_of_the_client() {
    let config: CompressionConfig = CompressionConfig::default();
    let encoding: Option<Compress> = config.pick("br, gzip");
    assert_eq!(encoding, Some(Compress::Gzip));
}

#[test]
fn refuses_an_encoding_that_is_not_offered() {
    let config: CompressionConfig = CompressionConfig::default();
    let encoding: Option<Compress> = config.pick("deflate");
    assert!(encoding.is_none());
}

#[test]
fn refuses_an_encoding_the_client_refused() {
    let config: CompressionConfig = CompressionConfig::default();
    let encoding: Option<Compress> = config.pick("gzip;q=0");
    assert!(encoding.is_none());
}

#[test]
fn accepts_every_encoding_when_the_client_asked_for_every_one() {
    let config: CompressionConfig = CompressionConfig::default();
    let encoding: Option<Compress> = config.pick("*");
    assert_eq!(encoding, Some(Compress::Gzip));
}

#[test]
fn leaves_a_body_that_is_already_compressed_alone() {
    let config: CompressionConfig = CompressionConfig::default();
    assert!(!config.is_compressible("image/png"));
    assert!(!config.is_compressible("application/zip"));
    assert!(!config.is_compressible("text/event-stream"));
}

#[test]
fn compresses_a_body_that_is_worth_compressing() {
    let config: CompressionConfig = CompressionConfig::default();
    assert!(config.is_compressible("text/html"));
    assert!(config.is_compressible("application/json"));
    assert!(config.is_compressible(""));
}

#[test]
fn compresses_a_body_over_the_default_size() {
    let config: CompressionConfig = CompressionConfig::default();
    assert_eq!(config.min_size, 1024);
    assert_eq!(config.encodings.len(), 2);
}

#[test]
fn stores_the_configuration_it_was_given() {
    let before: Option<&'static CompressionConfig> = CompressionPlugin::get();
    assert!(before.is_none());
    let config: CompressionConfig = CompressionConfig {
        min_size: 64,
        encodings: vec![Compress::Br],
    };
    assert!(CompressionPlugin::init(config).is_ok());
    let stored: Option<&'static CompressionConfig> = CompressionPlugin::get();
    let min_size: usize = match stored {
        Some(config) => config.min_size,
        None => 0,
    };
    assert_eq!(min_size, 64);
}

#[test]
fn is_a_unit_type() {
    let plugin: CompressionPlugin = CompressionPlugin;
    let copied: CompressionPlugin = plugin;
    assert_eq!(format!("{copied:?}"), format!("{plugin:?}"));
}
