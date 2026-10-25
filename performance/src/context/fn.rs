use super::*;

/// Adds only the current harness members and its editor to a comparison manifest.
///
/// # Arguments
///
/// - `&str` - The current source manifest.
/// - `&str` - The archived comparison manifest.
///
/// # Returns
///
/// - `Result<String, String>` - The preserved manifest with harness wiring.
pub(crate) fn render_context_manifest(source: &str, comparison: &str) -> Result<String, String> {
    let source: DocumentMut = match source.parse() {
        Ok(document) => document,
        Err(error) => return Err(format!("invalid source manifest: {error}")),
    };
    let mut comparison: DocumentMut = match comparison.parse() {
        Ok(document) => document,
        Err(error) => return Err(format!("invalid comparison manifest: {error}")),
    };
    let editor: &Item = source
        .get(WORKSPACE_FIELD)
        .and_then(Item::as_table)
        .and_then(|table: &Table| table.get(DEPENDENCIES_FIELD))
        .and_then(Item::as_table)
        .and_then(|table: &Table| table.get(EDITOR_FIELD))
        .ok_or_else(|| MISSING_EDITOR.to_owned())?;
    let workspace: &mut Table = comparison
        .get_mut(WORKSPACE_FIELD)
        .and_then(Item::as_table_mut)
        .ok_or_else(|| MISSING_WORKSPACE.to_owned())?;
    let members: &mut Array = workspace
        .get_mut(MEMBERS_FIELD)
        .and_then(Item::as_array_mut)
        .ok_or_else(|| MISSING_MEMBERS.to_owned())?;
    for member in CONTEXT_MEMBERS {
        if !members
            .iter()
            .any(|value: &Value| value.as_str() == Some(member))
        {
            members.push(member);
        }
    }
    let dependencies: &mut Table = workspace
        .get_mut(DEPENDENCIES_FIELD)
        .and_then(Item::as_table_mut)
        .ok_or_else(|| MISSING_DEPENDENCIES.to_owned())?;
    if !dependencies.contains_key(EDITOR_FIELD) {
        dependencies.insert(EDITOR_FIELD, editor.clone());
    }
    Ok(comparison.to_string())
}

/// Binds current harness members to an archived library without upgrading dependencies.
///
/// # Arguments
///
/// - `&Path` - The current performance source directory.
/// - `&Path` - The disposable comparison checkout.
///
/// # Returns
///
/// - `Result<(), String>` - Success, or the manifest/lock copying error.
pub(crate) fn prepare_context_manifest(source: &Path, context: &Path) -> Result<(), String> {
    let repository: &Path = source
        .parent()
        .ok_or_else(|| MISSING_SOURCE_ROOT.to_owned())?;
    let source_path: PathBuf = repository.join(CONTEXT_MANIFEST);
    let context_path: PathBuf = context.join(CONTEXT_MANIFEST);
    let current: String = match fs::read_to_string(&source_path) {
        Ok(text) => text,
        Err(error) => return Err(format!("could not read {}: {error}", source_path.display())),
    };
    let original: String = match fs::read_to_string(&context_path) {
        Ok(text) => text,
        Err(error) => {
            return Err(format!(
                "could not read {}: {error}",
                context_path.display()
            ));
        }
    };
    let rendered: String = render_context_manifest(&current, &original)?;
    if rendered != original
        && let Err(error) = fs::write(&context_path, rendered)
    {
        return Err(format!(
            "could not write {}: {error}",
            context_path.display()
        ));
    }
    match fs::copy(repository.join(CONTEXT_LOCK), context.join(CONTEXT_LOCK)) {
        Ok(_) => Ok(()),
        Err(error) => Err(format!("could not freeze the comparison lock: {error}")),
    }
}
