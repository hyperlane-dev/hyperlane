use super::*;

/// Overwrite a string `Item` in-place, keeping the original decor
/// (whitespace / comments) so the rendered document only differs in the
/// string literal itself.
///
/// # Arguments
///
/// - `&mut Item`: Slot holding the value to replace.
/// - `&str`: New string content.
pub(crate) fn set_item_string_preserving_decor(slot: &mut Item, new_string: &str) {
    let mut replacement: Item = value(new_string);
    if let Some(old_value) = slot.as_value()
        && let Some(new_value) = replacement.as_value_mut()
    {
        *new_value.decor_mut() = old_value.decor().clone();
    }
    *slot = replacement;
}

/// Read the workspace root version from a manifest: prefer
/// `[workspace.package].version`, fall back to `[package].version` (same
/// recognition rule as `execute_bump`).
///
/// # Arguments
///
/// - `&DocumentMut`: Parsed manifest.
///
/// # Returns
///
/// - `Result<String, SyncError>`: Root version, or an error if neither
///   field exists.
fn read_root_version(doc: &DocumentMut) -> Result<String, SyncError> {
    let workspace_version: Option<&str> = doc
        .get("workspace")
        .and_then(|workspace: &Item| workspace.get("package"))
        .and_then(|package: &Item| package.get("version"))
        .and_then(|version_item: &Item| version_item.as_str());
    let package_version: Option<&str> = doc
        .get("package")
        .and_then(|package: &Item| package.get("version"))
        .and_then(|version_item: &Item| version_item.as_str());
    workspace_version
        .or(package_version)
        .map(|version: &str| version.to_string())
        .ok_or_else(|| SyncError::WorkspaceVersionMissing("Cargo.toml".to_string()))
}

/// Read `[workspace.members]` list from a workspace manifest.
///
/// # Arguments
///
/// - `&DocumentMut`: Parsed manifest.
///
/// # Returns
///
/// - `Result<Vec<String>, SyncError>`: Member path list, or an error.
fn read_workspace_members(doc: &DocumentMut) -> Result<Vec<String>, SyncError> {
    let members: Vec<String> = doc
        .get("workspace")
        .and_then(|workspace: &Item| workspace.get("members"))
        .and_then(|members_item: &Item| members_item.as_array())
        .ok_or_else(|| SyncError::WorkspaceMembersMissing("Cargo.toml".to_string()))?
        .iter()
        .filter_map(|member: &TomlEditValue| member.as_str().map(|s: &str| s.to_string()))
        .collect();
    Ok(members)
}

/// Read `[package].name` from a single member crate's Cargo.toml.
///
/// # Arguments
///
/// - `&DocumentMut`: Parsed member manifest.
///
/// # Returns
///
/// - `Result<String, SyncError>`: Crate name, or an error if missing.
fn read_member_crate_name(doc: &DocumentMut) -> Result<String, SyncError> {
    let name: String = doc
        .get("package")
        .and_then(|package: &Item| package.get("name"))
        .and_then(|name_item: &Item| name_item.as_str())
        .ok_or_else(|| SyncError::MemberNameMissing("Cargo.toml".to_string()))?
        .to_string();
    Ok(name)
}

/// Find the dep LHS name whose entry points at `member_path`.
///
/// # Arguments
///
/// - `&dyn TableLike`: Parsed `[workspace.dependencies]` table.
/// - `&str`: Member path to look up (e.g. `\"type\"`).
///
/// # Returns
///
/// - `Option<String>`: The current dep LHS, if any entry references
///   `path = \"member_path\"`.
fn find_dep_alias_for_member_path(deps: &dyn TableLike, member_path: &str) -> Option<String> {
    for (alias, entry) in deps.iter() {
        if let Some(path) = entry
            .get("path")
            .and_then(|path_item: &Item| path_item.as_str())
            && path == member_path
        {
            return Some(alias.to_string());
        }
    }
    None
}

