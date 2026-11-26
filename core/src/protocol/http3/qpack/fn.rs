use super::*;

/// Returns the lazily initialized 8-bit RFC 7541 Huffman decode tables.
///
/// # Returns
///
/// - `&'static [[u16; 256]]` - The flat table slice rooted at index `0`.
fn huffman_tables() -> &'static [[u16; 256]] {
    HUFFMAN_DECODE_TABLES
        .get_or_init(build_huffman_tables)
        .as_slice()
}

/// Builds the 8-bit prefix lookup tables from the RFC 7541 Appendix B codes.
///
/// # Returns
///
/// - `Vec<[u16; 256]>` - The root and child lookup tables.
fn build_huffman_tables() -> Vec<[u16; 256]> {
    let mut nodes: Vec<HuffmanTrieNode> = vec![HuffmanTrieNode {
        left: u16::MAX,
        right: u16::MAX,
        sym: u16::MAX,
    }];
    for sym in 0..256usize {
        let code: u32 = HUFFMAN_CODES[sym];
        let bits: u8 = HUFFMAN_LENGTHS[sym];
        let mut cur: usize = 0;
        for shift in (0..bits).rev() {
            let bit: u32 = (code >> shift) & 1;
            let next: u16 = if bit == 0 {
                nodes[cur].left
            } else {
                nodes[cur].right
            };
            if next == u16::MAX {
                let idx: u16 = nodes.len() as u16;
                nodes.push(HuffmanTrieNode {
                    left: u16::MAX,
                    right: u16::MAX,
                    sym: u16::MAX,
                });
                if bit == 0 {
                    nodes[cur].left = idx;
                } else {
                    nodes[cur].right = idx;
                }
                cur = idx as usize;
            } else {
                cur = next as usize;
            }
        }
        nodes[cur].sym = sym as u16;
    }
    let mut tables: Vec<[u16; 256]> = Vec::new();
    let mut node_to_table: HashMap<usize, u16> = HashMap::new();
    let mut queue: VecDeque<usize> = VecDeque::new();
    node_to_table.insert(0, 0);
    tables.push([HUFFMAN_INVALID; 256]);
    queue.push_back(0);
    while let Some(root_node) = queue.pop_front() {
        let table_idx: usize = node_to_table[&root_node] as usize;
        for byte_val in 0..256usize {
            let mut cur: usize = root_node;
            let mut entry: u16 = HUFFMAN_INVALID;
            let mut reached_leaf: bool = false;
            for step in 0..8u16 {
                let bit: usize = (byte_val >> (7 - step)) & 1;
                let next: u16 = if bit == 0 {
                    nodes[cur].left
                } else {
                    nodes[cur].right
                };
                if next == u16::MAX {
                    entry = HUFFMAN_INVALID;
                    reached_leaf = true;
                    break;
                }
                cur = next as usize;
                if nodes[cur].sym != u16::MAX {
                    entry = ((step + 1) << 8) | nodes[cur].sym;
                    reached_leaf = true;
                    break;
                }
            }
            if !reached_leaf {
                let child_table: u16 = match node_to_table.get(&cur) {
                    Some(&existing) => existing,
                    None => {
                        let next_idx: u16 = tables.len() as u16;
                        node_to_table.insert(cur, next_idx);
                        tables.push([HUFFMAN_INVALID; 256]);
                        queue.push_back(cur);
                        next_idx
                    }
                };
                entry = HUFFMAN_BRANCH_FLAG | (child_table << 8);
            }
            tables[table_idx][byte_val] = entry;
        }
    }
    tables
}

/// Decodes an RFC 7541 Huffman-encoded byte slice into `out`.
///
/// # Arguments
///
/// - `&[u8]` - The Huffman-encoded bytes.
/// - `&mut Vec<u8>` - Destination buffer cleared and filled with decoded bytes.
///
/// # Returns
///
/// - `Result<(), RequestError>` - `Ok(())` on valid Huffman input, or bad request.
pub(super) fn decode_huffman(input: &[u8], out: &mut Vec<u8>) -> Result<(), RequestError> {
    out.clear();
    if input.is_empty() {
        return Ok(());
    }
    let tables: &[[u16; 256]] = huffman_tables();
    out.reserve((input.len() * 8) / 5 + 1);
    let mut bits: u64 = 0;
    let mut avail: u32 = 0;
    let mut pos: usize = 0;
    let mut table_idx: usize = 0;
    loop {
        while avail <= 56 && pos < input.len() {
            bits |= (input[pos] as u64) << (56 - avail);
            avail += 8;
            pos += 1;
        }
        if avail == 0 {
            break;
        }
        let lookup: usize = (bits >> 56) as usize;
        let entry: u16 = tables[table_idx][lookup];
        if entry == HUFFMAN_INVALID {
            return Err(RequestError::HttpRead(HttpStatus::BadRequest));
        }
        if (entry & HUFFMAN_BRANCH_FLAG) == 0 {
            let consumed: u32 = (entry >> 8) as u32;
            if consumed > avail {
                if table_idx != 0 || avail > 7 {
                    return Err(RequestError::HttpRead(HttpStatus::BadRequest));
                }
                let pad_mask: u64 = !0u64 << (64 - avail);
                if (bits & pad_mask) != pad_mask {
                    return Err(RequestError::HttpRead(HttpStatus::BadRequest));
                }
                return Ok(());
            }
            out.push(entry as u8);
            bits <<= consumed;
            avail -= consumed;
            table_idx = 0;
        } else {
            if avail < 8 {
                if table_idx != 0 || avail > 7 {
                    return Err(RequestError::HttpRead(HttpStatus::BadRequest));
                }
                let pad_mask: u64 = !0u64 << (64 - avail);
                if (bits & pad_mask) != pad_mask {
                    return Err(RequestError::HttpRead(HttpStatus::BadRequest));
                }
                return Ok(());
            }
            table_idx = ((entry & 0x7F00) >> 8) as usize;
            bits <<= 8;
            avail -= 8;
        }
    }
    if table_idx != 0 {
        return Err(RequestError::HttpRead(HttpStatus::BadRequest));
    }
    Ok(())
}

