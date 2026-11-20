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
