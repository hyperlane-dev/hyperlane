/// Represents different compression algorithms supported by the library.
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum Compress {
    /// Gzip compression algorithm.
    Gzip,
    /// Deflate compression algorithm.
    Deflate,
    /// Brotli compression algorithm.
    Br,
    /// Represents an unknown or unsupported compression algorithm.
    #[default]
    Unknown,
}
