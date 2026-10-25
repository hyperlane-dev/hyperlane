use super::*;

#[test]
fn comparison_manifest_preserves_library_identity_and_dependencies() {
    let source: &str =
        "[workspace]\nmembers = []\n[workspace.dependencies]\ntoml_edit = \"0.25.17\"\n";
    let comparison: &str = "[package]\nname = \"library\"\nversion = \"1.2.3\"\n[workspace]\nmembers = [\"core\"]\n[workspace.dependencies]\nserde = \"1\"\n[profile.release]\nopt-level = 3\n";
    let rendered: String = render_context_manifest(source, comparison).unwrap();
    let before: DocumentMut = comparison.parse().unwrap();
    let after: DocumentMut = rendered.parse().unwrap();
    assert_eq!(before["package"].to_string(), after["package"].to_string());
    assert_eq!(before["profile"].to_string(), after["profile"].to_string());
    assert_eq!(
        before["workspace"]["dependencies"]["serde"].to_string(),
        after["workspace"]["dependencies"]["serde"].to_string()
    );
    let members: &Array = after["workspace"]["members"].as_array().unwrap();
    assert_eq!(members.len(), 2);
    for member in CONTEXT_MEMBERS {
        assert!(
            members
                .iter()
                .any(|value: &Value| value.as_str() == Some(member))
        );
    }
    assert_eq!(
        render_context_manifest(source, &rendered).unwrap(),
        rendered
    );
}

#[test]
fn comparison_manifest_rejects_invalid_or_missing_workspace_wiring() {
    let source: &str =
        "[workspace]\nmembers = []\n[workspace.dependencies]\ntoml_edit = \"0.25.17\"\n";
    assert!(render_context_manifest(source, "not toml").is_err());
    assert!(render_context_manifest(source, "[package]\nname = \"library\"\n").is_err());
    assert!(render_context_manifest(source, "[workspace]\nmembers = false\n").is_err());
}

#[test]
fn comparison_manifest_keeps_an_existing_editor_requirement() {
    let source: &str =
        "[workspace]\nmembers = []\n[workspace.dependencies]\ntoml_edit = \"0.25.17\"\n";
    let comparison: &str =
        "[workspace]\nmembers = []\n[workspace.dependencies]\ntoml_edit = \"0.24\"\n";
    let rendered: String = render_context_manifest(source, comparison).unwrap();
    let after: DocumentMut = rendered.parse().unwrap();
    assert_eq!(
        after["workspace"]["dependencies"]["toml_edit"].as_str(),
        Some("0.24")
    );
}
