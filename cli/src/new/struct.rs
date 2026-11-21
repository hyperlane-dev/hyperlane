use super::*;

/// Configuration for creating a new project
#[derive(Clone, Data, Debug)]
pub struct NewProjectConfig {
    /// Name of the project to create
    pub(crate) project_name: String,
    /// URL of the template repository
    pub(crate) template_url: String,
}
