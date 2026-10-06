use super::*;

/// Compresses the given data using the Brotli compression algorithm.
///
/// The bytes are written through a `CompressorWriter` from the `brotli` crate, so the
/// produced output is a real Brotli stream that any `Content-Encoding: br` consumer
/// can decode. An empty owned `Vec<u8>` is returned when compression fails.
///
/// # Arguments
///
/// - `&'_ [u8]` - the bytes to compress.
/// - `usize` - the buffer size used by the compressor.
///
/// # Returns
///
/// - `Cow<'_, [u8]>`: the compressed bytes, or an empty owned `Vec<u8>` on failure.
pub fn encode(data: &'_ [u8], buffer_size: usize) -> Cow<'_, [u8]> {
    let mut encoder: CompressorWriter<Vec<u8>> = CompressorWriter::new(
        Vec::new(),
        buffer_size,
        BROTLI_DEFAULT_QUALITY,
        BROTLI_DEFAULT_WINDOW_BITS,
    );
    if encoder.write_all(data).is_err() {
        return Cow::Owned(Vec::new());
    }
    Cow::Owned(encoder.into_inner())
}

/// Decompresses Brotli-compressed data.
///
/// Decoding uses the `Decompressor` from the `brotli` crate. An empty owned `Vec<u8>`
/// is returned when the input is not a valid Brotli stream.
///
/// # Arguments
///
/// - `&'_ [u8]` - the Brotli-compressed bytes to decode.
/// - `usize` - the buffer size used by the decompressor.
///
/// # Returns
///
/// - `Cow<'_, [u8]>`: the decompressed bytes, or an empty owned `Vec<u8>` on failure.
pub fn decode(data: &'_ [u8], buffer_size: usize) -> Cow<'_, [u8]> {
    let mut decompressor: Decompressor<&[u8]> = Decompressor::new(data, buffer_size);
    let mut decompressed_data: Vec<u8> = Vec::new();
    match decompressor.read_to_end(&mut decompressed_data) {
        Ok(_) => Cow::Owned(decompressed_data),
        _ => Cow::Owned(Vec::new()),
    }
}
