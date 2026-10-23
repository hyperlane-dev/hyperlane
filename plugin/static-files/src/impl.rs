use super::*;

impl Default for StaticFilesConfig {
    /// Builds the configuration the plugin uses when none is given.
    ///
    /// # Returns
    ///
    /// - `Self` - The default configuration.
    fn default() -> Self {
        Self {
            root: PathBuf::from("."),
            index: INDEX_DEFAULT.to_owned(),
        }
    }
}

impl StaticFilesConfig {
    /// The file a path names under the root, when the path is allowed.
    ///
    /// A path that climbs out of the root is refused, so a request cannot
    /// reach a file the root does not hold.
    ///
    /// # Arguments
    ///
    /// - `&str` - The path of the file, relative to the root.
    ///
    /// # Returns
    ///
    /// - `Option<PathBuf>` - The file, when the path is allowed.
    pub fn resolve(&self, path: &str) -> Option<PathBuf> {
        let mut file: PathBuf = self.get_root().clone();
        if path.is_empty() {
            file.push(self.get_index());
            return Some(file);
        }
        for segment in path.split(SEPARATOR_SLASH) {
            if segment.is_empty()
                || segment == SEGMENT_CURRENT
                || segment == SEGMENT_PARENT
                || segment.contains(SEPARATOR_BACKSLASH)
            {
                return None;
            }
            file.push(segment);
        }
        Some(file)
    }

    /// The content type of a file, by the extension of its name.
    ///
    /// A file whose extension the framework does not know is served as a
    /// stream of bytes.
    ///
    /// # Arguments
    ///
    /// - `&Path` - The file that is served.
    ///
    /// # Returns
    ///
    /// - `String` - The content type of the file.
    pub fn content_type(&self, file: &Path) -> String {
        let path: String = file.to_string_lossy().to_string();
        let extension: String = FileExtension::get_extension_name(path);
        let content_type: &str = FileExtension::parse(extension).get_content_type();
        if content_type.is_empty() {
            return APPLICATION_OCTET_STREAM.to_owned();
        }
        content_type.to_owned()
    }
}

impl StaticFilesPlugin {
    /// Stores the configuration the plugin works from.
    ///
    /// # Arguments
    ///
    /// - `StaticFilesConfig` - The configuration to store.
    ///
    /// # Returns
    ///
    /// - `Result<(), String>` - Nothing, or the reason the configuration was
    ///   refused.
    pub fn init(config: StaticFilesConfig) -> Result<(), String> {
        CONFIG.set(config).map_err(|_: StaticFilesConfig| {
            "the static file configuration was already set".to_owned()
        })
    }

    /// The configuration the plugin works from.
    ///
    /// # Returns
    ///
    /// - `Option<&'static StaticFilesConfig>` - The configuration, when it was
    ///   set.
    pub fn get() -> Option<&'static StaticFilesConfig> {
        CONFIG.get()
    }

    /// The route pattern that serves the files under a prefix.
    ///
    /// # Arguments
    ///
    /// - `&str` - The path the files are served under.
    ///
    /// # Returns
    ///
    /// - `String` - The pattern of the route.
    pub fn pattern(prefix: &str) -> String {
        PATTERN_FORMAT.replace("{prefix}", prefix)
    }
}

impl ServerHook for StaticFilesRoute {
    /// Builds the route.
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - The stream the request arrived on.
    /// - `&mut Context` - The context of the request.
    ///
    /// # Returns
    ///
    /// - `Self` - The route.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    /// Answers with the file the path names.
    ///
    /// A path that names no file is answered with the status of a missing
    /// file.
    ///
    /// # Arguments
    ///
    /// - `Self` - The route.
    /// - `&mut Stream` - The stream the request arrived on.
    /// - `&mut Context` - The context of the request.
    ///
    /// # Returns
    ///
    /// - `Status` - Whether the pipeline continues.
    async fn handle(self, _: &mut Stream, ctx: &mut Context) -> Status {
        let config: &StaticFilesConfig = match CONFIG.get() {
            Some(config) => config,
            None => return Status::Continue,
        };
        let path: String = ctx.get_route_param(PARAM_PATH);
        let file: Option<PathBuf> = config.resolve(&path);
        let file: PathBuf = match file {
            Some(file) => file,
            None => return Self::not_found(ctx),
        };
        let data: Vec<u8> = match fs::read(&file).await {
            Ok(data) => data,
            Err(_) => return Self::not_found(ctx),
        };
        let content_type: String = config.content_type(&file);
        let response: &mut Response = ctx.get_mut_response();
        response.set_status_code(HttpStatus::Ok.code());
        response.set_header(CONTENT_TYPE, content_type);
        response.set_body(data);
        Status::Continue
    }
}

impl StaticFilesRoute {
    /// Answers with the status of a file that is not there.
    ///
    /// # Arguments
    ///
    /// - `&mut Context` - The context of the request.
    ///
    /// # Returns
    ///
    /// - `Status` - Whether the pipeline continues.
    fn not_found(ctx: &mut Context) -> Status {
        let response: &mut Response = ctx.get_mut_response();
        response.set_status_code(HttpStatus::NotFound.code());
        Status::Continue
    }
}
