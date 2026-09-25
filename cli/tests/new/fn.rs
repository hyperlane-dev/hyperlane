use super::*;

#[test]
fn test_new_project_config_creation() {
    let config: NewProjectConfig = NewProjectConfig::new("test-project".to_string());
    assert_eq!(config.project_name, "test-project");
    assert_eq!(
        config.template_url,
        "https://github.com/hyperlane-dev/hyperlane-quick-start"
    );
}

#[test]
fn test_new_error_display() {
    let error1: NewError = NewError::GitNotFound;
    assert!(error1.to_string().contains("Git is not installed"));
    let error2: NewError = NewError::ProjectExists("test".to_string());
    assert!(error2.to_string().contains("test"));
    let error3: NewError = NewError::CloneFailed("network error".to_string());
    assert!(error3.to_string().contains("network error"));
    let error4: NewError = NewError::InvalidName("bad name".to_string());
    assert!(error4.to_string().contains("bad name"));
}

#[test]
fn test_new_error_from_io() {
    let io_error: io::Error = io::Error::new(io::ErrorKind::NotFound, "test");
    let new_error: NewError = NewError::from(io_error);
    assert!(new_error.to_string().contains("test"));
}

#[test]
fn test_new_error_debug() {
    let error: NewError = NewError::GitNotFound;
    let debug_str: String = format!("{error:?}");
    assert!(debug_str.contains("GitNotFound"));
}

#[test]
fn test_new_project_config_clone() {
    let config: NewProjectConfig = NewProjectConfig::new("test".to_string());
    let cloned: NewProjectConfig = config.clone();
    assert_eq!(cloned.project_name, config.project_name);
    assert_eq!(cloned.template_url, config.template_url);
}

#[test]
fn test_new_project_config_debug() {
    let config: NewProjectConfig = NewProjectConfig::new("test".to_string());
    let debug_str: String = format!("{config:?}");
    assert!(debug_str.contains("test"));
}
