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
    /// - `&str` - The string to parse, which should be a compression algorithm name.
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
    /// Formats the `Compress` value as its `Content-Encoding` header token.
    ///
    /// `Compress::Unknown` formats as the empty string, since it has no
    /// `Content-Encoding` representation.
    ///
    /// # Arguments
    ///
    /// - `&mut fmt::Formatter<'_>` - The formatter to write the value into.
    ///
    /// # Returns
    ///
    /// - `fmt::Result` - The result of the formatting operation.
    #[inline(always)]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
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
    /// - `bool`: `true` if the instance is of type `Unknown`, `false` otherwise.
    #[inline(always)]
    pub fn is_unknown(&self) -> bool {
        *self == Self::Unknown
    }

    /// Renders the coding as its `Content-Encoding` field token.
    ///
    /// # Returns
    ///
    /// - `&'static str` - The token, empty for [`Compress::Unknown`].
    #[inline(always)]
    pub fn as_str(&self) -> &'static str {
        match *self {
            Self::Gzip => CONTENT_ENCODING_GZIP,
            Self::Deflate => CONTENT_ENCODING_DEFLATE,
            Self::Br => CONTENT_ENCODING_BROTLI,
            Self::Unknown => EMPTY_STR,
        }
    }

    /// Picks the coding this crate supports best out of an `Accept-Encoding` field value.
    ///
    /// The field is a comma separated list of codings, and every coding may carry a
    /// `q` parameter that scales how much the client wants it, so `gzip;q=0.5, br`
    /// asks for Brotli before Gzip. A coding the client refuses with `q=0` is never
    /// picked, a coding this crate does not implement is ignored, and the `*` coding
    /// stands for every supported coding the field does not name. Two codings the
    /// client wants equally are resolved in favour of the smaller output, so the
    /// answer is `br`, then `gzip`, then `deflate`, whatever order the field lists
    /// them in.
    ///
    /// The field is parsed without allocating, so a server may negotiate the coding
    /// of every response without a heap cost of its own.
    ///
    /// # Arguments
    ///
    /// - `H` - The field value, which requires the `AsRef<str>` bound.
    ///
    /// # Returns
    ///
    /// - `Option<Self>` - The coding to answer with, or `None` when the client
    ///   accepts none of the codings this crate implements.
    #[inline]
    pub fn from_accept_encoding<H>(header: H) -> Option<Self>
    where
        H: AsRef<str>,
    {
        let mut brotli: Option<f32> = None;
        let mut gzip: Option<f32> = None;
        let mut deflate: Option<f32> = None;
        let mut wildcard: Option<f32> = None;
        for item in header.as_ref().split(ACCEPT_ENCODING_SEPARATOR) {
            let Some((coding, quality)) = Self::split_accept_item(item) else {
                continue;
            };
            if coding == ACCEPT_ENCODING_WILDCARD {
                wildcard = Some(quality);
            } else if coding.eq_ignore_ascii_case(CONTENT_ENCODING_BROTLI) {
                brotli = Some(quality);
            } else if coding.eq_ignore_ascii_case(CONTENT_ENCODING_GZIP) {
                gzip = Some(quality);
            } else if coding.eq_ignore_ascii_case(CONTENT_ENCODING_DEFLATE) {
                deflate = Some(quality);
            }
        }
        let candidates: [(Self, Option<f32>); 3] = [
            (Self::Br, brotli),
            (Self::Gzip, gzip),
            (Self::Deflate, deflate),
        ];
        let mut best: Option<Self> = None;
        let mut best_quality: f32 = 0.0;
        for (coding, quality) in candidates {
            let quality: f32 = quality.or(wildcard).unwrap_or(0.0);
            if quality <= best_quality {
                continue;
            }
            best = Some(coding);
            best_quality = quality;
        }
        best
    }

    /// Splits one entry of an `Accept-Encoding` field value into its coding and quality.
    ///
    /// A quality that is absent counts as `1`, and a quality that is present but
    /// does not parse as a number inside the unit interval refuses the entry,
    /// because a client that cannot say how much it wants a coding cannot be
    /// answered with it.
    ///
    /// # Arguments
    ///
    /// - `&str` - The entry, with or without its parameters.
    ///
    /// # Returns
    ///
    /// - `Option<(&str, f32)>` - The coding and its quality, or `None` when the
    ///   entry names no coding or carries a quality the interval does not hold.
    #[inline]
    fn split_accept_item(item: &str) -> Option<(&str, f32)> {
        let (coding, parameters): (&str, &str) = match item.split_once(PARAMETER_SEPARATOR) {
            Some((coding, parameters)) => (coding, parameters),
            None => (item, EMPTY_STR),
        };
        let coding: &str = coding.trim();
        if coding.is_empty() {
            return None;
        }
        for parameter in parameters.split(PARAMETER_SEPARATOR) {
            let Some((name, value)) = parameter.split_once(PARAMETER_ASSIGN) else {
                continue;
            };
            if !name.trim().eq_ignore_ascii_case(ACCEPT_ENCODING_QUALITY) {
                continue;
            }
            let quality: f32 = value.trim().parse().ok()?;
            if !(0.0..=1.0).contains(&quality) {
                return None;
            }
            return Some((coding, quality));
        }
        Some((coding, DEFAULT_QUALITY))
    }

    /// Extracts the compression type from an HTTP header.
    ///
    /// This function looks for the `Content-Encoding` header in the provided `Header` and attempts
    /// to parse it into a `Compress` enum value.
    ///
    /// # Arguments
    ///
    /// - `&HashMap<String, String, BuildHasherDefault<XxHash3_64>>` - The HTTP header from which
    ///   the compression type is to be extracted.
    ///
    /// # Returns
    ///
    /// - `Self` - The `Compress` value corresponding to the `Content-Encoding` header, or
    ///   `Compress::Unknown` if the header does not match any known compression types.
    #[inline(always)]
    pub fn from(header: &HashMap<String, String, BuildHasherDefault<XxHash3_64>>) -> Self {
        header
            .get(CONTENT_ENCODING)
            .map(|value: &String| value.parse::<Compress>().unwrap_or_default())
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
    /// # Arguments
    ///
    /// - `&'a [u8]` - A reference to a byte slice containing the compressed data to be decoded.
    /// - `usize` - The buffer size to use for the decompression process. A larger buffer size can
    ///   improve performance for larger datasets.
    ///
    /// # Returns
    ///
    /// - `Cow<'a, [u8]>` - The decompressed data. If the compression algorithm
    ///   is `Unknown`, the original data is returned unchanged, as an owned buffer. Otherwise,
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
    /// # Arguments
    ///
    /// - `&'a [u8]` - A reference to a byte slice containing the data to be compressed.
    /// - `usize` - The buffer size to use for the compression process. A larger buffer size can
    ///   improve performance for larger datasets.
    ///
    /// # Returns
    ///
    /// - `Cow<'a, [u8]>` - The compressed data. If the compression algorithm
    ///   is `Unknown`, the original data is returned unchanged, as an owned buffer. Otherwise,
    ///   the compressed data is returned as an owned `Vec<u8>`.
    pub fn encode<'a>(&self, data: &'a [u8], buffer_size: usize) -> Cow<'a, [u8]> {
        match self {
            Self::Gzip => gzip::encode(data, buffer_size),
            Self::Deflate => deflate::encode(data, buffer_size),
            Self::Br => brotli::encode(data, buffer_size),
            Self::Unknown => Cow::Owned(data.to_vec()),
        }
    }
}
