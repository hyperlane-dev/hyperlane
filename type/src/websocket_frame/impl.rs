use super::*;

/// Implements the `Default` trait for `WebSocketFrame`.
///
/// Provides a default `WebSocketFrame` with `fin: false`, `opcode: WebSocketOpcode::Text`,
/// `mask: false`, and an empty `payload_data`.
impl Default for WebSocketFrame {
    /// Returns the default `WebSocketFrame`.
    ///
    /// # Returns
    ///
    /// - `Self` - A default `WebSocketFrame` instance.
    #[inline(always)]
    fn default() -> Self {
        Self {
            fin: false,
            opcode: WebSocketOpcode::Text,
            mask: false,
            payload_data: Vec::new(),
        }
    }
}

impl WebSocketOpcode {
    /// Creates a `WebSocketOpcode` from a raw u8 value.
    ///
    /// # Arguments
    ///
    /// - `u8`: The raw opcode value.
    ///
    /// # Returns
    ///
    /// - `Self` - A `WebSocketOpcode` enum variant corresponding to the raw value.
    #[inline(always)]
    pub fn from_u8(opcode: u8) -> Self {
        match opcode {
            0x0 => Self::Continuation,
            0x1 => Self::Text,
            0x2 => Self::Binary,
            0x8 => Self::Close,
            0x9 => Self::Ping,
            0xA => Self::Pong,
            _ => Self::Reserved(opcode),
        }
    }

    /// Converts the `WebSocketOpcode` to its raw u8 value.
    ///
    /// # Returns
    ///
    /// - `u8` - The raw u8 value of the opcode.
    #[inline(always)]
    pub fn to_u8(&self) -> u8 {
        match self {
            Self::Continuation => 0x0,
            Self::Text => 0x1,
            Self::Binary => 0x2,
            Self::Close => 0x8,
            Self::Ping => 0x9,
            Self::Pong => 0xA,
            Self::Reserved(code) => *code,
        }
    }

    /// Checks if the opcode is a control frame.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the opcode represents a control frame (Close, Ping, Pong), otherwise `false`.
    #[inline(always)]
    pub fn is_control(&self) -> bool {
        matches!(self, Self::Close | Self::Ping | Self::Pong)
    }

    /// Checks if the opcode is a data frame.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the opcode represents a data frame (Text, Binary, Continuation), otherwise `false`.
    #[inline(always)]
    pub fn is_data(&self) -> bool {
        matches!(self, Self::Text | Self::Binary | Self::Continuation)
    }

    /// Checks if the opcode is a continuation frame.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the opcode is `Continuation`, otherwise `false`.
    #[inline(always)]
    pub fn is_continuation(&self) -> bool {
        matches!(self, Self::Continuation)
    }

    /// Checks if the opcode is a text frame.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the opcode is `Text`, otherwise `false`.
    #[inline(always)]
    pub fn is_text(&self) -> bool {
        matches!(self, Self::Text)
    }

    /// Checks if the opcode is a binary frame.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the opcode is `Binary`, otherwise `false`.
    #[inline(always)]
    pub fn is_binary(&self) -> bool {
        matches!(self, Self::Binary)
    }

    /// Checks if the opcode is a close frame.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the opcode is `Close`, otherwise `false`.
    #[inline(always)]
    pub fn is_close(&self) -> bool {
        matches!(self, Self::Close)
    }

    /// Checks if the opcode is a ping frame.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the opcode is `Ping`, otherwise `false`.
    #[inline(always)]
    pub fn is_ping(&self) -> bool {
        matches!(self, Self::Ping)
    }

    /// Checks if the opcode is a pong frame.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the opcode is `Pong`, otherwise `false`.
    #[inline(always)]
    pub fn is_pong(&self) -> bool {
        matches!(self, Self::Pong)
    }

    /// Checks if the opcode is a reserved frame.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the opcode is `Reserved(_)`, otherwise `false`.
    #[inline(always)]
    pub fn is_reserved(&self) -> bool {
        matches!(self, Self::Reserved(_))
    }
}

