use super::*;

#[test]
fn test_bump_version_type_enum() {
    assert_eq!(BumpVersionType::Patch, BumpVersionType::Patch);
    assert_eq!(BumpVersionType::Minor, BumpVersionType::Minor);
    assert_eq!(BumpVersionType::Major, BumpVersionType::Major);
    assert_eq!(BumpVersionType::Release, BumpVersionType::Release);
    assert_eq!(BumpVersionType::Alpha, BumpVersionType::Alpha);
    assert_eq!(BumpVersionType::Beta, BumpVersionType::Beta);
    assert_eq!(BumpVersionType::Rc, BumpVersionType::Rc);
}

#[test]
fn test_version_struct_creation() {
    let version: Version = Version {
        major: 1,
        minor: 2,
        patch: 3,
        prerelease: Some("alpha.1".to_string()),
    };
    assert_eq!(version.major, 1);
    assert_eq!(version.minor, 2);
    assert_eq!(version.patch, 3);
    assert_eq!(version.prerelease, Some("alpha.1".to_string()));
}

#[test]
fn test_version_clone() {
    let version: Version = Version {
        major: 1,
        minor: 2,
        patch: 3,
        prerelease: Some("beta".to_string()),
    };
    let cloned: Version = version.clone();
    assert_eq!(cloned.major, version.major);
    assert_eq!(cloned.minor, version.minor);
    assert_eq!(cloned.patch, version.patch);
    assert_eq!(cloned.prerelease, version.prerelease);
}

#[tokio::test]
async fn test_execute_bump_integration() {
    let tmp_dir: PathBuf = PathBuf::from("./tmp/test_bump");
    create_dir_all(&tmp_dir).await.unwrap();
    let manifest_path: PathBuf = tmp_dir.join("Cargo.toml");
    let content: &str = r#"[package]
name = "test-package"
version = "0.1.0"
edition = "2024"
"#;
    write(&manifest_path, content).await.unwrap();
    let result: Result<String, Box<dyn std::error::Error>> =
        execute_bump(manifest_path.to_str().unwrap(), &BumpVersionType::Patch).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "0.1.1");
    let updated_content: String = read_to_string(&manifest_path).await.unwrap();
    assert!(updated_content.contains("version = \"0.1.1\""));
}

#[tokio::test]
async fn test_execute_bump_minor() {
    let tmp_dir: PathBuf = PathBuf::from("./tmp/test_bump_minor");
    create_dir_all(&tmp_dir).await.unwrap();
    let manifest_path: PathBuf = tmp_dir.join("Cargo.toml");
    let content: &str = r#"[package]
name = "test-package"
version = "0.1.0"
edition = "2024"
"#;
    write(&manifest_path, content).await.unwrap();
    let result: Result<String, Box<dyn std::error::Error>> =
        execute_bump(manifest_path.to_str().unwrap(), &BumpVersionType::Minor).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "0.2.0");
}

#[tokio::test]
async fn test_execute_bump_major() {
    let tmp_dir: PathBuf = PathBuf::from("./tmp/test_bump_major");
    create_dir_all(&tmp_dir).await.unwrap();
    let manifest_path: PathBuf = tmp_dir.join("Cargo.toml");
    let content: &str = r#"[package]
name = "test-package"
version = "0.1.0"
edition = "2024"
"#;
    write(&manifest_path, content).await.unwrap();
    let result: Result<String, Box<dyn std::error::Error>> =
        execute_bump(manifest_path.to_str().unwrap(), &BumpVersionType::Major).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "1.0.0");
}

#[tokio::test]
async fn test_execute_bump_alpha() {
    let tmp_dir: PathBuf = PathBuf::from("./tmp/test_bump_alpha");
    create_dir_all(&tmp_dir).await.unwrap();
    let manifest_path: PathBuf = tmp_dir.join("Cargo.toml");
    let content: &str = r#"[package]
name = "test-package"
version = "0.1.0"
edition = "2024"
"#;
    write(&manifest_path, content).await.unwrap();
    let result: Result<String, Box<dyn std::error::Error>> =
        execute_bump(manifest_path.to_str().unwrap(), &BumpVersionType::Alpha).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "0.1.0-alpha");
}

#[tokio::test]
async fn test_execute_bump_beta() {
    let tmp_dir: PathBuf = PathBuf::from("./tmp/test_bump_beta");
    create_dir_all(&tmp_dir).await.unwrap();
    let manifest_path: PathBuf = tmp_dir.join("Cargo.toml");
    let content: &str = r#"[package]
name = "test-package"
version = "0.1.0-alpha.2"
edition = "2024"
"#;
    write(&manifest_path, content).await.unwrap();
    let result: Result<String, Box<dyn std::error::Error>> =
        execute_bump(manifest_path.to_str().unwrap(), &BumpVersionType::Beta).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "0.1.0-beta.1");
}

#[tokio::test]
async fn test_execute_bump_rc() {
    let tmp_dir: PathBuf = PathBuf::from("./tmp/test_bump_rc");
    create_dir_all(&tmp_dir).await.unwrap();
    let manifest_path: PathBuf = tmp_dir.join("Cargo.toml");
    let content: &str = r#"[package]
name = "test-package"
version = "0.1.0-beta.1"
edition = "2024"
"#;
    write(&manifest_path, content).await.unwrap();
    let result: Result<String, Box<dyn std::error::Error>> =
        execute_bump(manifest_path.to_str().unwrap(), &BumpVersionType::Rc).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "0.1.0-rc.1");
}

#[tokio::test]
async fn test_execute_bump_release() {
    let tmp_dir: PathBuf = PathBuf::from("./tmp/test_bump_release");
    create_dir_all(&tmp_dir).await.unwrap();
    let manifest_path: PathBuf = tmp_dir.join("Cargo.toml");
    let content: &str = r#"[package]
name = "test-package"
version = "0.1.0-alpha"
edition = "2024"
"#;
    write(&manifest_path, content).await.unwrap();
    let result: Result<String, Box<dyn std::error::Error>> =
        execute_bump(manifest_path.to_str().unwrap(), &BumpVersionType::Release).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap(), "0.1.0");
}

#[tokio::test]
async fn test_execute_bump_no_version_field() {
    let tmp_dir: PathBuf = PathBuf::from("./tmp/test_bump_no_version");
    create_dir_all(&tmp_dir).await.unwrap();
    let manifest_path: PathBuf = tmp_dir.join("Cargo.toml");
    let content: &str = r#"[package]
name = "test-package"
edition = "2024"
"#;
    write(&manifest_path, content).await.unwrap();
    let result: Result<String, Box<dyn std::error::Error>> =
        execute_bump(manifest_path.to_str().unwrap(), &BumpVersionType::Patch).await;
    assert!(result.is_err());
}
