/// Lowercase name of the `Controller` template type and its directory.
pub(crate) const TEMPLATE_TYPE_CONTROLLER: &str = "controller";

/// Lowercase name of the `Domain` template type and its directory.
pub(crate) const TEMPLATE_TYPE_DOMAIN: &str = "domain";

/// Lowercase name of the `Exception` template type and its directory.
pub(crate) const TEMPLATE_TYPE_EXCEPTION: &str = "exception";

/// Lowercase name of the `Mapper` template type and its directory.
pub(crate) const TEMPLATE_TYPE_MAPPER: &str = "mapper";

/// Lowercase name of the `Model` template type and its directory.
pub(crate) const TEMPLATE_TYPE_MODEL: &str = "model";

/// Lowercase name of the `Repository` template type and its directory.
pub(crate) const TEMPLATE_TYPE_REPOSITORY: &str = "repository";

/// Lowercase name of the `Service` template type and its directory.
pub(crate) const TEMPLATE_TYPE_SERVICE: &str = "service";

/// Lowercase name of the `Utils` template type and its directory.
pub(crate) const TEMPLATE_TYPE_UTILS: &str = "utils";

/// Lowercase name of the `View` template type and its directory.
pub(crate) const TEMPLATE_TYPE_VIEW: &str = "view";

/// Lowercase name of the `Application` model sub type and its directory.
pub(crate) const MODEL_SUB_TYPE_APPLICATION: &str = "application";

/// Lowercase name of the `Request` model sub type and its directory.
pub(crate) const MODEL_SUB_TYPE_REQUEST: &str = "request";

/// Lowercase name of the `Response` model sub type and its directory.
pub(crate) const MODEL_SUB_TYPE_RESPONSE: &str = "response";

/// Directory that holds every generated component tree.
pub(crate) const TEMPLATE_CONFIG_BASE_DIRECTORY: &str = "./application";

/// Keyword file name of the `const` module inside a generated directory.
pub(crate) const MODULE_NAME_CONST: &str = "const";

/// Keyword file name of the `enum` module inside a generated directory.
pub(crate) const MODULE_NAME_ENUM: &str = "enum";

/// Keyword file name of the `fn` module inside a generated directory.
pub(crate) const MODULE_NAME_FN: &str = "fn";

/// Keyword file name of the `impl` module inside a generated directory.
pub(crate) const MODULE_NAME_IMPL: &str = "impl";

/// Keyword file name of the `static` module inside a generated directory.
pub(crate) const MODULE_NAME_STATIC: &str = "static";

/// Keyword file name of the `struct` module inside a generated directory.
pub(crate) const MODULE_NAME_STRUCT: &str = "struct";

/// File name of the generated module declaration file.
pub(crate) const FILE_NAME_MOD_RS: &str = "mod.rs";

/// File name of the generated `const` file.
pub(crate) const FILE_NAME_CONST_RS: &str = "const.rs";

/// File name of the generated `enum` file.
pub(crate) const FILE_NAME_ENUM_RS: &str = "enum.rs";

/// File name of the generated `fn` file.
pub(crate) const FILE_NAME_FN_RS: &str = "fn.rs";

/// File name of the generated `impl` file.
pub(crate) const FILE_NAME_IMPL_RS: &str = "impl.rs";

/// File name of the generated `static` file.
pub(crate) const FILE_NAME_STATIC_RS: &str = "static.rs";

/// File name of the generated `struct` file.
pub(crate) const FILE_NAME_STRUCT_RS: &str = "struct.rs";

/// Opening delimiter of the aggregated `pub use` line in a generated `mod.rs`.
pub(crate) const PUB_USE_PREFIX: &str = "pub use {";

/// First line written into every generated keyword file.
pub(crate) const KEYWORD_FILE_HEADER: &str = "use super::*;\n";

/// Message reported when a model template is requested without a sub type.
pub(crate) const ERROR_MISSING_MODEL_SUB_TYPE: &str = "Missing model subtype";

/// Program used to format the freshly generated template code.
pub(crate) const CARGO_FMT_PROGRAM: &str = "cargo";
