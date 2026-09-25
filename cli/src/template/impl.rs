use super::*;

impl FromStr for TemplateType {
    type Err = TemplateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "controller" => Ok(Self::Controller),
            "domain" => Ok(Self::Domain),
            "exception" => Ok(Self::Exception),
            "mapper" => Ok(Self::Mapper),
            "model" => Ok(Self::Model),
            "repository" => Ok(Self::Repository),
            "service" => Ok(Self::Service),
            "utils" => Ok(Self::Utils),
            "view" => Ok(Self::View),
            _ => Err(TemplateError::InvalidTemplateType(s.to_string())),
        }
    }
}

impl TemplateConfig {
    /// Create a new template configuration
    ///
    /// # Arguments
    ///
    /// - `TemplateType`: Type of template component
    /// - `String`: Name of the component
    /// - `Option<ModelSubType>`: Optional model subtype for model components
    ///
    /// # Returns
    ///
    /// - `Self`: Configuration instance
    pub fn new(
        template_type: TemplateType,
        component_name: String,
        model_sub_type: Option<ModelSubType>,
    ) -> Self {
        Self {
            template_type,
            component_name,
            model_sub_type,
            base_directory: "./application".to_string(),
        }
    }
}

impl FromStr for ModelSubType {
    type Err = TemplateError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "application" => Ok(Self::Application),
            "request" => Ok(Self::Request),
            "response" => Ok(Self::Response),
            _ => Err(TemplateError::InvalidModelSubType(s.to_string())),
        }
    }
}