impl WebSocketFrame {
    /// Decodes a WebSocket frame from the provided data slice.
    ///
    /// This function parses the raw bytes from a WebSocket stream according to the WebSocket protocol
    /// specification to reconstruct a `WebSocketFrame`. It handles FIN bit, opcode, mask bit,
    /// payload length (including extended lengths), mask key, and the payload data itself.
    ///
    /// # Arguments
    ///
    /// - `AsRef<[u8]>` - The raw data to decode into a WebSocket frame.
    ///
    /// # Returns
    ///
    /// - `Option<(WebSocketFrame, usize)>`
    ///     - `Some((WebSocketFrame, usize))`: If the frame is successfully decoded, returns the decoded frame
    ///       and the number of bytes consumed from the input slice.
    ///     - `None`: If the frame is incomplete or malformed.
    pub fn decode_ws_frame<D>(data: D) -> Option<(WebSocketFrame, usize)>
    where
        D: AsRef<[u8]>,
    {
        let data_ref: &[u8] = data.as_ref();
        if data_ref.len() < 2 {
            return None;
        }
        let mut index: usize = 0;
        let fin: bool = (data_ref[index] & 0b1000_0000) != 0;
        let opcode: WebSocketOpcode = WebSocketOpcode::from_u8(data_ref[index] & 0b0000_1111);
        index += 1;
        let mask: bool = (data_ref[index] & 0b1000_0000) != 0;
        let mut payload_len: usize = (data_ref[index] & 0b0111_1111) as usize;
        index += 1;
        if payload_len == 126 {
            if data_ref.len() < index + 2 {
                return None;
            }
            payload_len = u16::from_be_bytes(data_ref[index..index + 2].try_into().ok()?) as usize;
            index += 2;
        } else if payload_len == 127 {
            if data_ref.len() < index + 8 {
                return None;
            }
            payload_len = u64::from_be_bytes(data_ref[index..index + 8].try_into().ok()?) as usize;
            index += 8;
        }
        let mask_key: Option<[u8; 4]> = if mask {
            if data_ref.len() < index + 4 {
                return None;
            }
            let key: [u8; 4] = data_ref[index..index + 4].try_into().ok()?;
            index += 4;
            Some(key)
        } else {
            None
        };
        if data_ref.len() < index + payload_len {
            return None;
        }
        let mut payload: Vec<u8> = data_ref[index..index + payload_len].to_vec();
        if let Some(mask_key) = mask_key {
            for (byte_index, payload_byte) in payload.iter_mut().enumerate() {
                *payload_byte ^= mask_key[byte_index % 4];
            }
        }
        index += payload_len;
        let frame: WebSocketFrame = WebSocketFrame {
            fin,
            opcode,
            mask,
            payload_data: payload,
        };
        Some((frame, index))
    }

    /// Creates a list of response frames from the provided body.
    ///
    /// This method segments the response body into WebSocket frames, respecting the maximum frame size
    /// and handling UTF-8 character boundaries for text frames. It determines the appropriate opcode
    /// (Text or Binary) based on the body's content.
    ///
    /// # Arguments
    ///
    /// - `AsRef<[u8]>` - A reference to a response body (payload) as a byte slice.
    ///
    /// # Returns
    ///
    /// - `Vec<ResponseBody>` - A vector of `ResponseBody` (byte vectors), where each element represents a framed WebSocket message.
    pub fn create_frame_list<D>(data: D) -> Vec<ResponseBody>
    where
        D: AsRef<[u8]>,
    {
        let data_ref: &[u8] = data.as_ref();
        let total_len: usize = data_ref.len();
        let mut offset: usize = 0;
        let mut frames_list: Vec<ResponseBody> =
            Vec::with_capacity((total_len / MAX_FRAME_SIZE) + 1);
        let mut is_first_frame: bool = true;
        let is_valid_utf8: bool = std::str::from_utf8(data_ref).is_ok();
        let base_opcode: WebSocketOpcode = if is_valid_utf8 {
            WebSocketOpcode::Text
        } else {
            WebSocketOpcode::Binary
        };
        while offset < total_len {
            let remaining: usize = total_len - offset;
            let mut frame_size: usize = remaining.min(MAX_FRAME_SIZE);
            if is_valid_utf8 && frame_size < remaining {
                while frame_size > 0 && (data_ref[offset + frame_size] & 0xC0) == 0x80 {
                    frame_size -= 1;
                }
                if frame_size == 0 {
                    frame_size = remaining.min(MAX_FRAME_SIZE);
                }
            }
            let mut frame: ResponseBody = Vec::with_capacity(frame_size + 10);
            let opcode: WebSocketOpcode = if is_first_frame {
                base_opcode
            } else {
                WebSocketOpcode::Continuation
            };
            let fin: u8 = if remaining > frame_size { 0x00 } else { 0x80 };
            let opcode_byte: u8 = opcode.to_u8() & 0x0F;
            frame.push(fin | opcode_byte);
            if frame_size < 126 {
                frame.push(frame_size as u8);
            } else if frame_size <= MAX_FRAME_SIZE {
                frame.push(126);
                frame.extend_from_slice(&(frame_size as u16).to_be_bytes());
            } else {
                frame.push(127);
                frame.extend_from_slice(&(frame_size as u16).to_be_bytes());
            }
            let end: usize = offset + frame_size;
            frame.extend_from_slice(&data_ref[offset..end]);
            frames_list.push(frame);
            offset = end;
            is_first_frame = false;
        }
        frames_list
    }

