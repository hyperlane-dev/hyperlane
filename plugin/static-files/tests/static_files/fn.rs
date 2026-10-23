use super::*;

#[test]
fn resolves_a_path_under_the_root() {
    let config: StaticFilesConfig = StaticFilesConfig {
        root: PathBuf::from("/srv/public"),
        ..StaticFilesConfig::default()
    };
    let file: Option<PathBuf> = config.resolve("assets/app.js");
    assert_eq!(file, Some(PathBuf::from("/srv/public/assets/app.js")));
}

#[test]
fn refuses_a_path_that_climbs_out_of_the_root() {
    let config: StaticFilesConfig = StaticFilesConfig {
        root: PathBuf::from("/srv/public"),
        ..StaticFilesConfig::default()
    };
    let file: Option<PathBuf> = config.resolve("../../etc/passwd");
    assert!(file.is_none());
}

#[test]
fn answers_a_directory_with_the_index_file() {
    let config: StaticFilesConfig = StaticFilesConfig {
        root: PathBuf::from("/srv/public"),
        ..StaticFilesConfig::default()
    };
    let file: Option<PathBuf> = config.resolve("");
    assert_eq!(file, Some(PathBuf::from("/srv/public/index.html")));
}

#[test]
fn writes_the_content_type_of_the_extension() {
    let config: StaticFilesConfig = StaticFilesConfig::default();
    let content_type: String = config.content_type(Path::new("/srv/public/index.html"));
    assert_eq!(content_type, String::from("text/html"));
}

#[test]
fn writes_a_stream_of_bytes_for_an_unknown_extension() {
    let config: StaticFilesConfig = StaticFilesConfig::default();
    let content_type: String = config.content_type(Path::new("/srv/public/archive.unknown"));
    assert_eq!(content_type, String::from("application/octet-stream"));
}

#[test]
fn builds_the_pattern_of_a_route_that_serves_a_prefix() {
    let pattern: String = StaticFilesPlugin::pattern("/assets");
    assert_eq!(pattern, String::from("/assets/{path:.*}"));
}

#[test]
fn stores_the_configuration_it_was_given() {
    let before: Option<&'static StaticFilesConfig> = StaticFilesPlugin::get();
    assert!(before.is_none());
    let config: StaticFilesConfig = StaticFilesConfig {
        root: PathBuf::from("/srv/public"),
        index: String::from("home.html"),
    };
    assert!(StaticFilesPlugin::init(config).is_ok());
    let stored: Option<&'static StaticFilesConfig> = StaticFilesPlugin::get();
    let index: String = match stored {
        Some(config) => config.get_index().clone(),
        None => String::new(),
    };
    assert_eq!(index, String::from("home.html"));
}

#[test]
fn is_a_unit_type() {
    let plugin: StaticFilesPlugin = StaticFilesPlugin;
    let copied: StaticFilesPlugin = plugin;
    assert_eq!(format!("{copied:?}"), format!("{plugin:?}"));
}
