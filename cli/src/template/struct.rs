use super::*;

/// Configuration for template generation
#[derive(Clone, Data, Debug)]
pub struct TemplateConfig {
    /// Type of template component to generate
    pub(crate) template_type: TemplateType,
    /// Name of the component
    pub(crate) component_name: String,
    /// Model subtype (only used when template_type is Model)
    pub(crate) model_sub_type: Option<ModelSubType>,
    /// Base directory for generation (default: ./application)
    pub(crate) base_directory: String,
}
