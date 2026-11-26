use super::*;

/// Compresses the given data using the Deflate compression algorithm.
///
/// This function takes a byte slice of data and compresses it using the Deflate compression algorithm.
/// The data is written to a buffered writer to optimize performance. The compressed data is returned
/// as a `Cow<Vec<u8>>`. If compression succeeds, the resulting compressed data is returned as an
/// owned `Vec<u8>`. If any error occurs during the compression process, an empty `Vec<u8>` is returned.
///
/// # Arguments
///
/// - `&'_ [u8]` - A reference to a byte slice containing the data to be compressed.
/// - `usize` - The buffer size to use for the buffered writer. A larger buffer size can
///   improve performance for larger datasets.
///
/// # Returns
///
/// - `Cow<'_, [u8]>` - The compressed data. If compression is successful, the
///   compressed data is returned as an owned `Vec<u8>`. If an error occurs, an empty owned `Vec<u8>`
///   is returned.
pub fn encode(data: &'_ [u8], buffer_size: usize) -> Cow<'_, [u8]> {
    let initial_capacity: usize = data.len().min(buffer_size);
    let mut encoder: DeflateEncoder<Vec<u8>> =
        DeflateEncoder::new(Vec::with_capacity(initial_capacity), Compression::default());
    if encoder.write_all(data).is_err() {
        return Cow::Owned(Vec::new());
    }
    Cow::Owned(encoder.finish().unwrap_or_else(|_: io::Error| Vec::new()))
}

/// Decompresses Deflate compressed data.
///
/// # Arguments
///
/// - `&'_ [u8]` - A reference to a byte slice containing the compressed data to decode.
/// - `usize` - The capacity hint for decompression.
///
/// # Returns
///
/// - `Cow<'_, [u8]>` - The decompressed data.
pub fn decode(data: &'_ [u8], buffer_size: usize) -> Cow<'_, [u8]> {
    let decoder: DeflateDecoder<&[u8]> = DeflateDecoder::new(data);
    let mut limited: io::Take<DeflateDecoder<&[u8]>> =
        decoder.take((MAX_DECOMPRESSED_BYTES + 1) as u64);
    let initial_capacity: usize = data.len().min(buffer_size);
    let mut decompressed_data: Vec<u8> = Vec::with_capacity(initial_capacity);
    match limited.read_to_end(&mut decompressed_data) {
        Ok(_) if decompressed_data.len() <= MAX_DECOMPRESSED_BYTES => Cow::Owned(decompressed_data),
        _ => Cow::Owned(Vec::new()),
    }
}
