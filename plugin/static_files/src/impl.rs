use super::*;

/// Inherent constructors and builders for [`StaticFilesConfig`].
impl StaticFilesConfig {
    /// Creates a [`StaticFilesConfig`] serving files from `root_dir` mounted at `"/"`.
    ///
    /// # Arguments
    ///
    /// - `P` - Root directory path convertible into `PathBuf`.
    ///
    /// # Returns
    ///
    /// - `Self` - Constructed static files configuration.
    pub fn new<P>(root_dir: P) -> Self
    where
        P: Into<PathBuf>,
    {
        Self {
            root_dir: root_dir.into(),
            mount_prefix: DEFAULT_STATIC_MOUNT_PREFIX.to_owned(),
            index_file: DEFAULT_STATIC_INDEX_FILE.to_owned(),
            spa_fallback: None,
            cache_control: DEFAULT_STATIC_CACHE_CONTROL.to_owned(),
            serve_precompressed: true,
            allow_dotfiles: false,
            max_cache_bytes: DEFAULT_STATIC_MAX_CACHE_BYTES,
            max_file_cache_bytes: DEFAULT_STATIC_MAX_FILE_CACHE_BYTES,
        }
    }

    /// Sets the URL mount prefix (for example `"/assets"`).
    ///
    /// # Arguments
    ///
    /// - `P` - Mount prefix convertible into `String`.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_mount_prefix<P>(mut self, prefix: P) -> Self
    where
        P: Into<String>,
    {
        self.set_mount_prefix(prefix.into());
        self
    }

    /// Sets an SPA fallback file relative to `root_dir` (for example `"index.html"`).
    ///
    /// # Arguments
    ///
    /// - `F` - Fallback filename convertible into `String`.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_spa_fallback<F>(mut self, fallback: F) -> Self
    where
        F: Into<String>,
    {
        self.set_spa_fallback(Some(fallback.into()));
        self
    }

    /// Sets the `Cache-Control` response header value.
    ///
    /// # Arguments
    ///
    /// - `C` - Header value convertible into `String`.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_cache_control<C>(mut self, cache_control: C) -> Self
    where
        C: Into<String>,
    {
        self.set_cache_control(cache_control.into());
        self
    }

    /// Enables or disables `.br`/`.gz` precompressed sidecar negotiation.
    ///
    /// # Arguments
    ///
    /// - `bool` - Whether precompressed sidecars are checked.
    ///
    /// # Returns
    ///
    /// - `Self` - Updated configuration.
    pub fn with_serve_precompressed(mut self, enabled: bool) -> Self {
        self.set_serve_precompressed(enabled);
        self
    }
}

/// Inherent implementation of [`StaticFilesRuntime`].
impl StaticFilesRuntime {
    /// Compiles a [`StaticFilesRuntime`] by canonicalizing `config.root_dir`.
    ///
    /// # Arguments
    ///
    /// - `StaticFilesConfig` - Static file server configuration.
    ///
    /// # Returns
    ///
    /// - `Result<Self, String>` - Ready runtime or canonicalization error message.
    pub fn new(config: StaticFilesConfig) -> Result<Self, String> {
        let canonical_root: PathBuf =
            canonicalize(config.get_root_dir()).map_err(|err: io::Error| err.to_string())?;
        let trimmed: &str = config.get_mount_prefix().trim();
        let normalized_prefix: String = if trimmed.is_empty() || trimmed == "/" {
            "/".to_owned()
        } else if trimmed.starts_with('/') {
            trimmed.trim_end_matches('/').to_owned()
        } else {
            format!("/{}", trimmed.trim_end_matches('/'))
        };
        let cache_shards: StaticCacheShardArray =
            Box::new(from_fn(|_: usize| RwLock::new(hash_map_xx_hash3_64())));
        Ok(Self {
            inner: Arc::new(StaticFilesRuntimeInner {
                config,
                canonical_root,
                normalized_prefix,
                cached_bytes: AtomicUsize::new(0),
                cache_shards,
            }),
        })
    }