/// Decodes a QUIC variable-length integer (RFC 9000 §16).
///
/// # Arguments
///
/// - `&[u8]` - The input slice.
///
/// # Returns
///
/// - `Option<(u64, usize)>` - The decoded value and bytes consumed, or `None` if incomplete.
#[inline(always)]
pub(crate) fn decode_varint(buf: &[u8]) -> Option<(u64, usize)> {
    let first: u8 = *buf.first()?;
    let tag: u8 = first >> 6;
    let len: usize = 1usize << tag;
    if buf.len() < len {
        return None;
    }
    let val: u64 = match len {
        1 => u64::from(first & 0x3F),
        2 => u64::from(u16::from_be_bytes([first & 0x3F, buf[1]])),
        4 => u64::from(u32::from_be_bytes([first & 0x3F, buf[1], buf[2], buf[3]])),
        _ => u64::from_be_bytes([
            first & 0x3F,
            buf[1],
            buf[2],
            buf[3],
            buf[4],
            buf[5],
            buf[6],
            buf[7],
        ]),
    };
    Some((val, len))
}

/// Encodes a QUIC variable-length integer (RFC 9000 §16) into `out`.
///
/// # Arguments
///
/// - `u64` - The integer value (must be `< 2^62`).
/// - `&mut Vec<u8>` - The destination byte buffer.
#[inline(always)]
pub(crate) fn encode_varint(val: u64, out: &mut Vec<u8>) {
    if val < 64 {
        out.push(val as u8);
    } else if val < 16_384 {
        let bytes: [u8; 2] = ((val as u16) | 0x4000).to_be_bytes();
        out.extend_from_slice(&bytes);
    } else if val < 1_073_741_824 {
        let bytes: [u8; 4] = ((val as u32) | 0x8000_0000).to_be_bytes();
        out.extend_from_slice(&bytes);
    } else {
        let bytes: [u8; 8] = (val | 0xC000_0000_0000_0000).to_be_bytes();
        out.extend_from_slice(&bytes);
    }
}

/// Decodes a QPACK/HPACK prefix integer (RFC 7541 §5.1).
///
/// # Arguments
///
/// - `&[u8]` - The input slice starting at the prefix byte.
/// - `u8` - Number of prefix bits (`1..=8`).
///
/// # Returns
///
/// - `Result<(u64, usize), RequestError>` - The decoded integer and bytes consumed.
#[inline(always)]
fn decode_prefix_int(buf: &[u8], prefix_bits: u8) -> Result<(u64, usize), RequestError> {
    let first: u8 = *buf
        .first()
        .ok_or(RequestError::HttpRead(HttpStatus::BadRequest))?;
    let max_prefix: u8 = if prefix_bits == 8 {
        u8::MAX
    } else {
        (1u8 << prefix_bits) - 1
    };
    let initial: u8 = first & max_prefix;
    if initial < max_prefix {
        return Ok((u64::from(initial), 1));
    }
    let mut value: u64 = u64::from(max_prefix);
    let mut shift: u32 = 0;
    let mut idx: usize = 1;
    loop {
        let byte: u8 = *buf
            .get(idx)
            .ok_or(RequestError::HttpRead(HttpStatus::BadRequest))?;
        idx += 1;
        if shift >= 63 {
            return Err(RequestError::HttpRead(HttpStatus::BadRequest));
        }
        value = value
            .checked_add(u64::from(byte & 0x7F) << shift)
            .ok_or(RequestError::HttpRead(HttpStatus::BadRequest))?;
        if (byte & 0x80) == 0 {
            return Ok((value, idx));
        }
        shift += 7;
    }
}

/// Encodes a QPACK prefix integer into `out`.
///
/// # Arguments
///
/// - `u8` - High flag bits to OR into the first byte.
/// - `u8` - Number of prefix bits (`1..=8`).
/// - `u64` - The integer value to encode.
/// - `&mut Vec<u8>` - The destination buffer.
#[inline(always)]
fn encode_prefix_int(flags: u8, prefix_bits: u8, mut val: u64, out: &mut Vec<u8>) {
    let max_prefix: u64 = if prefix_bits == 8 {
        255
    } else {
        (1u64 << prefix_bits) - 1
    };
    if val < max_prefix {
        out.push(flags | (val as u8));
        return;
    }
    out.push(flags | (max_prefix as u8));
    val -= max_prefix;
    while val >= 128 {
        out.push(((val as u8) & 0x7F) | 0x80);
        val >>= 7;
    }
    out.push(val as u8);
}

/// Decodes a QPACK string literal (raw or Huffman) into a slice or lazily-borrowed scratch buffer.
///
/// # Arguments
///
/// - `&'a [u8]` - Input slice starting at the string length prefix byte.
/// - `u8` - Number of length prefix bits (`7` for 8-bit string, `3` for literal name).
/// - `&mut Option<Vec<u8>>` - Lazy scratch buffer taken from the pool only when Huffman-encoded.
///
/// # Returns
///
/// - `Result<(&'a [u8], bool, usize), RequestError>` - `(raw_slice_when_uncompressed, used_scratch, consumed_bytes)`.
#[inline(always)]
fn decode_qpack_string<'a>(
    buf: &'a [u8],
    prefix_bits: u8,
    scratch: &mut Option<Vec<u8>>,
) -> Result<(&'a [u8], bool, usize), RequestError> {
    let first: u8 = *buf
        .first()
        .ok_or(RequestError::HttpRead(HttpStatus::BadRequest))?;
    let huffman_mask: u8 = 1u8 << prefix_bits;
    let is_huffman: bool = (first & huffman_mask) != 0;
    let (len_u64, int_bytes): (u64, usize) = decode_prefix_int(buf, prefix_bits)?;
    let len: usize =
        usize::try_from(len_u64).map_err(|_| RequestError::HttpRead(HttpStatus::BadRequest))?;
    let end: usize = int_bytes
        .checked_add(len)
        .ok_or(RequestError::HttpRead(HttpStatus::BadRequest))?;
    let raw: &'a [u8] = buf
        .get(int_bytes..end)
        .ok_or(RequestError::HttpRead(HttpStatus::BadRequest))?;
    if is_huffman {
        let scratch_buf: &mut Vec<u8> = scratch.get_or_insert_with(take_h3_buffer);
        decode_huffman(raw, scratch_buf)?;
        Ok((&[], true, end))
    } else {
        Ok((raw, false, end))
    }
}

