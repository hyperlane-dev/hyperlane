use super::*;

/// Compresses the given data using Gzip compression.
///
/// This function takes a byte slice of data and compresses it using the `GzEncoder` from the `flate2` crate.
/// The compression is done in a buffered manner to optimize performance. If compression is successful,
/// the result is returned as an owned `Vec<u8>`. If an error occurs during compression, an empty `Vec<u8>`
/// is returned.
///
/// # Arguments
///
/// - `&'_ [u8]` - A reference to a byte slice containing the data to be compressed.
/// - `usize` - The buffer size to use for the buffered writer. A larger buffer size can improve
///   performance for larger datasets.
///
/// # Returns
///
/// - `Cow<'_, [u8]>` - The compressed data. If compression is successful, the
///   compressed data is returned as an owned `Vec<u8>`. If an error occurs, an empty owned `Vec<u8>`
///   is returned.
pub fn encode(data: &'_ [u8], buffer_size: usize) -> Cow<'_, [u8]> {
    let initial_capacity: usize = data.len().min(buffer_size);
    let mut encoder: GzEncoder<Vec<u8>> =
        GzEncoder::new(Vec::with_capacity(initial_capacity), Compression::default());
    if encoder.write_all(data).is_err() {
        return Cow::Owned(Vec::new());
    }
    Cow::Owned(encoder.finish().unwrap_or_else(|_: io::Error| Vec::new()))
}

/// Decompresses the given Gzip-compressed data.
///
/// This function takes a byte slice of Gzip-compressed data and decompresses it using the
/// `MultiGzDecoder` from the `flate2` crate, supporting both single-member and concatenated
/// multi-member gzip streams. If decompression is successful, the result is returned as an
/// owned `Vec<u8>`. If an error occurs during decompression, an empty `Vec<u8>` is returned.
///
/// # Arguments
///
/// - `&'_ [u8]` - A reference to a byte slice containing the Gzip-compressed data.
/// - `usize` - A capacity hint for the output buffer.
///
/// # Returns
///
/// - `Cow<'_, [u8]>` - The decompressed data. If decompression is successful, the
///   decompressed data is returned as an owned `Vec<u8>`. If an error occurs, an empty owned `Vec<u8>`
///   is returned.
pub fn decode(data: &'_ [u8], buffer_size: usize) -> Cow<'_, [u8]> {
    let decoder: MultiGzDecoder<&[u8]> = MultiGzDecoder::new(data);
    let mut limited: io::Take<MultiGzDecoder<&[u8]>> =
        decoder.take((MAX_DECOMPRESSED_BYTES + 1) as u64);
    let initial_capacity: usize = data.len().min(buffer_size);
    let mut decompressed_data: Vec<u8> = Vec::with_capacity(initial_capacity);
    match limited.read_to_end(&mut decompressed_data) {
        Ok(_) if decompressed_data.len() <= MAX_DECOMPRESSED_BYTES => Cow::Owned(decompressed_data),
        _ => Cow::Owned(Vec::new()),
    }
}
