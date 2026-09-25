use super::*;

/// Read `[workspace.package].version` from a workspace Cargo.toml.
///
/// # Arguments
///
/// - `&Value`: Parsed manifest.
///
/// # Returns
///
/// - `Result<String, SyncError>`: Workspace version, or an error if the
///   field is missing.
fn read_workspace_version(doc: &Value) -> Result<String, SyncError> {
    let version: String = doc
        .get("workspace")
        .and_then(|workspace: &Value| workspace.get("package"))
        .and_then(|package: &Value| package.get("version"))
        .and_then(|version_value: &Value| version_value.as_str())
        .ok_or_else(|| SyncError::WorkspaceVersionMissing("Cargo.toml".to_string()))?
        .to_string();
    Ok(version)
}

/// Read `[workspace.members]` list from a workspace Cargo.toml.
///
/// # Arguments
///
/// - `&Value`: Parsed manifest.
///
/// # Returns
///
/// - `Result<Vec<String>, SyncError>`: Member path list, or an error.
fn read_workspace_members(doc: &Value) -> Result<Vec<String>, SyncError> {
    let members: Vec<String> = doc
        .get("workspace")
        .and_then(|workspace: &Value| workspace.get("members"))
        .and_then(|members_value: &Value| members_value.as_array())
        .ok_or_else(|| SyncError::WorkspaceMembersMissing("Cargo.toml".to_string()))?
        .iter()
        .filter_map(|member: &Value| member.as_str().map(|s: &str| s.to_string()))
        .collect();
    Ok(members)
}

/// Read `[package].name` from a single member crate's Cargo.toml.
///
/// # Arguments
///
/// - `&Value`: Parsed member manifest.
///
/// # Returns
///
/// - `Result<String, SyncError>`: Crate name, or an error if missing.
fn read_member_crate_name(doc: &Value) -> Result<String, SyncError> {
    let name: String = doc
        .get("package")
        .and_then(|package: &Value| package.get("name"))
        .and_then(|name_value: &Value| name_value.as_str())
        .ok_or_else(|| SyncError::MemberNameMissing("Cargo.toml".to_string()))?
        .to_string();
    Ok(name)
}

/// Find the dep LHS name whose entry points at `member_path`.
///
/// # Arguments
///
/// - `&Value`: Parsed `[workspace.dependencies]` table.
/// - `&str`: Member path to look up (e.g. `"type"`).
///
/// # Returns
///
/// - `Option<String>`: The current dep LHS, if any entry references
///   `path = "member_path"`.
fn find_dep_alias_for_member_path(deps: &Value, member_path: &str) -> Option<String> {
    let table: &toml::map::Map<String, Value> = deps.as_table()?;
    for (alias, entry) in table {
        if let Some(entry_table) = entry.as_table()
            && let Some(path) = entry_table.get("path")
            && path.as_str() == Some(member_path)
        {
            return Some(alias.clone());
        }
    }
    None
}

/// Apply both the version literal and (if needed) the LHS alias rewrite
/// to a single `[workspace.dependencies]` entry.
///
/// # Arguments
///
/// - `&mut toml::map::Map<String, Value>`: Mutable `[workspace.dependencies]`
///   table to update.
/// - `&str`: Member path.
/// - `&str`: Current LHS alias to look up.
/// - `&str`: Canonical LHS alias (the member crate's `[package].name`).
/// - `&str`: Workspace version to write into the entry's `version` field.
///
/// # Returns
///
/// - `Option<(String, String)>`: `Some((old_alias, new_alias))` if the
///   alias was renamed; `None` otherwise. The function mutates the
///   table in-place regardless: at minimum it sets `version` to
///   `workspace_version`.
fn rewrite_dep_entry(
    deps: &mut toml::map::Map<String, Value>,
    member_path: &str,
    current_alias: &str,
    canonical_alias: &str,
    workspace_version: &str,
) -> Option<(String, String)> {
    let entry: &mut Value = deps.get_mut(current_alias)?;
    let entry_table: &mut toml::map::Map<String, Value> = entry.as_table_mut()?;
    let path_matches: bool = entry_table
        .get("path")
        .and_then(|path_value: &Value| path_value.as_str())
        .map(|s: &str| s == member_path)
        .unwrap_or(false);
    if !path_matches {
        return None;
    }
    entry_table.insert(
        "version".to_string(),
        Value::String(workspace_version.to_string()),
    );
    if current_alias != canonical_alias {
        let renamed: (String, String) = (current_alias.to_string(), canonical_alias.to_string());
        let new_entry: Value = entry.clone();
        deps.remove(current_alias);
        let entry_to_set: &mut Value = deps.entry(canonical_alias.to_string()).or_insert(new_entry);
        let canonical_table: &mut toml::map::Map<String, Value> =
            entry_to_set.as_table_mut().unwrap();
        canonical_table.insert("path".to_string(), Value::String(member_path.to_string()));
        canonical_table.insert(
            "version".to_string(),
            Value::String(workspace_version.to_string()),
        );
        Some(renamed)
    } else {
        None
    }
}

