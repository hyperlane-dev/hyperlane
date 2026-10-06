/// Message reported when the project name is empty.
pub(crate) const ERROR_PROJECT_NAME_EMPTY: &str = "Project name cannot be empty";

/// Message reported when the project name holds a path separator or a colon.
pub(crate) const ERROR_PROJECT_NAME_INVALID_CHARACTERS: &str =
    "Project name contains invalid characters";

/// Message reported when the project name starts with a dot or a dash.
pub(crate) const ERROR_PROJECT_NAME_INVALID_PREFIX: &str =
    "Project name cannot start with '.' or '-'";

/// Repository cloned by the `new` command when no template is configured.
pub(crate) const DEFAULT_TEMPLATE_URL: &str =
    "https://github.com/hyperlane-dev/hyperlane-quick-start";

/// Flag used to probe whether git is installed.
pub(crate) const GIT_FLAG_VERSION: &str = "--version";

/// Git sub-command that clones the template repository.
pub(crate) const GIT_SUBCOMMAND_CLONE: &str = "clone";
