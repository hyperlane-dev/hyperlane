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
/// - `data` - A reference to a byte slice (`&[u8]`) containing the data to be compressed.
/// - `buffer_size` - The buffer size to use for the buffered writer. A larger buffer size can improve
///   performance for larger datasets.
///
/// # Returns
///
/// - `Cow<[u8]>` - The compressed data as a `Cow<[u8]>`. If compression is successful, the
///   compressed data is returned as an owned `Vec<u8>`. If an error occurs, an empty owned `Vec<u8>`
///   is returned.
pub fn encode(data: &'_ [u8], buffer_size: usize) -> Cow<'_, [u8]> {
    let encoder: GzEncoder<Vec<u8>> = GzEncoder::new(Vec::new(), Compression::default());
    let mut buffered_writer: BufWriter<GzEncoder<Vec<u8>>> =
        BufWriter::with_capacity(buffer_size, encoder);
    if buffered_writer.write_all(data).is_err() {
        return Cow::Owned(Vec::new());
    }
    match buffered_writer.into_inner() {
        Ok(encoder) => Cow::Owned(encoder.finish().unwrap_or_else(|_| Vec::new())),
        Err(_) => Cow::Owned(Vec::new()),
    }
}

/// Decompresses the given Gzip-compressed data.
///
/// This function takes a byte slice of Gzip-compressed data and decompresses it using the `GzDecoder`
/// from the `flate2` crate. The decompression is done in a buffered manner to optimize performance.
/// If decompression is successful, the result is returned as an owned `Vec<u8>`. If an error occurs
/// during decompression, an empty `Vec<u8>` is returned.
///
/// # Arguments
///
/// - `data` - A reference to a byte slice (`&[u8]`) containing the Gzip-compressed data.
/// - `buffer_size` - The buffer size to use for the buffered reader. A larger buffer size can improve
///   performance for larger datasets.
///
/// # Returns
///
/// - `Cow<[u8]>` - The decompressed data as a `Cow<[u8]>`. If decompression is successful, the
///   decompressed data is returned as an owned `Vec<u8>`. If an error occurs, an empty owned `Vec<u8>`
///   is returned.
pub fn decode(data: &'_ [u8], buffer_size: usize) -> Cow<'_, [u8]> {
    let decoder: GzDecoder<&[u8]> = GzDecoder::new(data);
    let mut buffered_reader: BufReader<GzDecoder<&[u8]>> =
        BufReader::with_capacity(buffer_size, decoder);
    let mut decompressed_data: Vec<u8> = Vec::new();
    match buffered_reader.read_to_end(&mut decompressed_data) {
        Ok(_) => Cow::Owned(decompressed_data),
        _ => Cow::Owned(Vec::new()),
    }
}
