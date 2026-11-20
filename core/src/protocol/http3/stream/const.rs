/// QUIC close reason for malformed control frame.
pub(crate) const H3_REASON_MALFORMED_CONTROL: &[u8] = b"malformed control frame";

/// QUIC close reason for control stream closed.
pub(crate) const H3_REASON_CONTROL_CLOSED: &[u8] = b"control stream closed";

/// QUIC close reason for first control frame must be settings.
pub(crate) const H3_REASON_MISSING_SETTINGS: &[u8] = b"first control frame must be settings";

/// QUIC close reason for settings frame too large.
pub(crate) const H3_REASON_SETTINGS_TOO_LARGE: &[u8] = b"settings frame too large";

/// QUIC close reason for control stream closed during settings.
pub(crate) const H3_REASON_SETTINGS_TRUNCATED: &[u8] = b"control stream closed during settings";

/// QUIC close reason for control read error.
pub(crate) const H3_REASON_CONTROL_READ: &[u8] = b"control read error";

/// QUIC close reason for invalid settings payload.
pub(crate) const H3_REASON_INVALID_SETTINGS: &[u8] = b"invalid settings payload";

/// QUIC close reason for unexpected frame on control stream.
pub(crate) const H3_REASON_UNEXPECTED_CONTROL: &[u8] = b"unexpected frame on control stream";

/// QUIC close reason for control stream truncated.
pub(crate) const H3_REASON_CONTROL_TRUNCATED: &[u8] = b"control stream truncated";

/// QUIC close reason for duplicate control stream.
pub(crate) const H3_REASON_DUPLICATE_CONTROL: &[u8] = b"duplicate control stream";

/// QUIC close reason for duplicate qpack encoder stream.
pub(crate) const H3_REASON_DUPLICATE_ENCODER: &[u8] = b"duplicate qpack encoder stream";

/// QUIC close reason for qpack encoder stream closed.
pub(crate) const H3_REASON_ENCODER_CLOSED: &[u8] = b"qpack encoder stream closed";

/// QUIC close reason for duplicate qpack decoder stream.
pub(crate) const H3_REASON_DUPLICATE_DECODER: &[u8] = b"duplicate qpack decoder stream";

/// QUIC close reason for qpack decoder stream closed.
pub(crate) const H3_REASON_DECODER_CLOSED: &[u8] = b"qpack decoder stream closed";

/// QUIC close reason for client push stream forbidden.
pub(crate) const H3_REASON_CLIENT_PUSH: &[u8] = b"client push stream forbidden";

/// QUIC close reason for reserved HTTP/2 frame type received on HTTP/3.
pub(crate) const H3_REASON_RESERVED_FRAME: &[u8] = b"reserved http2 frame forbidden";

/// QUIC close reason for unexpected frame type received on a request stream.
pub(crate) const H3_REASON_UNEXPECTED_REQUEST_FRAME: &[u8] = b"unexpected frame on request stream";

/// QUIC close reason for frames arriving after trailing headers on a request stream.
pub(crate) const H3_REASON_FRAME_AFTER_TRAILERS: &[u8] = b"frame after trailing headers";

/// QUIC close reason for dynamic QPACK instruction when table capacity is zero.
pub(crate) const H3_REASON_QPACK_DYNAMIC_UNSUPPORTED: &[u8] =
    b"qpack dynamic table capacity exceeded";

/// Control or server-only frames forbidden on incoming client request streams.
pub(crate) const H3_FORBIDDEN_REQUEST_FRAMES: [u64; 5] = [
    H3_FRAME_CANCEL_PUSH,
    H3_FRAME_SETTINGS,
    H3_FRAME_PUSH_PROMISE,
    H3_FRAME_GOAWAY,
    H3_FRAME_MAX_PUSH_ID,
];

/// HTTP/3 `QPACK_ENCODER_STREAM_ERROR` application error code (RFC 9204 §6).
pub(crate) const H3_QPACK_ENCODER_STREAM_ERROR: u32 = 0x0201;

/// QPACK encoder instruction byte setting dynamic table capacity to zero (RFC 9204 §4.3.1).
pub(crate) const QPACK_SET_CAPACITY_ZERO: u8 = 0x20;

/// HTTP/3 `QPACK_DECODER_STREAM_ERROR` application error code (RFC 9204 §6).
pub(crate) const H3_QPACK_DECODER_STREAM_ERROR: u32 = 0x0202;

/// QUIC close reason for invalid QPACK decoder instruction when table capacity is zero.
pub(crate) const H3_REASON_QPACK_DECODER_INVALID: &[u8] = b"invalid qpack decoder instruction";

