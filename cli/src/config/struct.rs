use super::*;

/// Parsed command line arguments
#[derive(Clone, Debug)]
pub struct Args {
    /// The command to execute
    pub command: CommandType,
    /// Check mode for fmt
    pub check: bool,
    /// Manifest path for fmt, bump and publish
    pub manifest_path: Option<String>,
    /// Bump type for bump command
    pub bump_type: Option<BumpVersionType>,
    /// Maximum retry attempts for publish command
    pub max_retries: u32,
    /// Project name for new command
    pub project_name: Option<String>,
    /// Template type for template command
    pub template_type: Option<TemplateType>,
    /// Model subtype for template command (only when template_type is Model)
    pub model_sub_type: Option<ModelSubType>,
    /// Component name for template command
    pub component_name: Option<String>,
}
