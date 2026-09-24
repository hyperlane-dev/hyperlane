use super::*;

/// Compresses the given data using Gzip compression.
///
/// This function takes a byte slice of data and compresses it using the Gzip compression algorithm.
/// If the compression succeeds, the resulting compressed data is returned as a `Cow<Vec<u8>>`.
/// If any error occurs during the compression process, an empty `Vec<u8>` is returned.
///
/// # Parameters
/// - `data` - A reference to a byte slice (`&[u8]`) containing the data to be compressed.
///
/// # Returns
/// - `Cow<[u8]>` - The compressed data as a `Cow<[u8]>`. The compressed data is returned as an
///   owned `Vec<u8>`. If compression fails, an empty owned `Vec<u8>` is returned.
pub fn encode(data: &'_ [u8]) -> Cow<'_, [u8]> {
    let mut encoder: GzEncoder<Vec<u8>> = GzEncoder::new(Vec::new(), Compression::default());
    if encoder.write_all(data).is_err() {
        return Cow::Owned(Vec::new());
    }
    Cow::Owned(encoder.finish().unwrap_or_else(|_| Vec::new()))
}

/// Decompresses the given data using the specified decompressor.
///
/// This function takes a byte slice of compressed data and decompresses it using
/// a decompressor, returning the result as a `Cow<Vec<u8>>`. If decompression is successful,
/// the decompressed data is returned as an owned `Vec<u8>`. In case of an error, an empty
/// `Vec<u8>` is returned.
///
/// # Parameters
/// - `data` - A reference to a byte slice (`&[u8]`) containing the compressed data to be decoded.
/// - `buffer_size` - The buffer size to use for the decompression process. A larger buffer size can
///   improve performance for larger datasets.
///
/// # Returns
/// - `Cow<[u8]>` - The decompressed data as a `Cow<[u8]>`. If decompression is successful, the
///   decompressed data is returned as an owned `Vec<u8>`. In case of an error, an empty owned
///   `Vec<u8>` is returned.
pub fn decode(data: &'_ [u8], buffer_size: usize) -> Cow<'_, [u8]> {
    let mut decompressor: Decompressor<&[u8]> = Decompressor::new(data, buffer_size);
    let mut decompressed_data: Vec<u8> = Vec::new();
    match decompressor.read_to_end(&mut decompressed_data) {
        Ok(_) => Cow::Owned(decompressed_data),
        _ => Cow::Owned(Vec::new()),
    }
}