    /// Calculates the SHA-1 hash of the input data.
    ///
    /// This function implements the SHA-1 cryptographic hash algorithm according to RFC 3174.
    /// It processes the input data in 512-bit (64-byte) blocks and produces a 160-bit (20-byte) hash.
    ///
    /// # Arguments
    ///
    /// - `AsRef<[u8]>` - The input data to be hashed.
    ///
    /// # Returns
    ///
    /// - `[u8; 20]` - A 20-byte array representing the SHA-1 hash of the input data.
    pub fn sha1<D>(data: D) -> [u8; 20]
    where
        D: AsRef<[u8]>,
    {
        let data_ref: &[u8] = data.as_ref();
        let mut hash_state: [u32; 5] = HASH_STATE;
        let mut padded_data: Vec<u8> = Vec::from(data_ref);
        let original_size_bits: u64 = (padded_data.len() * 8) as u64;
        padded_data.push(0x80);
        while !(padded_data.len() + 8).is_multiple_of(64) {
            padded_data.push(0);
        }
        padded_data.extend_from_slice(&original_size_bits.to_be_bytes());
        for block in padded_data.chunks_exact(64) {
            let mut message_schedule: [u32; 80] = [0u32; 80];
            for (chunk_index, block_chunk) in block.chunks_exact(4).enumerate().take(16) {
                message_schedule[chunk_index] = u32::from_be_bytes([
                    block_chunk[0],
                    block_chunk[1],
                    block_chunk[2],
                    block_chunk[3],
                ]);
            }
            for schedule_index in 16..80 {
                message_schedule[schedule_index] = (message_schedule[schedule_index - 3]
                    ^ message_schedule[schedule_index - 8]
                    ^ message_schedule[schedule_index - 14]
                    ^ message_schedule[schedule_index - 16])
                    .rotate_left(1);
            }
            let [mut hash_a, mut hash_b, mut hash_c, mut hash_d, mut hash_e] = hash_state;
            for (round_index, &schedule_word) in message_schedule.iter().enumerate() {
                let (round_function, round_constant): (u32, u32) = match round_index {
                    0..=19 => ((hash_b & hash_c) | (!hash_b & hash_d), 0x5A827999),
                    20..=39 => (hash_b ^ hash_c ^ hash_d, 0x6ED9EBA1),
                    40..=59 => (
                        (hash_b & hash_c) | (hash_b & hash_d) | (hash_c & hash_d),
                        0x8F1BBCDC,
                    ),
                    _ => (hash_b ^ hash_c ^ hash_d, 0xCA62C1D6),
                };
                let temp: u32 = hash_a
                    .rotate_left(5)
                    .wrapping_add(round_function)
                    .wrapping_add(hash_e)
                    .wrapping_add(round_constant)
                    .wrapping_add(schedule_word);
                hash_e = hash_d;
                hash_d = hash_c;
                hash_c = hash_b.rotate_left(30);
                hash_b = hash_a;
                hash_a = temp;
            }
            hash_state[0] = hash_state[0].wrapping_add(hash_a);
            hash_state[1] = hash_state[1].wrapping_add(hash_b);
            hash_state[2] = hash_state[2].wrapping_add(hash_c);
            hash_state[3] = hash_state[3].wrapping_add(hash_d);
            hash_state[4] = hash_state[4].wrapping_add(hash_e);
        }
        let mut result: [u8; 20] = [0u8; 20];
        for (state_index, &state_value) in hash_state.iter().enumerate() {
            result[state_index * 4..(state_index + 1) * 4]
                .copy_from_slice(&state_value.to_be_bytes());
        }
        result
    }

    /// Generates a WebSocket accept key from the client-provided key, returning an `Option<String>`.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The client-provided key (typically from the `Sec-WebSocket-Key` header).
    ///
    /// # Returns
    ///
    /// - `Option<String>` - An optional string representing the generated WebSocket accept key (typically for the `Sec-WebSocket-Accept` header).
    #[inline(always)]
    pub fn try_generate_accept_key<K>(key: K) -> Option<String>
    where
        K: AsRef<str>,
    {
        let key_ref: &str = key.as_ref();
        let mut data: [u8; 60] = [0u8; 60];
        data[..24].copy_from_slice(&key_ref.as_bytes()[..24.min(key_ref.len())]);
        data[24..].copy_from_slice(GUID);
        let hash: [u8; 20] = Self::sha1(data);
        Self::try_base64_encode(hash)
    }

