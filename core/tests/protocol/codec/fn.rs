use super::*;

fn literal_block(fields: &[(&str, &str)]) -> Vec<u8> {
    let mut output: Vec<u8> = vec![0, 0];
    for &(name, value) in fields {
        let len: usize = name.len();
        assert!(len < 127 && value.len() < 127);
        if len < 7 {
            output.push(0x20 | len as u8);
        } else {
            output.push(0x27);
            output.push((len - 7) as u8);
        }
        output.extend_from_slice(name.as_bytes());
        output.push(value.len() as u8);
        output.extend_from_slice(value.as_bytes());
    }
    output
}

fn decode_test_request(payload: &[u8]) -> Result<Request, RequestError> {
    let mut request: Request = Request::default();
    let mut target: String = String::new();
    decode_qpack_request(
        payload,
        &RequestConfig::default(),
        &mut request,
        &mut target,
    )?;
    Ok(request)
}

#[test]
fn huffman_rfc_examples_decode_exactly() {
    let examples: [(&[u8], &[u8]); 3] = [
        (
            &[
                0xf1, 0xe3, 0xc2, 0xe5, 0xf2, 0x3a, 0x6b, 0xa0, 0xab, 0x90, 0xf4, 0xff,
            ],
            b"www.example.com",
        ),
        (&[0xa8, 0xeb, 0x10, 0x64, 0x9c, 0xbf], b"no-cache"),
        (
            &[0x25, 0xa8, 0x49, 0xe9, 0x5b, 0xa9, 0x7d, 0x7f],
            b"custom-key",
        ),
    ];
    for (encoded, expected) in examples {
        let mut output: Vec<u8> = Vec::new();
        decode_huffman(encoded, &mut output).unwrap();
        assert_eq!(output, expected);
    }
}

#[test]
fn huffman_all_octets_and_padding_boundaries_round_trip() {
    for length in [1usize, 2, 3, 7, 8, 9, 15, 16, 31, 32, 255, 256] {
        let input: Vec<u8> = (0..length).map(|idx: usize| (idx % 256) as u8).collect();
        let mut encoded: Vec<u8> = Vec::new();
        let mut current: u8 = 0;
        let mut filled: u8 = 0;
        for &byte in &input {
            let bits: u8 = HUFFMAN_LENGTHS[usize::from(byte)];
            let code: u32 = HUFFMAN_CODES[usize::from(byte)];
            for shift in (0..bits).rev() {
                current = (current << 1) | ((code >> shift) & 1) as u8;
                filled += 1;
                if filled == 8 {
                    encoded.push(current);
                    current = 0;
                    filled = 0;
                }
            }
        }
        if filled > 0 {
            let padding: u8 = 8 - filled;
            encoded.push((current << padding) | ((1u8 << padding) - 1));
        }
        let mut output: Vec<u8> = Vec::new();
        decode_huffman(&encoded, &mut output).unwrap();
        assert_eq!(output, input, "length={length}");
    }
    for invalid in [&[0xff][..], &[0xff, 0xff, 0xff, 0xff][..], &[0x00][..]] {
        assert!(decode_huffman(invalid, &mut Vec::new()).is_err());
    }
}

#[test]
fn qpack_rejects_negative_base_and_malformed_request_targets() {
    let normal: [(&str, &str); 4] = [
        (":method", "GET"),
        (":scheme", "https"),
        (":path", "/"),
        (":authority", "localhost"),
    ];
    let mut negative_base: Vec<u8> = literal_block(&normal);
    negative_base[1] = 0x80;
    assert!(decode_test_request(&negative_base).is_err());
    let invalid: [(usize, &str); 6] = [
        (0, "GE T"),
        (1, "1https"),
        (1, "ht tp"),
        (2, "relative"),
        (2, "/path#fragment"),
        (3, "user@localhost"),
    ];
    for (index, value) in invalid {
        let mut fields: [(&str, &str); 4] = normal;
        fields[index].1 = value;
        assert!(
            decode_test_request(&literal_block(&fields)).is_err(),
            "{index}: {value}"
        );
    }
}

#[test]
fn qpack_accepts_standard_connect_without_scheme_or_path() {
    let payload: Vec<u8> =
        literal_block(&[(":method", "CONNECT"), (":authority", "localhost:443")]);
    let request: Request = decode_test_request(&payload).unwrap();
    assert_eq!(request.get_method(), &Method::Connect);
    assert_eq!(request.get_host(), "localhost:443");
}

#[test]
fn native_response_encoder_rejects_the_same_invalid_fields_as_the_shared_parser() {
    for bytes in [
        &b"HTTP/1.1 200 OK\r\nbad name: x\r\n\r\n"[..],
        &b"HTTP/1.1 200 OK\r\nx-test: \x01\r\n\r\n"[..],
        &b"HTTP/1.1 200 OK\r\nx-test: \0\r\n\r\n"[..],
    ] {
        assert!(parse_response_head(bytes).is_err());
        assert!(encode_h3_response_head(bytes, false, &mut Vec::new()).is_err());
    }
}
