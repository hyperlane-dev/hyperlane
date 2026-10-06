use super::*;

impl FromStr for TemplateType {
    type Err = TemplateError;

    /// Parses a lowercase template type name.
    ///
    /// # Arguments
    ///
    /// - `&str` - The template type name to parse.
    ///
    /// # Returns
    ///
    /// - `Result<Self, Self::Err>` - The parsed template type or an error.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            TEMPLATE_TYPE_CONTROLLER => Ok(Self::Controller),
            TEMPLATE_TYPE_DOMAIN => Ok(Self::Domain),
            TEMPLATE_TYPE_EXCEPTION => Ok(Self::Exception),
            TEMPLATE_TYPE_MAPPER => Ok(Self::Mapper),
            TEMPLATE_TYPE_MODEL => Ok(Self::Model),
            TEMPLATE_TYPE_REPOSITORY => Ok(Self::Repository),
            TEMPLATE_TYPE_SERVICE => Ok(Self::Service),
            TEMPLATE_TYPE_UTILS => Ok(Self::Utils),
            TEMPLATE_TYPE_VIEW => Ok(Self::View),
            _ => Err(TemplateError::InvalidTemplateType(s.to_string())),
        }
    }
}

impl TemplateConfig {
    /// Create a new template configuration
    ///
    /// # Arguments
    ///
    /// - `TemplateType` - Type of template component
    /// - `String` - Name of the component
    /// - `Option<ModelSubType>` - Optional model subtype for model components
    ///
    /// # Returns
    ///
    /// - `Self` - Configuration instance
    pub fn new(
        template_type: TemplateType,
        component_name: String,
        model_sub_type: Option<ModelSubType>,
    ) -> Self {
        Self {
            template_type,
            component_name,
            model_sub_type,
            base_directory: TEMPLATE_CONFIG_BASE_DIRECTORY.to_string(),
        }
    }
}

impl FromStr for ModelSubType {
    type Err = TemplateError;

    /// Parses a lowercase model sub type name.
    ///
    /// # Arguments
    ///
    /// - `&str` - The model sub type name to parse.
    ///
    /// # Returns
    ///
    /// - `Result<Self, Self::Err>` - The parsed model sub type or an error.
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            MODEL_SUB_TYPE_APPLICATION => Ok(Self::Application),
            MODEL_SUB_TYPE_REQUEST => Ok(Self::Request),
            MODEL_SUB_TYPE_RESPONSE => Ok(Self::Response),
            _ => Err(TemplateError::InvalidModelSubType(s.to_string())),
        }
    }
}
