use super::*;

/// Configuration for template generation
#[derive(Clone, Debug)]
pub struct TemplateConfig {
    /// Type of template component to generate
    pub template_type: TemplateType,
    /// Name of the component
    pub component_name: String,
    /// Model subtype (only used when template_type is Model)
    pub model_sub_type: Option<ModelSubType>,
    /// Base directory for generation (default: ./application)
    pub base_directory: String,
}