/// Validates a decoded header name token and rejects uppercase bytes (RFC 9114 §4.2).
///
/// # Arguments
///
/// - `&[u8]` - The field name bytes.
///
/// # Returns
///
/// - `Result<&str, RequestError>` - The validated lowercase field name.
#[inline(always)]
fn validate_h3_header_name(bytes: &[u8]) -> Result<&str, RequestError> {
    if bytes.is_empty() {
        return Err(RequestError::HttpRead(HttpStatus::BadRequest));
    }
    for &byte in bytes {
        let valid: bool = byte.is_ascii_lowercase()
            || byte.is_ascii_digit()
            || H3_LOWER_TOKEN_BYTES.contains(&byte);
        if !valid {
            return Err(RequestError::HttpRead(HttpStatus::BadRequest));
        }
    }
    from_utf8(bytes).map_err(|_| RequestError::HttpRead(HttpStatus::BadRequest))
}

/// Validates a decoded header value (no NUL/CR/LF/control bytes except HTAB).
///
/// # Arguments
///
/// - `&[u8]` - The field value bytes.
///
/// # Returns
///
/// - `Result<&str, RequestError>` - The validated UTF-8 header value.
#[inline(always)]
fn validate_h3_header_value(bytes: &[u8]) -> Result<&str, RequestError> {
    for &byte in bytes {
        if (byte < 0x20 && byte != b'\t') || byte == 0x7f {
            return Err(RequestError::HttpRead(HttpStatus::BadRequest));
        }
    }
    from_utf8(bytes).map_err(|_| RequestError::HttpRead(HttpStatus::BadRequest))
}

/// Checks whether a header value carries the `100-continue` expectation token.
///
/// # Arguments
///
/// - `&str` - The `expect` header value.
///
/// # Returns
///
/// - `bool` - `true` when the value names `100-continue`.
fn value_has_continue_expectation(value: &str) -> bool {
    contains_continue_member(value.as_bytes())
}

/// Inserts a decoded lowercase header directly into `RequestHeaders` using pooled entries.
///
/// # Arguments
///
/// - `&mut RequestHeaders` - Destination header map.
/// - `&str` - Validated lowercase header name.
/// - `&str` - Validated header value.
#[inline(always)]
fn insert_pooled_h3_header(headers: &mut RequestHeaders, name: &str, value: &str) {
    if let Some(existing) = headers.get_mut(name) {
        existing.push_back(value.to_owned());
        return;
    }
    let (mut key, mut values): (String, VecDeque<String>) =
        take_protocol_header_entry().unwrap_or_else(|| (String::new(), VecDeque::new()));
    key.clear();
    key.push_str(name);
    while values.len() > 1 {
        values.pop_back();
    }
    if let Some(slot) = values.front_mut() {
        slot.clear();
        slot.push_str(value);
    } else {
        values.push_back(value.to_owned());
    }
    headers.insert(key, values);
}

/// Decodes a QPACK request header block directly into `request` and `target` in place.
///
/// Requires static-table-only encoding (`Required Insert Count == 0`), matching the
/// server's advertised `SETTINGS_QPACK_MAX_TABLE_CAPACITY = 0`.
///
/// # Arguments
///
/// - `&[u8]` - The `HEADERS` frame payload bytes.
/// - `&RequestConfig` - Security limits for path, headers and body size.
/// - `&mut Request` - The pooled framework request to populate in place.
/// - `&mut String` - The pooled target buffer to populate with the raw `:path`.
///
/// # Returns
///
/// - `Result<H3DecodedRequestMeta, RequestError>` - Extracted framing metadata or protocol error.
pub(crate) fn decode_qpack_request(
    payload: &[u8],
    config: &RequestConfig,
    request: &mut Request,
    target: &mut String,
) -> Result<H3DecodedRequestMeta, RequestError> {
    let mut name_scratch: Option<Vec<u8>> = None;
    let mut val_scratch: Option<Vec<u8>> = None;
    let result: Result<H3DecodedRequestMeta, RequestError> = decode_qpack_request_inner(
        payload,
        config,
        request,
        target,
        &mut name_scratch,
        &mut val_scratch,
    );
    if let Some(buf) = name_scratch {
        return_h3_buffer(buf);
    }
    if let Some(buf) = val_scratch {
        return_h3_buffer(buf);
    }
    result
}

