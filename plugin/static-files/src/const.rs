/// The name of the route parameter that carries the path of the file.
pub(crate) const PARAM_PATH: &str = "path";

/// The pattern of a route that serves the files under a prefix.
pub(crate) const PATTERN_FORMAT: &str = "{prefix}/{path:.*}";

/// The file that answers a request for a directory.
pub(crate) const INDEX_DEFAULT: &str = "index.html";
