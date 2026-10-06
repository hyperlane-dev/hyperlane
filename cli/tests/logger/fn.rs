use super::*;

#[test]
fn test_logger_unit_value_is_constructible() {
    let logger: Logger = Logger;
    let other: Logger = Logger;
    log::Log::flush(&logger);
    log::Log::flush(&other);
}

#[test]
fn test_logger_flush_is_a_noop() {
    let logger: Logger = Logger;
    log::Log::flush(&logger);
}

#[test]
fn test_logger_enabled_follows_max_level() {
    log::set_max_level(log::LevelFilter::Info);
    let logger: Logger = Logger;
    let error_metadata: log::Metadata<'_> = log::Metadata::builder()
        .level(log::Level::Error)
        .target("hyperlane-cli")
        .build();
    let trace_metadata: log::Metadata<'_> = log::Metadata::builder()
        .level(log::Level::Trace)
        .target("hyperlane-cli")
        .build();
    assert!(log::Log::enabled(&logger, &error_metadata));
    assert!(!log::Log::enabled(&logger, &trace_metadata));
}

#[test]
fn test_logger_init_sets_max_level() {
    Logger::init(log::LevelFilter::Warn);
    assert_eq!(log::max_level(), log::LevelFilter::Warn);
    log::set_max_level(log::LevelFilter::Info);
}

#[test]
fn test_logger_new_constructs_instance() {
    let logger: Logger = Logger::new();
    log::Log::flush(&logger);
}
