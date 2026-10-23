use super::*;

/// The configuration of the static file plugin.
#[derive(Clone, Debug, Getter)]
pub struct StaticFilesConfig {
    /// The directory the files are served from.
    pub root: PathBuf,
    /// The file that answers a request for a directory.
    pub index: String,
}

/// The plugin that serves the files of a directory.
#[derive(Clone, Copy, Debug, Default)]
pub struct StaticFilesPlugin;

/// The route that serves the files of the configured directory.
#[derive(Clone, Copy, Debug, Default)]
pub struct StaticFilesRoute;