    /// Returns the active [`StaticFilesConfig`].
    ///
    /// # Returns
    ///
    /// - `&StaticFilesConfig` - Active configuration.
    pub fn get_config(&self) -> &StaticFilesConfig {
        let Self { inner } = self;
        &inner.config
    }

    /// Returns current in-memory hot-file cache usage in bytes.
    ///
    /// # Returns
    ///
    /// - `usize` - Cached bytes across all 64 shards.
    pub fn cached_bytes(&self) -> usize {
        let Self { inner } = self;
        inner.cached_bytes.load(Ordering::Relaxed)
    }

    /// Strips the configured mount prefix on a path-segment boundary.
    ///
    /// # Arguments
    ///
    /// - `&'a str` - Incoming request path.
    ///
    /// # Returns
    ///
    /// - `Option<&'a str>` - Relative sub-path when matched, or `None`.
    pub(crate) fn strip_mount<'a>(&self, path: &'a str) -> Option<&'a str> {
        let Self { inner } = self;
        if inner.normalized_prefix == "/" {
            return Some(path);
        }
        let rest: &'a str = path.strip_prefix(inner.normalized_prefix.as_str())?;
        if rest.is_empty() || rest.starts_with('/') {
            Some(rest)
        } else {
            None
        }
    }

    /// Resolves a verified canonical file path inside `canonical_root`, applying directory index and sidecar selection.
    ///
    /// # Arguments
    ///
    /// - `&str` - Relative URL path under mount prefix.
    /// - `Option<&str>` - Optional `Accept-Encoding` header value.
    ///
    /// # Returns
    ///
    /// - `Option<(PathBuf, &'static str, Option<&'static str>)>` - `(disk_path, content_type, content_encoding)`.
    async fn resolve_target_file(
        &self,
        relative_url: &str,
        accept_encoding: Option<&str>,
    ) -> Option<(PathBuf, &'static str, Option<&'static str>)> {
        let Self { inner } = self;
        let candidate: PathBuf = resolve_safe_path(
            &inner.canonical_root,
            relative_url,
            inner.config.get_allow_dotfiles(),
        )?;
        let resolved_base: PathBuf = match fs::metadata(&candidate).await {
            Ok(meta) if meta.is_dir() => candidate.join(inner.config.get_index_file()),
            Ok(meta) if meta.is_file() => candidate,
            _ => match inner.config.get_spa_fallback() {
                Some(fallback) => inner.canonical_root.join(fallback),
                None => return None,
            },
        };
        let canonical_base: PathBuf = fs::canonicalize(&resolved_base).await.ok()?;
        if !canonical_base.starts_with(&inner.canonical_root) {
            return None;
        }
        let mime: &'static str = mime_for_path(&canonical_base);
        if inner.config.get_serve_precompressed()
            && let Some(ae) = accept_encoding
        {
            if accepts_encoding(ae, ENCODING_BR) {
                let mut br_os: OsString = canonical_base.as_os_str().to_os_string();
                br_os.push(SIDECAR_BR_EXT);
                let br_path: PathBuf = PathBuf::from(br_os);
                if let Ok(canon_br) = fs::canonicalize(&br_path).await
                    && canon_br.starts_with(&inner.canonical_root)
                    && fs::metadata(&canon_br)
                        .await
                        .is_ok_and(|m: Metadata| m.is_file())
                {
                    return Some((canon_br, mime, Some(ENCODING_BR)));
                }
            }
            if accepts_encoding(ae, ENCODING_GZIP) {
                let mut gz_os: OsString = canonical_base.as_os_str().to_os_string();
                gz_os.push(SIDECAR_GZ_EXT);
                let gz_path: PathBuf = PathBuf::from(gz_os);
                if let Ok(canon_gz) = fs::canonicalize(&gz_path).await
                    && canon_gz.starts_with(&inner.canonical_root)
                    && fs::metadata(&canon_gz)
                        .await
                        .is_ok_and(|m: Metadata| m.is_file())
                {
                    return Some((canon_gz, mime, Some(ENCODING_GZIP)));
                }
            }
        }
        Some((canonical_base, mime, None))
    }

    /// Loads `disk_path` from the 64-shard hot-file cache (revalidating mtime+size) or disk.
    ///
    /// # Arguments
    ///
    /// - `&Path` - Canonical target file path.
    /// - `&'static str` - Resolved MIME content type.
    /// - `Option<&'static str>` - Optional sidecar `Content-Encoding`.
    ///
    /// # Returns
    ///
    /// - `Option<Arc<CachedFile>>` - Loaded file entry, or `None` on I/O failure.
    async fn load_file(
        &self,
        disk_path: &Path,
        content_type: &'static str,
        content_encoding: Option<&'static str>,
    ) -> Option<Arc<CachedFile>> {
        let Self { inner } = self;
        let meta: Metadata = fs::metadata(disk_path).await.ok()?;
        if !meta.is_file() {
            return None;
        }
        let file_len: u64 = meta.len();
        let modified_ms: u64 = metadata_modified_ms(&meta);
        let shard_idx: usize = static_cache_shard_index(disk_path);
        let max_cache: usize = inner.config.get_max_cache_bytes();
        let max_file_cache: usize = inner.config.get_max_file_cache_bytes();
        if max_cache > 0
            && (file_len as usize) <= max_file_cache
            && let Ok(guard) = inner.cache_shards[shard_idx].read()
            && let Some(cached) = guard.get(disk_path)
            && cached.modified_ms == modified_ms
            && cached.bytes.len() as u64 == file_len
        {
            return Some(Arc::clone(cached));
        }
        let bytes: Vec<u8> = fs::read(disk_path).await.ok()?;
        let etag: String = compute_file_etag(bytes.len() as u64, modified_ms);
        let file: Arc<CachedFile> = Arc::new(CachedFile {
            bytes,
            etag,
            content_type,
            content_encoding,
            modified_ms,
        });
        if max_cache > 0 && file.bytes.len() <= max_file_cache {
            let size: usize = file.bytes.len();
            if inner.cached_bytes.load(Ordering::Relaxed) + size <= max_cache
                && let Ok(mut guard) = inner.cache_shards[shard_idx].write()
            {
                if let Some(prev) = guard.insert(disk_path.to_path_buf(), Arc::clone(&file)) {
                    inner
                        .cached_bytes
                        .fetch_sub(prev.bytes.len(), Ordering::Relaxed);
                }
                inner.cached_bytes.fetch_add(size, Ordering::Relaxed);
            }
        }
        Some(file)
    }

    /// Attempts to serve a static asset for `ctx`, populating status, headers, and body.
    ///
    /// # Arguments
    ///
    /// - `&mut Context` - Incoming request and outgoing response context.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` when a static file (or `304`/`206`/`416`) was served; `false` otherwise.
    pub async fn serve_context(&self, ctx: &mut Context) -> bool {
        let is_get: bool = ctx.get_request().get_method().is_get();
        let is_head: bool = ctx.get_request().get_method().is_head();
        if !is_get && !is_head {
            return false;
        }
        let req_path: String = ctx.get_request().get_path().clone();
        let Some(relative_url) = self.strip_mount(&req_path) else {
            return false;
        };
        let accept_encoding: Option<String> =
            ctx.get_request().try_get_header_back(ACCEPT_ENCODING);
        let Some((disk_path, mime, encoding)) = self
            .resolve_target_file(relative_url, accept_encoding.as_deref())
            .await
        else {
            return false;
        };
        let Some(file) = self.load_file(&disk_path, mime, encoding).await else {
            return false;
        };
        let if_none_match: Option<String> = ctx.get_request().try_get_header_back(IF_NONE_MATCH);
        let range_header: Option<String> = ctx.get_request().try_get_header_back(RANGE);
        let cache_control: &str = self.get_config().get_cache_control();
        let response: &mut Response = ctx.get_mut_response();
        response
            .set_header(CONTENT_TYPE, file.content_type)
            .set_header(ETAG, &file.etag)
            .set_header(ACCEPT_RANGES, ACCEPT_RANGES_BYTES);
        if !cache_control.is_empty() {
            response.set_header(CACHE_CONTROL, cache_control);
        }
        if let Some(enc) = file.content_encoding {
            response
                .set_header(CONTENT_ENCODING, enc)
                .set_header(VARY, VARY_ACCEPT_ENCODING);
        }
        if let Some(inm) = if_none_match
            && if_none_match_matches(&inm, &file.etag)
        {
            response
                .set_status_code(HttpStatus::NotModified.code())
                .set_body(Vec::new());
            return true;
        }
        let total: usize = file.bytes.len();
        if let Some(range_val) = range_header {
            match parse_byte_range(&range_val, total) {
                Some((start, end)) => {
                    let content_range: String = format!("bytes {start}-{end}/{total}");
                    response
                        .set_status_code(HttpStatus::PartialContent.code())
                        .set_header(CONTENT_RANGE, content_range);
                    if is_head {
                        response.set_body(Vec::new());
                    } else {
                        response.set_body(&file.bytes[start..=end]);
                    }
                }
                None => {
                    let content_range: String = format!("bytes */{total}");
                    response
                        .set_status_code(HttpStatus::RangeNotSatisfiable.code())
                        .set_header(CONTENT_RANGE, content_range)
                        .set_body(Vec::new());
                }
            }
            return true;
        }
        response.set_status_code(HttpStatus::Ok.code());
        if is_head {
            response.set_body(Vec::new());
        } else {
            response.set_body(file.bytes.clone());
        }
        true
    }
}

