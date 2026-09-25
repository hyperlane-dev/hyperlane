use super::*;

#[test]
fn test_sync_report_creation() {
    let report: SyncReport = SyncReport {
        workspace_version: "1.2.3".to_string(),
        renamed_entries: vec![("old".to_string(), "new".to_string())],
        versioned_entries: vec![("a".to_string(), "b".to_string())],
        file_changed: true,
    };
    assert_eq!(report.workspace_version, "1.2.3");
    assert_eq!(report.renamed_entries.len(), 1);
    assert_eq!(report.versioned_entries.len(), 1);
    assert!(report.file_changed);
}

#[test]
fn test_sync_report_clone() {
    let report: SyncReport = SyncReport {
        workspace_version: "0.1.0".to_string(),
        renamed_entries: Vec::new(),
        versioned_entries: Vec::new(),
        file_changed: false,
    };
    let cloned: SyncReport = report.clone();
    assert_eq!(cloned, report);
}

#[test]
fn test_sync_report_equality() {
    let report_a: SyncReport = SyncReport {
        workspace_version: "1.0.0".to_string(),
        renamed_entries: vec![],
        versioned_entries: vec![],
        file_changed: false,
    };
    let report_b: SyncReport = SyncReport {
        workspace_version: "1.0.0".to_string(),
        renamed_entries: vec![],
        versioned_entries: vec![],
        file_changed: false,
    };
    assert_eq!(report_a, report_b);
}

#[test]
fn test_sync_error_display() {
    let error_a: SyncError = SyncError::ManifestParseError;
    assert!(error_a.to_string().contains("Failed to parse"));
    let error_b: SyncError = SyncError::ManifestSerializeError;
    assert!(error_b.to_string().contains("Failed to serialize"));
    let error_c: SyncError = SyncError::WorkspaceVersionMissing("test".to_string());
    assert!(error_c.to_string().contains("workspace.package.version"));
    assert!(error_c.to_string().contains("test"));
    let error_d: SyncError = SyncError::WorkspaceMembersMissing("test".to_string());
    assert!(error_d.to_string().contains("workspace.members"));
    let error_e: SyncError = SyncError::MemberManifestMissing("test".to_string());
    assert!(error_e.to_string().contains("member crate Cargo.toml"));
    let error_f: SyncError = SyncError::MemberNameMissing("test".to_string());
    assert!(error_f.to_string().contains("[package].name"));
}

#[test]
fn test_sync_error_from_io() {
    let io_error: io::Error = io::Error::new(io::ErrorKind::NotFound, "missing");
    let sync_error: SyncError = SyncError::from(io_error);
    assert!(sync_error.to_string().contains("IO error"));
}

#[tokio::test]
async fn test_execute_sync_idempotent_when_already_synced() {
    let tmp_dir: PathBuf = PathBuf::from("./tmp/test_sync_idempotent");
    create_dir_all(&tmp_dir).await.unwrap();
    let workspace_manifest: PathBuf = tmp_dir.join("Cargo.toml");
    let member_dir: PathBuf = tmp_dir.join("alpha");
    create_dir_all(&member_dir).await.unwrap();
    let member_manifest: PathBuf = member_dir.join("Cargo.toml");
    let workspace_content: &str = r#"[workspace]
members = ["alpha"]

[workspace.package]
version = "0.1.0"

[workspace.dependencies]
alpha = { path = "alpha", version = "0.1.0" }
"#;
    write(&workspace_manifest, workspace_content).await.unwrap();
    let member_content: &str = r#"[package]
name = "alpha"
version = "0.1.0"
edition = "2024"
"#;
    write(&member_manifest, member_content).await.unwrap();
    let report: SyncReport = execute_sync(workspace_manifest.to_str().unwrap())
        .await
        .unwrap();
    assert_eq!(report.workspace_version, "0.1.0");
    assert_eq!(report.versioned_entries.len(), 0);
    assert_eq!(report.renamed_entries.len(), 0);
    assert!(!report.file_changed);
    let second_report: SyncReport = execute_sync(workspace_manifest.to_str().unwrap())
        .await
        .unwrap();
    assert!(!second_report.file_changed);
}

