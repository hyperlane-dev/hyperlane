use super::*;

#[test]
fn test_websocket_opcode_from_u8() {
    assert_eq!(WebSocketOpcode::from_u8(0x0), WebSocketOpcode::Continuation);
    assert_eq!(WebSocketOpcode::from_u8(0x1), WebSocketOpcode::Text);
    assert_eq!(WebSocketOpcode::from_u8(0x2), WebSocketOpcode::Binary);
    assert_eq!(WebSocketOpcode::from_u8(0x8), WebSocketOpcode::Close);
    assert_eq!(WebSocketOpcode::from_u8(0x9), WebSocketOpcode::Ping);
    assert_eq!(WebSocketOpcode::from_u8(0xA), WebSocketOpcode::Pong);
    assert_eq!(
        WebSocketOpcode::from_u8(0x3),
        WebSocketOpcode::Reserved(0x3)
    );
    assert_eq!(
        WebSocketOpcode::from_u8(0xF),
        WebSocketOpcode::Reserved(0xF)
    );
}

#[test]
fn test_websocket_opcode_to_u8() {
    assert_eq!(WebSocketOpcode::Continuation.to_u8(), 0x0);
    assert_eq!(WebSocketOpcode::Text.to_u8(), 0x1);
    assert_eq!(WebSocketOpcode::Binary.to_u8(), 0x2);
    assert_eq!(WebSocketOpcode::Close.to_u8(), 0x8);
    assert_eq!(WebSocketOpcode::Ping.to_u8(), 0x9);
    assert_eq!(WebSocketOpcode::Pong.to_u8(), 0xA);
    assert_eq!(WebSocketOpcode::Reserved(0x3).to_u8(), 0x3);
}

#[test]
fn test_websocket_opcode_is_control() {
    assert!(!WebSocketOpcode::Continuation.is_control());
    assert!(!WebSocketOpcode::Text.is_control());
    assert!(!WebSocketOpcode::Binary.is_control());
    assert!(WebSocketOpcode::Close.is_control());
    assert!(WebSocketOpcode::Ping.is_control());
    assert!(WebSocketOpcode::Pong.is_control());
    assert!(!WebSocketOpcode::Reserved(0x3).is_control());
}

#[test]
fn test_websocket_opcode_is_data() {
    assert!(WebSocketOpcode::Continuation.is_data());
    assert!(WebSocketOpcode::Text.is_data());
    assert!(WebSocketOpcode::Binary.is_data());
    assert!(!WebSocketOpcode::Close.is_data());
    assert!(!WebSocketOpcode::Ping.is_data());
    assert!(!WebSocketOpcode::Pong.is_data());
}

#[test]
fn test_websocket_opcode_is_continuation() {
    assert!(WebSocketOpcode::Continuation.is_continuation());
    assert!(!WebSocketOpcode::Text.is_continuation());
}

#[test]
fn test_websocket_opcode_is_text() {
    assert!(WebSocketOpcode::Text.is_text());
    assert!(!WebSocketOpcode::Binary.is_text());
}

#[test]
fn test_websocket_opcode_is_binary() {
    assert!(WebSocketOpcode::Binary.is_binary());
    assert!(!WebSocketOpcode::Text.is_binary());
}

#[test]
fn test_websocket_opcode_is_close() {
    assert!(WebSocketOpcode::Close.is_close());
    assert!(!WebSocketOpcode::Ping.is_close());
}

#[test]
fn test_websocket_opcode_is_ping() {
    assert!(WebSocketOpcode::Ping.is_ping());
    assert!(!WebSocketOpcode::Pong.is_ping());
}

#[test]
fn test_websocket_opcode_is_pong() {
    assert!(WebSocketOpcode::Pong.is_pong());
    assert!(!WebSocketOpcode::Ping.is_pong());
}

#[test]
fn test_websocket_opcode_is_reserved() {
    assert!(WebSocketOpcode::Reserved(0x3).is_reserved());
    assert!(!WebSocketOpcode::Text.is_reserved());
}

