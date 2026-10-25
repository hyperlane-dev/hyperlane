/// The workspace table key.
pub(crate) const WORKSPACE_FIELD: &str = "workspace";
/// The workspace member array key.
pub(crate) const MEMBERS_FIELD: &str = "members";
/// The workspace dependency table key.
pub(crate) const DEPENDENCIES_FIELD: &str = "dependencies";
/// The manifest editor dependency key.
pub(crate) const EDITOR_FIELD: &str = "toml_edit";
/// The Cargo manifest file name.
pub(crate) const CONTEXT_MANIFEST: &str = "Cargo.toml";
/// The frozen Cargo lock file name.
pub(crate) const CONTEXT_LOCK: &str = "Cargo.lock";
/// The harness members injected into an archived comparison context.
pub(crate) const CONTEXT_MEMBERS: [&str; 1] = ["performance"];
/// The error for a missing workspace table.
pub(crate) const MISSING_WORKSPACE: &str = "the build context has no workspace table";
/// The error for a malformed member list.
pub(crate) const MISSING_MEMBERS: &str = "the build context has no workspace member array";
/// The error for a missing dependency table.
pub(crate) const MISSING_DEPENDENCIES: &str = "the build context has no workspace dependency table";
/// The error for a missing harness-only editor dependency.
pub(crate) const MISSING_EDITOR: &str = "the source workspace has no manifest editor dependency";
/// The error for a source directory without a parent.
pub(crate) const MISSING_SOURCE_ROOT: &str = "the performance source has no repository parent";