/// HTTP/3 `DATA` frame type (RFC 9114 §7.2.1).
pub(crate) const H3_FRAME_DATA: u64 = 0x00;

/// HTTP/3 `HEADERS` frame type (RFC 9114 §7.2.2).
pub(crate) const H3_FRAME_HEADERS: u64 = 0x01;

/// HTTP/3 `CANCEL_PUSH` frame type (RFC 9114 §7.2.3).
pub(crate) const H3_FRAME_CANCEL_PUSH: u64 = 0x03;

/// HTTP/3 `SETTINGS` frame type (RFC 9114 §7.2.4).
pub(crate) const H3_FRAME_SETTINGS: u64 = 0x04;

/// HTTP/3 `PUSH_PROMISE` frame type (RFC 9114 §7.2.5).
pub(crate) const H3_FRAME_PUSH_PROMISE: u64 = 0x05;

/// HTTP/3 `GOAWAY` frame type (RFC 9114 §7.2.6).
pub(crate) const H3_FRAME_GOAWAY: u64 = 0x07;

/// HTTP/3 `MAX_PUSH_ID` frame type (RFC 9114 §7.2.7).
pub(crate) const H3_FRAME_MAX_PUSH_ID: u64 = 0x0d;

/// HTTP/3 unidirectional control stream type (RFC 9114 §6.2.1).
pub(crate) const H3_STREAM_CONTROL: u64 = 0x00;

/// HTTP/3 unidirectional push stream type (RFC 9114 §6.2.2).
pub(crate) const H3_STREAM_PUSH: u64 = 0x01;

/// HTTP/3 QPACK encoder unidirectional stream type (RFC 9204 §4.2).
pub(crate) const H3_STREAM_QPACK_ENCODER: u64 = 0x02;

/// HTTP/3 QPACK decoder unidirectional stream type (RFC 9204 §4.2).
pub(crate) const H3_STREAM_QPACK_DECODER: u64 = 0x03;

/// HTTP/3 `H3_NO_ERROR` application error code (RFC 9114 §8.1).
pub(crate) const H3_NO_ERROR: u32 = 0x0100;

/// HTTP/3 `H3_INTERNAL_ERROR` application error code (RFC 9114 §8.1).
pub(crate) const H3_INTERNAL_ERROR: u32 = 0x0102;

/// HTTP/3 `H3_STREAM_CREATION_ERROR` application error code (RFC 9114 §8.1).
pub(crate) const H3_STREAM_CREATION_ERROR: u32 = 0x0103;

/// HTTP/3 `H3_CLOSED_CRITICAL_STREAM` application error code (RFC 9114 §8.1).
pub(crate) const H3_CLOSED_CRITICAL_STREAM: u32 = 0x0104;

/// HTTP/3 `H3_FRAME_UNEXPECTED` application error code (RFC 9114 §8.1).
pub(crate) const H3_FRAME_UNEXPECTED: u32 = 0x0105;

/// HTTP/3 `H3_FRAME_ERROR` application error code (RFC 9114 §8.1).
pub(crate) const H3_FRAME_ERROR: u32 = 0x0106;

/// HTTP/3 `H3_SETTINGS_ERROR` application error code (RFC 9114 §8.1).
pub(crate) const H3_SETTINGS_ERROR: u32 = 0x0109;

/// HTTP/3 `H3_MISSING_SETTINGS` application error code (RFC 9114 §8.1).
pub(crate) const H3_MISSING_SETTINGS: u32 = 0x010a;

/// Pre-encoded server control stream preface: Control stream type `0x00` + empty `SETTINGS` frame (`0x04`, len `0x00`).
pub(crate) const H3_SERVER_CONTROL_SETTINGS: &[u8] = &[0x00, 0x04, 0x00];

/// Pre-encoded HTTP/3 `100 Continue` interim `HEADERS` frame (QPACK static index 63 = `:status 100`).
pub(crate) const H3_CONTINUE_FRAME: &[u8] = &[0x01, 0x04, 0x00, 0x00, 0xff, 0x00];

/// Maximum number of reusable HTTP/3 wire buffers kept per worker thread.
pub(crate) const MAX_POOLED_H3_BUFFERS: usize = 64;

/// Initial read chunk size for HTTP/3 request streams.
pub(crate) const H3_READ_CHUNK_SIZE: usize = 4 * 1024;

/// Maximum serialized HTTP/3 request HEADERS frame payload size accepted before rejecting.
pub(crate) const H3_MAX_HEADERS_FRAME_LEN: usize = 256 * 1024;

/// Maximum serialized HTTP/3 peer SETTINGS frame payload accepted on the control stream.
pub(crate) const H3_MAX_SETTINGS_FRAME_LEN: usize = 16 * 1024;
