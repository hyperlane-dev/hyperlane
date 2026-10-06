use super::*;

impl NewProjectConfig {
    /// Create a new project configuration with default template
    ///
    /// # Arguments
    ///
    /// - `String` - Name of the project
    ///
    /// # Returns
    ///
    /// - `Self` - Configuration instance
    pub fn new(project_name: String) -> Self {
        Self {
            project_name,
            template_url: DEFAULT_TEMPLATE_URL.to_string(),
        }
    }
}
