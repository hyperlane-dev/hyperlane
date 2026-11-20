use super::*;

/// Lazily constructed 8-bit prefix lookup table for RFC 7541 Huffman decoding.
pub(crate) static HUFFMAN_DECODE_TABLES: OnceLock<Vec<[u16; 256]>> = OnceLock::new();
