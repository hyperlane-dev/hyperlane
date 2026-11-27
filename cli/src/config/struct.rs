use super::*;

/// Parsed command line arguments
///
/// `hyperlane-cli` ships both a library and a `hyperlane` binary; the binary
/// is a separate crate that consumes this one, so the read path must stay
/// `pub` (§17.18 exempts a struct the package's own binary target reads).
#[derive(Clone, Data, Debug)]
pub struct Args {
    /// The command to execute
    #[get(type(copy))]
    pub command: CommandType,
    /// Project name for new command
    #[get(type(clone))]
    pub project_name: Option<String>,
    /// Template type for template command
    #[get(type(clone))]
    pub template_type: Option<TemplateType>,
    /// Model subtype for template command (only when template_type is Model)
    #[get(type(clone))]
    pub model_sub_type: Option<ModelSubType>,
    /// Component name for template command
    #[get(type(clone))]
    pub component_name: Option<String>,
    /// Verify formatting without writing files for the fmt command
    #[get(type(copy))]
    pub fmt_check: bool,
    /// Optional target directory or file path for the fmt command
    #[get(type(clone))]
    pub fmt_path: Option<PathBuf>,
}