#[test]
fn test_websocket_frame_default() {
    let frame: WebSocketFrame = WebSocketFrame::default();
    assert!(!frame.get_fin());
    assert_eq!(frame.get_opcode(), &WebSocketOpcode::Text);
    assert!(!frame.get_mask());
    assert!(frame.get_payload_data().is_empty());
}

#[test]
fn test_websocket_frame_decode_text_unmasked() {
    let data: [u8; 7] = [0x81, 0x05, 0x48, 0x65, 0x6C, 0x6C, 0x6F];
    let result: Option<(WebSocketFrame, usize)> = WebSocketFrame::decode_ws_frame(data);
    assert!(result.is_some());
    let (frame, consumed): (WebSocketFrame, usize) = result.unwrap();
    assert!(frame.get_fin());
    assert_eq!(frame.get_opcode(), &WebSocketOpcode::Text);
    assert!(!frame.get_mask());
    assert_eq!(frame.get_payload_data(), b"Hello");
    assert_eq!(consumed, 7);
}

#[test]
fn test_websocket_frame_decode_binary_masked() {
    let mask_key: [u8; 4] = [0x37, 0xFA, 0x21, 0x3D];
    let payload: [u8; 5] = [0x7F, 0x9F, 0x4D, 0x51, 0x58];
    let mut data: Vec<u8> = vec![0x82, 0x85];
    data.extend_from_slice(&mask_key);
    data.extend_from_slice(&payload);
    let result: Option<(WebSocketFrame, usize)> = WebSocketFrame::decode_ws_frame(data);
    assert!(result.is_some());
    let (frame, consumed): (WebSocketFrame, usize) = result.unwrap();
    assert!(frame.get_fin());
    assert_eq!(frame.get_opcode(), &WebSocketOpcode::Binary);
    assert!(frame.get_mask());
    assert_eq!(frame.get_payload_data(), b"Hello");
    assert_eq!(consumed, 11);
}

#[test]
fn test_websocket_frame_decode_incomplete() {
    let data: [u8; 1] = [0x81];
    let result: Option<(WebSocketFrame, usize)> = WebSocketFrame::decode_ws_frame(data);
    assert!(result.is_none());
}

#[test]
fn test_websocket_frame_decode_extended_payload_16() {
    let payload: Vec<u8> = vec![0x42; 200];
    let mut data: Vec<u8> = vec![0x82, 0x7E];
    data.extend_from_slice(&(200u16).to_be_bytes());
    data.extend_from_slice(&payload);
    let result: Option<(WebSocketFrame, usize)> = WebSocketFrame::decode_ws_frame(data);
    assert!(result.is_some());
    let (frame, consumed): (WebSocketFrame, usize) = result.unwrap();
    assert_eq!(frame.get_payload_data().len(), 200);
    assert_eq!(consumed, 204);
}

#[test]
fn test_websocket_frame_decode_close() {
    let data: [u8; 6] = [0x88, 0x02, 0x03, 0xE8, 0x48, 0x65];
    let result: Option<(WebSocketFrame, usize)> = WebSocketFrame::decode_ws_frame(data);
    assert!(result.is_some());
    let (frame, _consumed): (WebSocketFrame, usize) = result.unwrap();
    assert_eq!(frame.get_opcode(), &WebSocketOpcode::Close);
}

#[test]
fn test_websocket_frame_create_frame_list_text() {
    let frames: Vec<ResponseBody> = WebSocketFrame::create_frame_list("Hello");
    assert_eq!(frames.len(), 1);
    assert_eq!(frames[0][0], 0x81);
    assert_eq!(frames[0][1], 0x05);
    assert_eq!(&frames[0][2..], b"Hello");
}

