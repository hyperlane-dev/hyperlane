pub(crate) const WEBSOCKET_ROUTE_PATH: &str = "/ws";
pub(crate) const WEBSOCKET_PROTOCOL_VERSION: &str = "13";
pub(crate) const WEBSOCKET_SWITCHING_LINE: &str = "HTTP/1.1 101";
pub(crate) const WEBSOCKET_UPGRADE_VALUE: &str = "websocket";
pub(crate) const WEBSOCKET_CONNECTION_VALUE: &str = "Upgrade";

pub(crate) const RFC6455_SAMPLE_KEY: &str = "dGhlIHNhbXBsZSBub25jZQ==";
pub(crate) const RFC6455_SAMPLE_ACCEPT: &str = "s3pPLMBiTxaQ9kYGzzhZRbK+xOo=";

pub(crate) const WEBSOCKET_FIN_BIT: u8 = 0b1000_0000;
pub(crate) const WEBSOCKET_MASK_BIT: u8 = 0b1000_0000;
pub(crate) const WEBSOCKET_BINARY_OPCODE: u8 = 0x02;
pub(crate) const WEBSOCKET_CLOSE_OPCODE: u8 = 0x08;
pub(crate) const WEBSOCKET_EXTENDED_LENGTH_MARK: u8 = 126;
pub(crate) const WEBSOCKET_MAX_SHORT_FRAME: usize = 0xFFFF;
pub(crate) const WEBSOCKET_MAX_LENGTH_MARK: u8 = 127;
pub(crate) const RFC6455_HELLO_MASK: [u8; 4] = [0x37, 0xFA, 0x21, 0x3D];
pub(crate) const WEBSOCKET_TEXT_OPCODE: u8 = 0x01;
pub(crate) const RFC6455_HELLO_PAYLOAD: &str = "Hello";
pub(crate) const RFC6455_HELLO_FRAME: [u8; 11] = [
    0x81, 0x85, 0x37, 0xfa, 0x21, 0x3d, 0x7f, 0x9f, 0x4d, 0x51, 0x58,
];
pub(crate) const WEBSOCKET_CLIENT_MASK: [u8; 4] = [0x37, 0xFA, 0x21, 0x3D];

pub(crate) const WEBSOCKET_READ_CHUNK_LEN: usize = 8192;
pub(crate) const WEBSOCKET_EXCHANGE_TIMEOUT_SECS: u64 = 30;