/// Inner QPACK request decoder borrowing optional lazy Huffman scratch buffers.
///
/// # Arguments
///
/// - `&[u8]` - The `HEADERS` frame payload bytes.
/// - `&RequestConfig` - Security limits for path, headers and body size.
/// - `&mut Request` - The pooled framework request to populate in place.
/// - `&mut String` - The pooled target buffer to populate with the raw `:path`.
/// - `&mut Option<Vec<u8>>` - Lazy Huffman name scratch buffer.
/// - `&mut Option<Vec<u8>>` - Lazy Huffman value scratch buffer.
///
/// # Returns
///
/// - `Result<H3DecodedRequestMeta, RequestError>` - Extracted framing metadata or protocol error.
fn decode_qpack_request_inner(
    payload: &[u8],
    config: &RequestConfig,
    request: &mut Request,
    target: &mut String,
    name_scratch: &mut Option<Vec<u8>>,
    val_scratch: &mut Option<Vec<u8>>,
) -> Result<H3DecodedRequestMeta, RequestError> {
    if payload.len() < 2 {
        return Err(RequestError::HttpRead(HttpStatus::BadRequest));
    }
    let (ric, ric_len): (u64, usize) = decode_prefix_int(payload, 8)?;
    if ric != 0 {
        return Err(RequestError::HttpRead(HttpStatus::BadRequest));
    }
    let (delta_base, base_len): (u64, usize) = decode_prefix_int(&payload[ric_len..], 7)?;
    if delta_base != 0 || payload[ric_len] & 0x80 != 0 {
        return Err(RequestError::HttpRead(HttpStatus::BadRequest));
    }
    let mut pos: usize = ric_len + base_len;
    request.reset();
    request.set_version(HttpVersion::Http3);
    target.clear();
    let max_header_count: usize = config.get_max_header_count();
    let max_key_size: usize = config.get_max_header_key_size();
    let max_val_size: usize = config.get_max_header_value_size();
    let max_body_size: usize = config.get_max_body_size();
    let mut method: Option<Method> = None;
    let mut scheme: Option<&str> = None;
    let mut scheme_owned: Option<String> = None;
    let mut has_path: bool = false;
    let mut authority_buf: [u8; 128] = [0; 128];
    let mut authority_len: usize = 0;
    let mut has_authority: bool = false;
    let mut authority_owned: Option<String> = None;
    let mut host_buf: [u8; 128] = [0; 128];
    let mut host_len: usize = 0;
    let mut has_host: bool = false;
    let mut host_owned: Option<String> = None;
    let mut declared_content_length: Option<usize> = None;
    let mut expect_continue: bool = false;
    let mut seen_regular_header: bool = false;
    let mut header_count: usize = 0;
    while pos < payload.len() {
        let first: u8 = payload[pos];
        let (name_slice, val_slice): (&str, &str) = if (first & 0x80) != 0 {
            if (first & 0x40) == 0 {
                return Err(RequestError::HttpRead(HttpStatus::BadRequest));
            }
            let (idx_u64, consumed): (u64, usize) = decode_prefix_int(&payload[pos..], 6)?;
            pos += consumed;
            let idx: usize = usize::try_from(idx_u64)
                .map_err(|_| RequestError::HttpRead(HttpStatus::BadRequest))?;
            *QPACK_STATIC_TABLE
                .get(idx)
                .ok_or(RequestError::HttpRead(HttpStatus::BadRequest))?
        } else if (first & 0x40) != 0 {
            if (first & 0x10) == 0 {
                return Err(RequestError::HttpRead(HttpStatus::BadRequest));
            }
            let (idx_u64, name_consumed): (u64, usize) = decode_prefix_int(&payload[pos..], 4)?;
            pos += name_consumed;
            let idx: usize = usize::try_from(idx_u64)
                .map_err(|_| RequestError::HttpRead(HttpStatus::BadRequest))?;
            let (static_name, _): (&str, &str) = *QPACK_STATIC_TABLE
                .get(idx)
                .ok_or(RequestError::HttpRead(HttpStatus::BadRequest))?;
            let (raw_val, used_scratch, val_consumed): (&[u8], bool, usize) =
                decode_qpack_string(&payload[pos..], 7, val_scratch)?;
            pos += val_consumed;
            let val_bytes: &[u8] = if used_scratch {
                val_scratch.as_deref().unwrap_or(&[])
            } else {
                raw_val
            };
            let val_str: &str = validate_h3_header_value(val_bytes)?;
            (static_name, val_str)
        } else if (first & 0x20) != 0 {
            let (raw_name, used_name_scratch, name_consumed): (&[u8], bool, usize) =
                decode_qpack_string(&payload[pos..], 3, name_scratch)?;
            pos += name_consumed;
            let name_bytes: &[u8] = if used_name_scratch {
                name_scratch.as_deref().unwrap_or(&[])
            } else {
                raw_name
            };
            let (raw_val, used_val_scratch, val_consumed): (&[u8], bool, usize) =
                decode_qpack_string(&payload[pos..], 7, val_scratch)?;
            pos += val_consumed;
            let val_bytes: &[u8] = if used_val_scratch {
                val_scratch.as_deref().unwrap_or(&[])
            } else {
                raw_val
            };
            let name_str: &str = if name_bytes.starts_with(b":") {
                from_utf8(name_bytes).map_err(|_| RequestError::HttpRead(HttpStatus::BadRequest))?
            } else {
                validate_h3_header_name(name_bytes)?
            };
            let val_str: &str = validate_h3_header_value(val_bytes)?;
            (name_str, val_str)
        } else {
            return Err(RequestError::HttpRead(HttpStatus::BadRequest));
        };
        if name_slice.starts_with(':') {
            if seen_regular_header {
                return Err(RequestError::HttpRead(HttpStatus::BadRequest));
            }
            match name_slice {
                ":method" => {
                    if method.is_some() || val_slice.is_empty() {
                        return Err(RequestError::HttpRead(HttpStatus::BadRequest));
                    }
                    if !val_slice.bytes().all(|byte: u8| {
                        byte.is_ascii_alphanumeric() || H3_LOWER_TOKEN_BYTES.contains(&byte)
                    }) {
                        return Err(RequestError::HttpRead(HttpStatus::BadRequest));
                    }
                    method = Some(match val_slice.as_bytes() {
                        WIRE_METHOD_GET => Method::Get,
                        WIRE_METHOD_POST => Method::Post,
                        WIRE_METHOD_PUT => Method::Put,
                        WIRE_METHOD_DELETE => Method::Delete,
                        WIRE_METHOD_HEAD => Method::Head,
                        WIRE_METHOD_OPTIONS => Method::Options,
                        WIRE_METHOD_PATCH => Method::Patch,
                        WIRE_METHOD_CONNECT => Method::Connect,
                        WIRE_METHOD_TRACE => Method::Trace,
                        _ => Method::Unknown(val_slice.to_owned()),
                    });
                }
                ":scheme" => {
                    if scheme.is_some() || scheme_owned.is_some() || val_slice.is_empty() {
                        return Err(RequestError::HttpRead(HttpStatus::BadRequest));
                    }
                    if val_slice == HTTP_SCHEME {
                        scheme = Some(HTTP_SCHEME);
                    } else if val_slice == HTTPS_SCHEME {
                        scheme = Some(HTTPS_SCHEME);
                    } else {
                        scheme_owned = Some(val_slice.to_owned());
                    }
                }
                ":path" => {
                    if has_path || val_slice.is_empty() {
                        return Err(RequestError::HttpRead(HttpStatus::BadRequest));
                    }
                    has_path = true;
                    target.push_str(val_slice);
                }
                ":authority" => {
                    if has_authority {
                        return Err(RequestError::HttpRead(HttpStatus::BadRequest));
                    }
                    has_authority = true;
                    let bytes: &[u8] = val_slice.as_bytes();
                    if bytes.len() <= authority_buf.len() {
                        authority_buf[..bytes.len()].copy_from_slice(bytes);
                        authority_len = bytes.len();
                    } else {
                        authority_owned = Some(val_slice.to_owned());
                    }
                }
                _ => return Err(RequestError::HttpRead(HttpStatus::BadRequest)),
            }
            continue;
        }
        seen_regular_header = true;
        header_count += 1;
        if max_header_count != DEFAULT_LOW_SECURITY_MAX_HEADER_COUNT
            && header_count > max_header_count
        {
            return Err(RequestError::TooManyHeaders(
                HttpStatus::RequestHeaderFieldsTooLarge,
            ));
        }
        if max_key_size != DEFAULT_LOW_SECURITY_MAX_HEADER_KEY_SIZE
            && name_slice.len() > max_key_size
        {
            return Err(RequestError::HeaderKeyTooLong(
                HttpStatus::RequestHeaderFieldsTooLarge,
            ));
        }
        if max_val_size != DEFAULT_LOW_SECURITY_MAX_HEADER_VALUE_SIZE
            && val_slice.len() > max_val_size
        {
            return Err(RequestError::HeaderValueTooLong(
                HttpStatus::RequestHeaderFieldsTooLarge,
            ));
        }
        match name_slice {
            "connection" | "proxy-connection" | "keep-alive" | "upgrade" | "transfer-encoding" => {
                return Err(RequestError::HttpRead(HttpStatus::BadRequest));
            }
            "te" if !val_slice.eq_ignore_ascii_case("trailers") => {
                return Err(RequestError::HttpRead(HttpStatus::BadRequest));
            }
            "host" => {
                if has_host {
                    return Err(RequestError::HttpRead(HttpStatus::BadRequest));
                }
                has_host = true;
                let bytes: &[u8] = val_slice.as_bytes();
                if bytes.len() <= host_buf.len() {
                    host_buf[..bytes.len()].copy_from_slice(bytes);
                    host_len = bytes.len();
                } else {
                    host_owned = Some(val_slice.to_owned());
                }
            }
            CONTENT_LENGTH => {
                if declared_content_length.is_some() {
                    return Err(RequestError::InvalidContentLength(HttpStatus::BadRequest));
                }
                if val_slice.is_empty() || !val_slice.bytes().all(|b: u8| b.is_ascii_digit()) {
                    return Err(RequestError::InvalidContentLength(HttpStatus::BadRequest));
                }
                let parsed_len: usize = val_slice
                    .parse::<usize>()
                    .map_err(|_| RequestError::InvalidContentLength(HttpStatus::BadRequest))?;
                if max_body_size != DEFAULT_LOW_SECURITY_MAX_BODY_SIZE && parsed_len > max_body_size
                {
                    return Err(RequestError::ContentLengthTooLarge(
                        HttpStatus::PayloadTooLarge,
                    ));
                }
                declared_content_length = Some(parsed_len);
            }
            "expect" if value_has_continue_expectation(val_slice) => {
                expect_continue = true;
            }
            _ => {}
        }
        insert_pooled_h3_header(request.get_mut_headers(), name_slice, val_slice);
    }
    let method: Method = method.ok_or(RequestError::HttpRead(HttpStatus::BadRequest))?;
    let scheme_str: Option<&str> = scheme.or(scheme_owned.as_deref());
    if method == Method::Connect {
        if has_path || scheme_str.is_some() || !has_authority {
            return Err(RequestError::HttpRead(HttpStatus::BadRequest));
        }
        target.push_str(DEFAULT_HTTP_PATH);
    } else {
        let scheme_value: &str =
            scheme_str.ok_or(RequestError::HttpRead(HttpStatus::BadRequest))?;
        let scheme_bytes: &[u8] = scheme_value.as_bytes();
        let scheme_valid: bool = scheme_bytes.first().is_some_and(u8::is_ascii_alphabetic)
            && scheme_bytes.iter().all(|byte: &u8| {
                byte.is_ascii_alphanumeric() || H3_SCHEME_PUNCTUATION.contains(byte)
            });
        let scheme_ok: bool = has_path && scheme_valid && scheme_value.parse::<Scheme>().is_ok();
        if !scheme_ok {
            return Err(RequestError::HttpRead(HttpStatus::BadRequest));
        }
        if (!target.starts_with(DEFAULT_HTTP_PATH) && !(target == "*" && method == Method::Options))
            || target.contains(HASH)
        {
            return Err(RequestError::HttpRead(HttpStatus::BadRequest));
        }
    }
    let max_path_size: usize = config.get_max_path_size();
    if max_path_size != DEFAULT_LOW_SECURITY_MAX_PATH_SIZE && target.len() > max_path_size {
        return Err(RequestError::PathTooLong(HttpStatus::URITooLong));
    }
    let auth_str: Option<&str> = if has_authority {
        match authority_owned.as_deref() {
            Some(owned) => Some(owned),
            None => from_utf8(&authority_buf[..authority_len]).ok(),
        }
    } else {
        None
    };
    let host_str: Option<&str> = if has_host {
        match host_owned.as_deref() {
            Some(owned) => Some(owned),
            None => from_utf8(&host_buf[..host_len]).ok(),
        }
    } else {
        None
    };
    let effective_authority: &str = auth_str
        .or(host_str)
        .ok_or(RequestError::HttpRead(HttpStatus::BadRequest))?;
    let parsed_authority: Authority = effective_authority
        .parse::<Authority>()
        .map_err(|_| RequestError::HttpRead(HttpStatus::BadRequest))?;
    if effective_authority.contains('@') || parsed_authority.host().is_empty() {
        return Err(RequestError::HttpRead(HttpStatus::BadRequest));
    }
    if let (Some(auth), Some(host)) = (auth_str, host_str)
        && !same_authority(auth, host, scheme_str)
    {
        return Err(RequestError::HttpRead(HttpStatus::BadRequest));
    }
    let host_out: &mut String = request.get_mut_host();
    host_out.clear();
    if let Some(auth) = auth_str {
        host_out.push_str(auth);
    } else if let Some(host) = host_str {
        host_out.push_str(host);
    }
    let path_str: &str = target.as_str();
    let hash_index: Option<usize> = path_str.find(HASH);
    let query_index: Option<usize> = path_str.find(QUERY);
    let (clean_path, query_str): (&str, &str) = match (query_index, hash_index) {
        (Some(q), Some(h)) if q < h => (&path_str[..q], &path_str[q + 1..h]),
        (Some(q), None) => (&path_str[..q], &path_str[q + 1..]),
        (_, Some(h)) => (&path_str[..h], ""),
        (None, None) => (path_str, ""),
    };
    request.get_mut_path().push_str(clean_path);
    if !query_str.is_empty() {
        let querys: &mut RequestQuerys = request.get_mut_querys();
        let mut param_count: usize = 0;
        for pair in query_str.split(AND) {
            if param_count >= PROTOCOL_MAX_QUERY_PARAM_COUNT {
                break;
            }
            if let Some((k, v)) = pair.split_once(EQUAL) {
                if !k.is_empty() {
                    param_count += 1;
                    insert_pooled_h3_header(querys, k, v);
                }
            } else if !pair.is_empty() {
                param_count += 1;
                if !querys.contains_key(pair) {
                    querys.insert(pair.to_owned(), VecDeque::new());
                }
            }
        }
    }
    let is_head: bool = method.is_head();
    request.set_method(method);
    Ok(H3DecodedRequestMeta {
        declared_content_length,
        expect_continue: expect_continue && declared_content_length != Some(0),
        is_head,
    })
}