#[tokio::test]
async fn test_execute_sync_rewrites_version_literal() {
    let tmp_dir: PathBuf = PathBuf::from("./tmp/test_sync_rewrite_version");
    create_dir_all(&tmp_dir).await.unwrap();
    let workspace_manifest: PathBuf = tmp_dir.join("Cargo.toml");
    let member_dir: PathBuf = tmp_dir.join("beta");
    create_dir_all(&member_dir).await.unwrap();
    let member_manifest: PathBuf = member_dir.join("Cargo.toml");
    let workspace_content: &str = r#"[workspace]
members = ["beta"]

[workspace.package]
version = "0.2.0"

[workspace.dependencies]
beta = { path = "beta", version = "0.1.5" }
"#;
    write(&workspace_manifest, workspace_content).await.unwrap();
    let member_content: &str = r#"[package]
name = "beta"
version = "0.1.5"
edition = "2024"
"#;
    write(&member_manifest, member_content).await.unwrap();
    let report: SyncReport = execute_sync(workspace_manifest.to_str().unwrap())
        .await
        .unwrap();
    assert_eq!(report.workspace_version, "0.2.0");
    assert!(report.file_changed);
    let updated: String = read_to_string(&workspace_manifest).await.unwrap();
    assert!(updated.contains("[workspace.dependencies.beta]\npath = \"beta\""));
    assert!(updated.contains(r#"version = "0.2.0""#));
    assert!(!updated.contains(r#"version = "0.1.5""#));
    let second_report: SyncReport = execute_sync(workspace_manifest.to_str().unwrap())
        .await
        .unwrap();
    assert!(!second_report.file_changed);
}

#[tokio::test]
async fn test_execute_sync_renames_dep_alias_to_match_member_crate_name() {
    let tmp_dir: PathBuf = PathBuf::from("./tmp/test_sync_rename_alias");
    create_dir_all(&tmp_dir).await.unwrap();
    let workspace_manifest: PathBuf = tmp_dir.join("Cargo.toml");
    let member_dir: PathBuf = tmp_dir.join("gamma");
    create_dir_all(&member_dir).await.unwrap();
    let member_manifest: PathBuf = member_dir.join("Cargo.toml");
    let workspace_content: &str = r#"[workspace]
members = ["gamma"]

[workspace.package]
version = "0.3.0"

[workspace.dependencies]
stale_alias = { path = "gamma", version = "0.3.0" }
"#;
    write(&workspace_manifest, workspace_content).await.unwrap();
    let member_content: &str = r#"[package]
name = "gamma"
version = "0.3.0"
edition = "2024"
"#;
    write(&member_manifest, member_content).await.unwrap();
    let report: SyncReport = execute_sync(workspace_manifest.to_str().unwrap())
        .await
        .unwrap();
    assert_eq!(report.renamed_entries.len(), 1);
    assert_eq!(report.renamed_entries[0].0, "stale_alias");
    assert_eq!(report.renamed_entries[0].1, "gamma");
    assert!(report.file_changed);
    let updated: String = read_to_string(&workspace_manifest).await.unwrap();
    assert!(updated.contains("[workspace.dependencies.gamma]"));
    assert!(updated.contains(r#"path = "gamma""#));
    assert!(updated.contains(r#"version = "0.3.0""#));
    assert!(!updated.contains("stale_alias"));
}

#[tokio::test]
async fn test_execute_sync_handles_multiple_members() {
    let tmp_dir: PathBuf = PathBuf::from("./tmp/test_sync_multiple");
    create_dir_all(&tmp_dir).await.unwrap();
    let workspace_manifest: PathBuf = tmp_dir.join("Cargo.toml");
    for member in ["one", "two", "three"] {
        let member_dir: PathBuf = tmp_dir.join(member);
        create_dir_all(&member_dir).await.unwrap();
        let member_content: String = format!(
            r#"[package]
name = "{member}"
version = "0.0.0"
edition = "2024"
"#
        );
        write(&member_dir.join("Cargo.toml"), member_content)
            .await
            .unwrap();
    }
    let workspace_content: &str = r#"[workspace]
members = ["one", "two", "three"]

[workspace.package]
version = "9.9.9"

[workspace.dependencies]
one = { path = "one", version = "0.0.0" }
two = { path = "two", version = "0.0.0" }
three = { path = "three", version = "0.0.0" }
"#;
    write(&workspace_manifest, workspace_content).await.unwrap();
    let report: SyncReport = execute_sync(workspace_manifest.to_str().unwrap())
        .await
        .unwrap();
    assert_eq!(report.workspace_version, "9.9.9");
    assert_eq!(report.versioned_entries.len(), 3);
    let updated: String = read_to_string(&workspace_manifest).await.unwrap();
    for member in ["one", "two", "three"] {
        assert!(
            updated.contains(&format!("[workspace.dependencies.{member}]")),
            "expected dotted-form workspace.dependencies.{member} entry in:\n{updated}"
        );
        assert!(
            updated.contains(r#"version = "9.9.9""#),
            "expected version 9.9.9 in updated manifest:\n{updated}"
        );
    }
}

#[tokio::test]
async fn test_execute_sync_errors_on_missing_member_manifest() {
    let tmp_dir: PathBuf = PathBuf::from("./tmp/test_sync_missing_member");
    create_dir_all(&tmp_dir).await.unwrap();
    let workspace_manifest: PathBuf = tmp_dir.join("Cargo.toml");
    let workspace_content: &str = r#"[workspace]
members = ["does_not_exist"]

[workspace.package]
version = "1.0.0"
"#;
    write(&workspace_manifest, workspace_content).await.unwrap();
    let result: Result<SyncReport, SyncError> =
        execute_sync(workspace_manifest.to_str().unwrap()).await;
    assert!(result.is_err());
    match result.unwrap_err() {
        SyncError::MemberManifestMissing(_) => {}
        other => panic!("expected MemberManifestMissing, got {other:?}"),
    }
}

#[tokio::test]
async fn test_execute_sync_errors_on_missing_workspace_version() {
    let tmp_dir: PathBuf = PathBuf::from("./tmp/test_sync_no_version");
    create_dir_all(&tmp_dir).await.unwrap();
    let workspace_manifest: PathBuf = tmp_dir.join("Cargo.toml");
    let workspace_content: &str = r#"[workspace]
members = []
"#;
    write(&workspace_manifest, workspace_content).await.unwrap();
    let result: Result<SyncReport, SyncError> =
        execute_sync(workspace_manifest.to_str().unwrap()).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn test_execute_sync_skips_members_without_dep_entry() {
    let tmp_dir: PathBuf = PathBuf::from("./tmp/test_sync_no_dep");
    create_dir_all(&tmp_dir).await.unwrap();
    let workspace_manifest: PathBuf = tmp_dir.join("Cargo.toml");
    let member_dir: PathBuf = tmp_dir.join("solo");
    create_dir_all(&member_dir).await.unwrap();
    write(
        &member_dir.join("Cargo.toml"),
        r#"[package]
name = "solo"
version = "0.0.0"
edition = "2024"
"#,
    )
    .await
    .unwrap();
    let workspace_content: &str = r#"[workspace]
members = ["solo"]

[workspace.package]
version = "0.1.0"
"#;
    write(&workspace_manifest, workspace_content).await.unwrap();
    let report: SyncReport = execute_sync(workspace_manifest.to_str().unwrap())
        .await
        .unwrap();
    assert!(!report.file_changed);
    assert_eq!(report.versioned_entries.len(), 0);
}