#[test]
fn test_websocket_frame_create_frame_list_binary() {
    let binary_data: Vec<u8> = vec![0x00, 0x01, 0x02, 0x03];
    let frames: Vec<ResponseBody> = WebSocketFrame::create_frame_list(&binary_data);
    assert!(!frames.is_empty());
    assert_eq!(&frames[0][2..], &[0x00, 0x01, 0x02, 0x03]);
}

#[test]
fn test_websocket_frame_sha1() {
    let hash: [u8; 20] = WebSocketFrame::sha1("abc");
    let expected: [u8; 20] = [
        0xA9, 0x99, 0x3E, 0x36, 0x47, 0x06, 0x81, 0x6A, 0xBA, 0x3E, 0x25, 0x71, 0x78, 0x50, 0xC2,
        0x6C, 0x9C, 0xD0, 0xD8, 0x9D,
    ];
    assert_eq!(hash, expected);
}

#[test]
fn test_websocket_frame_try_generate_accept_key() {
    let key: &str = "dGhlIHNhbXBsZSBub25jZQ==";
    let accept_key: Option<String> = WebSocketFrame::try_generate_accept_key(key);
    assert_eq!(accept_key, Some("s3pPLMBiTxaQ9kYGzzhZRbK+xOo=".to_string()));
}

#[test]
fn test_websocket_frame_try_base64_encode() {
    let encoded: Option<String> = WebSocketFrame::try_base64_encode("Hello");
    assert_eq!(encoded, Some("SGVsbG8=".to_string()));
}

#[test]
fn test_websocket_frame_base64_encode() {
    let encoded: String = WebSocketFrame::base64_encode("Hello");
    assert_eq!(encoded, "SGVsbG8=");
}

#[test]
fn test_websocket_frame_is_text_opcode() {
    let frame: WebSocketFrame = WebSocketFrame {
        fin: true,
        opcode: WebSocketOpcode::Text,
        mask: false,
        payload_data: vec![],
    };
    assert!(frame.is_text_opcode());
    assert!(!frame.is_binary_opcode());
    assert!(!frame.is_close_opcode());
}

#[test]
fn test_websocket_frame_is_binary_opcode() {
    let frame: WebSocketFrame = WebSocketFrame {
        fin: true,
        opcode: WebSocketOpcode::Binary,
        mask: false,
        payload_data: vec![],
    };
    assert!(!frame.is_text_opcode());
    assert!(frame.is_binary_opcode());
}

#[test]
fn test_websocket_frame_is_close_opcode() {
    let frame: WebSocketFrame = WebSocketFrame {
        fin: true,
        opcode: WebSocketOpcode::Close,
        mask: false,
        payload_data: vec![],
    };
    assert!(frame.is_close_opcode());
    assert!(!frame.is_text_opcode());
}

#[test]
fn test_websocket_frame_is_ping_opcode() {
    let frame: WebSocketFrame = WebSocketFrame {
        fin: true,
        opcode: WebSocketOpcode::Ping,
        mask: false,
        payload_data: vec![],
    };
    assert!(frame.is_ping_opcode());
    assert!(!frame.is_pong_opcode());
}

#[test]
fn test_websocket_frame_is_pong_opcode() {
    let frame: WebSocketFrame = WebSocketFrame {
        fin: true,
        opcode: WebSocketOpcode::Pong,
        mask: false,
        payload_data: vec![],
    };
    assert!(frame.is_pong_opcode());
    assert!(!frame.is_ping_opcode());
}

#[test]
fn test_websocket_frame_is_continuation_opcode() {
    let frame: WebSocketFrame = WebSocketFrame {
        fin: false,
        opcode: WebSocketOpcode::Continuation,
        mask: false,
        payload_data: vec![],
    };
    assert!(frame.is_continuation_opcode());
}

#[test]
fn test_websocket_frame_is_reserved_opcode() {
    let frame: WebSocketFrame = WebSocketFrame {
        fin: true,
        opcode: WebSocketOpcode::Reserved(0x3),
        mask: false,
        payload_data: vec![],
    };
    assert!(frame.is_reserved_opcode());
}
