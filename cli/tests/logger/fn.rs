use super::*;

#[test]
fn test_logger_unit_value_is_constructible() {
    let logger: Logger = Logger;
    let other: Logger = Logger;
    Log::flush(&logger);
    Log::flush(&other);
}

#[test]
fn test_logger_flush_is_a_noop() {
    let logger: Logger = Logger;
    Log::flush(&logger);
}

#[test]
fn test_logger_enabled_follows_max_level() {
    set_max_level(LevelFilter::Info);
    let logger: Logger = Logger;
    let error_metadata: Metadata<'_> = Metadata::builder()
        .level(Level::Error)
        .target("hyperlane-cli")
        .build();
    let trace_metadata: Metadata<'_> = Metadata::builder()
        .level(Level::Trace)
        .target("hyperlane-cli")
        .build();
    assert!(Log::enabled(&logger, &error_metadata));
    assert!(!Log::enabled(&logger, &trace_metadata));
}

#[test]
fn test_logger_init_sets_max_level() {
    Logger::init(LevelFilter::Warn);
    assert_eq!(max_level(), LevelFilter::Warn);
    set_max_level(LevelFilter::Info);
}

#[test]
fn test_logger_new_constructs_instance() {
    let logger: Logger = Logger::new();
    Log::flush(&logger);
}