/// Writes a single QPACK header field line using static-table shortcuts where available.
///
/// # Arguments
///
/// - `&[u8]` - Lowercase header name bytes.
/// - `&[u8]` - Header value bytes.
/// - `&mut Vec<u8>` - Destination QPACK header block buffer.
#[inline(always)]
fn encode_qpack_header(name: &[u8], value: &[u8], out: &mut Vec<u8>) {
    match name {
        QPACK_CONTENT_TYPE_NAME => match value {
            QPACK_CONTENT_TYPE_HTML => {
                out.push(0xC0 | 52);
                return;
            }
            QPACK_CONTENT_TYPE_PLAIN => {
                out.push(0xC0 | 53);
                return;
            }
            QPACK_CONTENT_TYPE_PLAIN_UTF8 => {
                out.push(0xC0 | 54);
                return;
            }
            QPACK_CONTENT_TYPE_JSON => {
                out.push(0xC0 | 46);
                return;
            }
            _ => {
                encode_prefix_int(0x50, 4, 44, out);
                encode_prefix_int(0x00, 7, value.len() as u64, out);
                out.extend_from_slice(value);
                return;
            }
        },
        name if name == CONTENT_LENGTH.as_bytes() => {
            if value == b"0" {
                out.push(0xC0 | 4);
                return;
            }
            out.push(0x50 | 4);
            encode_prefix_int(0x00, 7, value.len() as u64, out);
            out.extend_from_slice(value);
            return;
        }
        QPACK_SET_COOKIE_NAME => {
            out.push(0x50 | 14);
            encode_prefix_int(0x00, 7, value.len() as u64, out);
            out.extend_from_slice(value);
            return;
        }
        QPACK_LOCATION_NAME => {
            out.push(0x50 | 12);
            encode_prefix_int(0x00, 7, value.len() as u64, out);
            out.extend_from_slice(value);
            return;
        }
        QPACK_SERVER_NAME => {
            encode_prefix_int(0x50, 4, 92, out);
            encode_prefix_int(0x00, 7, value.len() as u64, out);
            out.extend_from_slice(value);
            return;
        }
        QPACK_DATE_NAME => {
            out.push(0x50 | 6);
            encode_prefix_int(0x00, 7, value.len() as u64, out);
            out.extend_from_slice(value);
            return;
        }
        _ => {}
    }
    encode_prefix_int(0x20, 3, name.len() as u64, out);
    out.extend_from_slice(name);
    encode_prefix_int(0x00, 7, value.len() as u64, out);
    out.extend_from_slice(value);
}