/// Inherent implementation of [`StaticFilesPlugin`].
impl StaticFilesPlugin {
    /// Initializes the global [`StaticFilesPlugin`] once from `config`.
    ///
    /// # Arguments
    ///
    /// - `StaticFilesConfig` - Static file server configuration.
    ///
    /// # Returns
    ///
    /// - `Result<&'static Self, String>` - Global plugin reference or canonicalization error.
    pub fn init(config: StaticFilesConfig) -> Result<&'static Self, String> {
        let runtime: StaticFilesRuntime = StaticFilesRuntime::new(config)?;
        Ok(STATIC_FILES_PLUGIN.get_or_init(|| Self { runtime }))
    }

    /// Returns the initialized global [`StaticFilesPlugin`], if initialized.
    ///
    /// # Returns
    ///
    /// - `Option<&'static Self>` - Reference to the global plugin, or `None`.
    pub fn get() -> Option<&'static Self> {
        STATIC_FILES_PLUGIN.get()
    }

    /// Returns the active [`StaticFilesRuntime`].
    ///
    /// # Returns
    ///
    /// - `&StaticFilesRuntime` - Active static file runtime.
    pub fn get_runtime(&self) -> &StaticFilesRuntime {
        &self.runtime
    }
}

/// Implements [`ServerHook`] for [`StaticFilesHook`].
impl ServerHook for StaticFilesHook {
    /// Constructs a per-request [`StaticFilesHook`].
    ///
    /// # Arguments
    ///
    /// - `&mut Stream` - Connection stream.
    /// - `&mut Context` - Request context.
    ///
    /// # Returns
    ///
    /// - `Self` - Hook instance.
    async fn new(_: &mut Stream, _: &mut Context) -> Self {
        Self
    }

    /// Serves matching static files and short-circuits the pipeline once answered.
    ///
    /// # Arguments
    ///
    /// - `Self` - Hook instance.
    /// - `&mut Stream` - Connection stream.
    /// - `&mut Context` - Request context.
    ///
    /// # Returns
    ///
    /// - `Status` - `Status::Reject` once a static asset was served; `Status::Continue` otherwise.
    async fn handle(self, stream: &mut Stream, ctx: &mut Context) -> Status {
        let Some(plugin) = StaticFilesPlugin::get() else {
            return Status::Continue;
        };
        if plugin.get_runtime().serve_context(ctx).await {
            let wire: ResponseData = ctx.get_mut_response().build();
            let _: Result<(), ResponseError> = stream.try_send(&wire).await;
            return Status::Reject;
        }
        Status::Continue
    }
}
