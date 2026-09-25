use super::*;

/// Parsed command line arguments
#[derive(Clone, Debug)]
pub struct Args {
    /// The command to execute
    pub command: CommandType,
    /// Project name for new command
    pub project_name: Option<String>,
    /// Template type for template command
    pub template_type: Option<TemplateType>,
    /// Model subtype for template command (only when template_type is Model)
    pub model_sub_type: Option<ModelSubType>,
    /// Component name for template command
    pub component_name: Option<String>,
}