/// Encodes `:status` using QPACK static indices when available, or a literal field line.
///
/// # Arguments
///
/// - `usize` - Validated three-digit HTTP status code.
/// - `&mut Vec<u8>` - Destination QPACK header block buffer.
#[inline(always)]
fn encode_qpack_status(status_code: usize, out: &mut Vec<u8>) {
    let static_idx: Option<u64> = match status_code {
        200 => Some(25),
        103 => Some(24),
        304 => Some(26),
        404 => Some(27),
        503 => Some(28),
        100 => Some(63),
        204 => Some(64),
        206 => Some(65),
        302 => Some(66),
        400 => Some(67),
        403 => Some(68),
        421 => Some(69),
        425 => Some(70),
        500 => Some(71),
        _ => None,
    };
    if let Some(idx) = static_idx {
        encode_prefix_int(0xC0, 6, idx, out);
        return;
    }
    encode_prefix_int(0x50, 4, 25, out);
    let digits: [u8; 3] = [
        (status_code / 100) as u8 + ZERO_STR_U8,
        ((status_code / 10) % 10) as u8 + ZERO_STR_U8,
        (status_code % 10) as u8 + ZERO_STR_U8,
    ];
    out.push(3);
    out.extend_from_slice(&digits);
}

/// Validates an HTTP response header name token (RFC 9110 §5.6.2, case-insensitive ASCII token).
///
/// # Arguments
///
/// - `&[u8]` - Header name bytes.
///
/// # Returns
///
/// - `bool` - `true` when every byte is a legal HTTP token character.
#[inline(always)]
fn is_valid_response_header_name(bytes: &[u8]) -> bool {
    !bytes.is_empty()
        && bytes
            .iter()
            .all(|byte: &u8| byte.is_ascii_alphanumeric() || H3_LOWER_TOKEN_BYTES.contains(byte))
}

