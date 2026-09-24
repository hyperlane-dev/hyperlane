use super::*;

/// Enables parsing a string into a `Compress` enum variant.
///
/// This implementation allows converting string representations of compression
/// algorithms (like "gzip", "deflate", "br") into their corresponding `Compress`
/// enum variants. If the string does not match any known compression types,
/// it defaults to `Compress::Unknown`.
impl FromStr for Compress {
    type Err = ();

    /// Parses a string into a `Compress` enum variant.
    ///
    /// This method converts string representations of compression algorithms
    /// (case-insensitive) into their corresponding `Compress` enum variants.
    /// Unknown strings are converted to `Compress::Unknown`.
    ///
    /// # Arguments
    ///
    /// - `data` - The string to parse, which should be a compression algorithm name.
    ///
    /// # Returns
    ///
    /// - `Result<Self, Self::Err>` - Returns `Ok` with the matching `Compress` variant,
    ///   or `Ok(Compress::Unknown)` for unknown strings. Never returns `Err`.
    #[inline(always)]
    fn from_str(data: &str) -> Result<Self, Self::Err> {
        match data.to_lowercase().as_str() {
            _data if _data == CONTENT_ENCODING_GZIP => Ok(Self::Gzip),
            _data if _data == CONTENT_ENCODING_DEFLATE => Ok(Self::Deflate),
            _data if _data == CONTENT_ENCODING_BROTLI => Ok(Self::Br),
            _ => Ok(Self::Unknown),
        }
    }
}

/// Implements the `Display` trait for the `Compress` enum.
///
/// This allows the `Compress` enum variants to be formatted as strings,
/// typically used for outputting the `Content-Encoding` header value.
impl fmt::Display for Compress {
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let display_str: &str = match *self {
            Compress::Gzip => CONTENT_ENCODING_GZIP,
            Compress::Deflate => CONTENT_ENCODING_DEFLATE,
            Compress::Br => CONTENT_ENCODING_BROTLI,
            Compress::Unknown => EMPTY_STR,
        };
        write!(f, "{display_str}")
    }
}

/// Provides methods for interacting with the `Compress` enum.
impl Compress {
    /// Checks if the current instance is of the `Unknown` type.
    ///
    /// This method compares the current instance with the `Unknown` variant of the enum.
    /// It returns `true` if the instance is of type `Unknown`, otherwise `false`.
    ///
    /// # Returns
    ///
    /// - `true` if the instance is of type `Unknown`.
    /// - `false` otherwise.
    #[inline(always)]
    pub fn is_unknown(&self) -> bool {
        *self == Self::Unknown
    }

    /// Extracts the compression type from an HTTP header.
    ///
    /// This function looks for the `Content-Encoding` header in the provided `Header` and attempts
    /// to parse it into a `Compress` enum value.
    ///
    /// # Arguments
    ///
    /// - `header` - The HTTP header from which the compression type is to be extracted.
    ///
    /// # Returns
    ///
    /// - The `Compress` value corresponding to the `Content-Encoding` header, or `Compress::Unknown`
    ///   if the header does not match any known compression types.
    #[inline(always)]
    pub fn from(header: &HashMap<String, String, BuildHasherDefault<XxHash3_64>>) -> Self {
        header
            .get(CONTENT_ENCODING)
            .map(|value| value.parse::<Compress>().unwrap_or_default())
            .unwrap_or_default()
    }

    /// Decompresses the given data based on the selected compression algorithm.
    ///
    /// This method takes a byte slice of compressed data and decompresses it using one of the following
    /// compression algorithms, depending on the variant of the enum it is called on:
    /// - `Gzip` - Decompresses using Gzip compression.
    /// - `Deflate` - Decompresses using Deflate compression.
    /// - `Br` - Decompresses using Brotli compression.
    /// - `Unknown` - Returns the input data as-is (no decompression performed).
    ///
    /// # Parameters
    ///
    /// - `data` - A reference to a byte slice (`&[u8]`) containing the compressed data to be decoded.
    /// - `buffer_size` - The buffer size to use for the decompression process. A larger buffer size can
    ///   improve performance for larger datasets.
    ///
    /// # Returns
    ///
    /// - `Cow<[u8]>` - The decompressed data as a `Cow<[u8]>`. If the compression algorithm
    ///   is `Unknown`, the original data is returned unchanged, as a borrowed reference. Otherwise,
    ///   the decompressed data is returned as an owned `Vec<u8>`.
    pub fn decode<'a>(&self, data: &'a [u8], buffer_size: usize) -> Cow<'a, [u8]> {
        match self {
            Self::Gzip => gzip::decode(data, buffer_size),
            Self::Deflate => deflate::decode(data, buffer_size),
            Self::Br => brotli::decode(data, buffer_size),
            Self::Unknown => Cow::Owned(data.to_vec()),
        }
    }

    /// Compresses the given data based on the selected compression algorithm.
    ///
    /// This method takes a byte slice of data and compresses it using one of the following
    /// compression algorithms, depending on the variant of the enum it is called on:
    /// - `Gzip` - Compresses using Gzip compression.
    /// - `Deflate` - Compresses using Deflate compression.
    /// - `Br` - Compresses using Brotli compression.
    /// - `Unknown` - Returns the input data as-is (no compression performed).
    ///
    /// # Parameters
    ///
    /// - `data` - A reference to a byte slice (`&[u8]`) containing the data to be compressed.
    /// - `buffer_size` - The buffer size to use for the compression process. A larger buffer size can
    ///   improve performance for larger datasets.
    ///
    /// # Returns
    ///
    /// - `Cow<[u8]>` - The compressed data as a `Cow<[u8]>`. If the compression algorithm
    ///   is `Unknown`, the original data is returned unchanged, as a borrowed reference. Otherwise,
    ///   the compressed data is returned as an owned `Vec<u8>`.
    pub fn encode<'a>(&self, data: &'a [u8], buffer_size: usize) -> Cow<'a, [u8]> {
        match self {
            Self::Gzip => gzip::encode(data, buffer_size),
            Self::Deflate => deflate::encode(data, buffer_size),
            Self::Br => brotli::encode(data),
            Self::Unknown => Cow::Owned(data.to_vec()),
        }
    }
}
