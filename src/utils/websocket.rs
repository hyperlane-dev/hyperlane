use crate::*;

#[inline]
pub(crate) fn create_response_frame(body: &mut ResponseBody) {
    let original_len: usize = body.len();
    body.insert(0, 0x81);
    body.insert(1, original_len as u8);
}

#[inline]
pub(crate) fn sha1(data: &[u8]) -> [u8; 20] {
    let mut hash_state: [u32; 5] = HASH_STATE;
    let mut padded_data: Vec<u8> = Vec::from(data);
    let original_length_bits: u64 = (padded_data.len() * 8) as u64;
    padded_data.push(0x80);
    while (padded_data.len() + 8) % 64 != 0 {
        padded_data.push(0);
    }
    padded_data.extend_from_slice(&original_length_bits.to_be_bytes());
    for block in padded_data.chunks_exact(64) {
        let mut message_schedule: [u32; 80] = [0u32; 80];
        for (i, block_chunk) in block.chunks_exact(4).enumerate().take(16) {
            message_schedule[i] = u32::from_be_bytes([
                block_chunk[0],
                block_chunk[1],
                block_chunk[2],
                block_chunk[3],
            ]);
        }
        for i in 16..80 {
            message_schedule[i] = (message_schedule[i - 3]
                ^ message_schedule[i - 8]
                ^ message_schedule[i - 14]
                ^ message_schedule[i - 16])
                .rotate_left(1);
        }
        let [mut a, mut b, mut c, mut d, mut e] = hash_state;
        for (i, &word) in message_schedule.iter().enumerate() {
            let (f, k) = match i {
                0..=19 => ((b & c) | (!b & d), 0x5A827999),
                20..=39 => (b ^ c ^ d, 0x6ED9EBA1),
                40..=59 => ((b & c) | (b & d) | (c & d), 0x8F1BBCDC),
                _ => (b ^ c ^ d, 0xCA62C1D6),
            };
            let temp: u32 = a
                .rotate_left(5)
                .wrapping_add(f)
                .wrapping_add(e)
                .wrapping_add(k)
                .wrapping_add(word);
            e = d;
            d = c;
            c = b.rotate_left(30);
            b = a;
            a = temp;
        }
        hash_state[0] = hash_state[0].wrapping_add(a);
        hash_state[1] = hash_state[1].wrapping_add(b);
        hash_state[2] = hash_state[2].wrapping_add(c);
        hash_state[3] = hash_state[3].wrapping_add(d);
        hash_state[4] = hash_state[4].wrapping_add(e);
    }
    let mut result: [u8; 20] = [0u8; 20];
    for (i, &val) in hash_state.iter().enumerate() {
        result[i * 4..(i + 1) * 4].copy_from_slice(&val.to_be_bytes());
    }
    result
}

#[inline]
pub(crate) fn generate_accept_key(key: &str) -> String {
    let mut data: [u8; 60] = [0u8; 60];
    data[..24].copy_from_slice(&key.as_bytes()[..24.min(key.len())]);
    data[24..].copy_from_slice(GUID);
    let hash: [u8; 20] = sha1(&data);
    base64_encode(&hash)
}

#[inline]
pub(crate) fn base64_encode(data: &[u8]) -> String {
    let mut encoded_data: Vec<u8> = Vec::with_capacity((data.len() + 2) / 3 * 4);
    for chunk in data.chunks(3) {
        let mut buffer: [u8; 3] = [0u8; 3];
        buffer[..chunk.len()].copy_from_slice(chunk);
        let indices: [u8; 4] = [
            buffer[0] >> 2,
            ((buffer[0] & 0b11) << 4) | (buffer[1] >> 4),
            ((buffer[1] & 0b1111) << 2) | (buffer[2] >> 6),
            buffer[2] & 0b111111,
        ];
        for &idx in &indices[..chunk.len() + 1] {
            encoded_data.push(BASE64_CHARSET_TABLE[idx as usize]);
        }
        while encoded_data.len() % 4 != 0 {
            encoded_data.push(EQUAL_BYTES[0]);
        }
    }
    String::from_utf8(encoded_data).unwrap()
}

#[inline]
pub(crate) fn decode_websocket_frame(frame: &mut RequestBody) {
    if frame.len() < 2 {
        return;
    }
    let payload_len: usize = (frame[1] & 0x7F) as usize;
    if frame.len() < 2 + 4 + payload_len {
        return;
    }
    let mask_key: &[u8] = &frame[2..6];
    let payload: &[u8] = &frame[6..6 + payload_len];
    let decoded_body: RequestBody = payload
        .iter()
        .enumerate()
        .map(|(i, &byte)| byte ^ mask_key[i % 4])
        .collect();
    *frame = decoded_body;
}
