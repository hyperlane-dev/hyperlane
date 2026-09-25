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
/// - `data` - A reference to a byte slice (`&[u8]`) containing the data to be compressed.
/// - `buffer_size` - The buffer size to use for the buffered writer. A larger buffer size can
///   improve performance for larger datasets.
///
/// # Returns
///
/// - `Cow<[u8]>` - The compressed data as a `Cow<[u8]>`. If compression is successful, the
///   compressed data is returned as an owned `Vec<u8>`. If an error occurs, an empty owned `Vec<u8>`
///   is returned.
pub fn encode(data: &'_ [u8], buffer_size: usize) -> Cow<'_, [u8]> {
    let encoder: DeflateEncoder<Vec<u8>> = DeflateEncoder::new(Vec::new(), Compression::default());
    let mut buffered_writer: BufWriter<DeflateEncoder<Vec<u8>>> =
        BufWriter::with_capacity(buffer_size, encoder);
    if buffered_writer.write_all(data).is_err() {
        return Cow::Owned(Vec::new());
    }
    match buffered_writer.into_inner() {
        Ok(encoder) => Cow::Owned(encoder.finish().unwrap_or_else(|_| Vec::new())),
        Err(_) => Cow::Owned(Vec::new()),
    }
}

/// Decompresses Deflate compressed data.
///
/// # Arguments
///
/// - `&[u8]` - The compressed data to decode.
/// - `usize` - The buffer size for decompression.
///
/// # Returns
///
/// - `Cow<[u8]>` - The decompressed data.
pub fn decode(data: &'_ [u8], buffer_size: usize) -> Cow<'_, [u8]> {
    let decoder: DeflateDecoder<&[u8]> = DeflateDecoder::new(data);
    let mut buffered_reader: BufReader<DeflateDecoder<&[u8]>> =
        BufReader::with_capacity(buffer_size, decoder);
    let mut decompressed_data: Vec<u8> = Vec::new();
    match buffered_reader.read_to_end(&mut decompressed_data) {
        Ok(_) => Cow::Owned(decompressed_data),
        _ => Cow::Owned(Vec::new()),
    }
}
