use super::*;

impl NewProjectConfig {
    /// Create a new project configuration with default template
    ///
    /// # Arguments
    ///
    /// - `String`: Name of the project
    ///
    /// # Returns
    ///
    /// - `Self`: Configuration instance
    pub fn new(project_name: String) -> Self {
        Self {
            project_name,
            template_url: "https://github.com/hyperlane-dev/hyperlane-quick-start".to_string(),
        }
    }
}