/// Align every local path-only entry under `[workspace.dependencies]`
/// with the workspace version, and (optionally) the dep alias with the
/// member crate's actual `[package].name`.
///
/// # Arguments
///
/// - `&str`: Path to the workspace root Cargo.toml.
///
/// # Returns
///
/// - `Result<SyncReport, SyncError>`: Summary of what was rewritten.
///
/// # Behavior
///
/// * Reads `[workspace.package].version`. Errors out if it is missing.
/// * Reads `[workspace.members]`. Errors out if it is missing.
/// * For each member path:
///     1. Opens `<member_path>/Cargo.toml` and reads its `[package].name`.
///     2. Locates the existing `[workspace.dependencies]` entry whose
///        `path = "<member_path>"` and captures its current LHS alias.
///     3. If the entry is already aligned (alias + version both
///        correct), skip it.
///     4. Otherwise rewrite the entry: align `version` to the workspace
///        version, and rename the LHS alias to match the crate's
///        `[package].name` when the alias differs.
/// * `file_changed` is true iff at least one rename or version-rewrite
///   actually happened (so a no-op run is genuinely idempotent and
///   does not write the file).
pub async fn execute_sync(manifest_path: &str) -> Result<SyncReport, SyncError> {
    let path: &Path = Path::new(manifest_path);
    let content: String = read_to_string(path).await?;
    let mut doc: Value = toml::from_str(&content).map_err(|_| SyncError::ManifestParseError)?;
    let workspace_version: String = read_workspace_version(&doc)?;
    let members: Vec<String> = read_workspace_members(&doc)?;
    if members.is_empty() {
        log::info!("sync: no workspace members, nothing to do");
        return Ok(SyncReport {
            workspace_version,
            renamed_entries: Vec::new(),
            versioned_entries: Vec::new(),
            file_changed: false,
        });
    }
    let mut renamed_entries: Vec<(String, String)> = Vec::new();
    let mut versioned_entries: Vec<(String, String)> = Vec::new();
    let mut needs_rewrite: bool = false;
    let deps_value: Option<&Value> = doc
        .get("workspace")
        .and_then(|workspace: &Value| workspace.get("dependencies"));
    let mut deps: toml::map::Map<String, Value> = deps_value
        .and_then(|value: &Value| value.as_table())
        .cloned()
        .unwrap_or_default();
    for member_path in &members {
        let member_manifest_path: PathBuf = path
            .parent()
            .unwrap_or_else(|| Path::new("."))
            .join(member_path)
            .join("Cargo.toml");
        if !member_manifest_path.exists() {
            return Err(SyncError::MemberManifestMissing(
                member_manifest_path.display().to_string(),
            ));
        }
        let member_content: String = read_to_string(&member_manifest_path).await?;
        let member_doc: Value =
            toml::from_str(&member_content).map_err(|_| SyncError::ManifestParseError)?;
        let canonical_alias: String = read_member_crate_name(&member_doc)?;
        let current_alias: Option<String> =
            find_dep_alias_for_member_path(&Value::Table(deps.clone()), member_path);
        let current_alias: String = match current_alias {
            Some(alias) => alias,
            None => {
                log::info!(
                    "sync: {} -> no [workspace.dependencies] entry, skipping",
                    member_path
                );
                continue;
            }
        };
        let existing_version: Option<String> = deps
            .get(&current_alias)
            .and_then(|entry: &Value| entry.as_table())
            .and_then(|table: &toml::map::Map<String, Value>| table.get("version"))
            .and_then(|version_value: &Value| version_value.as_str())
            .map(|s: &str| s.to_string());
        let alias_needs_rename: bool = current_alias != canonical_alias;
        let version_needs_rewrite: bool = existing_version
            .as_deref()
            .map(|existing: &str| existing != workspace_version)
            .unwrap_or(true);
        if !alias_needs_rename && !version_needs_rewrite {
            continue;
        }
        needs_rewrite = true;
        let renamed: Option<(String, String)> = rewrite_dep_entry(
            &mut deps,
            member_path,
            &current_alias,
            &canonical_alias,
            &workspace_version,
        );
        if let Some((old, new)) = &renamed {
            log::info!("sync: {} renamed {} -> {}", member_path, old, new);
            renamed_entries.push((old.clone(), new.clone()));
        } else {
            log::info!(
                "sync: {} -> {} v{}",
                member_path,
                canonical_alias,
                workspace_version
            );
        }
        versioned_entries.push((member_path.clone(), canonical_alias));
    }
    let file_changed: bool = needs_rewrite;
    if file_changed {
        if let Some(workspace_table) = doc
            .get_mut("workspace")
            .and_then(|workspace: &mut Value| workspace.as_table_mut())
        {
            workspace_table.insert("dependencies".to_string(), Value::Table(deps.clone()));
        }
        let updated_content: String =
            toml::to_string(&doc).map_err(|_| SyncError::ManifestSerializeError)?;
        write(path, updated_content).await?;
        log::info!(
            "sync: wrote {} entries to v{}",
            versioned_entries.len(),
            workspace_version
        );
    } else {
        log::info!("sync: already in sync: v{}", workspace_version);
    }
    Ok(SyncReport {
        workspace_version,
        renamed_entries,
        versioned_entries,
        file_changed,
    })
}