/// Validates an HTTP response header value (matching `http::HeaderValue::from_bytes`).
///
/// # Arguments
///
/// - `&[u8]` - Header value bytes.
///
/// # Returns
///
/// - `bool` - `true` when every byte is visible ASCII, space, HTAB, or obs-text (`>= 0x80`).
#[inline(always)]
fn is_valid_response_header_value(bytes: &[u8]) -> bool {
    bytes
        .iter()
        .all(|&byte: &u8| byte >= 0x20 && byte != 0x7f || byte == b'\t')
}

/// Checks whether a response header name is a static hop-by-hop field forbidden in HTTP/3.
///
/// # Arguments
///
/// - `&str` - The header name.
///
/// # Returns
///
/// - `bool` - `true` when the header must be stripped.
#[inline(always)]
fn is_hop_by_hop_h3_header(name: &str) -> bool {
    name.eq_ignore_ascii_case(CONNECTION)
        || name.eq_ignore_ascii_case(KEEP_ALIVE)
        || name.eq_ignore_ascii_case(PROXY_CONNECTION)
        || name.eq_ignore_ascii_case(UPGRADE)
        || name.eq_ignore_ascii_case(TE)
        || name.eq_ignore_ascii_case(TRAILER_HEADER)
}

/// Fast-path QPACK encoder for canonical `Response::build()` 200 OK heads without UTF-8 or scratch buffers.
///
/// # Arguments
///
/// - `&[u8]` - Serialized HTTP response head bytes.
/// - `bool` - Whether the incoming request method was `HEAD`.
/// - `&mut Vec<u8>` - Destination wire buffer receiving the complete HTTP/3 `HEADERS` frame.
///
/// # Returns
///
/// - `Option<H3EncodedResponseMeta>` - Framing metadata when `head` matched the fast layout.
#[inline(always)]
fn try_encode_fast_h3_200_head(
    head: &[u8],
    is_head: bool,
    out: &mut Vec<u8>,
) -> Option<H3EncodedResponseMeta> {
    let rest: &[u8] = head
        .strip_prefix(FAST_PROTOCOL_H3_200_PREFIX)
        .or_else(|| head.strip_prefix(FAST_PROTOCOL_H11_200_PREFIX))?;
    let digits: &[u8] = rest.strip_suffix(FAST_PROTOCOL_DEFAULT_SUFFIX)?;
    if digits.is_empty() || digits.len() > 20 {
        return None;
    }
    let mut length: usize = 0;
    for &byte in digits {
        if !byte.is_ascii_digit() {
            return None;
        }
        length = length
            .checked_mul(10)?
            .checked_add((byte - b'0') as usize)?;
    }
    let block_len: usize = if digits == b"0" { 5 } else { 6 + digits.len() };
    out.reserve(2 + block_len);
    out.push(H3_FRAME_HEADERS as u8);
    out.push(block_len as u8);
    out.push(0x00);
    out.push(0x00);
    out.push(0xC0 | 25);
    if digits == b"0" {
        out.push(0xC0 | 4);
    } else {
        out.push(0x50 | 4);
        out.push(digits.len() as u8);
        out.extend_from_slice(digits);
    }
    out.push(0xC0 | 52);
    Some(H3EncodedResponseMeta {
        announced_len: if is_head { None } else { Some(length) },
        suppress_body: is_head,
    })
}

