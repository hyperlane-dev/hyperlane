use super::*;

/// Configuration for [`StaticFilesRuntime`] and [`StaticFilesPlugin`].
#[derive(Clone, Data, Debug, Eq, PartialEq)]
pub struct StaticFilesConfig {
    /// Filesystem root directory from which assets are served.
    pub(crate) root_dir: PathBuf,
    /// URL mount prefix (for example `"/static"` or `"/"`).
    pub(crate) mount_prefix: String,
    /// Default directory index filename (default `"index.html"`).
    pub(crate) index_file: String,
    /// Optional SPA fallback file relative to `root_dir` (for example `"index.html"`).
    #[get(type(clone))]
    pub(crate) spa_fallback: Option<String>,
    /// `Cache-Control` header value applied to served files.
    pub(crate) cache_control: String,
    /// Enables serving precompressed `.br` and `.gz` sidecar files when accepted by the client.
    #[get(type(copy))]
    pub(crate) serve_precompressed: bool,
    /// Permits serving dotfiles (`.well-known`, etc.) when `true` (default `false`).
    #[get(type(copy))]
    pub(crate) allow_dotfiles: bool,
    /// Total memory budget in bytes for the 64-way sharded hot-file cache (`0` disables caching).
    #[get(type(copy))]
    pub(crate) max_cache_bytes: usize,
    /// Maximum single file size eligible for in-memory hot caching.
    #[get(type(copy))]
    pub(crate) max_file_cache_bytes: usize,
}

/// Immutable cached static file payload stored inside [`StaticFilesRuntime`].
#[derive(Clone, Debug)]
pub(crate) struct CachedFile {
    /// File contents bytes.
    pub(crate) bytes: Vec<u8>,
    /// Precomputed strong `ETag`.
    pub(crate) etag: String,
    /// Resolved MIME `Content-Type`.
    pub(crate) content_type: &'static str,
    /// Optional precompressed `Content-Encoding` (`"br"` or `"gzip"`).
    pub(crate) content_encoding: Option<&'static str>,
    /// Source file modification timestamp in milliseconds for cache freshness validation.
    pub(crate) modified_ms: u64,
}

/// Shared interior state of [`StaticFilesRuntime`].
pub(crate) struct StaticFilesRuntimeInner {
    /// Original static file configuration.
    pub(crate) config: StaticFilesConfig,
    /// Canonicalized root directory path for symlink-escape verification.
    pub(crate) canonical_root: PathBuf,
    /// Normalized mount prefix (without trailing slash unless `"/"`).
    pub(crate) normalized_prefix: String,
    /// Current cached byte usage across all 64 shards.
    pub(crate) cached_bytes: AtomicUsize,
    /// 64-way sharded hot-file memory cache.
    pub(crate) cache_shards: StaticCacheShardArray,
}

/// Thread-safe static asset engine with 64-way sharded hot-file caching and security checks.
#[derive(Clone)]
pub struct StaticFilesRuntime {
    /// Shared runtime state.
    pub(crate) inner: Arc<StaticFilesRuntimeInner>,
}

/// Process-wide [`StaticFilesPlugin`] singleton storing [`StaticFilesRuntime`].
#[derive(Clone)]
pub struct StaticFilesPlugin {
    /// Compiled static-file serving engine.
    pub(crate) runtime: StaticFilesRuntime,
}

/// Opt-in [`ServerHook`] serving static assets on `GET` / `HEAD` requests under the configured mount prefix.
#[derive(Clone, Copy, Debug, Default)]
#[hyperlane::request_middleware(-55)]
pub struct StaticFilesHook;