    /// Generates a WebSocket accept key from the client-provided key.
    ///
    /// This function is used during the WebSocket handshake to validate the client's request.
    /// It concatenates the client's key with a specific GUID, calculates the SHA-1 hash of the result,
    /// and then encodes the hash in base64.
    ///
    /// # Arguments
    ///
    /// - `AsRef<str>` - The client-provided key (typically from the `Sec-WebSocket-Key` header).
    ///
    /// # Returns
    ///
    /// - `Option<String>` - An optional string representing the generated WebSocket accept key (typically for the `Sec-WebSocket-Accept` header).
    ///
    /// # Panics
    ///
    /// This function will panic if the input key cannot be converted to a UTF-8 string.
    #[inline(always)]
    pub fn generate_accept_key<K>(key: K) -> String
    where
        K: AsRef<str>,
    {
        let key_ref: &str = key.as_ref();
        let mut data: [u8; 60] = [0u8; 60];
        data[..24].copy_from_slice(&key_ref.as_bytes()[..24.min(key_ref.len())]);
        data[24..].copy_from_slice(GUID);
        let hash: [u8; 20] = Self::sha1(data);
        Self::base64_encode(hash)
    }

    /// Encodes the input data as a base64 string, returning an `Option<String>`.
    ///
    /// # Arguments
    ///
    /// - `AsRef<[u8]>` - The data to encode in base64.
    ///
    /// # Returns
    ///
    /// - `Option<String>` - An optional string with the base64 encoded representation of the input data.
    pub fn try_base64_encode<D>(data: D) -> Option<String>
    where
        D: AsRef<[u8]>,
    {
        let data_ref: &[u8] = data.as_ref();
        let mut encoded_data: Vec<u8> = Vec::with_capacity(data_ref.len().div_ceil(3) * 4);
        for chunk in data_ref.chunks(3) {
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
            while !encoded_data.len().is_multiple_of(4) {
                encoded_data.push(EQUAL_BYTES[0]);
            }
        }
        String::from_utf8(encoded_data).ok()
    }

    /// Encodes the input data as a base64 string.
    ///
    /// # Arguments
    ///
    /// - `AsRef<[u8]>` - The data to encode in base64.
    ///
    /// # Returns
    ///
    /// - `String` - A string with the base64 encoded representation of the input data.
    ///
    /// # Panics
    ///
    /// This function will panic if the input data cannot be converted to a UTF-8 string.
    #[inline(always)]
    pub fn base64_encode<D>(data: D) -> String
    where
        D: AsRef<[u8]>,
    {
        Self::try_base64_encode(data).unwrap()
    }

    /// Checks if the opcode is a continuation frame.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the opcode is `Continuation`, otherwise `false`.
    #[inline(always)]
    pub fn is_continuation_opcode(&self) -> bool {
        self.opcode.is_continuation()
    }

    /// Checks if the opcode is a text frame.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the opcode is `Text`, otherwise `false`.
    #[inline(always)]
    pub fn is_text_opcode(&self) -> bool {
        self.opcode.is_text()
    }

    /// Checks if the opcode is a binary frame.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the opcode is `Binary`, otherwise `false`.
    #[inline(always)]
    pub fn is_binary_opcode(&self) -> bool {
        self.opcode.is_binary()
    }

    /// Checks if the opcode is a close frame.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the opcode is `Close`, otherwise `false`.
    #[inline(always)]
    pub fn is_close_opcode(&self) -> bool {
        self.opcode.is_close()
    }

    /// Checks if the opcode is a ping frame.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the opcode is `Ping`, otherwise `false`.
    #[inline(always)]
    pub fn is_ping_opcode(&self) -> bool {
        self.opcode.is_ping()
    }

    /// Checks if the opcode is a pong frame.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the opcode is `Pong`, otherwise `false`.
    #[inline(always)]
    pub fn is_pong_opcode(&self) -> bool {
        self.opcode.is_pong()
    }

    /// Checks if the opcode is a reserved frame.
    ///
    /// # Returns
    ///
    /// - `bool` - `true` if the opcode is `Reserved(_)`, otherwise `false`.
    #[inline(always)]
    pub fn is_reserved_opcode(&self) -> bool {
        self.opcode.is_reserved()
    }

    /// Handles a decoded WebSocket Text or Binary frame and accumulates payload data.
    ///
    /// # Arguments
    ///
    /// - `&mut Vec<u8>`: The accumulated frame data.
    ///
    /// # Returns
    ///
    /// - `Result<Option<RequestBody>, RequestError>`: Some(request) if frame is complete, None to continue, or error.
    #[inline(always)]
    pub(crate) fn build_full_frame(
        &self,
        full_frame: &mut Vec<u8>,
    ) -> Result<Option<RequestBody>, RequestError> {
        let payload_data: &[u8] = self.get_payload_data();
        full_frame.extend_from_slice(payload_data);
        if *self.get_fin() {
            return Ok(Some(full_frame.clone()));
        }
        Ok(None)
    }
}