/// Parses an HTTP/1.1 response head and writes an HTTP/3 `HEADERS` frame directly into `out`.
///
/// Validates status code, header names/values, rejects `transfer-encoding`, strips hop-by-hop
/// headers and `Connection`-nominated headers, and suppresses entity length / body on `HEAD`,
/// `1xx`, `204`, and `304` responses without allocating an intermediate `http::HeaderMap`.
///
/// # Arguments
///
/// - `&[u8]` - The serialized HTTP/1.1 response head up to and including `\r\n\r\n`.
/// - `bool` - Whether the request method was `HEAD`.
/// - `&mut Vec<u8>` - Destination buffer where the HTTP/3 `HEADERS` frame is appended.
///
/// # Returns
///
/// - `Result<H3EncodedResponseMeta, ResponseError>` - Framing metadata or validation error.
pub(crate) fn encode_h3_response_head(
    head: &[u8],
    is_head: bool,
    out: &mut Vec<u8>,
) -> Result<H3EncodedResponseMeta, ResponseError> {
    if let Some(fast) = try_encode_fast_h3_200_head(head, is_head, out) {
        return Ok(fast);
    }
    let text: &str =
        from_utf8(head).map_err(|error: Utf8Error| ResponseError::Send(error.to_string()))?;
    let mut lines: SplitLines<'_> = text.split(CRLF);
    let status_line: &str = lines
        .next()
        .ok_or_else(|| ResponseError::Send(MISSING_RESPONSE_VERSION.to_owned()))?;
    let mut parts: SplitWhitespace<'_> = status_line.split_whitespace();
    let version: &str = parts
        .next()
        .ok_or_else(|| ResponseError::Send(MISSING_RESPONSE_VERSION.to_owned()))?;
    if !version.starts_with(HTTP_VERSION_PREFIX) {
        return Err(ResponseError::Send(MISSING_RESPONSE_VERSION.to_owned()));
    }
    let status_token: &str = parts
        .next()
        .ok_or_else(|| ResponseError::Send(MISSING_RESPONSE_STATUS.to_owned()))?;
    let status_code: usize = status_token
        .parse::<usize>()
        .map_err(|error: ParseIntError| ResponseError::Send(error.to_string()))?;
    validate_status_code(status_code)?;
    let hide_length: bool = hides_entity_length(status_code, is_head);
    let suppress_body: bool = is_bodyless_status(status_code, is_head);
    let mut parsed_lines: [(&str, &str); 32] = [("", ""); 32];
    let mut parsed_count: usize = 0;
    let mut overflow_lines: Vec<(&str, &str)> = Vec::new();
    let mut nominated: [&str; 8] = [""; 8];
    let mut nominated_count: usize = 0;
    let mut overflow_nominated: Vec<&str> = Vec::new();
    let mut announced_len: Option<usize> = None;
    for line in lines {
        if line.is_empty() {
            break;
        }
        let Some((name, value)) = line.split_once(COLON) else {
            return Err(ResponseError::Send(MALFORMED_RESPONSE_HEADER.to_owned()));
        };
        let trimmed_name: &str = name.trim();
        let trimmed_value: &str = value.trim();
        if !is_valid_response_header_name(trimmed_name.as_bytes())
            || !is_valid_response_header_value(trimmed_value.as_bytes())
        {
            return Err(ResponseError::Send(MALFORMED_RESPONSE_HEADER.to_owned()));
        }
        if trimmed_name.eq_ignore_ascii_case(TRANSFER_ENCODING) {
            return Err(ResponseError::Send(
                UNSUPPORTED_TRANSFER_ENCODING.to_owned(),
            ));
        }
        if trimmed_name.eq_ignore_ascii_case(CONNECTION) {
            for token in trimmed_value.split(COMMA_STR) {
                let token: &str = token.trim();
                if !token.is_empty()
                    && !token.eq_ignore_ascii_case(KEEP_ALIVE)
                    && !token.eq_ignore_ascii_case(CLOSE)
                {
                    if nominated_count < nominated.len() {
                        nominated[nominated_count] = token;
                        nominated_count += 1;
                    } else {
                        overflow_nominated.push(token);
                    }
                }
            }
            continue;
        }
        if trimmed_name.eq_ignore_ascii_case(CONTENT_LENGTH) {
            let value_len: usize = parse_ascii_length(trimmed_value)
                .map_err(|error: RequestError| ResponseError::Send(error.to_string()))?;
            match announced_len {
                None => announced_len = Some(value_len),
                Some(existing) if existing == value_len => {}
                Some(_) => {
                    return Err(ResponseError::Send(OVERFLOWING_RESPONSE_BODY.to_owned()));
                }
            }
            if hide_length {
                continue;
            }
        }
        if parsed_count < parsed_lines.len() {
            parsed_lines[parsed_count] = (trimmed_name, trimmed_value);
            parsed_count += 1;
        } else {
            overflow_lines.push((trimmed_name, trimmed_value));
        }
    }
    let frame_start: usize = out.len();
    out.push(H3_FRAME_HEADERS as u8);
    out.push(0);
    let payload_start: usize = out.len();
    out.extend_from_slice(&[0, 0]);
    encode_qpack_status(status_code, out);
    let mut lower_buf: [u8; 64] = [0; 64];
    let nom_slice: &[&str] = &nominated[..nominated_count];
    for &(name, value) in parsed_lines[..parsed_count]
        .iter()
        .chain(overflow_lines.iter())
    {
        if is_hop_by_hop_h3_header(name)
            || nom_slice
                .iter()
                .chain(overflow_nominated.iter())
                .any(|token: &&str| token.eq_ignore_ascii_case(name))
        {
            continue;
        }
        let name_bytes: &[u8] = name.as_bytes();
        let has_upper: bool = name_bytes.iter().any(|b: &u8| b.is_ascii_uppercase());
        if !has_upper {
            encode_qpack_header(name_bytes, value.as_bytes(), out);
        } else if name_bytes.len() <= lower_buf.len() {
            for (idx, &b) in name_bytes.iter().enumerate() {
                lower_buf[idx] = b.to_ascii_lowercase();
            }
            encode_qpack_header(&lower_buf[..name_bytes.len()], value.as_bytes(), out);
        } else {
            let lowered: String = name.to_ascii_lowercase();
            encode_qpack_header(lowered.as_bytes(), value.as_bytes(), out);
        }
    }
    let block_len: usize = out.len() - payload_start;
    if block_len < 64 {
        out[frame_start + 1] = block_len as u8;
    } else {
        let mut len_buf: Vec<u8> = Vec::with_capacity(8);
        encode_varint(block_len as u64, &mut len_buf);
        let extra: usize = len_buf.len() - 1;
        let old_len: usize = out.len();
        out.resize(old_len + extra, 0);
        out.copy_within(payload_start..old_len, payload_start + extra);
        out[frame_start + 1..frame_start + 1 + len_buf.len()].copy_from_slice(&len_buf);
    }
    Ok(H3EncodedResponseMeta {
        announced_len: if suppress_body { None } else { announced_len },
        suppress_body,
    })
}
