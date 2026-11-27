use super::*;

/// Global process-wide [`StaticFilesPlugin`] singleton initialized via [`StaticFilesPlugin::init`].
pub(crate) static STATIC_FILES_PLUGIN: OnceLock<StaticFilesPlugin> = OnceLock::new();
