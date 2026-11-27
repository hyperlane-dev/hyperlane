/// Default URL mount prefix served by [`crate::StaticFilesHook`].
pub const DEFAULT_STATIC_MOUNT_PREFIX: &str = "/";

/// Default directory index filename.
pub const DEFAULT_STATIC_INDEX_FILE: &str = "index.html";

/// Default `Cache-Control` header applied to served static files.
pub const DEFAULT_STATIC_CACHE_CONTROL: &str = "public, max-age=3600";

/// Default maximum total memory budget for the sharded hot-file cache (32 MiB).
pub const DEFAULT_STATIC_MAX_CACHE_BYTES: usize = 32 * 1024 * 1024;

/// Default maximum single file size eligible for in-memory hot caching (1 MiB).
pub const DEFAULT_STATIC_MAX_FILE_CACHE_BYTES: usize = 1024 * 1024;

/// Number of independent hot-file cache shards (`64`, power of two).
pub(crate) const STATIC_CACHE_SHARDS: usize = 64;

/// `Accept-Ranges` value advertising byte-range support.
pub(crate) const ACCEPT_RANGES_BYTES: &str = "bytes";

/// `Range` header unit prefix.
pub(crate) const RANGE_BYTES_PREFIX: &str = "bytes=";

/// `Content-Encoding` token for Brotli precompressed sidecars.
pub(crate) const ENCODING_BR: &str = "br";

/// `Content-Encoding` token for Gzip precompressed sidecars.
pub(crate) const ENCODING_GZIP: &str = "gzip";

/// File extension suffix for Brotli sidecar assets.
pub(crate) const SIDECAR_BR_EXT: &str = ".br";

/// File extension suffix for Gzip sidecar assets.
pub(crate) const SIDECAR_GZ_EXT: &str = ".gz";

/// `Vary` token emitted when serving negotiated precompressed sidecars.
pub(crate) const VARY_ACCEPT_ENCODING: &str = "Accept-Encoding";
