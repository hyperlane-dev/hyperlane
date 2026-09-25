use super::*;

/// Validate project name
///
/// # Arguments
///
/// - `&str`: Project name to validate
///
/// # Returns
///
/// - `Result<(), NewError>`: Ok if valid, error otherwise
fn validate_project_name(name: &str) -> Result<(), NewError> {
    if name.is_empty() {
        return Err(NewError::InvalidName(
            "Project name cannot be empty".to_string(),
        ));
    }
    if name.contains('/') || name.contains('\\') || name.contains(':') {
        return Err(NewError::InvalidName(
            "Project name contains invalid characters".to_string(),
        ));
    }
    if name.starts_with('.') || name.starts_with('-') {
        return Err(NewError::InvalidName(
            "Project name cannot start with '.' or '-'".to_string(),
        ));
    }
    Ok(())
}

/// Check if git is available in the system
///
/// # Returns
///
/// - `Result<(), NewError>`: Ok if git is available, error otherwise
async fn check_git_available() -> Result<(), NewError> {
    let output: std::process::Output = Command::new("git")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .output()
        .await
        .map_err(|_| NewError::GitNotFound)?;
    if output.status.success() {
        Ok(())
    } else {
        Err(NewError::GitNotFound)
    }
}

/// Execute git clone command
///
/// # Arguments
///
/// - `&NewProjectConfig`: Project configuration containing template URL and project name
///
/// # Returns
///
/// - `Result<(), NewError>`: Success or error
async fn git_clone(config: &NewProjectConfig) -> Result<(), NewError> {
    let project_path: PathBuf = PathBuf::from(&config.project_name);
    if project_path.exists() {
        return Err(NewError::ProjectExists(config.project_name.clone()));
    }
    let output: std::process::Output = Command::new("git")
        .arg("clone")
        .arg(&config.template_url)
        .arg(&config.project_name)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .output()
        .await
        .map_err(NewError::IoError)?;
    if output.status.success() {
        Ok(())
    } else {
        let stderr: String = String::from_utf8_lossy(&output.stderr).to_string();
        Err(NewError::CloneFailed(stderr))
    }
}

/// Execute new command to create a project from template
///
/// # Arguments
///
/// - `&str`: Name of the project to create
///
/// # Returns
///
/// - `Result<(), NewError>`: Success or error
pub async fn execute_new(project_name: &str) -> Result<(), NewError> {
    validate_project_name(project_name)?;
    check_git_available().await?;
    let config: NewProjectConfig = NewProjectConfig::new(project_name.to_string());
    log::info!(
        "Creating new project '{}' from template...",
        config.project_name
    );
    git_clone(&config).await?;
    log::info!("Successfully created project '{}'", config.project_name);
    log::info!("  cd {}", config.project_name);
    log::info!("  cargo build");
    Ok(())
}
