use super::*;

#[test]
fn test_package_creation() {
    let package: Package = Package {
        name: "test-package".to_string(),
        version: "0.1.0".to_string(),
        path: PathBuf::from("."),
        local_dependencies: vec![],
    };
    assert_eq!(package.name, "test-package");
    assert_eq!(package.version, "0.1.0");
    assert!(package.local_dependencies.is_empty());
}

#[test]
fn test_package_clone() {
    let package: Package = Package {
        name: "test-package".to_string(),
        version: "0.1.0".to_string(),
        path: PathBuf::from("."),
        local_dependencies: vec!["dep1".to_string()],
    };
    let cloned: Package = package.clone();
    assert_eq!(cloned.name, package.name);
    assert_eq!(cloned.version, package.version);
    assert_eq!(cloned.local_dependencies.len(), 1);
}

#[test]
fn test_package_equality() {
    let package1: Package = Package {
        name: "test".to_string(),
        version: "0.1.0".to_string(),
        path: PathBuf::from("."),
        local_dependencies: vec![],
    };
    let package2: Package = Package {
        name: "test".to_string(),
        version: "0.1.0".to_string(),
        path: PathBuf::from("."),
        local_dependencies: vec![],
    };
    assert_eq!(package1, package2);
}

#[test]
fn test_publish_result_success() {
    let result: PublishResult = PublishResult {
        package_name: "test".to_string(),
        success: true,
        error: None,
        retries: 0,
    };
    assert_eq!(result.package_name, "test");
    assert!(result.success);
    assert!(result.error.is_none());
    assert_eq!(result.retries, 0);
}

#[test]
fn test_publish_result_failure() {
    let result: PublishResult = PublishResult {
        package_name: "test".to_string(),
        success: false,
        error: Some("network error".to_string()),
        retries: 3,
    };
    assert!(!result.success);
    assert_eq!(result.error, Some("network error".to_string()));
    assert_eq!(result.retries, 3);
}

#[test]
fn test_publish_result_clone() {
    let result: PublishResult = PublishResult {
        package_name: "test".to_string(),
        success: true,
        error: None,
        retries: 0,
    };
    let cloned: PublishResult = result.clone();
    assert_eq!(cloned.package_name, result.package_name);
    assert_eq!(cloned.success, result.success);
    assert_eq!(cloned.error, result.error);
    assert_eq!(cloned.retries, result.retries);
}

#[test]
fn test_publish_error_display() {
    let error1: PublishError = PublishError::ManifestParseError;
    assert!(error1.to_string().contains("Failed to parse"));
    let error2: PublishError = PublishError::CircularDependency;
    assert!(error2.to_string().contains("Circular dependency"));
}

#[test]
fn test_publish_error_from_io() {
    let io_error: io::Error = io::Error::new(io::ErrorKind::NotFound, "test");
    let publish_error: PublishError = PublishError::from(io_error);
    assert!(publish_error.to_string().contains("IO error"));
}
