/// Configuration for creating a new project
#[derive(Clone, Debug)]
pub struct NewProjectConfig {
    /// Name of the project to create
    pub project_name: String,
    /// URL of the template repository
    pub template_url: String,
}
