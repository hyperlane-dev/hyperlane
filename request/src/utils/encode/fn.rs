use super::*;

/// Encodes binary data into a base64 string.
///
/// # Arguments
///
/// - `&[u8]` - The binary data to encode.
///
/// # Returns
///
/// - `String` - The base64 encoded string.
pub(crate) fn base64_encode(input: &[u8]) -> String {
    let mut result: String = String::new();
    for chunk in input.chunks(3) {
        let mut buf: [u8; 3] = [0u8; 3];
        for (i, &byte) in chunk.iter().enumerate() {
            buf[i] = byte;
        }
        let bits_value: u32 = ((buf[0] as u32) << 16) | ((buf[1] as u32) << 8) | (buf[2] as u32);
        result.push(CHARS[((bits_value >> 18) & 63) as usize] as char);
        result.push(CHARS[((bits_value >> 12) & 63) as usize] as char);
        result.push(if chunk.len() > 1 {
            CHARS[((bits_value >> 6) & 63) as usize] as char
        } else {
            '='
        });
        result.push(if chunk.len() > 2 {
            CHARS[(bits_value & 63) as usize] as char
        } else {
            '='
        });
    }
    result
}