/// Read the current dep alias and version literal for `member_path` from
/// the workspace manifest, without mutating anything.
///
/// # Arguments
///
/// - `&DocumentMut`: Parsed workspace manifest.
/// - `&str`: Member path.
///
/// # Returns
///
/// - `Option<(String, Option<String>)>`: `(current_alias,
///   existing_version)` if an entry references `path = \"member_path\"`.
fn scan_dep_entry(doc: &DocumentMut, member_path: &str) -> Option<(String, Option<String>)> {
    let deps: &dyn TableLike = doc
        .get("workspace")
        .and_then(|workspace: &Item| workspace.get("dependencies"))
        .and_then(|deps_item: &Item| deps_item.as_table_like())?;
    let current_alias: String = find_dep_alias_for_member_path(deps, member_path)?;
    let existing_version: Option<String> = deps
        .get(&current_alias)
        .and_then(|entry: &Item| entry.get("version"))
        .and_then(|version_item: &Item| version_item.as_str())
        .map(|version: &str| version.to_string());
    Some((current_alias, existing_version))
}

/// Set the entry's `version` field to `workspace_version`, in-place and
/// decor-preserving when the field already exists.
///
/// # Arguments
///
/// - `&mut dyn TableLike`: Mutable `[workspace.dependencies]` table.
/// - `&str`: Dep LHS alias of the entry to update.
/// - `&str`: Workspace version to write into the entry's `version` field.
fn rewrite_entry_version(deps: &mut dyn TableLike, current_alias: &str, workspace_version: &str) {
    let Some(entry) = deps.get_mut(current_alias) else {
        return;
    };
    match entry.get_mut("version") {
        Some(version_slot) => set_item_string_preserving_decor(version_slot, workspace_version),
        None => {
            if let Some(entry_table) = entry.as_table_like_mut() {
                entry_table.insert("version", value(workspace_version));
            }
        }
    }
}

/// Align every local path entry under `[workspace.dependencies]` with the
/// workspace root version, and (optionally) the dep alias with the member
/// crate's actual `[package].name`.
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
/// * Reads the root version via `read_root_version` (workspace package
///   version first, plain package version as fallback). Errors out if
///   neither exists.
/// * Reads `[workspace.members]`. Errors out if it is missing.
/// * For each member path:
///     1. Opens `<member_path>/Cargo.toml` and reads its `[package].name`.
///     2. Locates the existing `[workspace.dependencies]` entry whose
///        `path = \"<member_path>\"` and captures its current LHS alias.
///     3. If the entry is already aligned (alias + version both
///        correct), skip it.
///     4. Otherwise rewrite the entry in-place: align `version` to the
///        root version, and rename the LHS alias to match the crate's
///        `[package].name` when the alias differs.
/// * Edits are applied on a `toml_edit::DocumentMut`, so comments, key
///   order and inline-table formatting outside the edited values are
///   preserved byte-for-byte.
/// * `file_changed` is true iff at least one rename or version-rewrite
///   actually happened (so a no-op run is genuinely idempotent and
///   does not write the file).
pub async fn execute_sync(manifest_path: &str) -> Result<SyncReport, SyncError> {
    let path: &Path = Path::new(manifest_path);
    let content: String = read_to_string(path).await?;
    let mut doc: DocumentMut = content.parse().map_err(|_| SyncError::ManifestParseError)?;
    let workspace_version: String = read_root_version(&doc)?;
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
        let member_doc: DocumentMut = member_content
            .parse()
            .map_err(|_| SyncError::ManifestParseError)?;
        let canonical_alias: String = read_member_crate_name(&member_doc)?;
        let (current_alias, existing_version): (String, Option<String>) =
            match scan_dep_entry(&doc, member_path) {
                Some(scanned) => scanned,
                None => {
                    log::info!(
                        "sync: {} -> no [workspace.dependencies] entry, skipping",
                        member_path
                    );
                    continue;
                }
            };
        let alias_needs_rename: bool = current_alias != canonical_alias;
        let version_needs_rewrite: bool =
            existing_version.as_deref() != Some(workspace_version.as_str());
        if !alias_needs_rename && !version_needs_rewrite {
            continue;
        }
        needs_rewrite = true;
        let deps: &mut dyn TableLike = doc
            .get_mut("workspace")
            .and_then(|workspace: &mut Item| workspace.get_mut("dependencies"))
            .and_then(|deps_item: &mut Item| deps_item.as_table_like_mut())
            .ok_or(SyncError::ManifestParseError)?;
        rewrite_entry_version(deps, &current_alias, &workspace_version);
        if alias_needs_rename {
            if let Some(entry) = deps.remove(&current_alias) {
                deps.insert(&canonical_alias, entry);
            }
            log::info!(
                "sync: {} renamed {} -> {}",
                member_path,
                current_alias,
                canonical_alias
            );
            renamed_entries.push((current_alias, canonical_alias.clone()));
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
        write(path, doc.to_string()).await?;
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
