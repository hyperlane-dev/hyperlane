use super::*;

/// A single space character.
/// This constant is used to represent a space character in string
/// or byte operations.
pub const SPACE: &str = " ";

/// A const byte slice representation of the string `SPACE`.
pub const SPACE_BYTES: &[u8] = SPACE.as_bytes();

/// The byte representation of a single space character.
/// This constant provides the byte equivalent of the space character
/// for use in low-level operations.
pub const SPACE_U8: u8 = SPACE_BYTES[0];

/// A tab character.
/// This constant is used to represent a tab character in string
/// or byte operations.
pub const TAB: &str = "\t";

/// A const byte slice representation of the string `TAB`.
pub const TAB_BYTES: &[u8] = TAB.as_bytes();

/// The byte representation of a tab character.
/// This constant provides the byte equivalent of the tab character
/// for use in low-level operations.
pub const TAB_U8: u8 = TAB_BYTES[0];

/// A line break character (newline).
/// This constant is used to represent a line break character in
/// string or byte operations.
pub const BR: &str = "\n";

/// A const byte slice representation of the string `BR`.
pub const BR_BYTES: &[u8] = BR.as_bytes();

/// The byte representation of a line break character.
/// This constant provides the byte equivalent of the line break character
/// for use in low-level operations.
pub const BR_U8: u8 = BR_BYTES[0];

/// A double line break.
pub const DOUBLE_BR: &str = "\n\n";

/// A const byte slice representation of the string `DOUBLE_BR`.
pub const DOUBLE_BR_BYTES: &[u8] = DOUBLE_BR.as_bytes();

/// A colon followed by a space (`: `).
/// This constant is commonly used in formatted strings, such as
/// headers or key-value pairs, where a colon and a space are needed.
pub const COLON_SPACE: &str = ": ";

/// The byte representation of the first character in the `COLON_SPACE`.
/// This constant provides the byte equivalent of the colon character
/// from the `COLON_SPACE` string.
pub const COLON_SPACE_BYTES: &[u8] = COLON_SPACE.as_bytes();

/// A colon followed by a space symbol (`:`).
/// This constant is commonly used in formatted strings, such as
/// headers or key-value pairs, where a colon and a space are needed.
pub const COLON: &str = ":";

/// A const byte slice representation of the string `COLON`.
pub const COLON_BYTES: &[u8] = COLON.as_bytes();

/// The byte representation of a colon character.
/// This constant provides the byte equivalent of the colon character
/// for use in low-level operations.
pub const COLON_U8: u8 = COLON_BYTES[0];

/// A query symbol (`?`).
/// This constant represents the question mark character, which is
/// commonly used to denote the beginning of a query string in a URL.
pub const QUERY: &str = "?";

/// A const byte slice representation of the string `QUERY`.
pub const QUERY_BYTES: &[u8] = QUERY.as_bytes();

/// The byte representation of a query symbol.
/// This constant provides the byte equivalent of the query character
/// for use in low-level operations.
pub const QUERY_U8: u8 = QUERY_BYTES[0];

/// The string ",".
/// This constant is used to represent a comma.
pub const COMMA: &str = ",";

/// A const byte slice representation of the string `COMMA`.
pub const COMMA_BYTES: &[u8] = COMMA.as_bytes();

/// The byte representation of a comma character.
/// This constant provides the byte equivalent of the comma character
/// for use in low-level operations.
pub const COMMA_U8: u8 = COMMA_BYTES[0];

/// A hash symbol (`#`).
/// This constant represents the hash character, which is used to
/// identify a fragment or anchor in a URL.
pub const HASH: &str = "#";

/// A const byte slice representation of the string `HASH`.
pub const HASH_BYTES: &[u8] = HASH.as_bytes();

/// The byte representation of a hash symbol.
/// This constant provides the byte equivalent of the hash character
/// for use in low-level operations.
pub const HASH_U8: u8 = HASH_BYTES[0];

/// An empty string.
/// This constant represents an empty string, which can be used as a
/// default or placeholder value.
pub const EMPTY_STR: &str = "";

/// A const byte slice representation of the string `EMPTY_STR`.
pub const EMPTY_STR_BYTES: &[u8] = EMPTY_STR.as_bytes();

/// The default host address.
/// This constant represents the default host address, which is typically
/// used to bind a server to all available network interfaces.
pub const DEFAULT_HOST: &str = "0.0.0.0";

/// A const byte slice representation of the string `DEFAULT_HOST`.
pub const DEFAULT_HOST_BYTES: &[u8] = DEFAULT_HOST.as_bytes();

/// The default HTTP port.
/// This constant represents the default port for HTTP traffic.
pub const DEFAULT_HTTP_PORT: u16 = 80;

/// The string representation of the default HTTP port.
/// This constant represents the default port for HTTP traffic as a string.
pub const DEFAULT_HTTP_PORT_STR: &str = "80";

/// The default HTTPS port.
/// This constant represents the default port for HTTPS traffic.
pub const DEFAULT_HTTPS_PORT: u16 = 443;

/// The string representation of the default HTTPS port.
/// This constant represents the default port for HTTPS traffic as a string.
pub const DEFAULT_HTTPS_PORT_STR: &str = "443";

/// The default web port.
/// This constant represents the default port for HTTP traffic.
pub const DEFAULT_WEB_PORT: u16 = DEFAULT_HTTP_PORT;

/// The string representation of the default web port.
/// This constant represents the default port for HTTP traffic as a string.
pub const DEFAULT_WEB_PORT_STR: &str = DEFAULT_HTTP_PORT_STR;

/// An HTTP line break (`\r\n`).
/// This constant represents the standard line break sequence used in
/// the HTTP protocol.
pub const HTTP_BR: &str = "\r\n";

/// The byte representation of an HTTP line break.
/// This constant provides the byte equivalent of the HTTP line break
/// for use in low-level network operations.
pub const HTTP_BR_BYTES: &[u8] = HTTP_BR.as_bytes();

/// A double HTTP line break (`\r\n\r\n`).
/// This constant represents the sequence used to separate headers
/// from the body in an HTTP message.
pub const HTTP_DOUBLE_BR: &str = "\r\n\r\n";

/// The byte representation of a double HTTP line break.
/// This constant provides the byte equivalent of the double HTTP line
/// break for use in parsing and constructing HTTP messages.
pub const HTTP_DOUBLE_BR_BYTES: &[u8] = HTTP_DOUBLE_BR.as_bytes();

/// The default HTTP path.
/// This constant represents the root path of a URL, which is used
/// when no specific path is provided.
pub const DEFAULT_HTTP_PATH: &str = "/";

/// The byte representation of the default HTTP path.
/// This constant provides the byte equivalent of the default HTTP path
/// for use in low-level operations.
pub const DEFAULT_HTTP_PATH_BYTES: &[u8] = DEFAULT_HTTP_PATH.as_bytes();

/// An ampersand character (`&`).
/// This constant represents the ampersand character, which is used
/// to separate query parameters in a URL.
pub const AND: &str = "&";

/// The byte representation of an ampersand character.
/// This constant provides the byte equivalent of the ampersand character
/// for use in URL parsing and construction.
pub const AND_BYTES: &[u8] = AND.as_bytes();

/// The byte representation of an ampersand character.
/// This constant provides the byte equivalent of the ampersand character
/// for use in low-level operations.
pub const AND_U8: u8 = AND_BYTES[0];

/// An equal sign (`=`).
/// This constant represents the equal sign, which is used to separate
/// keys and values in query parameters.
pub const EQUAL: &str = "=";

/// The byte representation of an equal sign.
/// This constant provides the byte equivalent of the equal sign for
/// use in URL parsing and construction.
pub const EQUAL_BYTES: &[u8] = EQUAL.as_bytes();

/// The byte representation of an equal sign.
/// This constant provides the byte equivalent of the equal sign for
/// use in low-level operations.
pub const EQUAL_U8: u8 = EQUAL_BYTES[0];

/// The string representation of the number zero.
/// This constant represents the character '0' as a string.
pub const ZERO_STR: &str = "0";

/// The byte representation of the number zero.
/// This constant provides the byte equivalent of the character '0'.
pub const ZERO_STR_BYTES: &[u8] = ZERO_STR.as_bytes();

/// The byte representation of the number zero.
/// This constant provides the byte equivalent of the character '0'
/// for use in low-level operations.
pub const ZERO_STR_U8: u8 = ZERO_STR_BYTES[0];

/// The default buffer size.
/// This constant defines the default size for buffers used in I/O
/// operations, such as reading from a network stream.
pub const DEFAULT_BUFFER_SIZE: usize = KB_4;

/// The default maximum path size.
/// This constant defines the maximum size of the path component
/// in an HTTP request to prevent excessive memory usage.
pub const DEFAULT_MAX_PATH_SIZE: usize = KB_8;

/// The default maximum header count.
/// This constant defines the maximum number of HTTP headers allowed
/// in a single request to prevent resource exhaustion.
pub const DEFAULT_MAX_HEADER_COUNT: usize = 100;

/// The default maximum header key size.
/// This constant defines the maximum size of an HTTP header key
/// to prevent excessive memory usage and potential attacks.
pub const DEFAULT_MAX_HEADER_KEY_SIZE: usize = KB_8;

/// The default maximum header value size.
/// This constant defines the maximum size of an HTTP header value
/// to prevent excessive memory usage and potential attacks.
pub const DEFAULT_MAX_HEADER_VALUE_SIZE: usize = KB_8;

/// The default maximum body size.
/// This constant defines the maximum size of an HTTP request body
/// to prevent excessive memory usage and potential denial of service attacks.
pub const DEFAULT_MAX_BODY_SIZE: usize = MB_2;

/// The default read timeout in milliseconds.
/// This constant defines the maximum time to wait for an read operation
/// before timing out, helping to prevent hanging connections.
pub const DEFAULT_READ_TIMEOUT_MS: u64 = 6000;

// Low security HTTP request configuration constants
/// The low security buffer size.
/// This constant defines a permissive buffer size for low security
/// configurations to maximize compatibility and performance.
pub const DEFAULT_LOW_SECURITY_BUFFER_SIZE: usize = KB_4;

/// The low security maximum path size.
/// This constant defines a permissive limit on path size
/// in low security mode to maximize compatibility.
pub const DEFAULT_LOW_SECURITY_MAX_PATH_SIZE: usize = usize::MAX;

/// The low security maximum header count.
/// This constant defines a permissive limit on header count
/// in low security mode to maximize compatibility.
pub const DEFAULT_LOW_SECURITY_MAX_HEADER_COUNT: usize = usize::MAX;

/// The low security maximum header key size.
/// This constant defines a permissive limit on header key size
/// in low security mode to maximize compatibility.
pub const DEFAULT_LOW_SECURITY_MAX_HEADER_KEY_SIZE: usize = usize::MAX;

/// The low security maximum header value size.
/// This constant defines a permissive limit on header value size
/// in low security mode to maximize compatibility.
pub const DEFAULT_LOW_SECURITY_MAX_HEADER_VALUE_SIZE: usize = usize::MAX;

/// The low security maximum body size.
/// This constant defines a permissive limit on request body size
/// in low security mode to maximize compatibility.
pub const DEFAULT_LOW_SECURITY_MAX_BODY_SIZE: usize = usize::MAX;

/// The low security read timeout in milliseconds.
/// This constant defines a permissive timeout for read operations in low
/// security mode to maximize compatibility.
pub const DEFAULT_LOW_SECURITY_READ_TIMEOUT_MS: u64 = u64::MAX;

// High security HTTP request configuration constants
/// The high security buffer size.
/// This constant defines a more restrictive buffer size for high security
/// configurations to reduce potential attack surface.
pub const DEFAULT_HIGH_SECURITY_BUFFER_SIZE: usize = KB_4;

/// The high security maximum path size.
/// This constant defines a more restrictive limit on path size
/// in high security mode to reduce potential attack surface.
pub const DEFAULT_HIGH_SECURITY_MAX_PATH_SIZE: usize = KB_4;

/// The high security maximum header count.
/// This constant defines a more restrictive limit on header count
/// in high security mode to reduce potential attack surface.
pub const DEFAULT_HIGH_SECURITY_MAX_HEADER_COUNT: usize = 50;

/// The high security maximum header key size.
/// This constant defines a more restrictive limit on header key size
/// in high security mode to reduce potential attack surface.
pub const DEFAULT_HIGH_SECURITY_MAX_HEADER_KEY_SIZE: usize = KB_4;

/// The high security maximum header value size.
/// This constant defines a more restrictive limit on header value size
/// in high security mode to reduce potential attack surface.
pub const DEFAULT_HIGH_SECURITY_MAX_HEADER_VALUE_SIZE: usize = KB_4;

/// The high security maximum body size.
/// This constant defines a more restrictive limit on request body size
/// in high security mode to reduce potential attack surface.
pub const DEFAULT_HIGH_SECURITY_MAX_BODY_SIZE: usize = MB_1;

/// The default read timeout in milliseconds for high security configurations.
/// This constant defines a shorter timeout for read operations in high
/// security mode to reduce the window of opportunity for certain attacks.
pub const DEFAULT_HIGH_SECURITY_READ_TIMEOUT_MS: u64 = 3000;

/// The default maximum number of redirect times.
/// This constant specifies the default limit for the number of times
/// an HTTP client should follow a redirect.
pub const DEFAULT_MAX_REDIRECT_TIMES: usize = 8;

/// A point character (`.`).
/// This constant represents the period or dot character, which is
/// often used as a separator in file names or domain names.
pub const POINT: &str = ".";

/// A const byte slice representation of the string `POINT`.
pub const POINT_BYTES: &[u8] = POINT.as_bytes();

/// The byte representation of a point character.
/// This constant provides the byte equivalent of the point character
/// for use in low-level operations.
pub const POINT_U8: u8 = POINT_BYTES[0];

/// The root path.
/// This constant represents the root path in a file system or URL.
pub const ROOT_PATH: &str = "/";

/// A const byte slice representation of the string `ROOT_PATH`.
pub const ROOT_PATH_BYTES: &[u8] = ROOT_PATH.as_bytes();

/// The byte representation of the root path character.
/// This constant provides the byte equivalent of the root path character
/// for use in low-level operations.
pub const ROOT_PATH_U8: u8 = ROOT_PATH_BYTES[0];

/// A semicolon character (`;`).
/// This constant represents the semicolon character, which is used
/// as a separator in various contexts.
pub const SEMICOLON: &str = ";";

/// A const byte slice representation of the string `SEMICOLON`.
pub const SEMICOLON_BYTES: &[u8] = SEMICOLON.as_bytes();

/// The byte representation of a semicolon character.
/// This constant provides the byte equivalent of the semicolon character
/// for use in low-level operations.
pub const SEMICOLON_U8: u8 = SEMICOLON_BYTES[0];

/// A semicolon followed by a space (`; `).
/// This constant represents a semicolon followed by a space, which is
/// commonly used as a separator in formatted text.
pub const SEMICOLON_SPACE: &str = "; ";

/// Represents a timestamp or time field.
pub const TIME: &str = "time";

/// The `pub` keyword in Rust.
pub const PUB: &str = "pub";

/// The `pub(super)` visibility modifier in Rust.
pub const PUB_SUPER: &str = "pub(super)";

/// The `pub(crate)` visibility modifier in Rust.
pub const PUB_CRATE: &str = "pub(crate)";

/// Represents a URL (Uniform Resource Locator) string.
pub const URL: &str = "url";

/// A const byte slice representation of the string `SEMICOLON_SPACE`.
pub const SEMICOLON_SPACE_BYTES: &[u8] = SEMICOLON_SPACE.as_bytes();

/// A globally unique identifier (GUID) for WebSocket connections.
/// This constant is used in the WebSocket handshake process to create
/// the `Sec-WebSocket-Accept` header.
pub const GUID: &[u8; 36] = b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11";

/// The initial hash state for SHA-1.
/// This constant represents the initial values used in the SHA-1/// hashing algorithm.
pub const HASH_STATE: [u32; 5] = [
    0x67452301u32,
    0xEFCDAB89,
    0x98BADCFE,
    0x10325476,
    0xC3D2E1F0,
];

/// The Base64 character set table.
/// This constant contains the characters used for Base64 encoding.
pub const BASE64_CHARSET_TABLE: &[u8] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";

/// The maximum frame size for WebSockets.
/// This constant defines the maximum size of a WebSocket frame that
/// can be processed.
pub const MAX_FRAME_SIZE: usize = 65535;

/// Connection close frame (FIN=1, OPCODE=0x8, payload len=0)
pub const CLOSE_FRAME: [u8; 2] = [0x88, 0x00];

/// Ping frame (FIN=1, OPCODE=0x9, payload len=0)
pub const PING_FRAME: [u8; 2] = [0x89, 0x00];

/// Pong frame (FIN=1, OPCODE=0xA, payload len=0)
pub const PONG_FRAME: [u8; 2] = [0x8A, 0x00];

/// The maximum number of attempts to decode a UTF-8 character.
/// This constant specifies the maximum number of bytes that can be
/// part of a single UTF-8 character.
pub const MAX_UTF8_ATTEMPTS: usize = 4;

/// The default socket IPv4 host address.
pub const DEFAULT_HOST_IPV4_ADDR: Ipv4Addr = Ipv4Addr::new(0, 0, 0, 0);

/// The default socket IPv6 host address.
pub const DEFAULT_HOST_IPV6_ADDR: Ipv6Addr = Ipv6Addr::new(0, 0, 0, 0, 0, 0, 0, 0);

/// The default IPV4 address.
pub const DEFAULT_IPV4_ADDR: IpAddr = IpAddr::V4(DEFAULT_HOST_IPV4_ADDR);

/// The default IPV6 address.
pub const DEFAULT_IPV6_ADDR: IpAddr = IpAddr::V6(DEFAULT_HOST_IPV6_ADDR);

/// The default IP address.
pub const DEFAULT_IP_ADDR: IpAddr = IpAddr::V4(DEFAULT_HOST_IPV4_ADDR);

/// The maximum number of attempts to decode a UTF-8 character.
/// This constant specifies the maximum number of bytes that can be
/// part of a single UTF-8 character.
pub const DEFAULT_SOCKET_IPV4_ADDR: SocketAddrV4 =
    SocketAddrV4::new(DEFAULT_HOST_IPV4_ADDR, DEFAULT_WEB_PORT);

/// The default socket address.
/// This constant represents a default, unspecified socket address.
pub const DEFAULT_SOCKET_IPV6_ADDR: SocketAddrV6 =
    SocketAddrV6::new(DEFAULT_HOST_IPV6_ADDR, DEFAULT_WEB_PORT, 0, 0);

/// The default socket address.
/// This constant represents a default, unspecified socket address.
pub const DEFAULT_SOCKET_ADDR: SocketAddr =
    SocketAddr::V4(SocketAddrV4::new(DEFAULT_HOST_IPV4_ADDR, DEFAULT_WEB_PORT));

/// The loopback socket address (127.0.0.1).
/// This constant represents the loopback address, which is used for local network communication.
pub const SOCKET_ADDR_127_0_0_1: SocketAddr = SocketAddr::V4(SocketAddrV4::new(
    Ipv4Addr::new(127, 0, 0, 1),
    DEFAULT_WEB_PORT,
));

/// The string "hyperlane".
/// This constant is used for identification or naming purposes.
pub const HYPERLANE: &str = "hyperlane";

/// A const byte slice representation of the string `HYPERLANE`.
pub const HYPERLANE_BYTES: &[u8] = HYPERLANE.as_bytes();

/// The string "Hyperlane" in PascalCase.
/// This constant is a PascalCase version of the "hyperlane" string.
pub const HYPERLANE_PASCAL_CASE: &str = "Hyperlane";

/// A const byte slice representation of the string `HYPERLANE_PASCAL_CASE`.
pub const HYPERLANE_PASCAL_CASE_BYTES: &[u8] = HYPERLANE_PASCAL_CASE.as_bytes();

/// The string "Hyperlane" in UpperCase.
/// This constant is a UpperCase version of the "hyperlane" string.
pub const HYPERLANE_UPPERCASE: &str = "HYPERLANE";

/// A const byte slice representation of the string `HYPERLANE_UPPERCASE`.
pub const HYPERLANE_UPPERCASE_BYTES: &[u8] = HYPERLANE_UPPERCASE.as_bytes();

/// The default setting for inner printing.
/// This constant determines whether internal printing is enabled by
/// default.
pub const DEFAULT_INNER_PRINT: bool = true;

/// The default setting for inner logging.
/// This constant determines whether internal logging is enabled by
/// default.
pub const DEFAULT_INNER_LOG: bool = true;

/// The default setting for TCP_NODELAY.
/// This constant specifies the default value for the `TCP_NODELAY`
/// socket option.
pub const DEFAULT_NODELAY: Option<bool> = None;

/// The default setting for socket linger.
/// This constant specifies the default value for the `SO_LINGER`
/// socket option.
pub const DEFAULT_LINGER: Option<Duration> = None;

/// Default value for `SO_REUSEADDR` when no explicit configuration is provided.
pub const DEFAULT_REUSE_ADDRESS: Option<bool> = None;

/// Default listen backlog (maximum number of pending connections) passed to
/// the underlying `listen()` syscall when no explicit configuration is provided.
pub const DEFAULT_LISTEN_BACKLOG: Option<i32> = None;

/// Default non-blocking mode for the listening socket when no explicit
/// configuration is provided. Required for handoff to the tokio reactor.
pub const DEFAULT_NONBLOCKING: Option<bool> = None;

/// The default time-to-live (TTL) setting.
/// This constant specifies the default value for the IP_TTL socket
/// option.
pub const DEFAULT_TTI: Option<u32> = None;

/// The string "warning".
/// This constant is used to represent a warning message type.
pub const WARNING: &str = "warning";

/// A const byte slice representation of the string `WARNING`.
pub const WARNING_BYTES: &[u8] = WARNING.as_bytes();

/// The string "success".
/// This constant is used to represent a success message type.
pub const SUCCESS: &str = "success";

/// A const byte slice representation of the string `SUCCESS`.
pub const SUCCESS_BYTES: &[u8] = SUCCESS.as_bytes();

/// The string "fail".
/// This constant is used to represent a failure message type.
pub const FAIL: &str = "fail";

/// A const byte slice representation of the string `FAIL`.
pub const FAIL_BYTES: &[u8] = FAIL.as_bytes();

/// The string "error".
/// This constant is used to represent an error message type.
pub const ERROR: &str = "error";

/// A const byte slice representation of the string `ERROR`.
pub const ERROR_BYTES: &[u8] = ERROR.as_bytes();

/// The string "info".
/// This constant is used to represent an informational message type.
pub const INFO: &str = "info";

/// A const byte slice representation of the string `INFO`.
pub const INFO_BYTES: &[u8] = INFO.as_bytes();

/// The string "debug".
/// This constant is used to represent a debug message type.
pub const DEBUG: &str = "debug";

/// A const byte slice representation of the string `DEBUG`.
pub const DEBUG_BYTES: &[u8] = DEBUG.as_bytes();

/// The string "plain".
/// This constant is used to represent plain text content.
pub const PLAIN: &str = "plain";

/// A const byte slice representation of the string `PLAIN`.
pub const PLAIN_BYTES: &[u8] = PLAIN.as_bytes();

/// The string "binary".
/// This constant is used to represent binary content.
pub const BINARY: &str = "binary";

/// A const byte slice representation of the string `BINARY`.
pub const BINARY_BYTES: &[u8] = BINARY.as_bytes();

/// The string "{".
/// This constant is used to represent a left bracket.
pub const LEFT_BRACKET: &str = "{";

/// A const byte slice representation of the string `LEFT_BRACKET`.
pub const LEFT_BRACKET_BYTES: &[u8] = LEFT_BRACKET.as_bytes();

/// The byte representation of a left bracket character.
/// This constant provides the byte equivalent of the left bracket character
/// for use in low-level operations.
pub const LEFT_BRACKET_U8: u8 = LEFT_BRACKET_BYTES[0];

/// The string "}".
/// This constant is used to represent a right bracket.
pub const RIGHT_BRACKET: &str = "}";

/// A const byte slice representation of the string `RIGHT_BRACKET`.
pub const RIGHT_BRACKET_BYTES: &[u8] = RIGHT_BRACKET.as_bytes();

/// The byte representation of a right bracket character.
/// This constant provides the byte equivalent of the right bracket character
/// for use in low-level operations.
pub const RIGHT_BRACKET_U8: u8 = RIGHT_BRACKET_BYTES[0];

/// The string "(":
/// This constant is used to represent a left parenthesis.
pub const LEFT_PAREN: &str = "(";

/// A const byte slice representation of the string `LEFT_PAREN`.
pub const LEFT_PAREN_BYTES: &[u8] = LEFT_PAREN.as_bytes();

/// The byte representation of a left parenthesis character.
/// This constant provides the byte equivalent of the left parenthesis character
/// for use in low-level operations.
pub const LEFT_PAREN_U8: u8 = LEFT_PAREN_BYTES[0];

/// The string ")".
/// This constant is used to represent a right parenthesis.
pub const RIGHT_PAREN: &str = ")";

/// A const byte slice representation of the string `RIGHT_PAREN`.
pub const RIGHT_PAREN_BYTES: &[u8] = RIGHT_PAREN.as_bytes();

/// The byte representation of a right parenthesis character.
/// This constant provides the byte equivalent of the right parenthesis character
/// for use in low-level operations.
pub const RIGHT_PAREN_U8: u8 = RIGHT_PAREN_BYTES[0];

/// The string "[".
/// This constant is used to represent a left square bracket.
pub const LEFT_SQUARE_BRACKET: &str = "[";

/// A const byte slice representation of the string `LEFT_SQUARE_BRACKET`.
pub const LEFT_SQUARE_BRACKET_BYTES: &[u8] = LEFT_SQUARE_BRACKET.as_bytes();

/// The byte representation of a left square bracket character.
/// This constant provides the byte equivalent of the left square bracket character
/// for use in low-level operations.
pub const LEFT_SQUARE_BRACKET_U8: u8 = LEFT_SQUARE_BRACKET_BYTES[0];

/// The string "]".
/// This constant is used to represent a right square bracket.
pub const RIGHT_SQUARE_BRACKET: &str = "]";

/// A const byte slice representation of the string `RIGHT_SQUARE_BRACKET`.
pub const RIGHT_SQUARE_BRACKET_BYTES: &[u8] = RIGHT_SQUARE_BRACKET.as_bytes();

/// The byte representation of a right square bracket character.
/// This constant provides the byte equivalent of the right square bracket character
/// for use in low-level operations.
pub const RIGHT_SQUARE_BRACKET_U8: u8 = RIGHT_SQUARE_BRACKET_BYTES[0];

/// localhost
/// This constant is used to represent the localhost address.
pub const LOCALHOST: &str = "localhost";

/// A const byte slice representation of the string `LOCALHOST`.
pub const LOCALHOST_BYTES: &[u8] = LOCALHOST.as_bytes();

/// 127.0.0.1
/// This constant is used to represent the loopback address.
pub const LOOPBACK: &str = "127.0.0.1";

/// A const byte slice representation of the string `LOOPBACK`.
pub const LOOPBACK_BYTES: &[u8] = LOOPBACK.as_bytes();
// Storage unit constants from 1B to 1024B
/// 1B in bytes.
/// This constant represents 1 b converted to bytes for use in size calculations.
pub const B_1: usize = 1;

/// 2B in bytes.
/// This constant represents 2 b converted to bytes for use in size calculations.
pub const B_2: usize = 2;

/// 3B in bytes.
/// This constant represents 3 b converted to bytes for use in size calculations.
pub const B_3: usize = 3;

/// 4B in bytes.
/// This constant represents 4 b converted to bytes for use in size calculations.
pub const B_4: usize = 4;

/// 5B in bytes.
/// This constant represents 5 b converted to bytes for use in size calculations.
pub const B_5: usize = 5;

/// 6B in bytes.
/// This constant represents 6 b converted to bytes for use in size calculations.
pub const B_6: usize = 6;

/// 7B in bytes.
/// This constant represents 7 b converted to bytes for use in size calculations.
pub const B_7: usize = 7;

/// 8B in bytes.
/// This constant represents 8 b converted to bytes for use in size calculations.
pub const B_8: usize = 8;

/// 9B in bytes.
/// This constant represents 9 b converted to bytes for use in size calculations.
pub const B_9: usize = 9;

/// 10B in bytes.
/// This constant represents 10 b converted to bytes for use in size calculations.
pub const B_10: usize = 10;

/// 11B in bytes.
/// This constant represents 11 b converted to bytes for use in size calculations.
pub const B_11: usize = 11;

/// 12B in bytes.
/// This constant represents 12 b converted to bytes for use in size calculations.
pub const B_12: usize = 12;

/// 13B in bytes.
/// This constant represents 13 b converted to bytes for use in size calculations.
pub const B_13: usize = 13;

/// 14B in bytes.
/// This constant represents 14 b converted to bytes for use in size calculations.
pub const B_14: usize = 14;

/// 15B in bytes.
/// This constant represents 15 b converted to bytes for use in size calculations.
pub const B_15: usize = 15;

/// 16B in bytes.
/// This constant represents 16 b converted to bytes for use in size calculations.
pub const B_16: usize = 16;

/// 17B in bytes.
/// This constant represents 17 b converted to bytes for use in size calculations.
pub const B_17: usize = 17;

/// 18B in bytes.
/// This constant represents 18 b converted to bytes for use in size calculations.
pub const B_18: usize = 18;

/// 19B in bytes.
/// This constant represents 19 b converted to bytes for use in size calculations.
pub const B_19: usize = 19;

/// 20B in bytes.
/// This constant represents 20 b converted to bytes for use in size calculations.
pub const B_20: usize = 20;

/// 21B in bytes.
/// This constant represents 21 b converted to bytes for use in size calculations.
pub const B_21: usize = 21;

/// 22B in bytes.
/// This constant represents 22 b converted to bytes for use in size calculations.
pub const B_22: usize = 22;

/// 23B in bytes.
/// This constant represents 23 b converted to bytes for use in size calculations.
pub const B_23: usize = 23;

/// 24B in bytes.
/// This constant represents 24 b converted to bytes for use in size calculations.
pub const B_24: usize = 24;

/// 25B in bytes.
/// This constant represents 25 b converted to bytes for use in size calculations.
pub const B_25: usize = 25;

/// 26B in bytes.
/// This constant represents 26 b converted to bytes for use in size calculations.
pub const B_26: usize = 26;

/// 27B in bytes.
/// This constant represents 27 b converted to bytes for use in size calculations.
pub const B_27: usize = 27;

/// 28B in bytes.
/// This constant represents 28 b converted to bytes for use in size calculations.
pub const B_28: usize = 28;

/// 29B in bytes.
/// This constant represents 29 b converted to bytes for use in size calculations.
pub const B_29: usize = 29;

/// 30B in bytes.
/// This constant represents 30 b converted to bytes for use in size calculations.
pub const B_30: usize = 30;

/// 31B in bytes.
/// This constant represents 31 b converted to bytes for use in size calculations.
pub const B_31: usize = 31;

/// 32B in bytes.
/// This constant represents 32 b converted to bytes for use in size calculations.
pub const B_32: usize = 32;

/// 33B in bytes.
/// This constant represents 33 b converted to bytes for use in size calculations.
pub const B_33: usize = 33;

/// 34B in bytes.
/// This constant represents 34 b converted to bytes for use in size calculations.
pub const B_34: usize = 34;

/// 35B in bytes.
/// This constant represents 35 b converted to bytes for use in size calculations.
pub const B_35: usize = 35;

/// 36B in bytes.
/// This constant represents 36 b converted to bytes for use in size calculations.
pub const B_36: usize = 36;

/// 37B in bytes.
/// This constant represents 37 b converted to bytes for use in size calculations.
pub const B_37: usize = 37;

/// 38B in bytes.
/// This constant represents 38 b converted to bytes for use in size calculations.
pub const B_38: usize = 38;

/// 39B in bytes.
/// This constant represents 39 b converted to bytes for use in size calculations.
pub const B_39: usize = 39;

/// 40B in bytes.
/// This constant represents 40 b converted to bytes for use in size calculations.
pub const B_40: usize = 40;

/// 41B in bytes.
/// This constant represents 41 b converted to bytes for use in size calculations.
pub const B_41: usize = 41;

/// 42B in bytes.
/// This constant represents 42 b converted to bytes for use in size calculations.
pub const B_42: usize = 42;

/// 43B in bytes.
/// This constant represents 43 b converted to bytes for use in size calculations.
pub const B_43: usize = 43;

/// 44B in bytes.
/// This constant represents 44 b converted to bytes for use in size calculations.
pub const B_44: usize = 44;

/// 45B in bytes.
/// This constant represents 45 b converted to bytes for use in size calculations.
pub const B_45: usize = 45;

/// 46B in bytes.
/// This constant represents 46 b converted to bytes for use in size calculations.
pub const B_46: usize = 46;

/// 47B in bytes.
/// This constant represents 47 b converted to bytes for use in size calculations.
pub const B_47: usize = 47;

/// 48B in bytes.
/// This constant represents 48 b converted to bytes for use in size calculations.
pub const B_48: usize = 48;

/// 49B in bytes.
/// This constant represents 49 b converted to bytes for use in size calculations.
pub const B_49: usize = 49;

/// 50B in bytes.
/// This constant represents 50 b converted to bytes for use in size calculations.
pub const B_50: usize = 50;

/// 51B in bytes.
/// This constant represents 51 b converted to bytes for use in size calculations.
pub const B_51: usize = 51;

/// 52B in bytes.
/// This constant represents 52 b converted to bytes for use in size calculations.
pub const B_52: usize = 52;

/// 53B in bytes.
/// This constant represents 53 b converted to bytes for use in size calculations.
pub const B_53: usize = 53;

/// 54B in bytes.
/// This constant represents 54 b converted to bytes for use in size calculations.
pub const B_54: usize = 54;

/// 55B in bytes.
/// This constant represents 55 b converted to bytes for use in size calculations.
pub const B_55: usize = 55;

/// 56B in bytes.
/// This constant represents 56 b converted to bytes for use in size calculations.
pub const B_56: usize = 56;

/// 57B in bytes.
/// This constant represents 57 b converted to bytes for use in size calculations.
pub const B_57: usize = 57;

/// 58B in bytes.
/// This constant represents 58 b converted to bytes for use in size calculations.
pub const B_58: usize = 58;

/// 59B in bytes.
/// This constant represents 59 b converted to bytes for use in size calculations.
pub const B_59: usize = 59;

/// 60B in bytes.
/// This constant represents 60 b converted to bytes for use in size calculations.
pub const B_60: usize = 60;

/// 61B in bytes.
/// This constant represents 61 b converted to bytes for use in size calculations.
pub const B_61: usize = 61;

/// 62B in bytes.
/// This constant represents 62 b converted to bytes for use in size calculations.
pub const B_62: usize = 62;

/// 63B in bytes.
/// This constant represents 63 b converted to bytes for use in size calculations.
pub const B_63: usize = 63;

/// 64B in bytes.
/// This constant represents 64 b converted to bytes for use in size calculations.
pub const B_64: usize = 64;

/// 65B in bytes.
/// This constant represents 65 b converted to bytes for use in size calculations.
pub const B_65: usize = 65;

/// 66B in bytes.
/// This constant represents 66 b converted to bytes for use in size calculations.
pub const B_66: usize = 66;

/// 67B in bytes.
/// This constant represents 67 b converted to bytes for use in size calculations.
pub const B_67: usize = 67;

/// 68B in bytes.
/// This constant represents 68 b converted to bytes for use in size calculations.
pub const B_68: usize = 68;

/// 69B in bytes.
/// This constant represents 69 b converted to bytes for use in size calculations.
pub const B_69: usize = 69;

/// 70B in bytes.
/// This constant represents 70 b converted to bytes for use in size calculations.
pub const B_70: usize = 70;

/// 71B in bytes.
/// This constant represents 71 b converted to bytes for use in size calculations.
pub const B_71: usize = 71;

/// 72B in bytes.
/// This constant represents 72 b converted to bytes for use in size calculations.
pub const B_72: usize = 72;

/// 73B in bytes.
/// This constant represents 73 b converted to bytes for use in size calculations.
pub const B_73: usize = 73;

/// 74B in bytes.
/// This constant represents 74 b converted to bytes for use in size calculations.
pub const B_74: usize = 74;

/// 75B in bytes.
/// This constant represents 75 b converted to bytes for use in size calculations.
pub const B_75: usize = 75;

/// 76B in bytes.
/// This constant represents 76 b converted to bytes for use in size calculations.
pub const B_76: usize = 76;

/// 77B in bytes.
/// This constant represents 77 b converted to bytes for use in size calculations.
pub const B_77: usize = 77;

/// 78B in bytes.
/// This constant represents 78 b converted to bytes for use in size calculations.
pub const B_78: usize = 78;

/// 79B in bytes.
/// This constant represents 79 b converted to bytes for use in size calculations.
pub const B_79: usize = 79;

/// 80B in bytes.
/// This constant represents 80 b converted to bytes for use in size calculations.
pub const B_80: usize = 80;

/// 81B in bytes.
/// This constant represents 81 b converted to bytes for use in size calculations.
pub const B_81: usize = 81;

/// 82B in bytes.
/// This constant represents 82 b converted to bytes for use in size calculations.
pub const B_82: usize = 82;

/// 83B in bytes.
/// This constant represents 83 b converted to bytes for use in size calculations.
pub const B_83: usize = 83;

/// 84B in bytes.
/// This constant represents 84 b converted to bytes for use in size calculations.
pub const B_84: usize = 84;

/// 85B in bytes.
/// This constant represents 85 b converted to bytes for use in size calculations.
pub const B_85: usize = 85;

/// 86B in bytes.
/// This constant represents 86 b converted to bytes for use in size calculations.
pub const B_86: usize = 86;

/// 87B in bytes.
/// This constant represents 87 b converted to bytes for use in size calculations.
pub const B_87: usize = 87;

/// 88B in bytes.
/// This constant represents 88 b converted to bytes for use in size calculations.
pub const B_88: usize = 88;

/// 89B in bytes.
/// This constant represents 89 b converted to bytes for use in size calculations.
pub const B_89: usize = 89;

/// 90B in bytes.
/// This constant represents 90 b converted to bytes for use in size calculations.
pub const B_90: usize = 90;

/// 91B in bytes.
/// This constant represents 91 b converted to bytes for use in size calculations.
pub const B_91: usize = 91;

/// 92B in bytes.
/// This constant represents 92 b converted to bytes for use in size calculations.
pub const B_92: usize = 92;

/// 93B in bytes.
/// This constant represents 93 b converted to bytes for use in size calculations.
pub const B_93: usize = 93;

/// 94B in bytes.
/// This constant represents 94 b converted to bytes for use in size calculations.
pub const B_94: usize = 94;

/// 95B in bytes.
/// This constant represents 95 b converted to bytes for use in size calculations.
pub const B_95: usize = 95;

/// 96B in bytes.
/// This constant represents 96 b converted to bytes for use in size calculations.
pub const B_96: usize = 96;

/// 97B in bytes.
/// This constant represents 97 b converted to bytes for use in size calculations.
pub const B_97: usize = 97;

/// 98B in bytes.
/// This constant represents 98 b converted to bytes for use in size calculations.
pub const B_98: usize = 98;

/// 99B in bytes.
/// This constant represents 99 b converted to bytes for use in size calculations.
pub const B_99: usize = 99;

/// 100B in bytes.
/// This constant represents 100 b converted to bytes for use in size calculations.
pub const B_100: usize = 100;

/// 101B in bytes.
/// This constant represents 101 b converted to bytes for use in size calculations.
pub const B_101: usize = 101;

/// 102B in bytes.
/// This constant represents 102 b converted to bytes for use in size calculations.
pub const B_102: usize = 102;

/// 103B in bytes.
/// This constant represents 103 b converted to bytes for use in size calculations.
pub const B_103: usize = 103;

/// 104B in bytes.
/// This constant represents 104 b converted to bytes for use in size calculations.
pub const B_104: usize = 104;

/// 105B in bytes.
/// This constant represents 105 b converted to bytes for use in size calculations.
pub const B_105: usize = 105;

/// 106B in bytes.
/// This constant represents 106 b converted to bytes for use in size calculations.
pub const B_106: usize = 106;

/// 107B in bytes.
/// This constant represents 107 b converted to bytes for use in size calculations.
pub const B_107: usize = 107;

/// 108B in bytes.
/// This constant represents 108 b converted to bytes for use in size calculations.
pub const B_108: usize = 108;

/// 109B in bytes.
/// This constant represents 109 b converted to bytes for use in size calculations.
pub const B_109: usize = 109;

/// 110B in bytes.
/// This constant represents 110 b converted to bytes for use in size calculations.
pub const B_110: usize = 110;

/// 111B in bytes.
/// This constant represents 111 b converted to bytes for use in size calculations.
pub const B_111: usize = 111;

/// 112B in bytes.
/// This constant represents 112 b converted to bytes for use in size calculations.
pub const B_112: usize = 112;

/// 113B in bytes.
/// This constant represents 113 b converted to bytes for use in size calculations.
pub const B_113: usize = 113;

/// 114B in bytes.
/// This constant represents 114 b converted to bytes for use in size calculations.
pub const B_114: usize = 114;

/// 115B in bytes.
/// This constant represents 115 b converted to bytes for use in size calculations.
pub const B_115: usize = 115;

/// 116B in bytes.
/// This constant represents 116 b converted to bytes for use in size calculations.
pub const B_116: usize = 116;

/// 117B in bytes.
/// This constant represents 117 b converted to bytes for use in size calculations.
pub const B_117: usize = 117;

/// 118B in bytes.
/// This constant represents 118 b converted to bytes for use in size calculations.
pub const B_118: usize = 118;

/// 119B in bytes.
/// This constant represents 119 b converted to bytes for use in size calculations.
pub const B_119: usize = 119;

/// 120B in bytes.
/// This constant represents 120 b converted to bytes for use in size calculations.
pub const B_120: usize = 120;

/// 121B in bytes.
/// This constant represents 121 b converted to bytes for use in size calculations.
pub const B_121: usize = 121;

/// 122B in bytes.
/// This constant represents 122 b converted to bytes for use in size calculations.
pub const B_122: usize = 122;

/// 123B in bytes.
/// This constant represents 123 b converted to bytes for use in size calculations.
pub const B_123: usize = 123;

/// 124B in bytes.
/// This constant represents 124 b converted to bytes for use in size calculations.
pub const B_124: usize = 124;

/// 125B in bytes.
/// This constant represents 125 b converted to bytes for use in size calculations.
pub const B_125: usize = 125;

/// 126B in bytes.
/// This constant represents 126 b converted to bytes for use in size calculations.
pub const B_126: usize = 126;

/// 127B in bytes.
/// This constant represents 127 b converted to bytes for use in size calculations.
pub const B_127: usize = 127;

/// 128B in bytes.
/// This constant represents 128 b converted to bytes for use in size calculations.
pub const B_128: usize = 128;

/// 129B in bytes.
/// This constant represents 129 b converted to bytes for use in size calculations.
pub const B_129: usize = 129;

/// 130B in bytes.
/// This constant represents 130 b converted to bytes for use in size calculations.
pub const B_130: usize = 130;

/// 131B in bytes.
/// This constant represents 131 b converted to bytes for use in size calculations.
pub const B_131: usize = 131;

/// 132B in bytes.
/// This constant represents 132 b converted to bytes for use in size calculations.
pub const B_132: usize = 132;

/// 133B in bytes.
/// This constant represents 133 b converted to bytes for use in size calculations.
pub const B_133: usize = 133;

/// 134B in bytes.
/// This constant represents 134 b converted to bytes for use in size calculations.
pub const B_134: usize = 134;

/// 135B in bytes.
/// This constant represents 135 b converted to bytes for use in size calculations.
pub const B_135: usize = 135;

/// 136B in bytes.
/// This constant represents 136 b converted to bytes for use in size calculations.
pub const B_136: usize = 136;

/// 137B in bytes.
/// This constant represents 137 b converted to bytes for use in size calculations.
pub const B_137: usize = 137;

/// 138B in bytes.
/// This constant represents 138 b converted to bytes for use in size calculations.
pub const B_138: usize = 138;

/// 139B in bytes.
/// This constant represents 139 b converted to bytes for use in size calculations.
pub const B_139: usize = 139;

/// 140B in bytes.
/// This constant represents 140 b converted to bytes for use in size calculations.
pub const B_140: usize = 140;

/// 141B in bytes.
/// This constant represents 141 b converted to bytes for use in size calculations.
pub const B_141: usize = 141;

/// 142B in bytes.
/// This constant represents 142 b converted to bytes for use in size calculations.
pub const B_142: usize = 142;

/// 143B in bytes.
/// This constant represents 143 b converted to bytes for use in size calculations.
pub const B_143: usize = 143;

/// 144B in bytes.
/// This constant represents 144 b converted to bytes for use in size calculations.
pub const B_144: usize = 144;

/// 145B in bytes.
/// This constant represents 145 b converted to bytes for use in size calculations.
pub const B_145: usize = 145;

/// 146B in bytes.
/// This constant represents 146 b converted to bytes for use in size calculations.
pub const B_146: usize = 146;

/// 147B in bytes.
/// This constant represents 147 b converted to bytes for use in size calculations.
pub const B_147: usize = 147;

/// 148B in bytes.
/// This constant represents 148 b converted to bytes for use in size calculations.
pub const B_148: usize = 148;

/// 149B in bytes.
/// This constant represents 149 b converted to bytes for use in size calculations.
pub const B_149: usize = 149;

/// 150B in bytes.
/// This constant represents 150 b converted to bytes for use in size calculations.
pub const B_150: usize = 150;

/// 151B in bytes.
/// This constant represents 151 b converted to bytes for use in size calculations.
pub const B_151: usize = 151;

/// 152B in bytes.
/// This constant represents 152 b converted to bytes for use in size calculations.
pub const B_152: usize = 152;

/// 153B in bytes.
/// This constant represents 153 b converted to bytes for use in size calculations.
pub const B_153: usize = 153;

/// 154B in bytes.
/// This constant represents 154 b converted to bytes for use in size calculations.
pub const B_154: usize = 154;

/// 155B in bytes.
/// This constant represents 155 b converted to bytes for use in size calculations.
pub const B_155: usize = 155;

/// 156B in bytes.
/// This constant represents 156 b converted to bytes for use in size calculations.
pub const B_156: usize = 156;

/// 157B in bytes.
/// This constant represents 157 b converted to bytes for use in size calculations.
pub const B_157: usize = 157;

/// 158B in bytes.
/// This constant represents 158 b converted to bytes for use in size calculations.
pub const B_158: usize = 158;

/// 159B in bytes.
/// This constant represents 159 b converted to bytes for use in size calculations.
pub const B_159: usize = 159;

/// 160B in bytes.
/// This constant represents 160 b converted to bytes for use in size calculations.
pub const B_160: usize = 160;

/// 161B in bytes.
/// This constant represents 161 b converted to bytes for use in size calculations.
pub const B_161: usize = 161;

/// 162B in bytes.
/// This constant represents 162 b converted to bytes for use in size calculations.
pub const B_162: usize = 162;

/// 163B in bytes.
/// This constant represents 163 b converted to bytes for use in size calculations.
pub const B_163: usize = 163;

/// 164B in bytes.
/// This constant represents 164 b converted to bytes for use in size calculations.
pub const B_164: usize = 164;

/// 165B in bytes.
/// This constant represents 165 b converted to bytes for use in size calculations.
pub const B_165: usize = 165;

/// 166B in bytes.
/// This constant represents 166 b converted to bytes for use in size calculations.
pub const B_166: usize = 166;

/// 167B in bytes.
/// This constant represents 167 b converted to bytes for use in size calculations.
pub const B_167: usize = 167;

/// 168B in bytes.
/// This constant represents 168 b converted to bytes for use in size calculations.
pub const B_168: usize = 168;

/// 169B in bytes.
/// This constant represents 169 b converted to bytes for use in size calculations.
pub const B_169: usize = 169;

/// 170B in bytes.
/// This constant represents 170 b converted to bytes for use in size calculations.
pub const B_170: usize = 170;

/// 171B in bytes.
/// This constant represents 171 b converted to bytes for use in size calculations.
pub const B_171: usize = 171;

/// 172B in bytes.
/// This constant represents 172 b converted to bytes for use in size calculations.
pub const B_172: usize = 172;

/// 173B in bytes.
/// This constant represents 173 b converted to bytes for use in size calculations.
pub const B_173: usize = 173;

/// 174B in bytes.
/// This constant represents 174 b converted to bytes for use in size calculations.
pub const B_174: usize = 174;

/// 175B in bytes.
/// This constant represents 175 b converted to bytes for use in size calculations.
pub const B_175: usize = 175;

/// 176B in bytes.
/// This constant represents 176 b converted to bytes for use in size calculations.
pub const B_176: usize = 176;

/// 177B in bytes.
/// This constant represents 177 b converted to bytes for use in size calculations.
pub const B_177: usize = 177;

/// 178B in bytes.
/// This constant represents 178 b converted to bytes for use in size calculations.
pub const B_178: usize = 178;

/// 179B in bytes.
/// This constant represents 179 b converted to bytes for use in size calculations.
pub const B_179: usize = 179;

/// 180B in bytes.
/// This constant represents 180 b converted to bytes for use in size calculations.
pub const B_180: usize = 180;

/// 181B in bytes.
/// This constant represents 181 b converted to bytes for use in size calculations.
pub const B_181: usize = 181;

/// 182B in bytes.
/// This constant represents 182 b converted to bytes for use in size calculations.
pub const B_182: usize = 182;

/// 183B in bytes.
/// This constant represents 183 b converted to bytes for use in size calculations.
pub const B_183: usize = 183;

/// 184B in bytes.
/// This constant represents 184 b converted to bytes for use in size calculations.
pub const B_184: usize = 184;

/// 185B in bytes.
/// This constant represents 185 b converted to bytes for use in size calculations.
pub const B_185: usize = 185;

/// 186B in bytes.
/// This constant represents 186 b converted to bytes for use in size calculations.
pub const B_186: usize = 186;

/// 187B in bytes.
/// This constant represents 187 b converted to bytes for use in size calculations.
pub const B_187: usize = 187;

/// 188B in bytes.
/// This constant represents 188 b converted to bytes for use in size calculations.
pub const B_188: usize = 188;

/// 189B in bytes.
/// This constant represents 189 b converted to bytes for use in size calculations.
pub const B_189: usize = 189;

/// 190B in bytes.
/// This constant represents 190 b converted to bytes for use in size calculations.
pub const B_190: usize = 190;

/// 191B in bytes.
/// This constant represents 191 b converted to bytes for use in size calculations.
pub const B_191: usize = 191;

/// 192B in bytes.
/// This constant represents 192 b converted to bytes for use in size calculations.
pub const B_192: usize = 192;

/// 193B in bytes.
/// This constant represents 193 b converted to bytes for use in size calculations.
pub const B_193: usize = 193;

/// 194B in bytes.
/// This constant represents 194 b converted to bytes for use in size calculations.
pub const B_194: usize = 194;

/// 195B in bytes.
/// This constant represents 195 b converted to bytes for use in size calculations.
pub const B_195: usize = 195;

/// 196B in bytes.
/// This constant represents 196 b converted to bytes for use in size calculations.
pub const B_196: usize = 196;

/// 197B in bytes.
/// This constant represents 197 b converted to bytes for use in size calculations.
pub const B_197: usize = 197;

/// 198B in bytes.
/// This constant represents 198 b converted to bytes for use in size calculations.
pub const B_198: usize = 198;

/// 199B in bytes.
/// This constant represents 199 b converted to bytes for use in size calculations.
pub const B_199: usize = 199;

/// 200B in bytes.
/// This constant represents 200 b converted to bytes for use in size calculations.
pub const B_200: usize = 200;

/// 201B in bytes.
/// This constant represents 201 b converted to bytes for use in size calculations.
pub const B_201: usize = 201;

/// 202B in bytes.
/// This constant represents 202 b converted to bytes for use in size calculations.
pub const B_202: usize = 202;

/// 203B in bytes.
/// This constant represents 203 b converted to bytes for use in size calculations.
pub const B_203: usize = 203;

/// 204B in bytes.
/// This constant represents 204 b converted to bytes for use in size calculations.
pub const B_204: usize = 204;

/// 205B in bytes.
/// This constant represents 205 b converted to bytes for use in size calculations.
pub const B_205: usize = 205;

/// 206B in bytes.
/// This constant represents 206 b converted to bytes for use in size calculations.
pub const B_206: usize = 206;

/// 207B in bytes.
/// This constant represents 207 b converted to bytes for use in size calculations.
pub const B_207: usize = 207;

/// 208B in bytes.
/// This constant represents 208 b converted to bytes for use in size calculations.
pub const B_208: usize = 208;

/// 209B in bytes.
/// This constant represents 209 b converted to bytes for use in size calculations.
pub const B_209: usize = 209;

/// 210B in bytes.
/// This constant represents 210 b converted to bytes for use in size calculations.
pub const B_210: usize = 210;

/// 211B in bytes.
/// This constant represents 211 b converted to bytes for use in size calculations.
pub const B_211: usize = 211;

/// 212B in bytes.
/// This constant represents 212 b converted to bytes for use in size calculations.
pub const B_212: usize = 212;

/// 213B in bytes.
/// This constant represents 213 b converted to bytes for use in size calculations.
pub const B_213: usize = 213;

/// 214B in bytes.
/// This constant represents 214 b converted to bytes for use in size calculations.
pub const B_214: usize = 214;

/// 215B in bytes.
/// This constant represents 215 b converted to bytes for use in size calculations.
pub const B_215: usize = 215;

/// 216B in bytes.
/// This constant represents 216 b converted to bytes for use in size calculations.
pub const B_216: usize = 216;

/// 217B in bytes.
/// This constant represents 217 b converted to bytes for use in size calculations.
pub const B_217: usize = 217;

/// 218B in bytes.
/// This constant represents 218 b converted to bytes for use in size calculations.
pub const B_218: usize = 218;

/// 219B in bytes.
/// This constant represents 219 b converted to bytes for use in size calculations.
pub const B_219: usize = 219;

/// 220B in bytes.
/// This constant represents 220 b converted to bytes for use in size calculations.
pub const B_220: usize = 220;

/// 221B in bytes.
/// This constant represents 221 b converted to bytes for use in size calculations.
pub const B_221: usize = 221;

/// 222B in bytes.
/// This constant represents 222 b converted to bytes for use in size calculations.
pub const B_222: usize = 222;

/// 223B in bytes.
/// This constant represents 223 b converted to bytes for use in size calculations.
pub const B_223: usize = 223;

/// 224B in bytes.
/// This constant represents 224 b converted to bytes for use in size calculations.
pub const B_224: usize = 224;

/// 225B in bytes.
/// This constant represents 225 b converted to bytes for use in size calculations.
pub const B_225: usize = 225;

/// 226B in bytes.
/// This constant represents 226 b converted to bytes for use in size calculations.
pub const B_226: usize = 226;

/// 227B in bytes.
/// This constant represents 227 b converted to bytes for use in size calculations.
pub const B_227: usize = 227;

/// 228B in bytes.
/// This constant represents 228 b converted to bytes for use in size calculations.
pub const B_228: usize = 228;

/// 229B in bytes.
/// This constant represents 229 b converted to bytes for use in size calculations.
pub const B_229: usize = 229;

/// 230B in bytes.
/// This constant represents 230 b converted to bytes for use in size calculations.
pub const B_230: usize = 230;

/// 231B in bytes.
/// This constant represents 231 b converted to bytes for use in size calculations.
pub const B_231: usize = 231;

/// 232B in bytes.
/// This constant represents 232 b converted to bytes for use in size calculations.
pub const B_232: usize = 232;

/// 233B in bytes.
/// This constant represents 233 b converted to bytes for use in size calculations.
pub const B_233: usize = 233;

/// 234B in bytes.
/// This constant represents 234 b converted to bytes for use in size calculations.
pub const B_234: usize = 234;

/// 235B in bytes.
/// This constant represents 235 b converted to bytes for use in size calculations.
pub const B_235: usize = 235;

/// 236B in bytes.
/// This constant represents 236 b converted to bytes for use in size calculations.
pub const B_236: usize = 236;

/// 237B in bytes.
/// This constant represents 237 b converted to bytes for use in size calculations.
pub const B_237: usize = 237;

/// 238B in bytes.
/// This constant represents 238 b converted to bytes for use in size calculations.
pub const B_238: usize = 238;

/// 239B in bytes.
/// This constant represents 239 b converted to bytes for use in size calculations.
pub const B_239: usize = 239;

/// 240B in bytes.
/// This constant represents 240 b converted to bytes for use in size calculations.
pub const B_240: usize = 240;

/// 241B in bytes.
/// This constant represents 241 b converted to bytes for use in size calculations.
pub const B_241: usize = 241;

/// 242B in bytes.
/// This constant represents 242 b converted to bytes for use in size calculations.
pub const B_242: usize = 242;

/// 243B in bytes.
/// This constant represents 243 b converted to bytes for use in size calculations.
pub const B_243: usize = 243;

/// 244B in bytes.
/// This constant represents 244 b converted to bytes for use in size calculations.
pub const B_244: usize = 244;

/// 245B in bytes.
/// This constant represents 245 b converted to bytes for use in size calculations.
pub const B_245: usize = 245;

/// 246B in bytes.
/// This constant represents 246 b converted to bytes for use in size calculations.
pub const B_246: usize = 246;

/// 247B in bytes.
/// This constant represents 247 b converted to bytes for use in size calculations.
pub const B_247: usize = 247;

/// 248B in bytes.
/// This constant represents 248 b converted to bytes for use in size calculations.
pub const B_248: usize = 248;

/// 249B in bytes.
/// This constant represents 249 b converted to bytes for use in size calculations.
pub const B_249: usize = 249;

/// 250B in bytes.
/// This constant represents 250 b converted to bytes for use in size calculations.
pub const B_250: usize = 250;

/// 251B in bytes.
/// This constant represents 251 b converted to bytes for use in size calculations.
pub const B_251: usize = 251;

/// 252B in bytes.
/// This constant represents 252 b converted to bytes for use in size calculations.
pub const B_252: usize = 252;

/// 253B in bytes.
/// This constant represents 253 b converted to bytes for use in size calculations.
pub const B_253: usize = 253;

/// 254B in bytes.
/// This constant represents 254 b converted to bytes for use in size calculations.
pub const B_254: usize = 254;

/// 255B in bytes.
/// This constant represents 255 b converted to bytes for use in size calculations.
pub const B_255: usize = 255;

/// 256B in bytes.
/// This constant represents 256 b converted to bytes for use in size calculations.
pub const B_256: usize = 256;

/// 257B in bytes.
/// This constant represents 257 b converted to bytes for use in size calculations.
pub const B_257: usize = 257;

/// 258B in bytes.
/// This constant represents 258 b converted to bytes for use in size calculations.
pub const B_258: usize = 258;

/// 259B in bytes.
/// This constant represents 259 b converted to bytes for use in size calculations.
pub const B_259: usize = 259;

/// 260B in bytes.
/// This constant represents 260 b converted to bytes for use in size calculations.
pub const B_260: usize = 260;

/// 261B in bytes.
/// This constant represents 261 b converted to bytes for use in size calculations.
pub const B_261: usize = 261;

/// 262B in bytes.
/// This constant represents 262 b converted to bytes for use in size calculations.
pub const B_262: usize = 262;

/// 263B in bytes.
/// This constant represents 263 b converted to bytes for use in size calculations.
pub const B_263: usize = 263;

/// 264B in bytes.
/// This constant represents 264 b converted to bytes for use in size calculations.
pub const B_264: usize = 264;

/// 265B in bytes.
/// This constant represents 265 b converted to bytes for use in size calculations.
pub const B_265: usize = 265;

/// 266B in bytes.
/// This constant represents 266 b converted to bytes for use in size calculations.
pub const B_266: usize = 266;

/// 267B in bytes.
/// This constant represents 267 b converted to bytes for use in size calculations.
pub const B_267: usize = 267;

/// 268B in bytes.
/// This constant represents 268 b converted to bytes for use in size calculations.
pub const B_268: usize = 268;

/// 269B in bytes.
/// This constant represents 269 b converted to bytes for use in size calculations.
pub const B_269: usize = 269;

/// 270B in bytes.
/// This constant represents 270 b converted to bytes for use in size calculations.
pub const B_270: usize = 270;

/// 271B in bytes.
/// This constant represents 271 b converted to bytes for use in size calculations.
pub const B_271: usize = 271;

/// 272B in bytes.
/// This constant represents 272 b converted to bytes for use in size calculations.
pub const B_272: usize = 272;

/// 273B in bytes.
/// This constant represents 273 b converted to bytes for use in size calculations.
pub const B_273: usize = 273;

/// 274B in bytes.
/// This constant represents 274 b converted to bytes for use in size calculations.
pub const B_274: usize = 274;

/// 275B in bytes.
/// This constant represents 275 b converted to bytes for use in size calculations.
pub const B_275: usize = 275;

/// 276B in bytes.
/// This constant represents 276 b converted to bytes for use in size calculations.
pub const B_276: usize = 276;

/// 277B in bytes.
/// This constant represents 277 b converted to bytes for use in size calculations.
pub const B_277: usize = 277;

/// 278B in bytes.
/// This constant represents 278 b converted to bytes for use in size calculations.
pub const B_278: usize = 278;

/// 279B in bytes.
/// This constant represents 279 b converted to bytes for use in size calculations.
pub const B_279: usize = 279;

/// 280B in bytes.
/// This constant represents 280 b converted to bytes for use in size calculations.
pub const B_280: usize = 280;

/// 281B in bytes.
/// This constant represents 281 b converted to bytes for use in size calculations.
pub const B_281: usize = 281;

/// 282B in bytes.
/// This constant represents 282 b converted to bytes for use in size calculations.
pub const B_282: usize = 282;

/// 283B in bytes.
/// This constant represents 283 b converted to bytes for use in size calculations.
pub const B_283: usize = 283;

/// 284B in bytes.
/// This constant represents 284 b converted to bytes for use in size calculations.
pub const B_284: usize = 284;

/// 285B in bytes.
/// This constant represents 285 b converted to bytes for use in size calculations.
pub const B_285: usize = 285;

/// 286B in bytes.
/// This constant represents 286 b converted to bytes for use in size calculations.
pub const B_286: usize = 286;

/// 287B in bytes.
/// This constant represents 287 b converted to bytes for use in size calculations.
pub const B_287: usize = 287;

/// 288B in bytes.
/// This constant represents 288 b converted to bytes for use in size calculations.
pub const B_288: usize = 288;

/// 289B in bytes.
/// This constant represents 289 b converted to bytes for use in size calculations.
pub const B_289: usize = 289;

/// 290B in bytes.
/// This constant represents 290 b converted to bytes for use in size calculations.
pub const B_290: usize = 290;

/// 291B in bytes.
/// This constant represents 291 b converted to bytes for use in size calculations.
pub const B_291: usize = 291;

/// 292B in bytes.
/// This constant represents 292 b converted to bytes for use in size calculations.
pub const B_292: usize = 292;

/// 293B in bytes.
/// This constant represents 293 b converted to bytes for use in size calculations.
pub const B_293: usize = 293;

/// 294B in bytes.
/// This constant represents 294 b converted to bytes for use in size calculations.
pub const B_294: usize = 294;

/// 295B in bytes.
/// This constant represents 295 b converted to bytes for use in size calculations.
pub const B_295: usize = 295;

/// 296B in bytes.
/// This constant represents 296 b converted to bytes for use in size calculations.
pub const B_296: usize = 296;

/// 297B in bytes.
/// This constant represents 297 b converted to bytes for use in size calculations.
pub const B_297: usize = 297;

/// 298B in bytes.
/// This constant represents 298 b converted to bytes for use in size calculations.
pub const B_298: usize = 298;

/// 299B in bytes.
/// This constant represents 299 b converted to bytes for use in size calculations.
pub const B_299: usize = 299;

/// 300B in bytes.
/// This constant represents 300 b converted to bytes for use in size calculations.
pub const B_300: usize = 300;

/// 301B in bytes.
/// This constant represents 301 b converted to bytes for use in size calculations.
pub const B_301: usize = 301;

/// 302B in bytes.
/// This constant represents 302 b converted to bytes for use in size calculations.
pub const B_302: usize = 302;

/// 303B in bytes.
/// This constant represents 303 b converted to bytes for use in size calculations.
pub const B_303: usize = 303;

/// 304B in bytes.
/// This constant represents 304 b converted to bytes for use in size calculations.
pub const B_304: usize = 304;

/// 305B in bytes.
/// This constant represents 305 b converted to bytes for use in size calculations.
pub const B_305: usize = 305;

/// 306B in bytes.
/// This constant represents 306 b converted to bytes for use in size calculations.
pub const B_306: usize = 306;

/// 307B in bytes.
/// This constant represents 307 b converted to bytes for use in size calculations.
pub const B_307: usize = 307;

/// 308B in bytes.
/// This constant represents 308 b converted to bytes for use in size calculations.
pub const B_308: usize = 308;

/// 309B in bytes.
/// This constant represents 309 b converted to bytes for use in size calculations.
pub const B_309: usize = 309;

/// 310B in bytes.
/// This constant represents 310 b converted to bytes for use in size calculations.
pub const B_310: usize = 310;

/// 311B in bytes.
/// This constant represents 311 b converted to bytes for use in size calculations.
pub const B_311: usize = 311;

/// 312B in bytes.
/// This constant represents 312 b converted to bytes for use in size calculations.
pub const B_312: usize = 312;

/// 313B in bytes.
/// This constant represents 313 b converted to bytes for use in size calculations.
pub const B_313: usize = 313;

/// 314B in bytes.
/// This constant represents 314 b converted to bytes for use in size calculations.
pub const B_314: usize = 314;

/// 315B in bytes.
/// This constant represents 315 b converted to bytes for use in size calculations.
pub const B_315: usize = 315;

/// 316B in bytes.
/// This constant represents 316 b converted to bytes for use in size calculations.
pub const B_316: usize = 316;

/// 317B in bytes.
/// This constant represents 317 b converted to bytes for use in size calculations.
pub const B_317: usize = 317;

/// 318B in bytes.
/// This constant represents 318 b converted to bytes for use in size calculations.
pub const B_318: usize = 318;

/// 319B in bytes.
/// This constant represents 319 b converted to bytes for use in size calculations.
pub const B_319: usize = 319;

/// 320B in bytes.
/// This constant represents 320 b converted to bytes for use in size calculations.
pub const B_320: usize = 320;

/// 321B in bytes.
/// This constant represents 321 b converted to bytes for use in size calculations.
pub const B_321: usize = 321;

/// 322B in bytes.
/// This constant represents 322 b converted to bytes for use in size calculations.
pub const B_322: usize = 322;

/// 323B in bytes.
/// This constant represents 323 b converted to bytes for use in size calculations.
pub const B_323: usize = 323;

/// 324B in bytes.
/// This constant represents 324 b converted to bytes for use in size calculations.
pub const B_324: usize = 324;

/// 325B in bytes.
/// This constant represents 325 b converted to bytes for use in size calculations.
pub const B_325: usize = 325;

/// 326B in bytes.
/// This constant represents 326 b converted to bytes for use in size calculations.
pub const B_326: usize = 326;

/// 327B in bytes.
/// This constant represents 327 b converted to bytes for use in size calculations.
pub const B_327: usize = 327;

/// 328B in bytes.
/// This constant represents 328 b converted to bytes for use in size calculations.
pub const B_328: usize = 328;

/// 329B in bytes.
/// This constant represents 329 b converted to bytes for use in size calculations.
pub const B_329: usize = 329;

/// 330B in bytes.
/// This constant represents 330 b converted to bytes for use in size calculations.
pub const B_330: usize = 330;

/// 331B in bytes.
/// This constant represents 331 b converted to bytes for use in size calculations.
pub const B_331: usize = 331;

/// 332B in bytes.
/// This constant represents 332 b converted to bytes for use in size calculations.
pub const B_332: usize = 332;

/// 333B in bytes.
/// This constant represents 333 b converted to bytes for use in size calculations.
pub const B_333: usize = 333;

/// 334B in bytes.
/// This constant represents 334 b converted to bytes for use in size calculations.
pub const B_334: usize = 334;

/// 335B in bytes.
/// This constant represents 335 b converted to bytes for use in size calculations.
pub const B_335: usize = 335;

/// 336B in bytes.
/// This constant represents 336 b converted to bytes for use in size calculations.
pub const B_336: usize = 336;

/// 337B in bytes.
/// This constant represents 337 b converted to bytes for use in size calculations.
pub const B_337: usize = 337;

/// 338B in bytes.
/// This constant represents 338 b converted to bytes for use in size calculations.
pub const B_338: usize = 338;

/// 339B in bytes.
/// This constant represents 339 b converted to bytes for use in size calculations.
pub const B_339: usize = 339;

/// 340B in bytes.
/// This constant represents 340 b converted to bytes for use in size calculations.
pub const B_340: usize = 340;

/// 341B in bytes.
/// This constant represents 341 b converted to bytes for use in size calculations.
pub const B_341: usize = 341;

/// 342B in bytes.
/// This constant represents 342 b converted to bytes for use in size calculations.
pub const B_342: usize = 342;

/// 343B in bytes.
/// This constant represents 343 b converted to bytes for use in size calculations.
pub const B_343: usize = 343;

/// 344B in bytes.
/// This constant represents 344 b converted to bytes for use in size calculations.
pub const B_344: usize = 344;

/// 345B in bytes.
/// This constant represents 345 b converted to bytes for use in size calculations.
pub const B_345: usize = 345;

/// 346B in bytes.
/// This constant represents 346 b converted to bytes for use in size calculations.
pub const B_346: usize = 346;

/// 347B in bytes.
/// This constant represents 347 b converted to bytes for use in size calculations.
pub const B_347: usize = 347;

/// 348B in bytes.
/// This constant represents 348 b converted to bytes for use in size calculations.
pub const B_348: usize = 348;

/// 349B in bytes.
/// This constant represents 349 b converted to bytes for use in size calculations.
pub const B_349: usize = 349;

/// 350B in bytes.
/// This constant represents 350 b converted to bytes for use in size calculations.
pub const B_350: usize = 350;

/// 351B in bytes.
/// This constant represents 351 b converted to bytes for use in size calculations.
pub const B_351: usize = 351;

/// 352B in bytes.
/// This constant represents 352 b converted to bytes for use in size calculations.
pub const B_352: usize = 352;

/// 353B in bytes.
/// This constant represents 353 b converted to bytes for use in size calculations.
pub const B_353: usize = 353;

/// 354B in bytes.
/// This constant represents 354 b converted to bytes for use in size calculations.
pub const B_354: usize = 354;

/// 355B in bytes.
/// This constant represents 355 b converted to bytes for use in size calculations.
pub const B_355: usize = 355;

/// 356B in bytes.
/// This constant represents 356 b converted to bytes for use in size calculations.
pub const B_356: usize = 356;

/// 357B in bytes.
/// This constant represents 357 b converted to bytes for use in size calculations.
pub const B_357: usize = 357;

/// 358B in bytes.
/// This constant represents 358 b converted to bytes for use in size calculations.
pub const B_358: usize = 358;

/// 359B in bytes.
/// This constant represents 359 b converted to bytes for use in size calculations.
pub const B_359: usize = 359;

/// 360B in bytes.
/// This constant represents 360 b converted to bytes for use in size calculations.
pub const B_360: usize = 360;

/// 361B in bytes.
/// This constant represents 361 b converted to bytes for use in size calculations.
pub const B_361: usize = 361;

/// 362B in bytes.
/// This constant represents 362 b converted to bytes for use in size calculations.
pub const B_362: usize = 362;

/// 363B in bytes.
/// This constant represents 363 b converted to bytes for use in size calculations.
pub const B_363: usize = 363;

/// 364B in bytes.
/// This constant represents 364 b converted to bytes for use in size calculations.
pub const B_364: usize = 364;

/// 365B in bytes.
/// This constant represents 365 b converted to bytes for use in size calculations.
pub const B_365: usize = 365;

/// 366B in bytes.
/// This constant represents 366 b converted to bytes for use in size calculations.
pub const B_366: usize = 366;

/// 367B in bytes.
/// This constant represents 367 b converted to bytes for use in size calculations.
pub const B_367: usize = 367;

/// 368B in bytes.
/// This constant represents 368 b converted to bytes for use in size calculations.
pub const B_368: usize = 368;

/// 369B in bytes.
/// This constant represents 369 b converted to bytes for use in size calculations.
pub const B_369: usize = 369;

/// 370B in bytes.
/// This constant represents 370 b converted to bytes for use in size calculations.
pub const B_370: usize = 370;

/// 371B in bytes.
/// This constant represents 371 b converted to bytes for use in size calculations.
pub const B_371: usize = 371;

/// 372B in bytes.
/// This constant represents 372 b converted to bytes for use in size calculations.
pub const B_372: usize = 372;

/// 373B in bytes.
/// This constant represents 373 b converted to bytes for use in size calculations.
pub const B_373: usize = 373;

/// 374B in bytes.
/// This constant represents 374 b converted to bytes for use in size calculations.
pub const B_374: usize = 374;

/// 375B in bytes.
/// This constant represents 375 b converted to bytes for use in size calculations.
pub const B_375: usize = 375;

/// 376B in bytes.
/// This constant represents 376 b converted to bytes for use in size calculations.
pub const B_376: usize = 376;

/// 377B in bytes.
/// This constant represents 377 b converted to bytes for use in size calculations.
pub const B_377: usize = 377;

/// 378B in bytes.
/// This constant represents 378 b converted to bytes for use in size calculations.
pub const B_378: usize = 378;

/// 379B in bytes.
/// This constant represents 379 b converted to bytes for use in size calculations.
pub const B_379: usize = 379;

/// 380B in bytes.
/// This constant represents 380 b converted to bytes for use in size calculations.
pub const B_380: usize = 380;

/// 381B in bytes.
/// This constant represents 381 b converted to bytes for use in size calculations.
pub const B_381: usize = 381;

/// 382B in bytes.
/// This constant represents 382 b converted to bytes for use in size calculations.
pub const B_382: usize = 382;

/// 383B in bytes.
/// This constant represents 383 b converted to bytes for use in size calculations.
pub const B_383: usize = 383;

/// 384B in bytes.
/// This constant represents 384 b converted to bytes for use in size calculations.
pub const B_384: usize = 384;

/// 385B in bytes.
/// This constant represents 385 b converted to bytes for use in size calculations.
pub const B_385: usize = 385;

/// 386B in bytes.
/// This constant represents 386 b converted to bytes for use in size calculations.
pub const B_386: usize = 386;

/// 387B in bytes.
/// This constant represents 387 b converted to bytes for use in size calculations.
pub const B_387: usize = 387;

/// 388B in bytes.
/// This constant represents 388 b converted to bytes for use in size calculations.
pub const B_388: usize = 388;

/// 389B in bytes.
/// This constant represents 389 b converted to bytes for use in size calculations.
pub const B_389: usize = 389;

/// 390B in bytes.
/// This constant represents 390 b converted to bytes for use in size calculations.
pub const B_390: usize = 390;

/// 391B in bytes.
/// This constant represents 391 b converted to bytes for use in size calculations.
pub const B_391: usize = 391;

/// 392B in bytes.
/// This constant represents 392 b converted to bytes for use in size calculations.
pub const B_392: usize = 392;

/// 393B in bytes.
/// This constant represents 393 b converted to bytes for use in size calculations.
pub const B_393: usize = 393;

/// 394B in bytes.
/// This constant represents 394 b converted to bytes for use in size calculations.
pub const B_394: usize = 394;

/// 395B in bytes.
/// This constant represents 395 b converted to bytes for use in size calculations.
pub const B_395: usize = 395;

/// 396B in bytes.
/// This constant represents 396 b converted to bytes for use in size calculations.
pub const B_396: usize = 396;

/// 397B in bytes.
/// This constant represents 397 b converted to bytes for use in size calculations.
pub const B_397: usize = 397;

/// 398B in bytes.
/// This constant represents 398 b converted to bytes for use in size calculations.
pub const B_398: usize = 398;

/// 399B in bytes.
/// This constant represents 399 b converted to bytes for use in size calculations.
pub const B_399: usize = 399;

/// 400B in bytes.
/// This constant represents 400 b converted to bytes for use in size calculations.
pub const B_400: usize = 400;

/// 401B in bytes.
/// This constant represents 401 b converted to bytes for use in size calculations.
pub const B_401: usize = 401;

/// 402B in bytes.
/// This constant represents 402 b converted to bytes for use in size calculations.
pub const B_402: usize = 402;

/// 403B in bytes.
/// This constant represents 403 b converted to bytes for use in size calculations.
pub const B_403: usize = 403;

/// 404B in bytes.
/// This constant represents 404 b converted to bytes for use in size calculations.
pub const B_404: usize = 404;

/// 405B in bytes.
/// This constant represents 405 b converted to bytes for use in size calculations.
pub const B_405: usize = 405;

/// 406B in bytes.
/// This constant represents 406 b converted to bytes for use in size calculations.
pub const B_406: usize = 406;

/// 407B in bytes.
/// This constant represents 407 b converted to bytes for use in size calculations.
pub const B_407: usize = 407;

/// 408B in bytes.
/// This constant represents 408 b converted to bytes for use in size calculations.
pub const B_408: usize = 408;

/// 409B in bytes.
/// This constant represents 409 b converted to bytes for use in size calculations.
pub const B_409: usize = 409;

/// 410B in bytes.
/// This constant represents 410 b converted to bytes for use in size calculations.
pub const B_410: usize = 410;

/// 411B in bytes.
/// This constant represents 411 b converted to bytes for use in size calculations.
pub const B_411: usize = 411;

/// 412B in bytes.
/// This constant represents 412 b converted to bytes for use in size calculations.
pub const B_412: usize = 412;

/// 413B in bytes.
/// This constant represents 413 b converted to bytes for use in size calculations.
pub const B_413: usize = 413;

/// 414B in bytes.
/// This constant represents 414 b converted to bytes for use in size calculations.
pub const B_414: usize = 414;

/// 415B in bytes.
/// This constant represents 415 b converted to bytes for use in size calculations.
pub const B_415: usize = 415;

/// 416B in bytes.
/// This constant represents 416 b converted to bytes for use in size calculations.
pub const B_416: usize = 416;

/// 417B in bytes.
/// This constant represents 417 b converted to bytes for use in size calculations.
pub const B_417: usize = 417;

/// 418B in bytes.
/// This constant represents 418 b converted to bytes for use in size calculations.
pub const B_418: usize = 418;

/// 419B in bytes.
/// This constant represents 419 b converted to bytes for use in size calculations.
pub const B_419: usize = 419;

/// 420B in bytes.
/// This constant represents 420 b converted to bytes for use in size calculations.
pub const B_420: usize = 420;

/// 421B in bytes.
/// This constant represents 421 b converted to bytes for use in size calculations.
pub const B_421: usize = 421;

/// 422B in bytes.
/// This constant represents 422 b converted to bytes for use in size calculations.
pub const B_422: usize = 422;

/// 423B in bytes.
/// This constant represents 423 b converted to bytes for use in size calculations.
pub const B_423: usize = 423;

/// 424B in bytes.
/// This constant represents 424 b converted to bytes for use in size calculations.
pub const B_424: usize = 424;

/// 425B in bytes.
/// This constant represents 425 b converted to bytes for use in size calculations.
pub const B_425: usize = 425;

/// 426B in bytes.
/// This constant represents 426 b converted to bytes for use in size calculations.
pub const B_426: usize = 426;

/// 427B in bytes.
/// This constant represents 427 b converted to bytes for use in size calculations.
pub const B_427: usize = 427;

/// 428B in bytes.
/// This constant represents 428 b converted to bytes for use in size calculations.
pub const B_428: usize = 428;

/// 429B in bytes.
/// This constant represents 429 b converted to bytes for use in size calculations.
pub const B_429: usize = 429;

/// 430B in bytes.
/// This constant represents 430 b converted to bytes for use in size calculations.
pub const B_430: usize = 430;

/// 431B in bytes.
/// This constant represents 431 b converted to bytes for use in size calculations.
pub const B_431: usize = 431;

/// 432B in bytes.
/// This constant represents 432 b converted to bytes for use in size calculations.
pub const B_432: usize = 432;

/// 433B in bytes.
/// This constant represents 433 b converted to bytes for use in size calculations.
pub const B_433: usize = 433;

/// 434B in bytes.
/// This constant represents 434 b converted to bytes for use in size calculations.
pub const B_434: usize = 434;

/// 435B in bytes.
/// This constant represents 435 b converted to bytes for use in size calculations.
pub const B_435: usize = 435;

/// 436B in bytes.
/// This constant represents 436 b converted to bytes for use in size calculations.
pub const B_436: usize = 436;

/// 437B in bytes.
/// This constant represents 437 b converted to bytes for use in size calculations.
pub const B_437: usize = 437;

/// 438B in bytes.
/// This constant represents 438 b converted to bytes for use in size calculations.
pub const B_438: usize = 438;

/// 439B in bytes.
/// This constant represents 439 b converted to bytes for use in size calculations.
pub const B_439: usize = 439;

/// 440B in bytes.
/// This constant represents 440 b converted to bytes for use in size calculations.
pub const B_440: usize = 440;

/// 441B in bytes.
/// This constant represents 441 b converted to bytes for use in size calculations.
pub const B_441: usize = 441;

/// 442B in bytes.
/// This constant represents 442 b converted to bytes for use in size calculations.
pub const B_442: usize = 442;

/// 443B in bytes.
/// This constant represents 443 b converted to bytes for use in size calculations.
pub const B_443: usize = 443;

/// 444B in bytes.
/// This constant represents 444 b converted to bytes for use in size calculations.
pub const B_444: usize = 444;

/// 445B in bytes.
/// This constant represents 445 b converted to bytes for use in size calculations.
pub const B_445: usize = 445;

/// 446B in bytes.
/// This constant represents 446 b converted to bytes for use in size calculations.
pub const B_446: usize = 446;

/// 447B in bytes.
/// This constant represents 447 b converted to bytes for use in size calculations.
pub const B_447: usize = 447;

/// 448B in bytes.
/// This constant represents 448 b converted to bytes for use in size calculations.
pub const B_448: usize = 448;

/// 449B in bytes.
/// This constant represents 449 b converted to bytes for use in size calculations.
pub const B_449: usize = 449;

/// 450B in bytes.
/// This constant represents 450 b converted to bytes for use in size calculations.
pub const B_450: usize = 450;

/// 451B in bytes.
/// This constant represents 451 b converted to bytes for use in size calculations.
pub const B_451: usize = 451;

/// 452B in bytes.
/// This constant represents 452 b converted to bytes for use in size calculations.
pub const B_452: usize = 452;

/// 453B in bytes.
/// This constant represents 453 b converted to bytes for use in size calculations.
pub const B_453: usize = 453;

/// 454B in bytes.
/// This constant represents 454 b converted to bytes for use in size calculations.
pub const B_454: usize = 454;

/// 455B in bytes.
/// This constant represents 455 b converted to bytes for use in size calculations.
pub const B_455: usize = 455;

/// 456B in bytes.
/// This constant represents 456 b converted to bytes for use in size calculations.
pub const B_456: usize = 456;

/// 457B in bytes.
/// This constant represents 457 b converted to bytes for use in size calculations.
pub const B_457: usize = 457;

/// 458B in bytes.
/// This constant represents 458 b converted to bytes for use in size calculations.
pub const B_458: usize = 458;

/// 459B in bytes.
/// This constant represents 459 b converted to bytes for use in size calculations.
pub const B_459: usize = 459;

/// 460B in bytes.
/// This constant represents 460 b converted to bytes for use in size calculations.
pub const B_460: usize = 460;

/// 461B in bytes.
/// This constant represents 461 b converted to bytes for use in size calculations.
pub const B_461: usize = 461;

/// 462B in bytes.
/// This constant represents 462 b converted to bytes for use in size calculations.
pub const B_462: usize = 462;

/// 463B in bytes.
/// This constant represents 463 b converted to bytes for use in size calculations.
pub const B_463: usize = 463;

/// 464B in bytes.
/// This constant represents 464 b converted to bytes for use in size calculations.
pub const B_464: usize = 464;

/// 465B in bytes.
/// This constant represents 465 b converted to bytes for use in size calculations.
pub const B_465: usize = 465;

/// 466B in bytes.
/// This constant represents 466 b converted to bytes for use in size calculations.
pub const B_466: usize = 466;

/// 467B in bytes.
/// This constant represents 467 b converted to bytes for use in size calculations.
pub const B_467: usize = 467;

/// 468B in bytes.
/// This constant represents 468 b converted to bytes for use in size calculations.
pub const B_468: usize = 468;

/// 469B in bytes.
/// This constant represents 469 b converted to bytes for use in size calculations.
pub const B_469: usize = 469;

/// 470B in bytes.
/// This constant represents 470 b converted to bytes for use in size calculations.
pub const B_470: usize = 470;

/// 471B in bytes.
/// This constant represents 471 b converted to bytes for use in size calculations.
pub const B_471: usize = 471;

/// 472B in bytes.
/// This constant represents 472 b converted to bytes for use in size calculations.
pub const B_472: usize = 472;

/// 473B in bytes.
/// This constant represents 473 b converted to bytes for use in size calculations.
pub const B_473: usize = 473;

/// 474B in bytes.
/// This constant represents 474 b converted to bytes for use in size calculations.
pub const B_474: usize = 474;

/// 475B in bytes.
/// This constant represents 475 b converted to bytes for use in size calculations.
pub const B_475: usize = 475;

/// 476B in bytes.
/// This constant represents 476 b converted to bytes for use in size calculations.
pub const B_476: usize = 476;

/// 477B in bytes.
/// This constant represents 477 b converted to bytes for use in size calculations.
pub const B_477: usize = 477;

/// 478B in bytes.
/// This constant represents 478 b converted to bytes for use in size calculations.
pub const B_478: usize = 478;

/// 479B in bytes.
/// This constant represents 479 b converted to bytes for use in size calculations.
pub const B_479: usize = 479;

/// 480B in bytes.
/// This constant represents 480 b converted to bytes for use in size calculations.
pub const B_480: usize = 480;

/// 481B in bytes.
/// This constant represents 481 b converted to bytes for use in size calculations.
pub const B_481: usize = 481;

/// 482B in bytes.
/// This constant represents 482 b converted to bytes for use in size calculations.
pub const B_482: usize = 482;

/// 483B in bytes.
/// This constant represents 483 b converted to bytes for use in size calculations.
pub const B_483: usize = 483;

/// 484B in bytes.
/// This constant represents 484 b converted to bytes for use in size calculations.
pub const B_484: usize = 484;

/// 485B in bytes.
/// This constant represents 485 b converted to bytes for use in size calculations.
pub const B_485: usize = 485;

/// 486B in bytes.
/// This constant represents 486 b converted to bytes for use in size calculations.
pub const B_486: usize = 486;

/// 487B in bytes.
/// This constant represents 487 b converted to bytes for use in size calculations.
pub const B_487: usize = 487;

/// 488B in bytes.
/// This constant represents 488 b converted to bytes for use in size calculations.
pub const B_488: usize = 488;

/// 489B in bytes.
/// This constant represents 489 b converted to bytes for use in size calculations.
pub const B_489: usize = 489;

/// 490B in bytes.
/// This constant represents 490 b converted to bytes for use in size calculations.
pub const B_490: usize = 490;

/// 491B in bytes.
/// This constant represents 491 b converted to bytes for use in size calculations.
pub const B_491: usize = 491;

/// 492B in bytes.
/// This constant represents 492 b converted to bytes for use in size calculations.
pub const B_492: usize = 492;

/// 493B in bytes.
/// This constant represents 493 b converted to bytes for use in size calculations.
pub const B_493: usize = 493;

/// 494B in bytes.
/// This constant represents 494 b converted to bytes for use in size calculations.
pub const B_494: usize = 494;

/// 495B in bytes.
/// This constant represents 495 b converted to bytes for use in size calculations.
pub const B_495: usize = 495;

/// 496B in bytes.
/// This constant represents 496 b converted to bytes for use in size calculations.
pub const B_496: usize = 496;

/// 497B in bytes.
/// This constant represents 497 b converted to bytes for use in size calculations.
pub const B_497: usize = 497;

/// 498B in bytes.
/// This constant represents 498 b converted to bytes for use in size calculations.
pub const B_498: usize = 498;

/// 499B in bytes.
/// This constant represents 499 b converted to bytes for use in size calculations.
pub const B_499: usize = 499;

/// 500B in bytes.
/// This constant represents 500 b converted to bytes for use in size calculations.
pub const B_500: usize = 500;

/// 501B in bytes.
/// This constant represents 501 b converted to bytes for use in size calculations.
pub const B_501: usize = 501;

/// 502B in bytes.
/// This constant represents 502 b converted to bytes for use in size calculations.
pub const B_502: usize = 502;

/// 503B in bytes.
/// This constant represents 503 b converted to bytes for use in size calculations.
pub const B_503: usize = 503;

/// 504B in bytes.
/// This constant represents 504 b converted to bytes for use in size calculations.
pub const B_504: usize = 504;

/// 505B in bytes.
/// This constant represents 505 b converted to bytes for use in size calculations.
pub const B_505: usize = 505;

/// 506B in bytes.
/// This constant represents 506 b converted to bytes for use in size calculations.
pub const B_506: usize = 506;

/// 507B in bytes.
/// This constant represents 507 b converted to bytes for use in size calculations.
pub const B_507: usize = 507;

/// 508B in bytes.
/// This constant represents 508 b converted to bytes for use in size calculations.
pub const B_508: usize = 508;

/// 509B in bytes.
/// This constant represents 509 b converted to bytes for use in size calculations.
pub const B_509: usize = 509;

/// 510B in bytes.
/// This constant represents 510 b converted to bytes for use in size calculations.
pub const B_510: usize = 510;

/// 511B in bytes.
/// This constant represents 511 b converted to bytes for use in size calculations.
pub const B_511: usize = 511;

/// 512B in bytes.
/// This constant represents 512 b converted to bytes for use in size calculations.
pub const B_512: usize = 512;

/// 513B in bytes.
/// This constant represents 513 b converted to bytes for use in size calculations.
pub const B_513: usize = 513;

/// 514B in bytes.
/// This constant represents 514 b converted to bytes for use in size calculations.
pub const B_514: usize = 514;

/// 515B in bytes.
/// This constant represents 515 b converted to bytes for use in size calculations.
pub const B_515: usize = 515;

/// 516B in bytes.
/// This constant represents 516 b converted to bytes for use in size calculations.
pub const B_516: usize = 516;

/// 517B in bytes.
/// This constant represents 517 b converted to bytes for use in size calculations.
pub const B_517: usize = 517;

/// 518B in bytes.
/// This constant represents 518 b converted to bytes for use in size calculations.
pub const B_518: usize = 518;

/// 519B in bytes.
/// This constant represents 519 b converted to bytes for use in size calculations.
pub const B_519: usize = 519;

/// 520B in bytes.
/// This constant represents 520 b converted to bytes for use in size calculations.
pub const B_520: usize = 520;

/// 521B in bytes.
/// This constant represents 521 b converted to bytes for use in size calculations.
pub const B_521: usize = 521;

/// 522B in bytes.
/// This constant represents 522 b converted to bytes for use in size calculations.
pub const B_522: usize = 522;

/// 523B in bytes.
/// This constant represents 523 b converted to bytes for use in size calculations.
pub const B_523: usize = 523;

/// 524B in bytes.
/// This constant represents 524 b converted to bytes for use in size calculations.
pub const B_524: usize = 524;

/// 525B in bytes.
/// This constant represents 525 b converted to bytes for use in size calculations.
pub const B_525: usize = 525;

/// 526B in bytes.
/// This constant represents 526 b converted to bytes for use in size calculations.
pub const B_526: usize = 526;

/// 527B in bytes.
/// This constant represents 527 b converted to bytes for use in size calculations.
pub const B_527: usize = 527;

/// 528B in bytes.
/// This constant represents 528 b converted to bytes for use in size calculations.
pub const B_528: usize = 528;

/// 529B in bytes.
/// This constant represents 529 b converted to bytes for use in size calculations.
pub const B_529: usize = 529;

/// 530B in bytes.
/// This constant represents 530 b converted to bytes for use in size calculations.
pub const B_530: usize = 530;

/// 531B in bytes.
/// This constant represents 531 b converted to bytes for use in size calculations.
pub const B_531: usize = 531;

/// 532B in bytes.
/// This constant represents 532 b converted to bytes for use in size calculations.
pub const B_532: usize = 532;

/// 533B in bytes.
/// This constant represents 533 b converted to bytes for use in size calculations.
pub const B_533: usize = 533;

/// 534B in bytes.
/// This constant represents 534 b converted to bytes for use in size calculations.
pub const B_534: usize = 534;

/// 535B in bytes.
/// This constant represents 535 b converted to bytes for use in size calculations.
pub const B_535: usize = 535;

/// 536B in bytes.
/// This constant represents 536 b converted to bytes for use in size calculations.
pub const B_536: usize = 536;

/// 537B in bytes.
/// This constant represents 537 b converted to bytes for use in size calculations.
pub const B_537: usize = 537;

/// 538B in bytes.
/// This constant represents 538 b converted to bytes for use in size calculations.
pub const B_538: usize = 538;

/// 539B in bytes.
/// This constant represents 539 b converted to bytes for use in size calculations.
pub const B_539: usize = 539;

/// 540B in bytes.
/// This constant represents 540 b converted to bytes for use in size calculations.
pub const B_540: usize = 540;

/// 541B in bytes.
/// This constant represents 541 b converted to bytes for use in size calculations.
pub const B_541: usize = 541;

/// 542B in bytes.
/// This constant represents 542 b converted to bytes for use in size calculations.
pub const B_542: usize = 542;

/// 543B in bytes.
/// This constant represents 543 b converted to bytes for use in size calculations.
pub const B_543: usize = 543;

/// 544B in bytes.
/// This constant represents 544 b converted to bytes for use in size calculations.
pub const B_544: usize = 544;

/// 545B in bytes.
/// This constant represents 545 b converted to bytes for use in size calculations.
pub const B_545: usize = 545;

/// 546B in bytes.
/// This constant represents 546 b converted to bytes for use in size calculations.
pub const B_546: usize = 546;

/// 547B in bytes.
/// This constant represents 547 b converted to bytes for use in size calculations.
pub const B_547: usize = 547;

/// 548B in bytes.
/// This constant represents 548 b converted to bytes for use in size calculations.
pub const B_548: usize = 548;

/// 549B in bytes.
/// This constant represents 549 b converted to bytes for use in size calculations.
pub const B_549: usize = 549;

/// 550B in bytes.
/// This constant represents 550 b converted to bytes for use in size calculations.
pub const B_550: usize = 550;

/// 551B in bytes.
/// This constant represents 551 b converted to bytes for use in size calculations.
pub const B_551: usize = 551;

/// 552B in bytes.
/// This constant represents 552 b converted to bytes for use in size calculations.
pub const B_552: usize = 552;

/// 553B in bytes.
/// This constant represents 553 b converted to bytes for use in size calculations.
pub const B_553: usize = 553;

/// 554B in bytes.
/// This constant represents 554 b converted to bytes for use in size calculations.
pub const B_554: usize = 554;

/// 555B in bytes.
/// This constant represents 555 b converted to bytes for use in size calculations.
pub const B_555: usize = 555;

/// 556B in bytes.
/// This constant represents 556 b converted to bytes for use in size calculations.
pub const B_556: usize = 556;

/// 557B in bytes.
/// This constant represents 557 b converted to bytes for use in size calculations.
pub const B_557: usize = 557;

/// 558B in bytes.
/// This constant represents 558 b converted to bytes for use in size calculations.
pub const B_558: usize = 558;

/// 559B in bytes.
/// This constant represents 559 b converted to bytes for use in size calculations.
pub const B_559: usize = 559;

/// 560B in bytes.
/// This constant represents 560 b converted to bytes for use in size calculations.
pub const B_560: usize = 560;

/// 561B in bytes.
/// This constant represents 561 b converted to bytes for use in size calculations.
pub const B_561: usize = 561;

/// 562B in bytes.
/// This constant represents 562 b converted to bytes for use in size calculations.
pub const B_562: usize = 562;

/// 563B in bytes.
/// This constant represents 563 b converted to bytes for use in size calculations.
pub const B_563: usize = 563;

/// 564B in bytes.
/// This constant represents 564 b converted to bytes for use in size calculations.
pub const B_564: usize = 564;

/// 565B in bytes.
/// This constant represents 565 b converted to bytes for use in size calculations.
pub const B_565: usize = 565;

/// 566B in bytes.
/// This constant represents 566 b converted to bytes for use in size calculations.
pub const B_566: usize = 566;

/// 567B in bytes.
/// This constant represents 567 b converted to bytes for use in size calculations.
pub const B_567: usize = 567;

/// 568B in bytes.
/// This constant represents 568 b converted to bytes for use in size calculations.
pub const B_568: usize = 568;

/// 569B in bytes.
/// This constant represents 569 b converted to bytes for use in size calculations.
pub const B_569: usize = 569;

/// 570B in bytes.
/// This constant represents 570 b converted to bytes for use in size calculations.
pub const B_570: usize = 570;

/// 571B in bytes.
/// This constant represents 571 b converted to bytes for use in size calculations.
pub const B_571: usize = 571;

/// 572B in bytes.
/// This constant represents 572 b converted to bytes for use in size calculations.
pub const B_572: usize = 572;

/// 573B in bytes.
/// This constant represents 573 b converted to bytes for use in size calculations.
pub const B_573: usize = 573;

/// 574B in bytes.
/// This constant represents 574 b converted to bytes for use in size calculations.
pub const B_574: usize = 574;

/// 575B in bytes.
/// This constant represents 575 b converted to bytes for use in size calculations.
pub const B_575: usize = 575;

/// 576B in bytes.
/// This constant represents 576 b converted to bytes for use in size calculations.
pub const B_576: usize = 576;

/// 577B in bytes.
/// This constant represents 577 b converted to bytes for use in size calculations.
pub const B_577: usize = 577;

/// 578B in bytes.
/// This constant represents 578 b converted to bytes for use in size calculations.
pub const B_578: usize = 578;

/// 579B in bytes.
/// This constant represents 579 b converted to bytes for use in size calculations.
pub const B_579: usize = 579;

/// 580B in bytes.
/// This constant represents 580 b converted to bytes for use in size calculations.
pub const B_580: usize = 580;

/// 581B in bytes.
/// This constant represents 581 b converted to bytes for use in size calculations.
pub const B_581: usize = 581;

/// 582B in bytes.
/// This constant represents 582 b converted to bytes for use in size calculations.
pub const B_582: usize = 582;

/// 583B in bytes.
/// This constant represents 583 b converted to bytes for use in size calculations.
pub const B_583: usize = 583;

/// 584B in bytes.
/// This constant represents 584 b converted to bytes for use in size calculations.
pub const B_584: usize = 584;

/// 585B in bytes.
/// This constant represents 585 b converted to bytes for use in size calculations.
pub const B_585: usize = 585;

/// 586B in bytes.
/// This constant represents 586 b converted to bytes for use in size calculations.
pub const B_586: usize = 586;

/// 587B in bytes.
/// This constant represents 587 b converted to bytes for use in size calculations.
pub const B_587: usize = 587;

/// 588B in bytes.
/// This constant represents 588 b converted to bytes for use in size calculations.
pub const B_588: usize = 588;

/// 589B in bytes.
/// This constant represents 589 b converted to bytes for use in size calculations.
pub const B_589: usize = 589;

/// 590B in bytes.
/// This constant represents 590 b converted to bytes for use in size calculations.
pub const B_590: usize = 590;

/// 591B in bytes.
/// This constant represents 591 b converted to bytes for use in size calculations.
pub const B_591: usize = 591;

/// 592B in bytes.
/// This constant represents 592 b converted to bytes for use in size calculations.
pub const B_592: usize = 592;

/// 593B in bytes.
/// This constant represents 593 b converted to bytes for use in size calculations.
pub const B_593: usize = 593;

/// 594B in bytes.
/// This constant represents 594 b converted to bytes for use in size calculations.
pub const B_594: usize = 594;

/// 595B in bytes.
/// This constant represents 595 b converted to bytes for use in size calculations.
pub const B_595: usize = 595;

/// 596B in bytes.
/// This constant represents 596 b converted to bytes for use in size calculations.
pub const B_596: usize = 596;

/// 597B in bytes.
/// This constant represents 597 b converted to bytes for use in size calculations.
pub const B_597: usize = 597;

/// 598B in bytes.
/// This constant represents 598 b converted to bytes for use in size calculations.
pub const B_598: usize = 598;

/// 599B in bytes.
/// This constant represents 599 b converted to bytes for use in size calculations.
pub const B_599: usize = 599;

/// 600B in bytes.
/// This constant represents 600 b converted to bytes for use in size calculations.
pub const B_600: usize = 600;

/// 601B in bytes.
/// This constant represents 601 b converted to bytes for use in size calculations.
pub const B_601: usize = 601;

/// 602B in bytes.
/// This constant represents 602 b converted to bytes for use in size calculations.
pub const B_602: usize = 602;

/// 603B in bytes.
/// This constant represents 603 b converted to bytes for use in size calculations.
pub const B_603: usize = 603;

/// 604B in bytes.
/// This constant represents 604 b converted to bytes for use in size calculations.
pub const B_604: usize = 604;

/// 605B in bytes.
/// This constant represents 605 b converted to bytes for use in size calculations.
pub const B_605: usize = 605;

/// 606B in bytes.
/// This constant represents 606 b converted to bytes for use in size calculations.
pub const B_606: usize = 606;

/// 607B in bytes.
/// This constant represents 607 b converted to bytes for use in size calculations.
pub const B_607: usize = 607;

/// 608B in bytes.
/// This constant represents 608 b converted to bytes for use in size calculations.
pub const B_608: usize = 608;

/// 609B in bytes.
/// This constant represents 609 b converted to bytes for use in size calculations.
pub const B_609: usize = 609;

/// 610B in bytes.
/// This constant represents 610 b converted to bytes for use in size calculations.
pub const B_610: usize = 610;

/// 611B in bytes.
/// This constant represents 611 b converted to bytes for use in size calculations.
pub const B_611: usize = 611;

/// 612B in bytes.
/// This constant represents 612 b converted to bytes for use in size calculations.
pub const B_612: usize = 612;

/// 613B in bytes.
/// This constant represents 613 b converted to bytes for use in size calculations.
pub const B_613: usize = 613;

/// 614B in bytes.
/// This constant represents 614 b converted to bytes for use in size calculations.
pub const B_614: usize = 614;

/// 615B in bytes.
/// This constant represents 615 b converted to bytes for use in size calculations.
pub const B_615: usize = 615;

/// 616B in bytes.
/// This constant represents 616 b converted to bytes for use in size calculations.
pub const B_616: usize = 616;

/// 617B in bytes.
/// This constant represents 617 b converted to bytes for use in size calculations.
pub const B_617: usize = 617;

/// 618B in bytes.
/// This constant represents 618 b converted to bytes for use in size calculations.
pub const B_618: usize = 618;

/// 619B in bytes.
/// This constant represents 619 b converted to bytes for use in size calculations.
pub const B_619: usize = 619;

/// 620B in bytes.
/// This constant represents 620 b converted to bytes for use in size calculations.
pub const B_620: usize = 620;

/// 621B in bytes.
/// This constant represents 621 b converted to bytes for use in size calculations.
pub const B_621: usize = 621;

/// 622B in bytes.
/// This constant represents 622 b converted to bytes for use in size calculations.
pub const B_622: usize = 622;

/// 623B in bytes.
/// This constant represents 623 b converted to bytes for use in size calculations.
pub const B_623: usize = 623;

/// 624B in bytes.
/// This constant represents 624 b converted to bytes for use in size calculations.
pub const B_624: usize = 624;

/// 625B in bytes.
/// This constant represents 625 b converted to bytes for use in size calculations.
pub const B_625: usize = 625;

/// 626B in bytes.
/// This constant represents 626 b converted to bytes for use in size calculations.
pub const B_626: usize = 626;

/// 627B in bytes.
/// This constant represents 627 b converted to bytes for use in size calculations.
pub const B_627: usize = 627;

/// 628B in bytes.
/// This constant represents 628 b converted to bytes for use in size calculations.
pub const B_628: usize = 628;

/// 629B in bytes.
/// This constant represents 629 b converted to bytes for use in size calculations.
pub const B_629: usize = 629;

/// 630B in bytes.
/// This constant represents 630 b converted to bytes for use in size calculations.
pub const B_630: usize = 630;

/// 631B in bytes.
/// This constant represents 631 b converted to bytes for use in size calculations.
pub const B_631: usize = 631;

/// 632B in bytes.
/// This constant represents 632 b converted to bytes for use in size calculations.
pub const B_632: usize = 632;

/// 633B in bytes.
/// This constant represents 633 b converted to bytes for use in size calculations.
pub const B_633: usize = 633;

/// 634B in bytes.
/// This constant represents 634 b converted to bytes for use in size calculations.
pub const B_634: usize = 634;

/// 635B in bytes.
/// This constant represents 635 b converted to bytes for use in size calculations.
pub const B_635: usize = 635;

/// 636B in bytes.
/// This constant represents 636 b converted to bytes for use in size calculations.
pub const B_636: usize = 636;

/// 637B in bytes.
/// This constant represents 637 b converted to bytes for use in size calculations.
pub const B_637: usize = 637;

/// 638B in bytes.
/// This constant represents 638 b converted to bytes for use in size calculations.
pub const B_638: usize = 638;

/// 639B in bytes.
/// This constant represents 639 b converted to bytes for use in size calculations.
pub const B_639: usize = 639;

/// 640B in bytes.
/// This constant represents 640 b converted to bytes for use in size calculations.
pub const B_640: usize = 640;

/// 641B in bytes.
/// This constant represents 641 b converted to bytes for use in size calculations.
pub const B_641: usize = 641;

/// 642B in bytes.
/// This constant represents 642 b converted to bytes for use in size calculations.
pub const B_642: usize = 642;

/// 643B in bytes.
/// This constant represents 643 b converted to bytes for use in size calculations.
pub const B_643: usize = 643;

/// 644B in bytes.
/// This constant represents 644 b converted to bytes for use in size calculations.
pub const B_644: usize = 644;

/// 645B in bytes.
/// This constant represents 645 b converted to bytes for use in size calculations.
pub const B_645: usize = 645;

/// 646B in bytes.
/// This constant represents 646 b converted to bytes for use in size calculations.
pub const B_646: usize = 646;

/// 647B in bytes.
/// This constant represents 647 b converted to bytes for use in size calculations.
pub const B_647: usize = 647;

/// 648B in bytes.
/// This constant represents 648 b converted to bytes for use in size calculations.
pub const B_648: usize = 648;

/// 649B in bytes.
/// This constant represents 649 b converted to bytes for use in size calculations.
pub const B_649: usize = 649;

/// 650B in bytes.
/// This constant represents 650 b converted to bytes for use in size calculations.
pub const B_650: usize = 650;

/// 651B in bytes.
/// This constant represents 651 b converted to bytes for use in size calculations.
pub const B_651: usize = 651;

/// 652B in bytes.
/// This constant represents 652 b converted to bytes for use in size calculations.
pub const B_652: usize = 652;

/// 653B in bytes.
/// This constant represents 653 b converted to bytes for use in size calculations.
pub const B_653: usize = 653;

/// 654B in bytes.
/// This constant represents 654 b converted to bytes for use in size calculations.
pub const B_654: usize = 654;

/// 655B in bytes.
/// This constant represents 655 b converted to bytes for use in size calculations.
pub const B_655: usize = 655;

/// 656B in bytes.
/// This constant represents 656 b converted to bytes for use in size calculations.
pub const B_656: usize = 656;

/// 657B in bytes.
/// This constant represents 657 b converted to bytes for use in size calculations.
pub const B_657: usize = 657;

/// 658B in bytes.
/// This constant represents 658 b converted to bytes for use in size calculations.
pub const B_658: usize = 658;

/// 659B in bytes.
/// This constant represents 659 b converted to bytes for use in size calculations.
pub const B_659: usize = 659;

/// 660B in bytes.
/// This constant represents 660 b converted to bytes for use in size calculations.
pub const B_660: usize = 660;

/// 661B in bytes.
/// This constant represents 661 b converted to bytes for use in size calculations.
pub const B_661: usize = 661;

/// 662B in bytes.
/// This constant represents 662 b converted to bytes for use in size calculations.
pub const B_662: usize = 662;

/// 663B in bytes.
/// This constant represents 663 b converted to bytes for use in size calculations.
pub const B_663: usize = 663;

/// 664B in bytes.
/// This constant represents 664 b converted to bytes for use in size calculations.
pub const B_664: usize = 664;

/// 665B in bytes.
/// This constant represents 665 b converted to bytes for use in size calculations.
pub const B_665: usize = 665;

/// 666B in bytes.
/// This constant represents 666 b converted to bytes for use in size calculations.
pub const B_666: usize = 666;

/// 667B in bytes.
/// This constant represents 667 b converted to bytes for use in size calculations.
pub const B_667: usize = 667;

/// 668B in bytes.
/// This constant represents 668 b converted to bytes for use in size calculations.
pub const B_668: usize = 668;

/// 669B in bytes.
/// This constant represents 669 b converted to bytes for use in size calculations.
pub const B_669: usize = 669;

/// 670B in bytes.
/// This constant represents 670 b converted to bytes for use in size calculations.
pub const B_670: usize = 670;

/// 671B in bytes.
/// This constant represents 671 b converted to bytes for use in size calculations.
pub const B_671: usize = 671;

/// 672B in bytes.
/// This constant represents 672 b converted to bytes for use in size calculations.
pub const B_672: usize = 672;

/// 673B in bytes.
/// This constant represents 673 b converted to bytes for use in size calculations.
pub const B_673: usize = 673;

/// 674B in bytes.
/// This constant represents 674 b converted to bytes for use in size calculations.
pub const B_674: usize = 674;

/// 675B in bytes.
/// This constant represents 675 b converted to bytes for use in size calculations.
pub const B_675: usize = 675;

/// 676B in bytes.
/// This constant represents 676 b converted to bytes for use in size calculations.
pub const B_676: usize = 676;

/// 677B in bytes.
/// This constant represents 677 b converted to bytes for use in size calculations.
pub const B_677: usize = 677;

/// 678B in bytes.
/// This constant represents 678 b converted to bytes for use in size calculations.
pub const B_678: usize = 678;

/// 679B in bytes.
/// This constant represents 679 b converted to bytes for use in size calculations.
pub const B_679: usize = 679;

/// 680B in bytes.
/// This constant represents 680 b converted to bytes for use in size calculations.
pub const B_680: usize = 680;

/// 681B in bytes.
/// This constant represents 681 b converted to bytes for use in size calculations.
pub const B_681: usize = 681;

/// 682B in bytes.
/// This constant represents 682 b converted to bytes for use in size calculations.
pub const B_682: usize = 682;

/// 683B in bytes.
/// This constant represents 683 b converted to bytes for use in size calculations.
pub const B_683: usize = 683;

/// 684B in bytes.
/// This constant represents 684 b converted to bytes for use in size calculations.
pub const B_684: usize = 684;

/// 685B in bytes.
/// This constant represents 685 b converted to bytes for use in size calculations.
pub const B_685: usize = 685;

/// 686B in bytes.
/// This constant represents 686 b converted to bytes for use in size calculations.
pub const B_686: usize = 686;

/// 687B in bytes.
/// This constant represents 687 b converted to bytes for use in size calculations.
pub const B_687: usize = 687;

/// 688B in bytes.
/// This constant represents 688 b converted to bytes for use in size calculations.
pub const B_688: usize = 688;

/// 689B in bytes.
/// This constant represents 689 b converted to bytes for use in size calculations.
pub const B_689: usize = 689;

/// 690B in bytes.
/// This constant represents 690 b converted to bytes for use in size calculations.
pub const B_690: usize = 690;

/// 691B in bytes.
/// This constant represents 691 b converted to bytes for use in size calculations.
pub const B_691: usize = 691;

/// 692B in bytes.
/// This constant represents 692 b converted to bytes for use in size calculations.
pub const B_692: usize = 692;

/// 693B in bytes.
/// This constant represents 693 b converted to bytes for use in size calculations.
pub const B_693: usize = 693;

/// 694B in bytes.
/// This constant represents 694 b converted to bytes for use in size calculations.
pub const B_694: usize = 694;

/// 695B in bytes.
/// This constant represents 695 b converted to bytes for use in size calculations.
pub const B_695: usize = 695;

/// 696B in bytes.
/// This constant represents 696 b converted to bytes for use in size calculations.
pub const B_696: usize = 696;

/// 697B in bytes.
/// This constant represents 697 b converted to bytes for use in size calculations.
pub const B_697: usize = 697;

/// 698B in bytes.
/// This constant represents 698 b converted to bytes for use in size calculations.
pub const B_698: usize = 698;

/// 699B in bytes.
/// This constant represents 699 b converted to bytes for use in size calculations.
pub const B_699: usize = 699;

/// 700B in bytes.
/// This constant represents 700 b converted to bytes for use in size calculations.
pub const B_700: usize = 700;

/// 701B in bytes.
/// This constant represents 701 b converted to bytes for use in size calculations.
pub const B_701: usize = 701;

/// 702B in bytes.
/// This constant represents 702 b converted to bytes for use in size calculations.
pub const B_702: usize = 702;

/// 703B in bytes.
/// This constant represents 703 b converted to bytes for use in size calculations.
pub const B_703: usize = 703;

/// 704B in bytes.
/// This constant represents 704 b converted to bytes for use in size calculations.
pub const B_704: usize = 704;

/// 705B in bytes.
/// This constant represents 705 b converted to bytes for use in size calculations.
pub const B_705: usize = 705;

/// 706B in bytes.
/// This constant represents 706 b converted to bytes for use in size calculations.
pub const B_706: usize = 706;

/// 707B in bytes.
/// This constant represents 707 b converted to bytes for use in size calculations.
pub const B_707: usize = 707;

/// 708B in bytes.
/// This constant represents 708 b converted to bytes for use in size calculations.
pub const B_708: usize = 708;

/// 709B in bytes.
/// This constant represents 709 b converted to bytes for use in size calculations.
pub const B_709: usize = 709;

/// 710B in bytes.
/// This constant represents 710 b converted to bytes for use in size calculations.
pub const B_710: usize = 710;

/// 711B in bytes.
/// This constant represents 711 b converted to bytes for use in size calculations.
pub const B_711: usize = 711;

/// 712B in bytes.
/// This constant represents 712 b converted to bytes for use in size calculations.
pub const B_712: usize = 712;

/// 713B in bytes.
/// This constant represents 713 b converted to bytes for use in size calculations.
pub const B_713: usize = 713;

/// 714B in bytes.
/// This constant represents 714 b converted to bytes for use in size calculations.
pub const B_714: usize = 714;

/// 715B in bytes.
/// This constant represents 715 b converted to bytes for use in size calculations.
pub const B_715: usize = 715;

/// 716B in bytes.
/// This constant represents 716 b converted to bytes for use in size calculations.
pub const B_716: usize = 716;

/// 717B in bytes.
/// This constant represents 717 b converted to bytes for use in size calculations.
pub const B_717: usize = 717;

/// 718B in bytes.
/// This constant represents 718 b converted to bytes for use in size calculations.
pub const B_718: usize = 718;

/// 719B in bytes.
/// This constant represents 719 b converted to bytes for use in size calculations.
pub const B_719: usize = 719;

/// 720B in bytes.
/// This constant represents 720 b converted to bytes for use in size calculations.
pub const B_720: usize = 720;

/// 721B in bytes.
/// This constant represents 721 b converted to bytes for use in size calculations.
pub const B_721: usize = 721;

/// 722B in bytes.
/// This constant represents 722 b converted to bytes for use in size calculations.
pub const B_722: usize = 722;

/// 723B in bytes.
/// This constant represents 723 b converted to bytes for use in size calculations.
pub const B_723: usize = 723;

/// 724B in bytes.
/// This constant represents 724 b converted to bytes for use in size calculations.
pub const B_724: usize = 724;

/// 725B in bytes.
/// This constant represents 725 b converted to bytes for use in size calculations.
pub const B_725: usize = 725;

/// 726B in bytes.
/// This constant represents 726 b converted to bytes for use in size calculations.
pub const B_726: usize = 726;

/// 727B in bytes.
/// This constant represents 727 b converted to bytes for use in size calculations.
pub const B_727: usize = 727;

/// 728B in bytes.
/// This constant represents 728 b converted to bytes for use in size calculations.
pub const B_728: usize = 728;

/// 729B in bytes.
/// This constant represents 729 b converted to bytes for use in size calculations.
pub const B_729: usize = 729;

/// 730B in bytes.
/// This constant represents 730 b converted to bytes for use in size calculations.
pub const B_730: usize = 730;

/// 731B in bytes.
/// This constant represents 731 b converted to bytes for use in size calculations.
pub const B_731: usize = 731;

/// 732B in bytes.
/// This constant represents 732 b converted to bytes for use in size calculations.
pub const B_732: usize = 732;

/// 733B in bytes.
/// This constant represents 733 b converted to bytes for use in size calculations.
pub const B_733: usize = 733;

/// 734B in bytes.
/// This constant represents 734 b converted to bytes for use in size calculations.
pub const B_734: usize = 734;

/// 735B in bytes.
/// This constant represents 735 b converted to bytes for use in size calculations.
pub const B_735: usize = 735;

/// 736B in bytes.
/// This constant represents 736 b converted to bytes for use in size calculations.
pub const B_736: usize = 736;

/// 737B in bytes.
/// This constant represents 737 b converted to bytes for use in size calculations.
pub const B_737: usize = 737;

/// 738B in bytes.
/// This constant represents 738 b converted to bytes for use in size calculations.
pub const B_738: usize = 738;

/// 739B in bytes.
/// This constant represents 739 b converted to bytes for use in size calculations.
pub const B_739: usize = 739;

/// 740B in bytes.
/// This constant represents 740 b converted to bytes for use in size calculations.
pub const B_740: usize = 740;

/// 741B in bytes.
/// This constant represents 741 b converted to bytes for use in size calculations.
pub const B_741: usize = 741;

/// 742B in bytes.
/// This constant represents 742 b converted to bytes for use in size calculations.
pub const B_742: usize = 742;

/// 743B in bytes.
/// This constant represents 743 b converted to bytes for use in size calculations.
pub const B_743: usize = 743;

/// 744B in bytes.
/// This constant represents 744 b converted to bytes for use in size calculations.
pub const B_744: usize = 744;

/// 745B in bytes.
/// This constant represents 745 b converted to bytes for use in size calculations.
pub const B_745: usize = 745;

/// 746B in bytes.
/// This constant represents 746 b converted to bytes for use in size calculations.
pub const B_746: usize = 746;

/// 747B in bytes.
/// This constant represents 747 b converted to bytes for use in size calculations.
pub const B_747: usize = 747;

/// 748B in bytes.
/// This constant represents 748 b converted to bytes for use in size calculations.
pub const B_748: usize = 748;

/// 749B in bytes.
/// This constant represents 749 b converted to bytes for use in size calculations.
pub const B_749: usize = 749;

/// 750B in bytes.
/// This constant represents 750 b converted to bytes for use in size calculations.
pub const B_750: usize = 750;

/// 751B in bytes.
/// This constant represents 751 b converted to bytes for use in size calculations.
pub const B_751: usize = 751;

/// 752B in bytes.
/// This constant represents 752 b converted to bytes for use in size calculations.
pub const B_752: usize = 752;

/// 753B in bytes.
/// This constant represents 753 b converted to bytes for use in size calculations.
pub const B_753: usize = 753;

/// 754B in bytes.
/// This constant represents 754 b converted to bytes for use in size calculations.
pub const B_754: usize = 754;

/// 755B in bytes.
/// This constant represents 755 b converted to bytes for use in size calculations.
pub const B_755: usize = 755;

/// 756B in bytes.
/// This constant represents 756 b converted to bytes for use in size calculations.
pub const B_756: usize = 756;

/// 757B in bytes.
/// This constant represents 757 b converted to bytes for use in size calculations.
pub const B_757: usize = 757;

/// 758B in bytes.
/// This constant represents 758 b converted to bytes for use in size calculations.
pub const B_758: usize = 758;

/// 759B in bytes.
/// This constant represents 759 b converted to bytes for use in size calculations.
pub const B_759: usize = 759;

/// 760B in bytes.
/// This constant represents 760 b converted to bytes for use in size calculations.
pub const B_760: usize = 760;

/// 761B in bytes.
/// This constant represents 761 b converted to bytes for use in size calculations.
pub const B_761: usize = 761;

/// 762B in bytes.
/// This constant represents 762 b converted to bytes for use in size calculations.
pub const B_762: usize = 762;

/// 763B in bytes.
/// This constant represents 763 b converted to bytes for use in size calculations.
pub const B_763: usize = 763;

/// 764B in bytes.
/// This constant represents 764 b converted to bytes for use in size calculations.
pub const B_764: usize = 764;

/// 765B in bytes.
/// This constant represents 765 b converted to bytes for use in size calculations.
pub const B_765: usize = 765;

/// 766B in bytes.
/// This constant represents 766 b converted to bytes for use in size calculations.
pub const B_766: usize = 766;

/// 767B in bytes.
/// This constant represents 767 b converted to bytes for use in size calculations.
pub const B_767: usize = 767;

/// 768B in bytes.
/// This constant represents 768 b converted to bytes for use in size calculations.
pub const B_768: usize = 768;

/// 769B in bytes.
/// This constant represents 769 b converted to bytes for use in size calculations.
pub const B_769: usize = 769;

/// 770B in bytes.
/// This constant represents 770 b converted to bytes for use in size calculations.
pub const B_770: usize = 770;

/// 771B in bytes.
/// This constant represents 771 b converted to bytes for use in size calculations.
pub const B_771: usize = 771;

/// 772B in bytes.
/// This constant represents 772 b converted to bytes for use in size calculations.
pub const B_772: usize = 772;

/// 773B in bytes.
/// This constant represents 773 b converted to bytes for use in size calculations.
pub const B_773: usize = 773;

/// 774B in bytes.
/// This constant represents 774 b converted to bytes for use in size calculations.
pub const B_774: usize = 774;

/// 775B in bytes.
/// This constant represents 775 b converted to bytes for use in size calculations.
pub const B_775: usize = 775;

/// 776B in bytes.
/// This constant represents 776 b converted to bytes for use in size calculations.
pub const B_776: usize = 776;

/// 777B in bytes.
/// This constant represents 777 b converted to bytes for use in size calculations.
pub const B_777: usize = 777;

/// 778B in bytes.
/// This constant represents 778 b converted to bytes for use in size calculations.
pub const B_778: usize = 778;

/// 779B in bytes.
/// This constant represents 779 b converted to bytes for use in size calculations.
pub const B_779: usize = 779;

/// 780B in bytes.
/// This constant represents 780 b converted to bytes for use in size calculations.
pub const B_780: usize = 780;

/// 781B in bytes.
/// This constant represents 781 b converted to bytes for use in size calculations.
pub const B_781: usize = 781;

/// 782B in bytes.
/// This constant represents 782 b converted to bytes for use in size calculations.
pub const B_782: usize = 782;

/// 783B in bytes.
/// This constant represents 783 b converted to bytes for use in size calculations.
pub const B_783: usize = 783;

/// 784B in bytes.
/// This constant represents 784 b converted to bytes for use in size calculations.
pub const B_784: usize = 784;

/// 785B in bytes.
/// This constant represents 785 b converted to bytes for use in size calculations.
pub const B_785: usize = 785;

/// 786B in bytes.
/// This constant represents 786 b converted to bytes for use in size calculations.
pub const B_786: usize = 786;

/// 787B in bytes.
/// This constant represents 787 b converted to bytes for use in size calculations.
pub const B_787: usize = 787;

/// 788B in bytes.
/// This constant represents 788 b converted to bytes for use in size calculations.
pub const B_788: usize = 788;

/// 789B in bytes.
/// This constant represents 789 b converted to bytes for use in size calculations.
pub const B_789: usize = 789;

/// 790B in bytes.
/// This constant represents 790 b converted to bytes for use in size calculations.
pub const B_790: usize = 790;

/// 791B in bytes.
/// This constant represents 791 b converted to bytes for use in size calculations.
pub const B_791: usize = 791;

/// 792B in bytes.
/// This constant represents 792 b converted to bytes for use in size calculations.
pub const B_792: usize = 792;

/// 793B in bytes.
/// This constant represents 793 b converted to bytes for use in size calculations.
pub const B_793: usize = 793;

/// 794B in bytes.
/// This constant represents 794 b converted to bytes for use in size calculations.
pub const B_794: usize = 794;

/// 795B in bytes.
/// This constant represents 795 b converted to bytes for use in size calculations.
pub const B_795: usize = 795;

/// 796B in bytes.
/// This constant represents 796 b converted to bytes for use in size calculations.
pub const B_796: usize = 796;

/// 797B in bytes.
/// This constant represents 797 b converted to bytes for use in size calculations.
pub const B_797: usize = 797;

/// 798B in bytes.
/// This constant represents 798 b converted to bytes for use in size calculations.
pub const B_798: usize = 798;

/// 799B in bytes.
/// This constant represents 799 b converted to bytes for use in size calculations.
pub const B_799: usize = 799;

/// 800B in bytes.
/// This constant represents 800 b converted to bytes for use in size calculations.
pub const B_800: usize = 800;

/// 801B in bytes.
/// This constant represents 801 b converted to bytes for use in size calculations.
pub const B_801: usize = 801;

/// 802B in bytes.
/// This constant represents 802 b converted to bytes for use in size calculations.
pub const B_802: usize = 802;

/// 803B in bytes.
/// This constant represents 803 b converted to bytes for use in size calculations.
pub const B_803: usize = 803;

/// 804B in bytes.
/// This constant represents 804 b converted to bytes for use in size calculations.
pub const B_804: usize = 804;

/// 805B in bytes.
/// This constant represents 805 b converted to bytes for use in size calculations.
pub const B_805: usize = 805;

/// 806B in bytes.
/// This constant represents 806 b converted to bytes for use in size calculations.
pub const B_806: usize = 806;

/// 807B in bytes.
/// This constant represents 807 b converted to bytes for use in size calculations.
pub const B_807: usize = 807;

/// 808B in bytes.
/// This constant represents 808 b converted to bytes for use in size calculations.
pub const B_808: usize = 808;

/// 809B in bytes.
/// This constant represents 809 b converted to bytes for use in size calculations.
pub const B_809: usize = 809;

/// 810B in bytes.
/// This constant represents 810 b converted to bytes for use in size calculations.
pub const B_810: usize = 810;

/// 811B in bytes.
/// This constant represents 811 b converted to bytes for use in size calculations.
pub const B_811: usize = 811;

/// 812B in bytes.
/// This constant represents 812 b converted to bytes for use in size calculations.
pub const B_812: usize = 812;

/// 813B in bytes.
/// This constant represents 813 b converted to bytes for use in size calculations.
pub const B_813: usize = 813;

/// 814B in bytes.
/// This constant represents 814 b converted to bytes for use in size calculations.
pub const B_814: usize = 814;

/// 815B in bytes.
/// This constant represents 815 b converted to bytes for use in size calculations.
pub const B_815: usize = 815;

/// 816B in bytes.
/// This constant represents 816 b converted to bytes for use in size calculations.
pub const B_816: usize = 816;

/// 817B in bytes.
/// This constant represents 817 b converted to bytes for use in size calculations.
pub const B_817: usize = 817;

/// 818B in bytes.
/// This constant represents 818 b converted to bytes for use in size calculations.
pub const B_818: usize = 818;

/// 819B in bytes.
/// This constant represents 819 b converted to bytes for use in size calculations.
pub const B_819: usize = 819;

/// 820B in bytes.
/// This constant represents 820 b converted to bytes for use in size calculations.
pub const B_820: usize = 820;

/// 821B in bytes.
/// This constant represents 821 b converted to bytes for use in size calculations.
pub const B_821: usize = 821;

/// 822B in bytes.
/// This constant represents 822 b converted to bytes for use in size calculations.
pub const B_822: usize = 822;

/// 823B in bytes.
/// This constant represents 823 b converted to bytes for use in size calculations.
pub const B_823: usize = 823;

/// 824B in bytes.
/// This constant represents 824 b converted to bytes for use in size calculations.
pub const B_824: usize = 824;

/// 825B in bytes.
/// This constant represents 825 b converted to bytes for use in size calculations.
pub const B_825: usize = 825;

/// 826B in bytes.
/// This constant represents 826 b converted to bytes for use in size calculations.
pub const B_826: usize = 826;

/// 827B in bytes.
/// This constant represents 827 b converted to bytes for use in size calculations.
pub const B_827: usize = 827;

/// 828B in bytes.
/// This constant represents 828 b converted to bytes for use in size calculations.
pub const B_828: usize = 828;

/// 829B in bytes.
/// This constant represents 829 b converted to bytes for use in size calculations.
pub const B_829: usize = 829;

/// 830B in bytes.
/// This constant represents 830 b converted to bytes for use in size calculations.
pub const B_830: usize = 830;

/// 831B in bytes.
/// This constant represents 831 b converted to bytes for use in size calculations.
pub const B_831: usize = 831;

/// 832B in bytes.
/// This constant represents 832 b converted to bytes for use in size calculations.
pub const B_832: usize = 832;

/// 833B in bytes.
/// This constant represents 833 b converted to bytes for use in size calculations.
pub const B_833: usize = 833;

/// 834B in bytes.
/// This constant represents 834 b converted to bytes for use in size calculations.
pub const B_834: usize = 834;

/// 835B in bytes.
/// This constant represents 835 b converted to bytes for use in size calculations.
pub const B_835: usize = 835;

/// 836B in bytes.
/// This constant represents 836 b converted to bytes for use in size calculations.
pub const B_836: usize = 836;

/// 837B in bytes.
/// This constant represents 837 b converted to bytes for use in size calculations.
pub const B_837: usize = 837;

/// 838B in bytes.
/// This constant represents 838 b converted to bytes for use in size calculations.
pub const B_838: usize = 838;

/// 839B in bytes.
/// This constant represents 839 b converted to bytes for use in size calculations.
pub const B_839: usize = 839;

/// 840B in bytes.
/// This constant represents 840 b converted to bytes for use in size calculations.
pub const B_840: usize = 840;

/// 841B in bytes.
/// This constant represents 841 b converted to bytes for use in size calculations.
pub const B_841: usize = 841;

/// 842B in bytes.
/// This constant represents 842 b converted to bytes for use in size calculations.
pub const B_842: usize = 842;

/// 843B in bytes.
/// This constant represents 843 b converted to bytes for use in size calculations.
pub const B_843: usize = 843;

/// 844B in bytes.
/// This constant represents 844 b converted to bytes for use in size calculations.
pub const B_844: usize = 844;

/// 845B in bytes.
/// This constant represents 845 b converted to bytes for use in size calculations.
pub const B_845: usize = 845;

/// 846B in bytes.
/// This constant represents 846 b converted to bytes for use in size calculations.
pub const B_846: usize = 846;

/// 847B in bytes.
/// This constant represents 847 b converted to bytes for use in size calculations.
pub const B_847: usize = 847;

/// 848B in bytes.
/// This constant represents 848 b converted to bytes for use in size calculations.
pub const B_848: usize = 848;

/// 849B in bytes.
/// This constant represents 849 b converted to bytes for use in size calculations.
pub const B_849: usize = 849;

/// 850B in bytes.
/// This constant represents 850 b converted to bytes for use in size calculations.
pub const B_850: usize = 850;

/// 851B in bytes.
/// This constant represents 851 b converted to bytes for use in size calculations.
pub const B_851: usize = 851;

/// 852B in bytes.
/// This constant represents 852 b converted to bytes for use in size calculations.
pub const B_852: usize = 852;

/// 853B in bytes.
/// This constant represents 853 b converted to bytes for use in size calculations.
pub const B_853: usize = 853;

/// 854B in bytes.
/// This constant represents 854 b converted to bytes for use in size calculations.
pub const B_854: usize = 854;

/// 855B in bytes.
/// This constant represents 855 b converted to bytes for use in size calculations.
pub const B_855: usize = 855;

/// 856B in bytes.
/// This constant represents 856 b converted to bytes for use in size calculations.
pub const B_856: usize = 856;

/// 857B in bytes.
/// This constant represents 857 b converted to bytes for use in size calculations.
pub const B_857: usize = 857;

/// 858B in bytes.
/// This constant represents 858 b converted to bytes for use in size calculations.
pub const B_858: usize = 858;

/// 859B in bytes.
/// This constant represents 859 b converted to bytes for use in size calculations.
pub const B_859: usize = 859;

/// 860B in bytes.
/// This constant represents 860 b converted to bytes for use in size calculations.
pub const B_860: usize = 860;

/// 861B in bytes.
/// This constant represents 861 b converted to bytes for use in size calculations.
pub const B_861: usize = 861;

/// 862B in bytes.
/// This constant represents 862 b converted to bytes for use in size calculations.
pub const B_862: usize = 862;

/// 863B in bytes.
/// This constant represents 863 b converted to bytes for use in size calculations.
pub const B_863: usize = 863;

/// 864B in bytes.
/// This constant represents 864 b converted to bytes for use in size calculations.
pub const B_864: usize = 864;

/// 865B in bytes.
/// This constant represents 865 b converted to bytes for use in size calculations.
pub const B_865: usize = 865;

/// 866B in bytes.
/// This constant represents 866 b converted to bytes for use in size calculations.
pub const B_866: usize = 866;

/// 867B in bytes.
/// This constant represents 867 b converted to bytes for use in size calculations.
pub const B_867: usize = 867;

/// 868B in bytes.
/// This constant represents 868 b converted to bytes for use in size calculations.
pub const B_868: usize = 868;

/// 869B in bytes.
/// This constant represents 869 b converted to bytes for use in size calculations.
pub const B_869: usize = 869;

/// 870B in bytes.
/// This constant represents 870 b converted to bytes for use in size calculations.
pub const B_870: usize = 870;

/// 871B in bytes.
/// This constant represents 871 b converted to bytes for use in size calculations.
pub const B_871: usize = 871;

/// 872B in bytes.
/// This constant represents 872 b converted to bytes for use in size calculations.
pub const B_872: usize = 872;

/// 873B in bytes.
/// This constant represents 873 b converted to bytes for use in size calculations.
pub const B_873: usize = 873;

/// 874B in bytes.
/// This constant represents 874 b converted to bytes for use in size calculations.
pub const B_874: usize = 874;

/// 875B in bytes.
/// This constant represents 875 b converted to bytes for use in size calculations.
pub const B_875: usize = 875;

/// 876B in bytes.
/// This constant represents 876 b converted to bytes for use in size calculations.
pub const B_876: usize = 876;

/// 877B in bytes.
/// This constant represents 877 b converted to bytes for use in size calculations.
pub const B_877: usize = 877;

/// 878B in bytes.
/// This constant represents 878 b converted to bytes for use in size calculations.
pub const B_878: usize = 878;

/// 879B in bytes.
/// This constant represents 879 b converted to bytes for use in size calculations.
pub const B_879: usize = 879;

/// 880B in bytes.
/// This constant represents 880 b converted to bytes for use in size calculations.
pub const B_880: usize = 880;

/// 881B in bytes.
/// This constant represents 881 b converted to bytes for use in size calculations.
pub const B_881: usize = 881;

/// 882B in bytes.
/// This constant represents 882 b converted to bytes for use in size calculations.
pub const B_882: usize = 882;

/// 883B in bytes.
/// This constant represents 883 b converted to bytes for use in size calculations.
pub const B_883: usize = 883;

/// 884B in bytes.
/// This constant represents 884 b converted to bytes for use in size calculations.
pub const B_884: usize = 884;

/// 885B in bytes.
/// This constant represents 885 b converted to bytes for use in size calculations.
pub const B_885: usize = 885;

/// 886B in bytes.
/// This constant represents 886 b converted to bytes for use in size calculations.
pub const B_886: usize = 886;

/// 887B in bytes.
/// This constant represents 887 b converted to bytes for use in size calculations.
pub const B_887: usize = 887;

/// 888B in bytes.
/// This constant represents 888 b converted to bytes for use in size calculations.
pub const B_888: usize = 888;

/// 889B in bytes.
/// This constant represents 889 b converted to bytes for use in size calculations.
pub const B_889: usize = 889;

/// 890B in bytes.
/// This constant represents 890 b converted to bytes for use in size calculations.
pub const B_890: usize = 890;

/// 891B in bytes.
/// This constant represents 891 b converted to bytes for use in size calculations.
pub const B_891: usize = 891;

/// 892B in bytes.
/// This constant represents 892 b converted to bytes for use in size calculations.
pub const B_892: usize = 892;

/// 893B in bytes.
/// This constant represents 893 b converted to bytes for use in size calculations.
pub const B_893: usize = 893;

/// 894B in bytes.
/// This constant represents 894 b converted to bytes for use in size calculations.
pub const B_894: usize = 894;

/// 895B in bytes.
/// This constant represents 895 b converted to bytes for use in size calculations.
pub const B_895: usize = 895;

/// 896B in bytes.
/// This constant represents 896 b converted to bytes for use in size calculations.
pub const B_896: usize = 896;

/// 897B in bytes.
/// This constant represents 897 b converted to bytes for use in size calculations.
pub const B_897: usize = 897;

/// 898B in bytes.
/// This constant represents 898 b converted to bytes for use in size calculations.
pub const B_898: usize = 898;

/// 899B in bytes.
/// This constant represents 899 b converted to bytes for use in size calculations.
pub const B_899: usize = 899;

/// 900B in bytes.
/// This constant represents 900 b converted to bytes for use in size calculations.
pub const B_900: usize = 900;

/// 901B in bytes.
/// This constant represents 901 b converted to bytes for use in size calculations.
pub const B_901: usize = 901;

/// 902B in bytes.
/// This constant represents 902 b converted to bytes for use in size calculations.
pub const B_902: usize = 902;

/// 903B in bytes.
/// This constant represents 903 b converted to bytes for use in size calculations.
pub const B_903: usize = 903;

/// 904B in bytes.
/// This constant represents 904 b converted to bytes for use in size calculations.
pub const B_904: usize = 904;

/// 905B in bytes.
/// This constant represents 905 b converted to bytes for use in size calculations.
pub const B_905: usize = 905;

/// 906B in bytes.
/// This constant represents 906 b converted to bytes for use in size calculations.
pub const B_906: usize = 906;

/// 907B in bytes.
/// This constant represents 907 b converted to bytes for use in size calculations.
pub const B_907: usize = 907;

/// 908B in bytes.
/// This constant represents 908 b converted to bytes for use in size calculations.
pub const B_908: usize = 908;

/// 909B in bytes.
/// This constant represents 909 b converted to bytes for use in size calculations.
pub const B_909: usize = 909;

/// 910B in bytes.
/// This constant represents 910 b converted to bytes for use in size calculations.
pub const B_910: usize = 910;

/// 911B in bytes.
/// This constant represents 911 b converted to bytes for use in size calculations.
pub const B_911: usize = 911;

/// 912B in bytes.
/// This constant represents 912 b converted to bytes for use in size calculations.
pub const B_912: usize = 912;

/// 913B in bytes.
/// This constant represents 913 b converted to bytes for use in size calculations.
pub const B_913: usize = 913;

/// 914B in bytes.
/// This constant represents 914 b converted to bytes for use in size calculations.
pub const B_914: usize = 914;

/// 915B in bytes.
/// This constant represents 915 b converted to bytes for use in size calculations.
pub const B_915: usize = 915;

/// 916B in bytes.
/// This constant represents 916 b converted to bytes for use in size calculations.
pub const B_916: usize = 916;

/// 917B in bytes.
/// This constant represents 917 b converted to bytes for use in size calculations.
pub const B_917: usize = 917;

/// 918B in bytes.
/// This constant represents 918 b converted to bytes for use in size calculations.
pub const B_918: usize = 918;

/// 919B in bytes.
/// This constant represents 919 b converted to bytes for use in size calculations.
pub const B_919: usize = 919;

/// 920B in bytes.
/// This constant represents 920 b converted to bytes for use in size calculations.
pub const B_920: usize = 920;

/// 921B in bytes.
/// This constant represents 921 b converted to bytes for use in size calculations.
pub const B_921: usize = 921;

/// 922B in bytes.
/// This constant represents 922 b converted to bytes for use in size calculations.
pub const B_922: usize = 922;

/// 923B in bytes.
/// This constant represents 923 b converted to bytes for use in size calculations.
pub const B_923: usize = 923;

/// 924B in bytes.
/// This constant represents 924 b converted to bytes for use in size calculations.
pub const B_924: usize = 924;

/// 925B in bytes.
/// This constant represents 925 b converted to bytes for use in size calculations.
pub const B_925: usize = 925;

/// 926B in bytes.
/// This constant represents 926 b converted to bytes for use in size calculations.
pub const B_926: usize = 926;

/// 927B in bytes.
/// This constant represents 927 b converted to bytes for use in size calculations.
pub const B_927: usize = 927;

/// 928B in bytes.
/// This constant represents 928 b converted to bytes for use in size calculations.
pub const B_928: usize = 928;

/// 929B in bytes.
/// This constant represents 929 b converted to bytes for use in size calculations.
pub const B_929: usize = 929;

/// 930B in bytes.
/// This constant represents 930 b converted to bytes for use in size calculations.
pub const B_930: usize = 930;

/// 931B in bytes.
/// This constant represents 931 b converted to bytes for use in size calculations.
pub const B_931: usize = 931;

/// 932B in bytes.
/// This constant represents 932 b converted to bytes for use in size calculations.
pub const B_932: usize = 932;

/// 933B in bytes.
/// This constant represents 933 b converted to bytes for use in size calculations.
pub const B_933: usize = 933;

/// 934B in bytes.
/// This constant represents 934 b converted to bytes for use in size calculations.
pub const B_934: usize = 934;

/// 935B in bytes.
/// This constant represents 935 b converted to bytes for use in size calculations.
pub const B_935: usize = 935;

/// 936B in bytes.
/// This constant represents 936 b converted to bytes for use in size calculations.
pub const B_936: usize = 936;

/// 937B in bytes.
/// This constant represents 937 b converted to bytes for use in size calculations.
pub const B_937: usize = 937;

/// 938B in bytes.
/// This constant represents 938 b converted to bytes for use in size calculations.
pub const B_938: usize = 938;

/// 939B in bytes.
/// This constant represents 939 b converted to bytes for use in size calculations.
pub const B_939: usize = 939;

/// 940B in bytes.
/// This constant represents 940 b converted to bytes for use in size calculations.
pub const B_940: usize = 940;

/// 941B in bytes.
/// This constant represents 941 b converted to bytes for use in size calculations.
pub const B_941: usize = 941;

/// 942B in bytes.
/// This constant represents 942 b converted to bytes for use in size calculations.
pub const B_942: usize = 942;

/// 943B in bytes.
/// This constant represents 943 b converted to bytes for use in size calculations.
pub const B_943: usize = 943;

/// 944B in bytes.
/// This constant represents 944 b converted to bytes for use in size calculations.
pub const B_944: usize = 944;

/// 945B in bytes.
/// This constant represents 945 b converted to bytes for use in size calculations.
pub const B_945: usize = 945;

/// 946B in bytes.
/// This constant represents 946 b converted to bytes for use in size calculations.
pub const B_946: usize = 946;

/// 947B in bytes.
/// This constant represents 947 b converted to bytes for use in size calculations.
pub const B_947: usize = 947;

/// 948B in bytes.
/// This constant represents 948 b converted to bytes for use in size calculations.
pub const B_948: usize = 948;

/// 949B in bytes.
/// This constant represents 949 b converted to bytes for use in size calculations.
pub const B_949: usize = 949;

/// 950B in bytes.
/// This constant represents 950 b converted to bytes for use in size calculations.
pub const B_950: usize = 950;

/// 951B in bytes.
/// This constant represents 951 b converted to bytes for use in size calculations.
pub const B_951: usize = 951;

/// 952B in bytes.
/// This constant represents 952 b converted to bytes for use in size calculations.
pub const B_952: usize = 952;

/// 953B in bytes.
/// This constant represents 953 b converted to bytes for use in size calculations.
pub const B_953: usize = 953;

/// 954B in bytes.
/// This constant represents 954 b converted to bytes for use in size calculations.
pub const B_954: usize = 954;

/// 955B in bytes.
/// This constant represents 955 b converted to bytes for use in size calculations.
pub const B_955: usize = 955;

/// 956B in bytes.
/// This constant represents 956 b converted to bytes for use in size calculations.
pub const B_956: usize = 956;

/// 957B in bytes.
/// This constant represents 957 b converted to bytes for use in size calculations.
pub const B_957: usize = 957;

/// 958B in bytes.
/// This constant represents 958 b converted to bytes for use in size calculations.
pub const B_958: usize = 958;

/// 959B in bytes.
/// This constant represents 959 b converted to bytes for use in size calculations.
pub const B_959: usize = 959;

/// 960B in bytes.
/// This constant represents 960 b converted to bytes for use in size calculations.
pub const B_960: usize = 960;

/// 961B in bytes.
/// This constant represents 961 b converted to bytes for use in size calculations.
pub const B_961: usize = 961;

/// 962B in bytes.
/// This constant represents 962 b converted to bytes for use in size calculations.
pub const B_962: usize = 962;

/// 963B in bytes.
/// This constant represents 963 b converted to bytes for use in size calculations.
pub const B_963: usize = 963;

/// 964B in bytes.
/// This constant represents 964 b converted to bytes for use in size calculations.
pub const B_964: usize = 964;

/// 965B in bytes.
/// This constant represents 965 b converted to bytes for use in size calculations.
pub const B_965: usize = 965;

/// 966B in bytes.
/// This constant represents 966 b converted to bytes for use in size calculations.
pub const B_966: usize = 966;

/// 967B in bytes.
/// This constant represents 967 b converted to bytes for use in size calculations.
pub const B_967: usize = 967;

/// 968B in bytes.
/// This constant represents 968 b converted to bytes for use in size calculations.
pub const B_968: usize = 968;

/// 969B in bytes.
/// This constant represents 969 b converted to bytes for use in size calculations.
pub const B_969: usize = 969;

/// 970B in bytes.
/// This constant represents 970 b converted to bytes for use in size calculations.
pub const B_970: usize = 970;

/// 971B in bytes.
/// This constant represents 971 b converted to bytes for use in size calculations.
pub const B_971: usize = 971;

/// 972B in bytes.
/// This constant represents 972 b converted to bytes for use in size calculations.
pub const B_972: usize = 972;

/// 973B in bytes.
/// This constant represents 973 b converted to bytes for use in size calculations.
pub const B_973: usize = 973;

/// 974B in bytes.
/// This constant represents 974 b converted to bytes for use in size calculations.
pub const B_974: usize = 974;

/// 975B in bytes.
/// This constant represents 975 b converted to bytes for use in size calculations.
pub const B_975: usize = 975;

/// 976B in bytes.
/// This constant represents 976 b converted to bytes for use in size calculations.
pub const B_976: usize = 976;

/// 977B in bytes.
/// This constant represents 977 b converted to bytes for use in size calculations.
pub const B_977: usize = 977;

/// 978B in bytes.
/// This constant represents 978 b converted to bytes for use in size calculations.
pub const B_978: usize = 978;

/// 979B in bytes.
/// This constant represents 979 b converted to bytes for use in size calculations.
pub const B_979: usize = 979;

/// 980B in bytes.
/// This constant represents 980 b converted to bytes for use in size calculations.
pub const B_980: usize = 980;

/// 981B in bytes.
/// This constant represents 981 b converted to bytes for use in size calculations.
pub const B_981: usize = 981;

/// 982B in bytes.
/// This constant represents 982 b converted to bytes for use in size calculations.
pub const B_982: usize = 982;

/// 983B in bytes.
/// This constant represents 983 b converted to bytes for use in size calculations.
pub const B_983: usize = 983;

/// 984B in bytes.
/// This constant represents 984 b converted to bytes for use in size calculations.
pub const B_984: usize = 984;

/// 985B in bytes.
/// This constant represents 985 b converted to bytes for use in size calculations.
pub const B_985: usize = 985;

/// 986B in bytes.
/// This constant represents 986 b converted to bytes for use in size calculations.
pub const B_986: usize = 986;

/// 987B in bytes.
/// This constant represents 987 b converted to bytes for use in size calculations.
pub const B_987: usize = 987;

/// 988B in bytes.
/// This constant represents 988 b converted to bytes for use in size calculations.
pub const B_988: usize = 988;

/// 989B in bytes.
/// This constant represents 989 b converted to bytes for use in size calculations.
pub const B_989: usize = 989;

/// 990B in bytes.
/// This constant represents 990 b converted to bytes for use in size calculations.
pub const B_990: usize = 990;

/// 991B in bytes.
/// This constant represents 991 b converted to bytes for use in size calculations.
pub const B_991: usize = 991;

/// 992B in bytes.
/// This constant represents 992 b converted to bytes for use in size calculations.
pub const B_992: usize = 992;

/// 993B in bytes.
/// This constant represents 993 b converted to bytes for use in size calculations.
pub const B_993: usize = 993;

/// 994B in bytes.
/// This constant represents 994 b converted to bytes for use in size calculations.
pub const B_994: usize = 994;

/// 995B in bytes.
/// This constant represents 995 b converted to bytes for use in size calculations.
pub const B_995: usize = 995;

/// 996B in bytes.
/// This constant represents 996 b converted to bytes for use in size calculations.
pub const B_996: usize = 996;

/// 997B in bytes.
/// This constant represents 997 b converted to bytes for use in size calculations.
pub const B_997: usize = 997;

/// 998B in bytes.
/// This constant represents 998 b converted to bytes for use in size calculations.
pub const B_998: usize = 998;

/// 999B in bytes.
/// This constant represents 999 b converted to bytes for use in size calculations.
pub const B_999: usize = 999;

/// 1000B in bytes.
/// This constant represents 1000 b converted to bytes for use in size calculations.
pub const B_1000: usize = 1000;

/// 1001B in bytes.
/// This constant represents 1001 b converted to bytes for use in size calculations.
pub const B_1001: usize = 1001;

/// 1002B in bytes.
/// This constant represents 1002 b converted to bytes for use in size calculations.
pub const B_1002: usize = 1002;

/// 1003B in bytes.
/// This constant represents 1003 b converted to bytes for use in size calculations.
pub const B_1003: usize = 1003;

/// 1004B in bytes.
/// This constant represents 1004 b converted to bytes for use in size calculations.
pub const B_1004: usize = 1004;

/// 1005B in bytes.
/// This constant represents 1005 b converted to bytes for use in size calculations.
pub const B_1005: usize = 1005;

/// 1006B in bytes.
/// This constant represents 1006 b converted to bytes for use in size calculations.
pub const B_1006: usize = 1006;

/// 1007B in bytes.
/// This constant represents 1007 b converted to bytes for use in size calculations.
pub const B_1007: usize = 1007;

/// 1008B in bytes.
/// This constant represents 1008 b converted to bytes for use in size calculations.
pub const B_1008: usize = 1008;

/// 1009B in bytes.
/// This constant represents 1009 b converted to bytes for use in size calculations.
pub const B_1009: usize = 1009;

/// 1010B in bytes.
/// This constant represents 1010 b converted to bytes for use in size calculations.
pub const B_1010: usize = 1010;

/// 1011B in bytes.
/// This constant represents 1011 b converted to bytes for use in size calculations.
pub const B_1011: usize = 1011;

/// 1012B in bytes.
/// This constant represents 1012 b converted to bytes for use in size calculations.
pub const B_1012: usize = 1012;

/// 1013B in bytes.
/// This constant represents 1013 b converted to bytes for use in size calculations.
pub const B_1013: usize = 1013;

/// 1014B in bytes.
/// This constant represents 1014 b converted to bytes for use in size calculations.
pub const B_1014: usize = 1014;

/// 1015B in bytes.
/// This constant represents 1015 b converted to bytes for use in size calculations.
pub const B_1015: usize = 1015;

/// 1016B in bytes.
/// This constant represents 1016 b converted to bytes for use in size calculations.
pub const B_1016: usize = 1016;

/// 1017B in bytes.
/// This constant represents 1017 b converted to bytes for use in size calculations.
pub const B_1017: usize = 1017;

/// 1018B in bytes.
/// This constant represents 1018 b converted to bytes for use in size calculations.
pub const B_1018: usize = 1018;

/// 1019B in bytes.
/// This constant represents 1019 b converted to bytes for use in size calculations.
pub const B_1019: usize = 1019;

/// 1020B in bytes.
/// This constant represents 1020 b converted to bytes for use in size calculations.
pub const B_1020: usize = 1020;

/// 1021B in bytes.
/// This constant represents 1021 b converted to bytes for use in size calculations.
pub const B_1021: usize = 1021;

/// 1022B in bytes.
/// This constant represents 1022 b converted to bytes for use in size calculations.
pub const B_1022: usize = 1022;

/// 1023B in bytes.
/// This constant represents 1023 b converted to bytes for use in size calculations.
pub const B_1023: usize = 1023;

/// 1024B in bytes.
/// This constant represents 1024 b converted to bytes for use in size calculations.
pub const B_1024: usize = 1024;

// Storage unit constants from 1KB to 1024KB
/// 1KB in bytes.
/// This constant represents 1 kb converted to bytes for use in size calculations.
pub const KB_1: usize = 1024;

/// 2KB in bytes.
/// This constant represents 2 kb converted to bytes for use in size calculations.
pub const KB_2: usize = 2048;

/// 3KB in bytes.
/// This constant represents 3 kb converted to bytes for use in size calculations.
pub const KB_3: usize = 3072;

/// 4KB in bytes.
/// This constant represents 4 kb converted to bytes for use in size calculations.
pub const KB_4: usize = 4096;

/// 5KB in bytes.
/// This constant represents 5 kb converted to bytes for use in size calculations.
pub const KB_5: usize = 5120;

/// 6KB in bytes.
/// This constant represents 6 kb converted to bytes for use in size calculations.
pub const KB_6: usize = 6144;

/// 7KB in bytes.
/// This constant represents 7 kb converted to bytes for use in size calculations.
pub const KB_7: usize = 7168;

/// 8KB in bytes.
/// This constant represents 8 kb converted to bytes for use in size calculations.
pub const KB_8: usize = 8192;

/// 9KB in bytes.
/// This constant represents 9 kb converted to bytes for use in size calculations.
pub const KB_9: usize = 9216;

/// 10KB in bytes.
/// This constant represents 10 kb converted to bytes for use in size calculations.
pub const KB_10: usize = 10240;

/// 11KB in bytes.
/// This constant represents 11 kb converted to bytes for use in size calculations.
pub const KB_11: usize = 11264;

/// 12KB in bytes.
/// This constant represents 12 kb converted to bytes for use in size calculations.
pub const KB_12: usize = 12288;

/// 13KB in bytes.
/// This constant represents 13 kb converted to bytes for use in size calculations.
pub const KB_13: usize = 13312;

/// 14KB in bytes.
/// This constant represents 14 kb converted to bytes for use in size calculations.
pub const KB_14: usize = 14336;

/// 15KB in bytes.
/// This constant represents 15 kb converted to bytes for use in size calculations.
pub const KB_15: usize = 15360;

/// 16KB in bytes.
/// This constant represents 16 kb converted to bytes for use in size calculations.
pub const KB_16: usize = 16384;

/// 17KB in bytes.
/// This constant represents 17 kb converted to bytes for use in size calculations.
pub const KB_17: usize = 17408;

/// 18KB in bytes.
/// This constant represents 18 kb converted to bytes for use in size calculations.
pub const KB_18: usize = 18432;

/// 19KB in bytes.
/// This constant represents 19 kb converted to bytes for use in size calculations.
pub const KB_19: usize = 19456;

/// 20KB in bytes.
/// This constant represents 20 kb converted to bytes for use in size calculations.
pub const KB_20: usize = 20480;

/// 21KB in bytes.
/// This constant represents 21 kb converted to bytes for use in size calculations.
pub const KB_21: usize = 21504;

/// 22KB in bytes.
/// This constant represents 22 kb converted to bytes for use in size calculations.
pub const KB_22: usize = 22528;

/// 23KB in bytes.
/// This constant represents 23 kb converted to bytes for use in size calculations.
pub const KB_23: usize = 23552;

/// 24KB in bytes.
/// This constant represents 24 kb converted to bytes for use in size calculations.
pub const KB_24: usize = 24576;

/// 25KB in bytes.
/// This constant represents 25 kb converted to bytes for use in size calculations.
pub const KB_25: usize = 25600;

/// 26KB in bytes.
/// This constant represents 26 kb converted to bytes for use in size calculations.
pub const KB_26: usize = 26624;

/// 27KB in bytes.
/// This constant represents 27 kb converted to bytes for use in size calculations.
pub const KB_27: usize = 27648;

/// 28KB in bytes.
/// This constant represents 28 kb converted to bytes for use in size calculations.
pub const KB_28: usize = 28672;

/// 29KB in bytes.
/// This constant represents 29 kb converted to bytes for use in size calculations.
pub const KB_29: usize = 29696;

/// 30KB in bytes.
/// This constant represents 30 kb converted to bytes for use in size calculations.
pub const KB_30: usize = 30720;

/// 31KB in bytes.
/// This constant represents 31 kb converted to bytes for use in size calculations.
pub const KB_31: usize = 31744;

/// 32KB in bytes.
/// This constant represents 32 kb converted to bytes for use in size calculations.
pub const KB_32: usize = 32768;

/// 33KB in bytes.
/// This constant represents 33 kb converted to bytes for use in size calculations.
pub const KB_33: usize = 33792;

/// 34KB in bytes.
/// This constant represents 34 kb converted to bytes for use in size calculations.
pub const KB_34: usize = 34816;

/// 35KB in bytes.
/// This constant represents 35 kb converted to bytes for use in size calculations.
pub const KB_35: usize = 35840;

/// 36KB in bytes.
/// This constant represents 36 kb converted to bytes for use in size calculations.
pub const KB_36: usize = 36864;

/// 37KB in bytes.
/// This constant represents 37 kb converted to bytes for use in size calculations.
pub const KB_37: usize = 37888;

/// 38KB in bytes.
/// This constant represents 38 kb converted to bytes for use in size calculations.
pub const KB_38: usize = 38912;

/// 39KB in bytes.
/// This constant represents 39 kb converted to bytes for use in size calculations.
pub const KB_39: usize = 39936;

/// 40KB in bytes.
/// This constant represents 40 kb converted to bytes for use in size calculations.
pub const KB_40: usize = 40960;

/// 41KB in bytes.
/// This constant represents 41 kb converted to bytes for use in size calculations.
pub const KB_41: usize = 41984;

/// 42KB in bytes.
/// This constant represents 42 kb converted to bytes for use in size calculations.
pub const KB_42: usize = 43008;

/// 43KB in bytes.
/// This constant represents 43 kb converted to bytes for use in size calculations.
pub const KB_43: usize = 44032;

/// 44KB in bytes.
/// This constant represents 44 kb converted to bytes for use in size calculations.
pub const KB_44: usize = 45056;

/// 45KB in bytes.
/// This constant represents 45 kb converted to bytes for use in size calculations.
pub const KB_45: usize = 46080;

/// 46KB in bytes.
/// This constant represents 46 kb converted to bytes for use in size calculations.
pub const KB_46: usize = 47104;

/// 47KB in bytes.
/// This constant represents 47 kb converted to bytes for use in size calculations.
pub const KB_47: usize = 48128;

/// 48KB in bytes.
/// This constant represents 48 kb converted to bytes for use in size calculations.
pub const KB_48: usize = 49152;

/// 49KB in bytes.
/// This constant represents 49 kb converted to bytes for use in size calculations.
pub const KB_49: usize = 50176;

/// 50KB in bytes.
/// This constant represents 50 kb converted to bytes for use in size calculations.
pub const KB_50: usize = 51200;

/// 51KB in bytes.
/// This constant represents 51 kb converted to bytes for use in size calculations.
pub const KB_51: usize = 52224;

/// 52KB in bytes.
/// This constant represents 52 kb converted to bytes for use in size calculations.
pub const KB_52: usize = 53248;

/// 53KB in bytes.
/// This constant represents 53 kb converted to bytes for use in size calculations.
pub const KB_53: usize = 54272;

/// 54KB in bytes.
/// This constant represents 54 kb converted to bytes for use in size calculations.
pub const KB_54: usize = 55296;

/// 55KB in bytes.
/// This constant represents 55 kb converted to bytes for use in size calculations.
pub const KB_55: usize = 56320;

/// 56KB in bytes.
/// This constant represents 56 kb converted to bytes for use in size calculations.
pub const KB_56: usize = 57344;

/// 57KB in bytes.
/// This constant represents 57 kb converted to bytes for use in size calculations.
pub const KB_57: usize = 58368;

/// 58KB in bytes.
/// This constant represents 58 kb converted to bytes for use in size calculations.
pub const KB_58: usize = 59392;

/// 59KB in bytes.
/// This constant represents 59 kb converted to bytes for use in size calculations.
pub const KB_59: usize = 60416;

/// 60KB in bytes.
/// This constant represents 60 kb converted to bytes for use in size calculations.
pub const KB_60: usize = 61440;

/// 61KB in bytes.
/// This constant represents 61 kb converted to bytes for use in size calculations.
pub const KB_61: usize = 62464;

/// 62KB in bytes.
/// This constant represents 62 kb converted to bytes for use in size calculations.
pub const KB_62: usize = 63488;

/// 63KB in bytes.
/// This constant represents 63 kb converted to bytes for use in size calculations.
pub const KB_63: usize = 64512;

/// 64KB in bytes.
/// This constant represents 64 kb converted to bytes for use in size calculations.
pub const KB_64: usize = 65536;

/// 65KB in bytes.
/// This constant represents 65 kb converted to bytes for use in size calculations.
pub const KB_65: usize = 66560;

/// 66KB in bytes.
/// This constant represents 66 kb converted to bytes for use in size calculations.
pub const KB_66: usize = 67584;

/// 67KB in bytes.
/// This constant represents 67 kb converted to bytes for use in size calculations.
pub const KB_67: usize = 68608;

/// 68KB in bytes.
/// This constant represents 68 kb converted to bytes for use in size calculations.
pub const KB_68: usize = 69632;

/// 69KB in bytes.
/// This constant represents 69 kb converted to bytes for use in size calculations.
pub const KB_69: usize = 70656;

/// 70KB in bytes.
/// This constant represents 70 kb converted to bytes for use in size calculations.
pub const KB_70: usize = 71680;

/// 71KB in bytes.
/// This constant represents 71 kb converted to bytes for use in size calculations.
pub const KB_71: usize = 72704;

/// 72KB in bytes.
/// This constant represents 72 kb converted to bytes for use in size calculations.
pub const KB_72: usize = 73728;

/// 73KB in bytes.
/// This constant represents 73 kb converted to bytes for use in size calculations.
pub const KB_73: usize = 74752;

/// 74KB in bytes.
/// This constant represents 74 kb converted to bytes for use in size calculations.
pub const KB_74: usize = 75776;

/// 75KB in bytes.
/// This constant represents 75 kb converted to bytes for use in size calculations.
pub const KB_75: usize = 76800;

/// 76KB in bytes.
/// This constant represents 76 kb converted to bytes for use in size calculations.
pub const KB_76: usize = 77824;

/// 77KB in bytes.
/// This constant represents 77 kb converted to bytes for use in size calculations.
pub const KB_77: usize = 78848;

/// 78KB in bytes.
/// This constant represents 78 kb converted to bytes for use in size calculations.
pub const KB_78: usize = 79872;

/// 79KB in bytes.
/// This constant represents 79 kb converted to bytes for use in size calculations.
pub const KB_79: usize = 80896;

/// 80KB in bytes.
/// This constant represents 80 kb converted to bytes for use in size calculations.
pub const KB_80: usize = 81920;

/// 81KB in bytes.
/// This constant represents 81 kb converted to bytes for use in size calculations.
pub const KB_81: usize = 82944;

/// 82KB in bytes.
/// This constant represents 82 kb converted to bytes for use in size calculations.
pub const KB_82: usize = 83968;

/// 83KB in bytes.
/// This constant represents 83 kb converted to bytes for use in size calculations.
pub const KB_83: usize = 84992;

/// 84KB in bytes.
/// This constant represents 84 kb converted to bytes for use in size calculations.
pub const KB_84: usize = 86016;

/// 85KB in bytes.
/// This constant represents 85 kb converted to bytes for use in size calculations.
pub const KB_85: usize = 87040;

/// 86KB in bytes.
/// This constant represents 86 kb converted to bytes for use in size calculations.
pub const KB_86: usize = 88064;

/// 87KB in bytes.
/// This constant represents 87 kb converted to bytes for use in size calculations.
pub const KB_87: usize = 89088;

/// 88KB in bytes.
/// This constant represents 88 kb converted to bytes for use in size calculations.
pub const KB_88: usize = 90112;

/// 89KB in bytes.
/// This constant represents 89 kb converted to bytes for use in size calculations.
pub const KB_89: usize = 91136;

/// 90KB in bytes.
/// This constant represents 90 kb converted to bytes for use in size calculations.
pub const KB_90: usize = 92160;

/// 91KB in bytes.
/// This constant represents 91 kb converted to bytes for use in size calculations.
pub const KB_91: usize = 93184;

/// 92KB in bytes.
/// This constant represents 92 kb converted to bytes for use in size calculations.
pub const KB_92: usize = 94208;

/// 93KB in bytes.
/// This constant represents 93 kb converted to bytes for use in size calculations.
pub const KB_93: usize = 95232;

/// 94KB in bytes.
/// This constant represents 94 kb converted to bytes for use in size calculations.
pub const KB_94: usize = 96256;

/// 95KB in bytes.
/// This constant represents 95 kb converted to bytes for use in size calculations.
pub const KB_95: usize = 97280;

/// 96KB in bytes.
/// This constant represents 96 kb converted to bytes for use in size calculations.
pub const KB_96: usize = 98304;

/// 97KB in bytes.
/// This constant represents 97 kb converted to bytes for use in size calculations.
pub const KB_97: usize = 99328;

/// 98KB in bytes.
/// This constant represents 98 kb converted to bytes for use in size calculations.
pub const KB_98: usize = 100352;

/// 99KB in bytes.
/// This constant represents 99 kb converted to bytes for use in size calculations.
pub const KB_99: usize = 101376;

/// 100KB in bytes.
/// This constant represents 100 kb converted to bytes for use in size calculations.
pub const KB_100: usize = 102400;

/// 101KB in bytes.
/// This constant represents 101 kb converted to bytes for use in size calculations.
pub const KB_101: usize = 103424;

/// 102KB in bytes.
/// This constant represents 102 kb converted to bytes for use in size calculations.
pub const KB_102: usize = 104448;

/// 103KB in bytes.
/// This constant represents 103 kb converted to bytes for use in size calculations.
pub const KB_103: usize = 105472;

/// 104KB in bytes.
/// This constant represents 104 kb converted to bytes for use in size calculations.
pub const KB_104: usize = 106496;

/// 105KB in bytes.
/// This constant represents 105 kb converted to bytes for use in size calculations.
pub const KB_105: usize = 107520;

/// 106KB in bytes.
/// This constant represents 106 kb converted to bytes for use in size calculations.
pub const KB_106: usize = 108544;

/// 107KB in bytes.
/// This constant represents 107 kb converted to bytes for use in size calculations.
pub const KB_107: usize = 109568;

/// 108KB in bytes.
/// This constant represents 108 kb converted to bytes for use in size calculations.
pub const KB_108: usize = 110592;

/// 109KB in bytes.
/// This constant represents 109 kb converted to bytes for use in size calculations.
pub const KB_109: usize = 111616;

/// 110KB in bytes.
/// This constant represents 110 kb converted to bytes for use in size calculations.
pub const KB_110: usize = 112640;

/// 111KB in bytes.
/// This constant represents 111 kb converted to bytes for use in size calculations.
pub const KB_111: usize = 113664;

/// 112KB in bytes.
/// This constant represents 112 kb converted to bytes for use in size calculations.
pub const KB_112: usize = 114688;

/// 113KB in bytes.
/// This constant represents 113 kb converted to bytes for use in size calculations.
pub const KB_113: usize = 115712;

/// 114KB in bytes.
/// This constant represents 114 kb converted to bytes for use in size calculations.
pub const KB_114: usize = 116736;

/// 115KB in bytes.
/// This constant represents 115 kb converted to bytes for use in size calculations.
pub const KB_115: usize = 117760;

/// 116KB in bytes.
/// This constant represents 116 kb converted to bytes for use in size calculations.
pub const KB_116: usize = 118784;

/// 117KB in bytes.
/// This constant represents 117 kb converted to bytes for use in size calculations.
pub const KB_117: usize = 119808;

/// 118KB in bytes.
/// This constant represents 118 kb converted to bytes for use in size calculations.
pub const KB_118: usize = 120832;

/// 119KB in bytes.
/// This constant represents 119 kb converted to bytes for use in size calculations.
pub const KB_119: usize = 121856;

/// 120KB in bytes.
/// This constant represents 120 kb converted to bytes for use in size calculations.
pub const KB_120: usize = 122880;

/// 121KB in bytes.
/// This constant represents 121 kb converted to bytes for use in size calculations.
pub const KB_121: usize = 123904;

/// 122KB in bytes.
/// This constant represents 122 kb converted to bytes for use in size calculations.
pub const KB_122: usize = 124928;

/// 123KB in bytes.
/// This constant represents 123 kb converted to bytes for use in size calculations.
pub const KB_123: usize = 125952;

/// 124KB in bytes.
/// This constant represents 124 kb converted to bytes for use in size calculations.
pub const KB_124: usize = 126976;

/// 125KB in bytes.
/// This constant represents 125 kb converted to bytes for use in size calculations.
pub const KB_125: usize = 128000;

/// 126KB in bytes.
/// This constant represents 126 kb converted to bytes for use in size calculations.
pub const KB_126: usize = 129024;

/// 127KB in bytes.
/// This constant represents 127 kb converted to bytes for use in size calculations.
pub const KB_127: usize = 130048;

/// 128KB in bytes.
/// This constant represents 128 kb converted to bytes for use in size calculations.
pub const KB_128: usize = 131072;

/// 129KB in bytes.
/// This constant represents 129 kb converted to bytes for use in size calculations.
pub const KB_129: usize = 132096;

/// 130KB in bytes.
/// This constant represents 130 kb converted to bytes for use in size calculations.
pub const KB_130: usize = 133120;

/// 131KB in bytes.
/// This constant represents 131 kb converted to bytes for use in size calculations.
pub const KB_131: usize = 134144;

/// 132KB in bytes.
/// This constant represents 132 kb converted to bytes for use in size calculations.
pub const KB_132: usize = 135168;

/// 133KB in bytes.
/// This constant represents 133 kb converted to bytes for use in size calculations.
pub const KB_133: usize = 136192;

/// 134KB in bytes.
/// This constant represents 134 kb converted to bytes for use in size calculations.
pub const KB_134: usize = 137216;

/// 135KB in bytes.
/// This constant represents 135 kb converted to bytes for use in size calculations.
pub const KB_135: usize = 138240;

/// 136KB in bytes.
/// This constant represents 136 kb converted to bytes for use in size calculations.
pub const KB_136: usize = 139264;

/// 137KB in bytes.
/// This constant represents 137 kb converted to bytes for use in size calculations.
pub const KB_137: usize = 140288;

/// 138KB in bytes.
/// This constant represents 138 kb converted to bytes for use in size calculations.
pub const KB_138: usize = 141312;

/// 139KB in bytes.
/// This constant represents 139 kb converted to bytes for use in size calculations.
pub const KB_139: usize = 142336;

/// 140KB in bytes.
/// This constant represents 140 kb converted to bytes for use in size calculations.
pub const KB_140: usize = 143360;

/// 141KB in bytes.
/// This constant represents 141 kb converted to bytes for use in size calculations.
pub const KB_141: usize = 144384;

/// 142KB in bytes.
/// This constant represents 142 kb converted to bytes for use in size calculations.
pub const KB_142: usize = 145408;

/// 143KB in bytes.
/// This constant represents 143 kb converted to bytes for use in size calculations.
pub const KB_143: usize = 146432;

/// 144KB in bytes.
/// This constant represents 144 kb converted to bytes for use in size calculations.
pub const KB_144: usize = 147456;

/// 145KB in bytes.
/// This constant represents 145 kb converted to bytes for use in size calculations.
pub const KB_145: usize = 148480;

/// 146KB in bytes.
/// This constant represents 146 kb converted to bytes for use in size calculations.
pub const KB_146: usize = 149504;

/// 147KB in bytes.
/// This constant represents 147 kb converted to bytes for use in size calculations.
pub const KB_147: usize = 150528;

/// 148KB in bytes.
/// This constant represents 148 kb converted to bytes for use in size calculations.
pub const KB_148: usize = 151552;

/// 149KB in bytes.
/// This constant represents 149 kb converted to bytes for use in size calculations.
pub const KB_149: usize = 152576;

/// 150KB in bytes.
/// This constant represents 150 kb converted to bytes for use in size calculations.
pub const KB_150: usize = 153600;

/// 151KB in bytes.
/// This constant represents 151 kb converted to bytes for use in size calculations.
pub const KB_151: usize = 154624;

/// 152KB in bytes.
/// This constant represents 152 kb converted to bytes for use in size calculations.
pub const KB_152: usize = 155648;

/// 153KB in bytes.
/// This constant represents 153 kb converted to bytes for use in size calculations.
pub const KB_153: usize = 156672;

/// 154KB in bytes.
/// This constant represents 154 kb converted to bytes for use in size calculations.
pub const KB_154: usize = 157696;

/// 155KB in bytes.
/// This constant represents 155 kb converted to bytes for use in size calculations.
pub const KB_155: usize = 158720;

/// 156KB in bytes.
/// This constant represents 156 kb converted to bytes for use in size calculations.
pub const KB_156: usize = 159744;

/// 157KB in bytes.
/// This constant represents 157 kb converted to bytes for use in size calculations.
pub const KB_157: usize = 160768;

/// 158KB in bytes.
/// This constant represents 158 kb converted to bytes for use in size calculations.
pub const KB_158: usize = 161792;

/// 159KB in bytes.
/// This constant represents 159 kb converted to bytes for use in size calculations.
pub const KB_159: usize = 162816;

/// 160KB in bytes.
/// This constant represents 160 kb converted to bytes for use in size calculations.
pub const KB_160: usize = 163840;

/// 161KB in bytes.
/// This constant represents 161 kb converted to bytes for use in size calculations.
pub const KB_161: usize = 164864;

/// 162KB in bytes.
/// This constant represents 162 kb converted to bytes for use in size calculations.
pub const KB_162: usize = 165888;

/// 163KB in bytes.
/// This constant represents 163 kb converted to bytes for use in size calculations.
pub const KB_163: usize = 166912;

/// 164KB in bytes.
/// This constant represents 164 kb converted to bytes for use in size calculations.
pub const KB_164: usize = 167936;

/// 165KB in bytes.
/// This constant represents 165 kb converted to bytes for use in size calculations.
pub const KB_165: usize = 168960;

/// 166KB in bytes.
/// This constant represents 166 kb converted to bytes for use in size calculations.
pub const KB_166: usize = 169984;

/// 167KB in bytes.
/// This constant represents 167 kb converted to bytes for use in size calculations.
pub const KB_167: usize = 171008;

/// 168KB in bytes.
/// This constant represents 168 kb converted to bytes for use in size calculations.
pub const KB_168: usize = 172032;

/// 169KB in bytes.
/// This constant represents 169 kb converted to bytes for use in size calculations.
pub const KB_169: usize = 173056;

/// 170KB in bytes.
/// This constant represents 170 kb converted to bytes for use in size calculations.
pub const KB_170: usize = 174080;

/// 171KB in bytes.
/// This constant represents 171 kb converted to bytes for use in size calculations.
pub const KB_171: usize = 175104;

/// 172KB in bytes.
/// This constant represents 172 kb converted to bytes for use in size calculations.
pub const KB_172: usize = 176128;

/// 173KB in bytes.
/// This constant represents 173 kb converted to bytes for use in size calculations.
pub const KB_173: usize = 177152;

/// 174KB in bytes.
/// This constant represents 174 kb converted to bytes for use in size calculations.
pub const KB_174: usize = 178176;

/// 175KB in bytes.
/// This constant represents 175 kb converted to bytes for use in size calculations.
pub const KB_175: usize = 179200;

/// 176KB in bytes.
/// This constant represents 176 kb converted to bytes for use in size calculations.
pub const KB_176: usize = 180224;

/// 177KB in bytes.
/// This constant represents 177 kb converted to bytes for use in size calculations.
pub const KB_177: usize = 181248;

/// 178KB in bytes.
/// This constant represents 178 kb converted to bytes for use in size calculations.
pub const KB_178: usize = 182272;

/// 179KB in bytes.
/// This constant represents 179 kb converted to bytes for use in size calculations.
pub const KB_179: usize = 183296;

/// 180KB in bytes.
/// This constant represents 180 kb converted to bytes for use in size calculations.
pub const KB_180: usize = 184320;

/// 181KB in bytes.
/// This constant represents 181 kb converted to bytes for use in size calculations.
pub const KB_181: usize = 185344;

/// 182KB in bytes.
/// This constant represents 182 kb converted to bytes for use in size calculations.
pub const KB_182: usize = 186368;

/// 183KB in bytes.
/// This constant represents 183 kb converted to bytes for use in size calculations.
pub const KB_183: usize = 187392;

/// 184KB in bytes.
/// This constant represents 184 kb converted to bytes for use in size calculations.
pub const KB_184: usize = 188416;

/// 185KB in bytes.
/// This constant represents 185 kb converted to bytes for use in size calculations.
pub const KB_185: usize = 189440;

/// 186KB in bytes.
/// This constant represents 186 kb converted to bytes for use in size calculations.
pub const KB_186: usize = 190464;

/// 187KB in bytes.
/// This constant represents 187 kb converted to bytes for use in size calculations.
pub const KB_187: usize = 191488;

/// 188KB in bytes.
/// This constant represents 188 kb converted to bytes for use in size calculations.
pub const KB_188: usize = 192512;

/// 189KB in bytes.
/// This constant represents 189 kb converted to bytes for use in size calculations.
pub const KB_189: usize = 193536;

/// 190KB in bytes.
/// This constant represents 190 kb converted to bytes for use in size calculations.
pub const KB_190: usize = 194560;

/// 191KB in bytes.
/// This constant represents 191 kb converted to bytes for use in size calculations.
pub const KB_191: usize = 195584;

/// 192KB in bytes.
/// This constant represents 192 kb converted to bytes for use in size calculations.
pub const KB_192: usize = 196608;

/// 193KB in bytes.
/// This constant represents 193 kb converted to bytes for use in size calculations.
pub const KB_193: usize = 197632;

/// 194KB in bytes.
/// This constant represents 194 kb converted to bytes for use in size calculations.
pub const KB_194: usize = 198656;

/// 195KB in bytes.
/// This constant represents 195 kb converted to bytes for use in size calculations.
pub const KB_195: usize = 199680;

/// 196KB in bytes.
/// This constant represents 196 kb converted to bytes for use in size calculations.
pub const KB_196: usize = 200704;

/// 197KB in bytes.
/// This constant represents 197 kb converted to bytes for use in size calculations.
pub const KB_197: usize = 201728;

/// 198KB in bytes.
/// This constant represents 198 kb converted to bytes for use in size calculations.
pub const KB_198: usize = 202752;

/// 199KB in bytes.
/// This constant represents 199 kb converted to bytes for use in size calculations.
pub const KB_199: usize = 203776;

/// 200KB in bytes.
/// This constant represents 200 kb converted to bytes for use in size calculations.
pub const KB_200: usize = 204800;

/// 201KB in bytes.
/// This constant represents 201 kb converted to bytes for use in size calculations.
pub const KB_201: usize = 205824;

/// 202KB in bytes.
/// This constant represents 202 kb converted to bytes for use in size calculations.
pub const KB_202: usize = 206848;

/// 203KB in bytes.
/// This constant represents 203 kb converted to bytes for use in size calculations.
pub const KB_203: usize = 207872;

/// 204KB in bytes.
/// This constant represents 204 kb converted to bytes for use in size calculations.
pub const KB_204: usize = 208896;

/// 205KB in bytes.
/// This constant represents 205 kb converted to bytes for use in size calculations.
pub const KB_205: usize = 209920;

/// 206KB in bytes.
/// This constant represents 206 kb converted to bytes for use in size calculations.
pub const KB_206: usize = 210944;

/// 207KB in bytes.
/// This constant represents 207 kb converted to bytes for use in size calculations.
pub const KB_207: usize = 211968;

/// 208KB in bytes.
/// This constant represents 208 kb converted to bytes for use in size calculations.
pub const KB_208: usize = 212992;

/// 209KB in bytes.
/// This constant represents 209 kb converted to bytes for use in size calculations.
pub const KB_209: usize = 214016;

/// 210KB in bytes.
/// This constant represents 210 kb converted to bytes for use in size calculations.
pub const KB_210: usize = 215040;

/// 211KB in bytes.
/// This constant represents 211 kb converted to bytes for use in size calculations.
pub const KB_211: usize = 216064;

/// 212KB in bytes.
/// This constant represents 212 kb converted to bytes for use in size calculations.
pub const KB_212: usize = 217088;

/// 213KB in bytes.
/// This constant represents 213 kb converted to bytes for use in size calculations.
pub const KB_213: usize = 218112;

/// 214KB in bytes.
/// This constant represents 214 kb converted to bytes for use in size calculations.
pub const KB_214: usize = 219136;

/// 215KB in bytes.
/// This constant represents 215 kb converted to bytes for use in size calculations.
pub const KB_215: usize = 220160;

/// 216KB in bytes.
/// This constant represents 216 kb converted to bytes for use in size calculations.
pub const KB_216: usize = 221184;

/// 217KB in bytes.
/// This constant represents 217 kb converted to bytes for use in size calculations.
pub const KB_217: usize = 222208;

/// 218KB in bytes.
/// This constant represents 218 kb converted to bytes for use in size calculations.
pub const KB_218: usize = 223232;

/// 219KB in bytes.
/// This constant represents 219 kb converted to bytes for use in size calculations.
pub const KB_219: usize = 224256;

/// 220KB in bytes.
/// This constant represents 220 kb converted to bytes for use in size calculations.
pub const KB_220: usize = 225280;

/// 221KB in bytes.
/// This constant represents 221 kb converted to bytes for use in size calculations.
pub const KB_221: usize = 226304;

/// 222KB in bytes.
/// This constant represents 222 kb converted to bytes for use in size calculations.
pub const KB_222: usize = 227328;

/// 223KB in bytes.
/// This constant represents 223 kb converted to bytes for use in size calculations.
pub const KB_223: usize = 228352;

/// 224KB in bytes.
/// This constant represents 224 kb converted to bytes for use in size calculations.
pub const KB_224: usize = 229376;

/// 225KB in bytes.
/// This constant represents 225 kb converted to bytes for use in size calculations.
pub const KB_225: usize = 230400;

/// 226KB in bytes.
/// This constant represents 226 kb converted to bytes for use in size calculations.
pub const KB_226: usize = 231424;

/// 227KB in bytes.
/// This constant represents 227 kb converted to bytes for use in size calculations.
pub const KB_227: usize = 232448;

/// 228KB in bytes.
/// This constant represents 228 kb converted to bytes for use in size calculations.
pub const KB_228: usize = 233472;

/// 229KB in bytes.
/// This constant represents 229 kb converted to bytes for use in size calculations.
pub const KB_229: usize = 234496;

/// 230KB in bytes.
/// This constant represents 230 kb converted to bytes for use in size calculations.
pub const KB_230: usize = 235520;

/// 231KB in bytes.
/// This constant represents 231 kb converted to bytes for use in size calculations.
pub const KB_231: usize = 236544;

/// 232KB in bytes.
/// This constant represents 232 kb converted to bytes for use in size calculations.
pub const KB_232: usize = 237568;

/// 233KB in bytes.
/// This constant represents 233 kb converted to bytes for use in size calculations.
pub const KB_233: usize = 238592;

/// 234KB in bytes.
/// This constant represents 234 kb converted to bytes for use in size calculations.
pub const KB_234: usize = 239616;

/// 235KB in bytes.
/// This constant represents 235 kb converted to bytes for use in size calculations.
pub const KB_235: usize = 240640;

/// 236KB in bytes.
/// This constant represents 236 kb converted to bytes for use in size calculations.
pub const KB_236: usize = 241664;

/// 237KB in bytes.
/// This constant represents 237 kb converted to bytes for use in size calculations.
pub const KB_237: usize = 242688;

/// 238KB in bytes.
/// This constant represents 238 kb converted to bytes for use in size calculations.
pub const KB_238: usize = 243712;

/// 239KB in bytes.
/// This constant represents 239 kb converted to bytes for use in size calculations.
pub const KB_239: usize = 244736;

/// 240KB in bytes.
/// This constant represents 240 kb converted to bytes for use in size calculations.
pub const KB_240: usize = 245760;

/// 241KB in bytes.
/// This constant represents 241 kb converted to bytes for use in size calculations.
pub const KB_241: usize = 246784;

/// 242KB in bytes.
/// This constant represents 242 kb converted to bytes for use in size calculations.
pub const KB_242: usize = 247808;

/// 243KB in bytes.
/// This constant represents 243 kb converted to bytes for use in size calculations.
pub const KB_243: usize = 248832;

/// 244KB in bytes.
/// This constant represents 244 kb converted to bytes for use in size calculations.
pub const KB_244: usize = 249856;

/// 245KB in bytes.
/// This constant represents 245 kb converted to bytes for use in size calculations.
pub const KB_245: usize = 250880;

/// 246KB in bytes.
/// This constant represents 246 kb converted to bytes for use in size calculations.
pub const KB_246: usize = 251904;

/// 247KB in bytes.
/// This constant represents 247 kb converted to bytes for use in size calculations.
pub const KB_247: usize = 252928;

/// 248KB in bytes.
/// This constant represents 248 kb converted to bytes for use in size calculations.
pub const KB_248: usize = 253952;

/// 249KB in bytes.
/// This constant represents 249 kb converted to bytes for use in size calculations.
pub const KB_249: usize = 254976;

/// 250KB in bytes.
/// This constant represents 250 kb converted to bytes for use in size calculations.
pub const KB_250: usize = 256000;

/// 251KB in bytes.
/// This constant represents 251 kb converted to bytes for use in size calculations.
pub const KB_251: usize = 257024;

/// 252KB in bytes.
/// This constant represents 252 kb converted to bytes for use in size calculations.
pub const KB_252: usize = 258048;

/// 253KB in bytes.
/// This constant represents 253 kb converted to bytes for use in size calculations.
pub const KB_253: usize = 259072;

/// 254KB in bytes.
/// This constant represents 254 kb converted to bytes for use in size calculations.
pub const KB_254: usize = 260096;

/// 255KB in bytes.
/// This constant represents 255 kb converted to bytes for use in size calculations.
pub const KB_255: usize = 261120;

/// 256KB in bytes.
/// This constant represents 256 kb converted to bytes for use in size calculations.
pub const KB_256: usize = 262144;

/// 257KB in bytes.
/// This constant represents 257 kb converted to bytes for use in size calculations.
pub const KB_257: usize = 263168;

/// 258KB in bytes.
/// This constant represents 258 kb converted to bytes for use in size calculations.
pub const KB_258: usize = 264192;

/// 259KB in bytes.
/// This constant represents 259 kb converted to bytes for use in size calculations.
pub const KB_259: usize = 265216;

/// 260KB in bytes.
/// This constant represents 260 kb converted to bytes for use in size calculations.
pub const KB_260: usize = 266240;

/// 261KB in bytes.
/// This constant represents 261 kb converted to bytes for use in size calculations.
pub const KB_261: usize = 267264;

/// 262KB in bytes.
/// This constant represents 262 kb converted to bytes for use in size calculations.
pub const KB_262: usize = 268288;

/// 263KB in bytes.
/// This constant represents 263 kb converted to bytes for use in size calculations.
pub const KB_263: usize = 269312;

/// 264KB in bytes.
/// This constant represents 264 kb converted to bytes for use in size calculations.
pub const KB_264: usize = 270336;

/// 265KB in bytes.
/// This constant represents 265 kb converted to bytes for use in size calculations.
pub const KB_265: usize = 271360;

/// 266KB in bytes.
/// This constant represents 266 kb converted to bytes for use in size calculations.
pub const KB_266: usize = 272384;

/// 267KB in bytes.
/// This constant represents 267 kb converted to bytes for use in size calculations.
pub const KB_267: usize = 273408;

/// 268KB in bytes.
/// This constant represents 268 kb converted to bytes for use in size calculations.
pub const KB_268: usize = 274432;

/// 269KB in bytes.
/// This constant represents 269 kb converted to bytes for use in size calculations.
pub const KB_269: usize = 275456;

/// 270KB in bytes.
/// This constant represents 270 kb converted to bytes for use in size calculations.
pub const KB_270: usize = 276480;

/// 271KB in bytes.
/// This constant represents 271 kb converted to bytes for use in size calculations.
pub const KB_271: usize = 277504;

/// 272KB in bytes.
/// This constant represents 272 kb converted to bytes for use in size calculations.
pub const KB_272: usize = 278528;

/// 273KB in bytes.
/// This constant represents 273 kb converted to bytes for use in size calculations.
pub const KB_273: usize = 279552;

/// 274KB in bytes.
/// This constant represents 274 kb converted to bytes for use in size calculations.
pub const KB_274: usize = 280576;

/// 275KB in bytes.
/// This constant represents 275 kb converted to bytes for use in size calculations.
pub const KB_275: usize = 281600;

/// 276KB in bytes.
/// This constant represents 276 kb converted to bytes for use in size calculations.
pub const KB_276: usize = 282624;

/// 277KB in bytes.
/// This constant represents 277 kb converted to bytes for use in size calculations.
pub const KB_277: usize = 283648;

/// 278KB in bytes.
/// This constant represents 278 kb converted to bytes for use in size calculations.
pub const KB_278: usize = 284672;

/// 279KB in bytes.
/// This constant represents 279 kb converted to bytes for use in size calculations.
pub const KB_279: usize = 285696;

/// 280KB in bytes.
/// This constant represents 280 kb converted to bytes for use in size calculations.
pub const KB_280: usize = 286720;

/// 281KB in bytes.
/// This constant represents 281 kb converted to bytes for use in size calculations.
pub const KB_281: usize = 287744;

/// 282KB in bytes.
/// This constant represents 282 kb converted to bytes for use in size calculations.
pub const KB_282: usize = 288768;

/// 283KB in bytes.
/// This constant represents 283 kb converted to bytes for use in size calculations.
pub const KB_283: usize = 289792;

/// 284KB in bytes.
/// This constant represents 284 kb converted to bytes for use in size calculations.
pub const KB_284: usize = 290816;

/// 285KB in bytes.
/// This constant represents 285 kb converted to bytes for use in size calculations.
pub const KB_285: usize = 291840;

/// 286KB in bytes.
/// This constant represents 286 kb converted to bytes for use in size calculations.
pub const KB_286: usize = 292864;

/// 287KB in bytes.
/// This constant represents 287 kb converted to bytes for use in size calculations.
pub const KB_287: usize = 293888;

/// 288KB in bytes.
/// This constant represents 288 kb converted to bytes for use in size calculations.
pub const KB_288: usize = 294912;

/// 289KB in bytes.
/// This constant represents 289 kb converted to bytes for use in size calculations.
pub const KB_289: usize = 295936;

/// 290KB in bytes.
/// This constant represents 290 kb converted to bytes for use in size calculations.
pub const KB_290: usize = 296960;

/// 291KB in bytes.
/// This constant represents 291 kb converted to bytes for use in size calculations.
pub const KB_291: usize = 297984;

/// 292KB in bytes.
/// This constant represents 292 kb converted to bytes for use in size calculations.
pub const KB_292: usize = 299008;

/// 293KB in bytes.
/// This constant represents 293 kb converted to bytes for use in size calculations.
pub const KB_293: usize = 300032;

/// 294KB in bytes.
/// This constant represents 294 kb converted to bytes for use in size calculations.
pub const KB_294: usize = 301056;

/// 295KB in bytes.
/// This constant represents 295 kb converted to bytes for use in size calculations.
pub const KB_295: usize = 302080;

/// 296KB in bytes.
/// This constant represents 296 kb converted to bytes for use in size calculations.
pub const KB_296: usize = 303104;

/// 297KB in bytes.
/// This constant represents 297 kb converted to bytes for use in size calculations.
pub const KB_297: usize = 304128;

/// 298KB in bytes.
/// This constant represents 298 kb converted to bytes for use in size calculations.
pub const KB_298: usize = 305152;

/// 299KB in bytes.
/// This constant represents 299 kb converted to bytes for use in size calculations.
pub const KB_299: usize = 306176;

/// 300KB in bytes.
/// This constant represents 300 kb converted to bytes for use in size calculations.
pub const KB_300: usize = 307200;

/// 301KB in bytes.
/// This constant represents 301 kb converted to bytes for use in size calculations.
pub const KB_301: usize = 308224;

/// 302KB in bytes.
/// This constant represents 302 kb converted to bytes for use in size calculations.
pub const KB_302: usize = 309248;

/// 303KB in bytes.
/// This constant represents 303 kb converted to bytes for use in size calculations.
pub const KB_303: usize = 310272;

/// 304KB in bytes.
/// This constant represents 304 kb converted to bytes for use in size calculations.
pub const KB_304: usize = 311296;

/// 305KB in bytes.
/// This constant represents 305 kb converted to bytes for use in size calculations.
pub const KB_305: usize = 312320;

/// 306KB in bytes.
/// This constant represents 306 kb converted to bytes for use in size calculations.
pub const KB_306: usize = 313344;

/// 307KB in bytes.
/// This constant represents 307 kb converted to bytes for use in size calculations.
pub const KB_307: usize = 314368;

/// 308KB in bytes.
/// This constant represents 308 kb converted to bytes for use in size calculations.
pub const KB_308: usize = 315392;

/// 309KB in bytes.
/// This constant represents 309 kb converted to bytes for use in size calculations.
pub const KB_309: usize = 316416;

/// 310KB in bytes.
/// This constant represents 310 kb converted to bytes for use in size calculations.
pub const KB_310: usize = 317440;

/// 311KB in bytes.
/// This constant represents 311 kb converted to bytes for use in size calculations.
pub const KB_311: usize = 318464;

/// 312KB in bytes.
/// This constant represents 312 kb converted to bytes for use in size calculations.
pub const KB_312: usize = 319488;

/// 313KB in bytes.
/// This constant represents 313 kb converted to bytes for use in size calculations.
pub const KB_313: usize = 320512;

/// 314KB in bytes.
/// This constant represents 314 kb converted to bytes for use in size calculations.
pub const KB_314: usize = 321536;

/// 315KB in bytes.
/// This constant represents 315 kb converted to bytes for use in size calculations.
pub const KB_315: usize = 322560;

/// 316KB in bytes.
/// This constant represents 316 kb converted to bytes for use in size calculations.
pub const KB_316: usize = 323584;

/// 317KB in bytes.
/// This constant represents 317 kb converted to bytes for use in size calculations.
pub const KB_317: usize = 324608;

/// 318KB in bytes.
/// This constant represents 318 kb converted to bytes for use in size calculations.
pub const KB_318: usize = 325632;

/// 319KB in bytes.
/// This constant represents 319 kb converted to bytes for use in size calculations.
pub const KB_319: usize = 326656;

/// 320KB in bytes.
/// This constant represents 320 kb converted to bytes for use in size calculations.
pub const KB_320: usize = 327680;

/// 321KB in bytes.
/// This constant represents 321 kb converted to bytes for use in size calculations.
pub const KB_321: usize = 328704;

/// 322KB in bytes.
/// This constant represents 322 kb converted to bytes for use in size calculations.
pub const KB_322: usize = 329728;

/// 323KB in bytes.
/// This constant represents 323 kb converted to bytes for use in size calculations.
pub const KB_323: usize = 330752;

/// 324KB in bytes.
/// This constant represents 324 kb converted to bytes for use in size calculations.
pub const KB_324: usize = 331776;

/// 325KB in bytes.
/// This constant represents 325 kb converted to bytes for use in size calculations.
pub const KB_325: usize = 332800;

/// 326KB in bytes.
/// This constant represents 326 kb converted to bytes for use in size calculations.
pub const KB_326: usize = 333824;

/// 327KB in bytes.
/// This constant represents 327 kb converted to bytes for use in size calculations.
pub const KB_327: usize = 334848;

/// 328KB in bytes.
/// This constant represents 328 kb converted to bytes for use in size calculations.
pub const KB_328: usize = 335872;

/// 329KB in bytes.
/// This constant represents 329 kb converted to bytes for use in size calculations.
pub const KB_329: usize = 336896;

/// 330KB in bytes.
/// This constant represents 330 kb converted to bytes for use in size calculations.
pub const KB_330: usize = 337920;

/// 331KB in bytes.
/// This constant represents 331 kb converted to bytes for use in size calculations.
pub const KB_331: usize = 338944;

/// 332KB in bytes.
/// This constant represents 332 kb converted to bytes for use in size calculations.
pub const KB_332: usize = 339968;

/// 333KB in bytes.
/// This constant represents 333 kb converted to bytes for use in size calculations.
pub const KB_333: usize = 340992;

/// 334KB in bytes.
/// This constant represents 334 kb converted to bytes for use in size calculations.
pub const KB_334: usize = 342016;

/// 335KB in bytes.
/// This constant represents 335 kb converted to bytes for use in size calculations.
pub const KB_335: usize = 343040;

/// 336KB in bytes.
/// This constant represents 336 kb converted to bytes for use in size calculations.
pub const KB_336: usize = 344064;

/// 337KB in bytes.
/// This constant represents 337 kb converted to bytes for use in size calculations.
pub const KB_337: usize = 345088;

/// 338KB in bytes.
/// This constant represents 338 kb converted to bytes for use in size calculations.
pub const KB_338: usize = 346112;

/// 339KB in bytes.
/// This constant represents 339 kb converted to bytes for use in size calculations.
pub const KB_339: usize = 347136;

/// 340KB in bytes.
/// This constant represents 340 kb converted to bytes for use in size calculations.
pub const KB_340: usize = 348160;

/// 341KB in bytes.
/// This constant represents 341 kb converted to bytes for use in size calculations.
pub const KB_341: usize = 349184;

/// 342KB in bytes.
/// This constant represents 342 kb converted to bytes for use in size calculations.
pub const KB_342: usize = 350208;

/// 343KB in bytes.
/// This constant represents 343 kb converted to bytes for use in size calculations.
pub const KB_343: usize = 351232;

/// 344KB in bytes.
/// This constant represents 344 kb converted to bytes for use in size calculations.
pub const KB_344: usize = 352256;

/// 345KB in bytes.
/// This constant represents 345 kb converted to bytes for use in size calculations.
pub const KB_345: usize = 353280;

/// 346KB in bytes.
/// This constant represents 346 kb converted to bytes for use in size calculations.
pub const KB_346: usize = 354304;

/// 347KB in bytes.
/// This constant represents 347 kb converted to bytes for use in size calculations.
pub const KB_347: usize = 355328;

/// 348KB in bytes.
/// This constant represents 348 kb converted to bytes for use in size calculations.
pub const KB_348: usize = 356352;

/// 349KB in bytes.
/// This constant represents 349 kb converted to bytes for use in size calculations.
pub const KB_349: usize = 357376;

/// 350KB in bytes.
/// This constant represents 350 kb converted to bytes for use in size calculations.
pub const KB_350: usize = 358400;

/// 351KB in bytes.
/// This constant represents 351 kb converted to bytes for use in size calculations.
pub const KB_351: usize = 359424;

/// 352KB in bytes.
/// This constant represents 352 kb converted to bytes for use in size calculations.
pub const KB_352: usize = 360448;

/// 353KB in bytes.
/// This constant represents 353 kb converted to bytes for use in size calculations.
pub const KB_353: usize = 361472;

/// 354KB in bytes.
/// This constant represents 354 kb converted to bytes for use in size calculations.
pub const KB_354: usize = 362496;

/// 355KB in bytes.
/// This constant represents 355 kb converted to bytes for use in size calculations.
pub const KB_355: usize = 363520;

/// 356KB in bytes.
/// This constant represents 356 kb converted to bytes for use in size calculations.
pub const KB_356: usize = 364544;

/// 357KB in bytes.
/// This constant represents 357 kb converted to bytes for use in size calculations.
pub const KB_357: usize = 365568;

/// 358KB in bytes.
/// This constant represents 358 kb converted to bytes for use in size calculations.
pub const KB_358: usize = 366592;

/// 359KB in bytes.
/// This constant represents 359 kb converted to bytes for use in size calculations.
pub const KB_359: usize = 367616;

/// 360KB in bytes.
/// This constant represents 360 kb converted to bytes for use in size calculations.
pub const KB_360: usize = 368640;

/// 361KB in bytes.
/// This constant represents 361 kb converted to bytes for use in size calculations.
pub const KB_361: usize = 369664;

/// 362KB in bytes.
/// This constant represents 362 kb converted to bytes for use in size calculations.
pub const KB_362: usize = 370688;

/// 363KB in bytes.
/// This constant represents 363 kb converted to bytes for use in size calculations.
pub const KB_363: usize = 371712;

/// 364KB in bytes.
/// This constant represents 364 kb converted to bytes for use in size calculations.
pub const KB_364: usize = 372736;

/// 365KB in bytes.
/// This constant represents 365 kb converted to bytes for use in size calculations.
pub const KB_365: usize = 373760;

/// 366KB in bytes.
/// This constant represents 366 kb converted to bytes for use in size calculations.
pub const KB_366: usize = 374784;

/// 367KB in bytes.
/// This constant represents 367 kb converted to bytes for use in size calculations.
pub const KB_367: usize = 375808;

/// 368KB in bytes.
/// This constant represents 368 kb converted to bytes for use in size calculations.
pub const KB_368: usize = 376832;

/// 369KB in bytes.
/// This constant represents 369 kb converted to bytes for use in size calculations.
pub const KB_369: usize = 377856;

/// 370KB in bytes.
/// This constant represents 370 kb converted to bytes for use in size calculations.
pub const KB_370: usize = 378880;

/// 371KB in bytes.
/// This constant represents 371 kb converted to bytes for use in size calculations.
pub const KB_371: usize = 379904;

/// 372KB in bytes.
/// This constant represents 372 kb converted to bytes for use in size calculations.
pub const KB_372: usize = 380928;

/// 373KB in bytes.
/// This constant represents 373 kb converted to bytes for use in size calculations.
pub const KB_373: usize = 381952;

/// 374KB in bytes.
/// This constant represents 374 kb converted to bytes for use in size calculations.
pub const KB_374: usize = 382976;

/// 375KB in bytes.
/// This constant represents 375 kb converted to bytes for use in size calculations.
pub const KB_375: usize = 384000;

/// 376KB in bytes.
/// This constant represents 376 kb converted to bytes for use in size calculations.
pub const KB_376: usize = 385024;

/// 377KB in bytes.
/// This constant represents 377 kb converted to bytes for use in size calculations.
pub const KB_377: usize = 386048;

/// 378KB in bytes.
/// This constant represents 378 kb converted to bytes for use in size calculations.
pub const KB_378: usize = 387072;

/// 379KB in bytes.
/// This constant represents 379 kb converted to bytes for use in size calculations.
pub const KB_379: usize = 388096;

/// 380KB in bytes.
/// This constant represents 380 kb converted to bytes for use in size calculations.
pub const KB_380: usize = 389120;

/// 381KB in bytes.
/// This constant represents 381 kb converted to bytes for use in size calculations.
pub const KB_381: usize = 390144;

/// 382KB in bytes.
/// This constant represents 382 kb converted to bytes for use in size calculations.
pub const KB_382: usize = 391168;

/// 383KB in bytes.
/// This constant represents 383 kb converted to bytes for use in size calculations.
pub const KB_383: usize = 392192;

/// 384KB in bytes.
/// This constant represents 384 kb converted to bytes for use in size calculations.
pub const KB_384: usize = 393216;

/// 385KB in bytes.
/// This constant represents 385 kb converted to bytes for use in size calculations.
pub const KB_385: usize = 394240;

/// 386KB in bytes.
/// This constant represents 386 kb converted to bytes for use in size calculations.
pub const KB_386: usize = 395264;

/// 387KB in bytes.
/// This constant represents 387 kb converted to bytes for use in size calculations.
pub const KB_387: usize = 396288;

/// 388KB in bytes.
/// This constant represents 388 kb converted to bytes for use in size calculations.
pub const KB_388: usize = 397312;

/// 389KB in bytes.
/// This constant represents 389 kb converted to bytes for use in size calculations.
pub const KB_389: usize = 398336;

/// 390KB in bytes.
/// This constant represents 390 kb converted to bytes for use in size calculations.
pub const KB_390: usize = 399360;

/// 391KB in bytes.
/// This constant represents 391 kb converted to bytes for use in size calculations.
pub const KB_391: usize = 400384;

/// 392KB in bytes.
/// This constant represents 392 kb converted to bytes for use in size calculations.
pub const KB_392: usize = 401408;

/// 393KB in bytes.
/// This constant represents 393 kb converted to bytes for use in size calculations.
pub const KB_393: usize = 402432;

/// 394KB in bytes.
/// This constant represents 394 kb converted to bytes for use in size calculations.
pub const KB_394: usize = 403456;

/// 395KB in bytes.
/// This constant represents 395 kb converted to bytes for use in size calculations.
pub const KB_395: usize = 404480;

/// 396KB in bytes.
/// This constant represents 396 kb converted to bytes for use in size calculations.
pub const KB_396: usize = 405504;

/// 397KB in bytes.
/// This constant represents 397 kb converted to bytes for use in size calculations.
pub const KB_397: usize = 406528;

/// 398KB in bytes.
/// This constant represents 398 kb converted to bytes for use in size calculations.
pub const KB_398: usize = 407552;

/// 399KB in bytes.
/// This constant represents 399 kb converted to bytes for use in size calculations.
pub const KB_399: usize = 408576;

/// 400KB in bytes.
/// This constant represents 400 kb converted to bytes for use in size calculations.
pub const KB_400: usize = 409600;

/// 401KB in bytes.
/// This constant represents 401 kb converted to bytes for use in size calculations.
pub const KB_401: usize = 410624;

/// 402KB in bytes.
/// This constant represents 402 kb converted to bytes for use in size calculations.
pub const KB_402: usize = 411648;

/// 403KB in bytes.
/// This constant represents 403 kb converted to bytes for use in size calculations.
pub const KB_403: usize = 412672;

/// 404KB in bytes.
/// This constant represents 404 kb converted to bytes for use in size calculations.
pub const KB_404: usize = 413696;

/// 405KB in bytes.
/// This constant represents 405 kb converted to bytes for use in size calculations.
pub const KB_405: usize = 414720;

/// 406KB in bytes.
/// This constant represents 406 kb converted to bytes for use in size calculations.
pub const KB_406: usize = 415744;

/// 407KB in bytes.
/// This constant represents 407 kb converted to bytes for use in size calculations.
pub const KB_407: usize = 416768;

/// 408KB in bytes.
/// This constant represents 408 kb converted to bytes for use in size calculations.
pub const KB_408: usize = 417792;

/// 409KB in bytes.
/// This constant represents 409 kb converted to bytes for use in size calculations.
pub const KB_409: usize = 418816;

/// 410KB in bytes.
/// This constant represents 410 kb converted to bytes for use in size calculations.
pub const KB_410: usize = 419840;

/// 411KB in bytes.
/// This constant represents 411 kb converted to bytes for use in size calculations.
pub const KB_411: usize = 420864;

/// 412KB in bytes.
/// This constant represents 412 kb converted to bytes for use in size calculations.
pub const KB_412: usize = 421888;

/// 413KB in bytes.
/// This constant represents 413 kb converted to bytes for use in size calculations.
pub const KB_413: usize = 422912;

/// 414KB in bytes.
/// This constant represents 414 kb converted to bytes for use in size calculations.
pub const KB_414: usize = 423936;

/// 415KB in bytes.
/// This constant represents 415 kb converted to bytes for use in size calculations.
pub const KB_415: usize = 424960;

/// 416KB in bytes.
/// This constant represents 416 kb converted to bytes for use in size calculations.
pub const KB_416: usize = 425984;

/// 417KB in bytes.
/// This constant represents 417 kb converted to bytes for use in size calculations.
pub const KB_417: usize = 427008;

/// 418KB in bytes.
/// This constant represents 418 kb converted to bytes for use in size calculations.
pub const KB_418: usize = 428032;

/// 419KB in bytes.
/// This constant represents 419 kb converted to bytes for use in size calculations.
pub const KB_419: usize = 429056;

/// 420KB in bytes.
/// This constant represents 420 kb converted to bytes for use in size calculations.
pub const KB_420: usize = 430080;

/// 421KB in bytes.
/// This constant represents 421 kb converted to bytes for use in size calculations.
pub const KB_421: usize = 431104;

/// 422KB in bytes.
/// This constant represents 422 kb converted to bytes for use in size calculations.
pub const KB_422: usize = 432128;

/// 423KB in bytes.
/// This constant represents 423 kb converted to bytes for use in size calculations.
pub const KB_423: usize = 433152;

/// 424KB in bytes.
/// This constant represents 424 kb converted to bytes for use in size calculations.
pub const KB_424: usize = 434176;

/// 425KB in bytes.
/// This constant represents 425 kb converted to bytes for use in size calculations.
pub const KB_425: usize = 435200;

/// 426KB in bytes.
/// This constant represents 426 kb converted to bytes for use in size calculations.
pub const KB_426: usize = 436224;

/// 427KB in bytes.
/// This constant represents 427 kb converted to bytes for use in size calculations.
pub const KB_427: usize = 437248;

/// 428KB in bytes.
/// This constant represents 428 kb converted to bytes for use in size calculations.
pub const KB_428: usize = 438272;

/// 429KB in bytes.
/// This constant represents 429 kb converted to bytes for use in size calculations.
pub const KB_429: usize = 439296;

/// 430KB in bytes.
/// This constant represents 430 kb converted to bytes for use in size calculations.
pub const KB_430: usize = 440320;

/// 431KB in bytes.
/// This constant represents 431 kb converted to bytes for use in size calculations.
pub const KB_431: usize = 441344;

/// 432KB in bytes.
/// This constant represents 432 kb converted to bytes for use in size calculations.
pub const KB_432: usize = 442368;

/// 433KB in bytes.
/// This constant represents 433 kb converted to bytes for use in size calculations.
pub const KB_433: usize = 443392;

/// 434KB in bytes.
/// This constant represents 434 kb converted to bytes for use in size calculations.
pub const KB_434: usize = 444416;

/// 435KB in bytes.
/// This constant represents 435 kb converted to bytes for use in size calculations.
pub const KB_435: usize = 445440;

/// 436KB in bytes.
/// This constant represents 436 kb converted to bytes for use in size calculations.
pub const KB_436: usize = 446464;

/// 437KB in bytes.
/// This constant represents 437 kb converted to bytes for use in size calculations.
pub const KB_437: usize = 447488;

/// 438KB in bytes.
/// This constant represents 438 kb converted to bytes for use in size calculations.
pub const KB_438: usize = 448512;

/// 439KB in bytes.
/// This constant represents 439 kb converted to bytes for use in size calculations.
pub const KB_439: usize = 449536;

/// 440KB in bytes.
/// This constant represents 440 kb converted to bytes for use in size calculations.
pub const KB_440: usize = 450560;

/// 441KB in bytes.
/// This constant represents 441 kb converted to bytes for use in size calculations.
pub const KB_441: usize = 451584;

/// 442KB in bytes.
/// This constant represents 442 kb converted to bytes for use in size calculations.
pub const KB_442: usize = 452608;

/// 443KB in bytes.
/// This constant represents 443 kb converted to bytes for use in size calculations.
pub const KB_443: usize = 453632;

/// 444KB in bytes.
/// This constant represents 444 kb converted to bytes for use in size calculations.
pub const KB_444: usize = 454656;

/// 445KB in bytes.
/// This constant represents 445 kb converted to bytes for use in size calculations.
pub const KB_445: usize = 455680;

/// 446KB in bytes.
/// This constant represents 446 kb converted to bytes for use in size calculations.
pub const KB_446: usize = 456704;

/// 447KB in bytes.
/// This constant represents 447 kb converted to bytes for use in size calculations.
pub const KB_447: usize = 457728;

/// 448KB in bytes.
/// This constant represents 448 kb converted to bytes for use in size calculations.
pub const KB_448: usize = 458752;

/// 449KB in bytes.
/// This constant represents 449 kb converted to bytes for use in size calculations.
pub const KB_449: usize = 459776;

/// 450KB in bytes.
/// This constant represents 450 kb converted to bytes for use in size calculations.
pub const KB_450: usize = 460800;

/// 451KB in bytes.
/// This constant represents 451 kb converted to bytes for use in size calculations.
pub const KB_451: usize = 461824;

/// 452KB in bytes.
/// This constant represents 452 kb converted to bytes for use in size calculations.
pub const KB_452: usize = 462848;

/// 453KB in bytes.
/// This constant represents 453 kb converted to bytes for use in size calculations.
pub const KB_453: usize = 463872;

/// 454KB in bytes.
/// This constant represents 454 kb converted to bytes for use in size calculations.
pub const KB_454: usize = 464896;

/// 455KB in bytes.
/// This constant represents 455 kb converted to bytes for use in size calculations.
pub const KB_455: usize = 465920;

/// 456KB in bytes.
/// This constant represents 456 kb converted to bytes for use in size calculations.
pub const KB_456: usize = 466944;

/// 457KB in bytes.
/// This constant represents 457 kb converted to bytes for use in size calculations.
pub const KB_457: usize = 467968;

/// 458KB in bytes.
/// This constant represents 458 kb converted to bytes for use in size calculations.
pub const KB_458: usize = 468992;

/// 459KB in bytes.
/// This constant represents 459 kb converted to bytes for use in size calculations.
pub const KB_459: usize = 470016;

/// 460KB in bytes.
/// This constant represents 460 kb converted to bytes for use in size calculations.
pub const KB_460: usize = 471040;

/// 461KB in bytes.
/// This constant represents 461 kb converted to bytes for use in size calculations.
pub const KB_461: usize = 472064;

/// 462KB in bytes.
/// This constant represents 462 kb converted to bytes for use in size calculations.
pub const KB_462: usize = 473088;

/// 463KB in bytes.
/// This constant represents 463 kb converted to bytes for use in size calculations.
pub const KB_463: usize = 474112;

/// 464KB in bytes.
/// This constant represents 464 kb converted to bytes for use in size calculations.
pub const KB_464: usize = 475136;

/// 465KB in bytes.
/// This constant represents 465 kb converted to bytes for use in size calculations.
pub const KB_465: usize = 476160;

/// 466KB in bytes.
/// This constant represents 466 kb converted to bytes for use in size calculations.
pub const KB_466: usize = 477184;

/// 467KB in bytes.
/// This constant represents 467 kb converted to bytes for use in size calculations.
pub const KB_467: usize = 478208;

/// 468KB in bytes.
/// This constant represents 468 kb converted to bytes for use in size calculations.
pub const KB_468: usize = 479232;

/// 469KB in bytes.
/// This constant represents 469 kb converted to bytes for use in size calculations.
pub const KB_469: usize = 480256;

/// 470KB in bytes.
/// This constant represents 470 kb converted to bytes for use in size calculations.
pub const KB_470: usize = 481280;

/// 471KB in bytes.
/// This constant represents 471 kb converted to bytes for use in size calculations.
pub const KB_471: usize = 482304;

/// 472KB in bytes.
/// This constant represents 472 kb converted to bytes for use in size calculations.
pub const KB_472: usize = 483328;

/// 473KB in bytes.
/// This constant represents 473 kb converted to bytes for use in size calculations.
pub const KB_473: usize = 484352;

/// 474KB in bytes.
/// This constant represents 474 kb converted to bytes for use in size calculations.
pub const KB_474: usize = 485376;

/// 475KB in bytes.
/// This constant represents 475 kb converted to bytes for use in size calculations.
pub const KB_475: usize = 486400;

/// 476KB in bytes.
/// This constant represents 476 kb converted to bytes for use in size calculations.
pub const KB_476: usize = 487424;

/// 477KB in bytes.
/// This constant represents 477 kb converted to bytes for use in size calculations.
pub const KB_477: usize = 488448;

/// 478KB in bytes.
/// This constant represents 478 kb converted to bytes for use in size calculations.
pub const KB_478: usize = 489472;

/// 479KB in bytes.
/// This constant represents 479 kb converted to bytes for use in size calculations.
pub const KB_479: usize = 490496;

/// 480KB in bytes.
/// This constant represents 480 kb converted to bytes for use in size calculations.
pub const KB_480: usize = 491520;

/// 481KB in bytes.
/// This constant represents 481 kb converted to bytes for use in size calculations.
pub const KB_481: usize = 492544;

/// 482KB in bytes.
/// This constant represents 482 kb converted to bytes for use in size calculations.
pub const KB_482: usize = 493568;

/// 483KB in bytes.
/// This constant represents 483 kb converted to bytes for use in size calculations.
pub const KB_483: usize = 494592;

/// 484KB in bytes.
/// This constant represents 484 kb converted to bytes for use in size calculations.
pub const KB_484: usize = 495616;

/// 485KB in bytes.
/// This constant represents 485 kb converted to bytes for use in size calculations.
pub const KB_485: usize = 496640;

/// 486KB in bytes.
/// This constant represents 486 kb converted to bytes for use in size calculations.
pub const KB_486: usize = 497664;

/// 487KB in bytes.
/// This constant represents 487 kb converted to bytes for use in size calculations.
pub const KB_487: usize = 498688;

/// 488KB in bytes.
/// This constant represents 488 kb converted to bytes for use in size calculations.
pub const KB_488: usize = 499712;

/// 489KB in bytes.
/// This constant represents 489 kb converted to bytes for use in size calculations.
pub const KB_489: usize = 500736;

/// 490KB in bytes.
/// This constant represents 490 kb converted to bytes for use in size calculations.
pub const KB_490: usize = 501760;

/// 491KB in bytes.
/// This constant represents 491 kb converted to bytes for use in size calculations.
pub const KB_491: usize = 502784;

/// 492KB in bytes.
/// This constant represents 492 kb converted to bytes for use in size calculations.
pub const KB_492: usize = 503808;

/// 493KB in bytes.
/// This constant represents 493 kb converted to bytes for use in size calculations.
pub const KB_493: usize = 504832;

/// 494KB in bytes.
/// This constant represents 494 kb converted to bytes for use in size calculations.
pub const KB_494: usize = 505856;

/// 495KB in bytes.
/// This constant represents 495 kb converted to bytes for use in size calculations.
pub const KB_495: usize = 506880;

/// 496KB in bytes.
/// This constant represents 496 kb converted to bytes for use in size calculations.
pub const KB_496: usize = 507904;

/// 497KB in bytes.
/// This constant represents 497 kb converted to bytes for use in size calculations.
pub const KB_497: usize = 508928;

/// 498KB in bytes.
/// This constant represents 498 kb converted to bytes for use in size calculations.
pub const KB_498: usize = 509952;

/// 499KB in bytes.
/// This constant represents 499 kb converted to bytes for use in size calculations.
pub const KB_499: usize = 510976;

/// 500KB in bytes.
/// This constant represents 500 kb converted to bytes for use in size calculations.
pub const KB_500: usize = 512000;

/// 501KB in bytes.
/// This constant represents 501 kb converted to bytes for use in size calculations.
pub const KB_501: usize = 513024;

/// 502KB in bytes.
/// This constant represents 502 kb converted to bytes for use in size calculations.
pub const KB_502: usize = 514048;

/// 503KB in bytes.
/// This constant represents 503 kb converted to bytes for use in size calculations.
pub const KB_503: usize = 515072;

/// 504KB in bytes.
/// This constant represents 504 kb converted to bytes for use in size calculations.
pub const KB_504: usize = 516096;

/// 505KB in bytes.
/// This constant represents 505 kb converted to bytes for use in size calculations.
pub const KB_505: usize = 517120;

/// 506KB in bytes.
/// This constant represents 506 kb converted to bytes for use in size calculations.
pub const KB_506: usize = 518144;

/// 507KB in bytes.
/// This constant represents 507 kb converted to bytes for use in size calculations.
pub const KB_507: usize = 519168;

/// 508KB in bytes.
/// This constant represents 508 kb converted to bytes for use in size calculations.
pub const KB_508: usize = 520192;

/// 509KB in bytes.
/// This constant represents 509 kb converted to bytes for use in size calculations.
pub const KB_509: usize = 521216;

/// 510KB in bytes.
/// This constant represents 510 kb converted to bytes for use in size calculations.
pub const KB_510: usize = 522240;

/// 511KB in bytes.
/// This constant represents 511 kb converted to bytes for use in size calculations.
pub const KB_511: usize = 523264;

/// 512KB in bytes.
/// This constant represents 512 kb converted to bytes for use in size calculations.
pub const KB_512: usize = 524288;

/// 513KB in bytes.
/// This constant represents 513 kb converted to bytes for use in size calculations.
pub const KB_513: usize = 525312;

/// 514KB in bytes.
/// This constant represents 514 kb converted to bytes for use in size calculations.
pub const KB_514: usize = 526336;

/// 515KB in bytes.
/// This constant represents 515 kb converted to bytes for use in size calculations.
pub const KB_515: usize = 527360;

/// 516KB in bytes.
/// This constant represents 516 kb converted to bytes for use in size calculations.
pub const KB_516: usize = 528384;

/// 517KB in bytes.
/// This constant represents 517 kb converted to bytes for use in size calculations.
pub const KB_517: usize = 529408;

/// 518KB in bytes.
/// This constant represents 518 kb converted to bytes for use in size calculations.
pub const KB_518: usize = 530432;

/// 519KB in bytes.
/// This constant represents 519 kb converted to bytes for use in size calculations.
pub const KB_519: usize = 531456;

/// 520KB in bytes.
/// This constant represents 520 kb converted to bytes for use in size calculations.
pub const KB_520: usize = 532480;

/// 521KB in bytes.
/// This constant represents 521 kb converted to bytes for use in size calculations.
pub const KB_521: usize = 533504;

/// 522KB in bytes.
/// This constant represents 522 kb converted to bytes for use in size calculations.
pub const KB_522: usize = 534528;

/// 523KB in bytes.
/// This constant represents 523 kb converted to bytes for use in size calculations.
pub const KB_523: usize = 535552;

/// 524KB in bytes.
/// This constant represents 524 kb converted to bytes for use in size calculations.
pub const KB_524: usize = 536576;

/// 525KB in bytes.
/// This constant represents 525 kb converted to bytes for use in size calculations.
pub const KB_525: usize = 537600;

/// 526KB in bytes.
/// This constant represents 526 kb converted to bytes for use in size calculations.
pub const KB_526: usize = 538624;

/// 527KB in bytes.
/// This constant represents 527 kb converted to bytes for use in size calculations.
pub const KB_527: usize = 539648;

/// 528KB in bytes.
/// This constant represents 528 kb converted to bytes for use in size calculations.
pub const KB_528: usize = 540672;

/// 529KB in bytes.
/// This constant represents 529 kb converted to bytes for use in size calculations.
pub const KB_529: usize = 541696;

/// 530KB in bytes.
/// This constant represents 530 kb converted to bytes for use in size calculations.
pub const KB_530: usize = 542720;

/// 531KB in bytes.
/// This constant represents 531 kb converted to bytes for use in size calculations.
pub const KB_531: usize = 543744;

/// 532KB in bytes.
/// This constant represents 532 kb converted to bytes for use in size calculations.
pub const KB_532: usize = 544768;

/// 533KB in bytes.
/// This constant represents 533 kb converted to bytes for use in size calculations.
pub const KB_533: usize = 545792;

/// 534KB in bytes.
/// This constant represents 534 kb converted to bytes for use in size calculations.
pub const KB_534: usize = 546816;

/// 535KB in bytes.
/// This constant represents 535 kb converted to bytes for use in size calculations.
pub const KB_535: usize = 547840;

/// 536KB in bytes.
/// This constant represents 536 kb converted to bytes for use in size calculations.
pub const KB_536: usize = 548864;

/// 537KB in bytes.
/// This constant represents 537 kb converted to bytes for use in size calculations.
pub const KB_537: usize = 549888;

/// 538KB in bytes.
/// This constant represents 538 kb converted to bytes for use in size calculations.
pub const KB_538: usize = 550912;

/// 539KB in bytes.
/// This constant represents 539 kb converted to bytes for use in size calculations.
pub const KB_539: usize = 551936;

/// 540KB in bytes.
/// This constant represents 540 kb converted to bytes for use in size calculations.
pub const KB_540: usize = 552960;

/// 541KB in bytes.
/// This constant represents 541 kb converted to bytes for use in size calculations.
pub const KB_541: usize = 553984;

/// 542KB in bytes.
/// This constant represents 542 kb converted to bytes for use in size calculations.
pub const KB_542: usize = 555008;

/// 543KB in bytes.
/// This constant represents 543 kb converted to bytes for use in size calculations.
pub const KB_543: usize = 556032;

/// 544KB in bytes.
/// This constant represents 544 kb converted to bytes for use in size calculations.
pub const KB_544: usize = 557056;

/// 545KB in bytes.
/// This constant represents 545 kb converted to bytes for use in size calculations.
pub const KB_545: usize = 558080;

/// 546KB in bytes.
/// This constant represents 546 kb converted to bytes for use in size calculations.
pub const KB_546: usize = 559104;

/// 547KB in bytes.
/// This constant represents 547 kb converted to bytes for use in size calculations.
pub const KB_547: usize = 560128;

/// 548KB in bytes.
/// This constant represents 548 kb converted to bytes for use in size calculations.
pub const KB_548: usize = 561152;

/// 549KB in bytes.
/// This constant represents 549 kb converted to bytes for use in size calculations.
pub const KB_549: usize = 562176;

/// 550KB in bytes.
/// This constant represents 550 kb converted to bytes for use in size calculations.
pub const KB_550: usize = 563200;

/// 551KB in bytes.
/// This constant represents 551 kb converted to bytes for use in size calculations.
pub const KB_551: usize = 564224;

/// 552KB in bytes.
/// This constant represents 552 kb converted to bytes for use in size calculations.
pub const KB_552: usize = 565248;

/// 553KB in bytes.
/// This constant represents 553 kb converted to bytes for use in size calculations.
pub const KB_553: usize = 566272;

/// 554KB in bytes.
/// This constant represents 554 kb converted to bytes for use in size calculations.
pub const KB_554: usize = 567296;

/// 555KB in bytes.
/// This constant represents 555 kb converted to bytes for use in size calculations.
pub const KB_555: usize = 568320;

/// 556KB in bytes.
/// This constant represents 556 kb converted to bytes for use in size calculations.
pub const KB_556: usize = 569344;

/// 557KB in bytes.
/// This constant represents 557 kb converted to bytes for use in size calculations.
pub const KB_557: usize = 570368;

/// 558KB in bytes.
/// This constant represents 558 kb converted to bytes for use in size calculations.
pub const KB_558: usize = 571392;

/// 559KB in bytes.
/// This constant represents 559 kb converted to bytes for use in size calculations.
pub const KB_559: usize = 572416;

/// 560KB in bytes.
/// This constant represents 560 kb converted to bytes for use in size calculations.
pub const KB_560: usize = 573440;

/// 561KB in bytes.
/// This constant represents 561 kb converted to bytes for use in size calculations.
pub const KB_561: usize = 574464;

/// 562KB in bytes.
/// This constant represents 562 kb converted to bytes for use in size calculations.
pub const KB_562: usize = 575488;

/// 563KB in bytes.
/// This constant represents 563 kb converted to bytes for use in size calculations.
pub const KB_563: usize = 576512;

/// 564KB in bytes.
/// This constant represents 564 kb converted to bytes for use in size calculations.
pub const KB_564: usize = 577536;

/// 565KB in bytes.
/// This constant represents 565 kb converted to bytes for use in size calculations.
pub const KB_565: usize = 578560;

/// 566KB in bytes.
/// This constant represents 566 kb converted to bytes for use in size calculations.
pub const KB_566: usize = 579584;

/// 567KB in bytes.
/// This constant represents 567 kb converted to bytes for use in size calculations.
pub const KB_567: usize = 580608;

/// 568KB in bytes.
/// This constant represents 568 kb converted to bytes for use in size calculations.
pub const KB_568: usize = 581632;

/// 569KB in bytes.
/// This constant represents 569 kb converted to bytes for use in size calculations.
pub const KB_569: usize = 582656;

/// 570KB in bytes.
/// This constant represents 570 kb converted to bytes for use in size calculations.
pub const KB_570: usize = 583680;

/// 571KB in bytes.
/// This constant represents 571 kb converted to bytes for use in size calculations.
pub const KB_571: usize = 584704;

/// 572KB in bytes.
/// This constant represents 572 kb converted to bytes for use in size calculations.
pub const KB_572: usize = 585728;

/// 573KB in bytes.
/// This constant represents 573 kb converted to bytes for use in size calculations.
pub const KB_573: usize = 586752;

/// 574KB in bytes.
/// This constant represents 574 kb converted to bytes for use in size calculations.
pub const KB_574: usize = 587776;

/// 575KB in bytes.
/// This constant represents 575 kb converted to bytes for use in size calculations.
pub const KB_575: usize = 588800;

/// 576KB in bytes.
/// This constant represents 576 kb converted to bytes for use in size calculations.
pub const KB_576: usize = 589824;

/// 577KB in bytes.
/// This constant represents 577 kb converted to bytes for use in size calculations.
pub const KB_577: usize = 590848;

/// 578KB in bytes.
/// This constant represents 578 kb converted to bytes for use in size calculations.
pub const KB_578: usize = 591872;

/// 579KB in bytes.
/// This constant represents 579 kb converted to bytes for use in size calculations.
pub const KB_579: usize = 592896;

/// 580KB in bytes.
/// This constant represents 580 kb converted to bytes for use in size calculations.
pub const KB_580: usize = 593920;

/// 581KB in bytes.
/// This constant represents 581 kb converted to bytes for use in size calculations.
pub const KB_581: usize = 594944;

/// 582KB in bytes.
/// This constant represents 582 kb converted to bytes for use in size calculations.
pub const KB_582: usize = 595968;

/// 583KB in bytes.
/// This constant represents 583 kb converted to bytes for use in size calculations.
pub const KB_583: usize = 596992;

/// 584KB in bytes.
/// This constant represents 584 kb converted to bytes for use in size calculations.
pub const KB_584: usize = 598016;

/// 585KB in bytes.
/// This constant represents 585 kb converted to bytes for use in size calculations.
pub const KB_585: usize = 599040;

/// 586KB in bytes.
/// This constant represents 586 kb converted to bytes for use in size calculations.
pub const KB_586: usize = 600064;

/// 587KB in bytes.
/// This constant represents 587 kb converted to bytes for use in size calculations.
pub const KB_587: usize = 601088;

/// 588KB in bytes.
/// This constant represents 588 kb converted to bytes for use in size calculations.
pub const KB_588: usize = 602112;

/// 589KB in bytes.
/// This constant represents 589 kb converted to bytes for use in size calculations.
pub const KB_589: usize = 603136;

/// 590KB in bytes.
/// This constant represents 590 kb converted to bytes for use in size calculations.
pub const KB_590: usize = 604160;

/// 591KB in bytes.
/// This constant represents 591 kb converted to bytes for use in size calculations.
pub const KB_591: usize = 605184;

/// 592KB in bytes.
/// This constant represents 592 kb converted to bytes for use in size calculations.
pub const KB_592: usize = 606208;

/// 593KB in bytes.
/// This constant represents 593 kb converted to bytes for use in size calculations.
pub const KB_593: usize = 607232;

/// 594KB in bytes.
/// This constant represents 594 kb converted to bytes for use in size calculations.
pub const KB_594: usize = 608256;

/// 595KB in bytes.
/// This constant represents 595 kb converted to bytes for use in size calculations.
pub const KB_595: usize = 609280;

/// 596KB in bytes.
/// This constant represents 596 kb converted to bytes for use in size calculations.
pub const KB_596: usize = 610304;

/// 597KB in bytes.
/// This constant represents 597 kb converted to bytes for use in size calculations.
pub const KB_597: usize = 611328;

/// 598KB in bytes.
/// This constant represents 598 kb converted to bytes for use in size calculations.
pub const KB_598: usize = 612352;

/// 599KB in bytes.
/// This constant represents 599 kb converted to bytes for use in size calculations.
pub const KB_599: usize = 613376;

/// 600KB in bytes.
/// This constant represents 600 kb converted to bytes for use in size calculations.
pub const KB_600: usize = 614400;

/// 601KB in bytes.
/// This constant represents 601 kb converted to bytes for use in size calculations.
pub const KB_601: usize = 615424;

/// 602KB in bytes.
/// This constant represents 602 kb converted to bytes for use in size calculations.
pub const KB_602: usize = 616448;

/// 603KB in bytes.
/// This constant represents 603 kb converted to bytes for use in size calculations.
pub const KB_603: usize = 617472;

/// 604KB in bytes.
/// This constant represents 604 kb converted to bytes for use in size calculations.
pub const KB_604: usize = 618496;

/// 605KB in bytes.
/// This constant represents 605 kb converted to bytes for use in size calculations.
pub const KB_605: usize = 619520;

/// 606KB in bytes.
/// This constant represents 606 kb converted to bytes for use in size calculations.
pub const KB_606: usize = 620544;

/// 607KB in bytes.
/// This constant represents 607 kb converted to bytes for use in size calculations.
pub const KB_607: usize = 621568;

/// 608KB in bytes.
/// This constant represents 608 kb converted to bytes for use in size calculations.
pub const KB_608: usize = 622592;

/// 609KB in bytes.
/// This constant represents 609 kb converted to bytes for use in size calculations.
pub const KB_609: usize = 623616;

/// 610KB in bytes.
/// This constant represents 610 kb converted to bytes for use in size calculations.
pub const KB_610: usize = 624640;

/// 611KB in bytes.
/// This constant represents 611 kb converted to bytes for use in size calculations.
pub const KB_611: usize = 625664;

/// 612KB in bytes.
/// This constant represents 612 kb converted to bytes for use in size calculations.
pub const KB_612: usize = 626688;

/// 613KB in bytes.
/// This constant represents 613 kb converted to bytes for use in size calculations.
pub const KB_613: usize = 627712;

/// 614KB in bytes.
/// This constant represents 614 kb converted to bytes for use in size calculations.
pub const KB_614: usize = 628736;

/// 615KB in bytes.
/// This constant represents 615 kb converted to bytes for use in size calculations.
pub const KB_615: usize = 629760;

/// 616KB in bytes.
/// This constant represents 616 kb converted to bytes for use in size calculations.
pub const KB_616: usize = 630784;

/// 617KB in bytes.
/// This constant represents 617 kb converted to bytes for use in size calculations.
pub const KB_617: usize = 631808;

/// 618KB in bytes.
/// This constant represents 618 kb converted to bytes for use in size calculations.
pub const KB_618: usize = 632832;

/// 619KB in bytes.
/// This constant represents 619 kb converted to bytes for use in size calculations.
pub const KB_619: usize = 633856;

/// 620KB in bytes.
/// This constant represents 620 kb converted to bytes for use in size calculations.
pub const KB_620: usize = 634880;

/// 621KB in bytes.
/// This constant represents 621 kb converted to bytes for use in size calculations.
pub const KB_621: usize = 635904;

/// 622KB in bytes.
/// This constant represents 622 kb converted to bytes for use in size calculations.
pub const KB_622: usize = 636928;

/// 623KB in bytes.
/// This constant represents 623 kb converted to bytes for use in size calculations.
pub const KB_623: usize = 637952;

/// 624KB in bytes.
/// This constant represents 624 kb converted to bytes for use in size calculations.
pub const KB_624: usize = 638976;

/// 625KB in bytes.
/// This constant represents 625 kb converted to bytes for use in size calculations.
pub const KB_625: usize = 640000;

/// 626KB in bytes.
/// This constant represents 626 kb converted to bytes for use in size calculations.
pub const KB_626: usize = 641024;

/// 627KB in bytes.
/// This constant represents 627 kb converted to bytes for use in size calculations.
pub const KB_627: usize = 642048;

/// 628KB in bytes.
/// This constant represents 628 kb converted to bytes for use in size calculations.
pub const KB_628: usize = 643072;

/// 629KB in bytes.
/// This constant represents 629 kb converted to bytes for use in size calculations.
pub const KB_629: usize = 644096;

/// 630KB in bytes.
/// This constant represents 630 kb converted to bytes for use in size calculations.
pub const KB_630: usize = 645120;

/// 631KB in bytes.
/// This constant represents 631 kb converted to bytes for use in size calculations.
pub const KB_631: usize = 646144;

/// 632KB in bytes.
/// This constant represents 632 kb converted to bytes for use in size calculations.
pub const KB_632: usize = 647168;

/// 633KB in bytes.
/// This constant represents 633 kb converted to bytes for use in size calculations.
pub const KB_633: usize = 648192;

/// 634KB in bytes.
/// This constant represents 634 kb converted to bytes for use in size calculations.
pub const KB_634: usize = 649216;

/// 635KB in bytes.
/// This constant represents 635 kb converted to bytes for use in size calculations.
pub const KB_635: usize = 650240;

/// 636KB in bytes.
/// This constant represents 636 kb converted to bytes for use in size calculations.
pub const KB_636: usize = 651264;

/// 637KB in bytes.
/// This constant represents 637 kb converted to bytes for use in size calculations.
pub const KB_637: usize = 652288;

/// 638KB in bytes.
/// This constant represents 638 kb converted to bytes for use in size calculations.
pub const KB_638: usize = 653312;

/// 639KB in bytes.
/// This constant represents 639 kb converted to bytes for use in size calculations.
pub const KB_639: usize = 654336;

/// 640KB in bytes.
/// This constant represents 640 kb converted to bytes for use in size calculations.
pub const KB_640: usize = 655360;

/// 641KB in bytes.
/// This constant represents 641 kb converted to bytes for use in size calculations.
pub const KB_641: usize = 656384;

/// 642KB in bytes.
/// This constant represents 642 kb converted to bytes for use in size calculations.
pub const KB_642: usize = 657408;

/// 643KB in bytes.
/// This constant represents 643 kb converted to bytes for use in size calculations.
pub const KB_643: usize = 658432;

/// 644KB in bytes.
/// This constant represents 644 kb converted to bytes for use in size calculations.
pub const KB_644: usize = 659456;

/// 645KB in bytes.
/// This constant represents 645 kb converted to bytes for use in size calculations.
pub const KB_645: usize = 660480;

/// 646KB in bytes.
/// This constant represents 646 kb converted to bytes for use in size calculations.
pub const KB_646: usize = 661504;

/// 647KB in bytes.
/// This constant represents 647 kb converted to bytes for use in size calculations.
pub const KB_647: usize = 662528;

/// 648KB in bytes.
/// This constant represents 648 kb converted to bytes for use in size calculations.
pub const KB_648: usize = 663552;

/// 649KB in bytes.
/// This constant represents 649 kb converted to bytes for use in size calculations.
pub const KB_649: usize = 664576;

/// 650KB in bytes.
/// This constant represents 650 kb converted to bytes for use in size calculations.
pub const KB_650: usize = 665600;

/// 651KB in bytes.
/// This constant represents 651 kb converted to bytes for use in size calculations.
pub const KB_651: usize = 666624;

/// 652KB in bytes.
/// This constant represents 652 kb converted to bytes for use in size calculations.
pub const KB_652: usize = 667648;

/// 653KB in bytes.
/// This constant represents 653 kb converted to bytes for use in size calculations.
pub const KB_653: usize = 668672;

/// 654KB in bytes.
/// This constant represents 654 kb converted to bytes for use in size calculations.
pub const KB_654: usize = 669696;

/// 655KB in bytes.
/// This constant represents 655 kb converted to bytes for use in size calculations.
pub const KB_655: usize = 670720;

/// 656KB in bytes.
/// This constant represents 656 kb converted to bytes for use in size calculations.
pub const KB_656: usize = 671744;

/// 657KB in bytes.
/// This constant represents 657 kb converted to bytes for use in size calculations.
pub const KB_657: usize = 672768;

/// 658KB in bytes.
/// This constant represents 658 kb converted to bytes for use in size calculations.
pub const KB_658: usize = 673792;

/// 659KB in bytes.
/// This constant represents 659 kb converted to bytes for use in size calculations.
pub const KB_659: usize = 674816;

/// 660KB in bytes.
/// This constant represents 660 kb converted to bytes for use in size calculations.
pub const KB_660: usize = 675840;

/// 661KB in bytes.
/// This constant represents 661 kb converted to bytes for use in size calculations.
pub const KB_661: usize = 676864;

/// 662KB in bytes.
/// This constant represents 662 kb converted to bytes for use in size calculations.
pub const KB_662: usize = 677888;

/// 663KB in bytes.
/// This constant represents 663 kb converted to bytes for use in size calculations.
pub const KB_663: usize = 678912;

/// 664KB in bytes.
/// This constant represents 664 kb converted to bytes for use in size calculations.
pub const KB_664: usize = 679936;

/// 665KB in bytes.
/// This constant represents 665 kb converted to bytes for use in size calculations.
pub const KB_665: usize = 680960;

/// 666KB in bytes.
/// This constant represents 666 kb converted to bytes for use in size calculations.
pub const KB_666: usize = 681984;

/// 667KB in bytes.
/// This constant represents 667 kb converted to bytes for use in size calculations.
pub const KB_667: usize = 683008;

/// 668KB in bytes.
/// This constant represents 668 kb converted to bytes for use in size calculations.
pub const KB_668: usize = 684032;

/// 669KB in bytes.
/// This constant represents 669 kb converted to bytes for use in size calculations.
pub const KB_669: usize = 685056;

/// 670KB in bytes.
/// This constant represents 670 kb converted to bytes for use in size calculations.
pub const KB_670: usize = 686080;

/// 671KB in bytes.
/// This constant represents 671 kb converted to bytes for use in size calculations.
pub const KB_671: usize = 687104;

/// 672KB in bytes.
/// This constant represents 672 kb converted to bytes for use in size calculations.
pub const KB_672: usize = 688128;

/// 673KB in bytes.
/// This constant represents 673 kb converted to bytes for use in size calculations.
pub const KB_673: usize = 689152;

/// 674KB in bytes.
/// This constant represents 674 kb converted to bytes for use in size calculations.
pub const KB_674: usize = 690176;

/// 675KB in bytes.
/// This constant represents 675 kb converted to bytes for use in size calculations.
pub const KB_675: usize = 691200;

/// 676KB in bytes.
/// This constant represents 676 kb converted to bytes for use in size calculations.
pub const KB_676: usize = 692224;

/// 677KB in bytes.
/// This constant represents 677 kb converted to bytes for use in size calculations.
pub const KB_677: usize = 693248;

/// 678KB in bytes.
/// This constant represents 678 kb converted to bytes for use in size calculations.
pub const KB_678: usize = 694272;

/// 679KB in bytes.
/// This constant represents 679 kb converted to bytes for use in size calculations.
pub const KB_679: usize = 695296;

/// 680KB in bytes.
/// This constant represents 680 kb converted to bytes for use in size calculations.
pub const KB_680: usize = 696320;

/// 681KB in bytes.
/// This constant represents 681 kb converted to bytes for use in size calculations.
pub const KB_681: usize = 697344;

/// 682KB in bytes.
/// This constant represents 682 kb converted to bytes for use in size calculations.
pub const KB_682: usize = 698368;

/// 683KB in bytes.
/// This constant represents 683 kb converted to bytes for use in size calculations.
pub const KB_683: usize = 699392;

/// 684KB in bytes.
/// This constant represents 684 kb converted to bytes for use in size calculations.
pub const KB_684: usize = 700416;

/// 685KB in bytes.
/// This constant represents 685 kb converted to bytes for use in size calculations.
pub const KB_685: usize = 701440;

/// 686KB in bytes.
/// This constant represents 686 kb converted to bytes for use in size calculations.
pub const KB_686: usize = 702464;

/// 687KB in bytes.
/// This constant represents 687 kb converted to bytes for use in size calculations.
pub const KB_687: usize = 703488;

/// 688KB in bytes.
/// This constant represents 688 kb converted to bytes for use in size calculations.
pub const KB_688: usize = 704512;

/// 689KB in bytes.
/// This constant represents 689 kb converted to bytes for use in size calculations.
pub const KB_689: usize = 705536;

/// 690KB in bytes.
/// This constant represents 690 kb converted to bytes for use in size calculations.
pub const KB_690: usize = 706560;

/// 691KB in bytes.
/// This constant represents 691 kb converted to bytes for use in size calculations.
pub const KB_691: usize = 707584;

/// 692KB in bytes.
/// This constant represents 692 kb converted to bytes for use in size calculations.
pub const KB_692: usize = 708608;

/// 693KB in bytes.
/// This constant represents 693 kb converted to bytes for use in size calculations.
pub const KB_693: usize = 709632;

/// 694KB in bytes.
/// This constant represents 694 kb converted to bytes for use in size calculations.
pub const KB_694: usize = 710656;

/// 695KB in bytes.
/// This constant represents 695 kb converted to bytes for use in size calculations.
pub const KB_695: usize = 711680;

/// 696KB in bytes.
/// This constant represents 696 kb converted to bytes for use in size calculations.
pub const KB_696: usize = 712704;

/// 697KB in bytes.
/// This constant represents 697 kb converted to bytes for use in size calculations.
pub const KB_697: usize = 713728;

/// 698KB in bytes.
/// This constant represents 698 kb converted to bytes for use in size calculations.
pub const KB_698: usize = 714752;

/// 699KB in bytes.
/// This constant represents 699 kb converted to bytes for use in size calculations.
pub const KB_699: usize = 715776;

/// 700KB in bytes.
/// This constant represents 700 kb converted to bytes for use in size calculations.
pub const KB_700: usize = 716800;

/// 701KB in bytes.
/// This constant represents 701 kb converted to bytes for use in size calculations.
pub const KB_701: usize = 717824;

/// 702KB in bytes.
/// This constant represents 702 kb converted to bytes for use in size calculations.
pub const KB_702: usize = 718848;

/// 703KB in bytes.
/// This constant represents 703 kb converted to bytes for use in size calculations.
pub const KB_703: usize = 719872;

/// 704KB in bytes.
/// This constant represents 704 kb converted to bytes for use in size calculations.
pub const KB_704: usize = 720896;

/// 705KB in bytes.
/// This constant represents 705 kb converted to bytes for use in size calculations.
pub const KB_705: usize = 721920;

/// 706KB in bytes.
/// This constant represents 706 kb converted to bytes for use in size calculations.
pub const KB_706: usize = 722944;

/// 707KB in bytes.
/// This constant represents 707 kb converted to bytes for use in size calculations.
pub const KB_707: usize = 723968;

/// 708KB in bytes.
/// This constant represents 708 kb converted to bytes for use in size calculations.
pub const KB_708: usize = 724992;

/// 709KB in bytes.
/// This constant represents 709 kb converted to bytes for use in size calculations.
pub const KB_709: usize = 726016;

/// 710KB in bytes.
/// This constant represents 710 kb converted to bytes for use in size calculations.
pub const KB_710: usize = 727040;

/// 711KB in bytes.
/// This constant represents 711 kb converted to bytes for use in size calculations.
pub const KB_711: usize = 728064;

/// 712KB in bytes.
/// This constant represents 712 kb converted to bytes for use in size calculations.
pub const KB_712: usize = 729088;

/// 713KB in bytes.
/// This constant represents 713 kb converted to bytes for use in size calculations.
pub const KB_713: usize = 730112;

/// 714KB in bytes.
/// This constant represents 714 kb converted to bytes for use in size calculations.
pub const KB_714: usize = 731136;

/// 715KB in bytes.
/// This constant represents 715 kb converted to bytes for use in size calculations.
pub const KB_715: usize = 732160;

/// 716KB in bytes.
/// This constant represents 716 kb converted to bytes for use in size calculations.
pub const KB_716: usize = 733184;

/// 717KB in bytes.
/// This constant represents 717 kb converted to bytes for use in size calculations.
pub const KB_717: usize = 734208;

/// 718KB in bytes.
/// This constant represents 718 kb converted to bytes for use in size calculations.
pub const KB_718: usize = 735232;

/// 719KB in bytes.
/// This constant represents 719 kb converted to bytes for use in size calculations.
pub const KB_719: usize = 736256;

/// 720KB in bytes.
/// This constant represents 720 kb converted to bytes for use in size calculations.
pub const KB_720: usize = 737280;

/// 721KB in bytes.
/// This constant represents 721 kb converted to bytes for use in size calculations.
pub const KB_721: usize = 738304;

/// 722KB in bytes.
/// This constant represents 722 kb converted to bytes for use in size calculations.
pub const KB_722: usize = 739328;

/// 723KB in bytes.
/// This constant represents 723 kb converted to bytes for use in size calculations.
pub const KB_723: usize = 740352;

/// 724KB in bytes.
/// This constant represents 724 kb converted to bytes for use in size calculations.
pub const KB_724: usize = 741376;

/// 725KB in bytes.
/// This constant represents 725 kb converted to bytes for use in size calculations.
pub const KB_725: usize = 742400;

/// 726KB in bytes.
/// This constant represents 726 kb converted to bytes for use in size calculations.
pub const KB_726: usize = 743424;

/// 727KB in bytes.
/// This constant represents 727 kb converted to bytes for use in size calculations.
pub const KB_727: usize = 744448;

/// 728KB in bytes.
/// This constant represents 728 kb converted to bytes for use in size calculations.
pub const KB_728: usize = 745472;

/// 729KB in bytes.
/// This constant represents 729 kb converted to bytes for use in size calculations.
pub const KB_729: usize = 746496;

/// 730KB in bytes.
/// This constant represents 730 kb converted to bytes for use in size calculations.
pub const KB_730: usize = 747520;

/// 731KB in bytes.
/// This constant represents 731 kb converted to bytes for use in size calculations.
pub const KB_731: usize = 748544;

/// 732KB in bytes.
/// This constant represents 732 kb converted to bytes for use in size calculations.
pub const KB_732: usize = 749568;

/// 733KB in bytes.
/// This constant represents 733 kb converted to bytes for use in size calculations.
pub const KB_733: usize = 750592;

/// 734KB in bytes.
/// This constant represents 734 kb converted to bytes for use in size calculations.
pub const KB_734: usize = 751616;

/// 735KB in bytes.
/// This constant represents 735 kb converted to bytes for use in size calculations.
pub const KB_735: usize = 752640;

/// 736KB in bytes.
/// This constant represents 736 kb converted to bytes for use in size calculations.
pub const KB_736: usize = 753664;

/// 737KB in bytes.
/// This constant represents 737 kb converted to bytes for use in size calculations.
pub const KB_737: usize = 754688;

/// 738KB in bytes.
/// This constant represents 738 kb converted to bytes for use in size calculations.
pub const KB_738: usize = 755712;

/// 739KB in bytes.
/// This constant represents 739 kb converted to bytes for use in size calculations.
pub const KB_739: usize = 756736;

/// 740KB in bytes.
/// This constant represents 740 kb converted to bytes for use in size calculations.
pub const KB_740: usize = 757760;

/// 741KB in bytes.
/// This constant represents 741 kb converted to bytes for use in size calculations.
pub const KB_741: usize = 758784;

/// 742KB in bytes.
/// This constant represents 742 kb converted to bytes for use in size calculations.
pub const KB_742: usize = 759808;

/// 743KB in bytes.
/// This constant represents 743 kb converted to bytes for use in size calculations.
pub const KB_743: usize = 760832;

/// 744KB in bytes.
/// This constant represents 744 kb converted to bytes for use in size calculations.
pub const KB_744: usize = 761856;

/// 745KB in bytes.
/// This constant represents 745 kb converted to bytes for use in size calculations.
pub const KB_745: usize = 762880;

/// 746KB in bytes.
/// This constant represents 746 kb converted to bytes for use in size calculations.
pub const KB_746: usize = 763904;

/// 747KB in bytes.
/// This constant represents 747 kb converted to bytes for use in size calculations.
pub const KB_747: usize = 764928;

/// 748KB in bytes.
/// This constant represents 748 kb converted to bytes for use in size calculations.
pub const KB_748: usize = 765952;

/// 749KB in bytes.
/// This constant represents 749 kb converted to bytes for use in size calculations.
pub const KB_749: usize = 766976;

/// 750KB in bytes.
/// This constant represents 750 kb converted to bytes for use in size calculations.
pub const KB_750: usize = 768000;

/// 751KB in bytes.
/// This constant represents 751 kb converted to bytes for use in size calculations.
pub const KB_751: usize = 769024;

/// 752KB in bytes.
/// This constant represents 752 kb converted to bytes for use in size calculations.
pub const KB_752: usize = 770048;

/// 753KB in bytes.
/// This constant represents 753 kb converted to bytes for use in size calculations.
pub const KB_753: usize = 771072;

/// 754KB in bytes.
/// This constant represents 754 kb converted to bytes for use in size calculations.
pub const KB_754: usize = 772096;

/// 755KB in bytes.
/// This constant represents 755 kb converted to bytes for use in size calculations.
pub const KB_755: usize = 773120;

/// 756KB in bytes.
/// This constant represents 756 kb converted to bytes for use in size calculations.
pub const KB_756: usize = 774144;

/// 757KB in bytes.
/// This constant represents 757 kb converted to bytes for use in size calculations.
pub const KB_757: usize = 775168;

/// 758KB in bytes.
/// This constant represents 758 kb converted to bytes for use in size calculations.
pub const KB_758: usize = 776192;

/// 759KB in bytes.
/// This constant represents 759 kb converted to bytes for use in size calculations.
pub const KB_759: usize = 777216;

/// 760KB in bytes.
/// This constant represents 760 kb converted to bytes for use in size calculations.
pub const KB_760: usize = 778240;

/// 761KB in bytes.
/// This constant represents 761 kb converted to bytes for use in size calculations.
pub const KB_761: usize = 779264;

/// 762KB in bytes.
/// This constant represents 762 kb converted to bytes for use in size calculations.
pub const KB_762: usize = 780288;

/// 763KB in bytes.
/// This constant represents 763 kb converted to bytes for use in size calculations.
pub const KB_763: usize = 781312;

/// 764KB in bytes.
/// This constant represents 764 kb converted to bytes for use in size calculations.
pub const KB_764: usize = 782336;

/// 765KB in bytes.
/// This constant represents 765 kb converted to bytes for use in size calculations.
pub const KB_765: usize = 783360;

/// 766KB in bytes.
/// This constant represents 766 kb converted to bytes for use in size calculations.
pub const KB_766: usize = 784384;

/// 767KB in bytes.
/// This constant represents 767 kb converted to bytes for use in size calculations.
pub const KB_767: usize = 785408;

/// 768KB in bytes.
/// This constant represents 768 kb converted to bytes for use in size calculations.
pub const KB_768: usize = 786432;

/// 769KB in bytes.
/// This constant represents 769 kb converted to bytes for use in size calculations.
pub const KB_769: usize = 787456;

/// 770KB in bytes.
/// This constant represents 770 kb converted to bytes for use in size calculations.
pub const KB_770: usize = 788480;

/// 771KB in bytes.
/// This constant represents 771 kb converted to bytes for use in size calculations.
pub const KB_771: usize = 789504;

/// 772KB in bytes.
/// This constant represents 772 kb converted to bytes for use in size calculations.
pub const KB_772: usize = 790528;

/// 773KB in bytes.
/// This constant represents 773 kb converted to bytes for use in size calculations.
pub const KB_773: usize = 791552;

/// 774KB in bytes.
/// This constant represents 774 kb converted to bytes for use in size calculations.
pub const KB_774: usize = 792576;

/// 775KB in bytes.
/// This constant represents 775 kb converted to bytes for use in size calculations.
pub const KB_775: usize = 793600;

/// 776KB in bytes.
/// This constant represents 776 kb converted to bytes for use in size calculations.
pub const KB_776: usize = 794624;

/// 777KB in bytes.
/// This constant represents 777 kb converted to bytes for use in size calculations.
pub const KB_777: usize = 795648;

/// 778KB in bytes.
/// This constant represents 778 kb converted to bytes for use in size calculations.
pub const KB_778: usize = 796672;

/// 779KB in bytes.
/// This constant represents 779 kb converted to bytes for use in size calculations.
pub const KB_779: usize = 797696;

/// 780KB in bytes.
/// This constant represents 780 kb converted to bytes for use in size calculations.
pub const KB_780: usize = 798720;

/// 781KB in bytes.
/// This constant represents 781 kb converted to bytes for use in size calculations.
pub const KB_781: usize = 799744;

/// 782KB in bytes.
/// This constant represents 782 kb converted to bytes for use in size calculations.
pub const KB_782: usize = 800768;

/// 783KB in bytes.
/// This constant represents 783 kb converted to bytes for use in size calculations.
pub const KB_783: usize = 801792;

/// 784KB in bytes.
/// This constant represents 784 kb converted to bytes for use in size calculations.
pub const KB_784: usize = 802816;

/// 785KB in bytes.
/// This constant represents 785 kb converted to bytes for use in size calculations.
pub const KB_785: usize = 803840;

/// 786KB in bytes.
/// This constant represents 786 kb converted to bytes for use in size calculations.
pub const KB_786: usize = 804864;

/// 787KB in bytes.
/// This constant represents 787 kb converted to bytes for use in size calculations.
pub const KB_787: usize = 805888;

/// 788KB in bytes.
/// This constant represents 788 kb converted to bytes for use in size calculations.
pub const KB_788: usize = 806912;

/// 789KB in bytes.
/// This constant represents 789 kb converted to bytes for use in size calculations.
pub const KB_789: usize = 807936;

/// 790KB in bytes.
/// This constant represents 790 kb converted to bytes for use in size calculations.
pub const KB_790: usize = 808960;

/// 791KB in bytes.
/// This constant represents 791 kb converted to bytes for use in size calculations.
pub const KB_791: usize = 809984;

/// 792KB in bytes.
/// This constant represents 792 kb converted to bytes for use in size calculations.
pub const KB_792: usize = 811008;

/// 793KB in bytes.
/// This constant represents 793 kb converted to bytes for use in size calculations.
pub const KB_793: usize = 812032;

/// 794KB in bytes.
/// This constant represents 794 kb converted to bytes for use in size calculations.
pub const KB_794: usize = 813056;

/// 795KB in bytes.
/// This constant represents 795 kb converted to bytes for use in size calculations.
pub const KB_795: usize = 814080;

/// 796KB in bytes.
/// This constant represents 796 kb converted to bytes for use in size calculations.
pub const KB_796: usize = 815104;

/// 797KB in bytes.
/// This constant represents 797 kb converted to bytes for use in size calculations.
pub const KB_797: usize = 816128;

/// 798KB in bytes.
/// This constant represents 798 kb converted to bytes for use in size calculations.
pub const KB_798: usize = 817152;

/// 799KB in bytes.
/// This constant represents 799 kb converted to bytes for use in size calculations.
pub const KB_799: usize = 818176;

/// 800KB in bytes.
/// This constant represents 800 kb converted to bytes for use in size calculations.
pub const KB_800: usize = 819200;

/// 801KB in bytes.
/// This constant represents 801 kb converted to bytes for use in size calculations.
pub const KB_801: usize = 820224;

/// 802KB in bytes.
/// This constant represents 802 kb converted to bytes for use in size calculations.
pub const KB_802: usize = 821248;

/// 803KB in bytes.
/// This constant represents 803 kb converted to bytes for use in size calculations.
pub const KB_803: usize = 822272;

/// 804KB in bytes.
/// This constant represents 804 kb converted to bytes for use in size calculations.
pub const KB_804: usize = 823296;

/// 805KB in bytes.
/// This constant represents 805 kb converted to bytes for use in size calculations.
pub const KB_805: usize = 824320;

/// 806KB in bytes.
/// This constant represents 806 kb converted to bytes for use in size calculations.
pub const KB_806: usize = 825344;

/// 807KB in bytes.
/// This constant represents 807 kb converted to bytes for use in size calculations.
pub const KB_807: usize = 826368;

/// 808KB in bytes.
/// This constant represents 808 kb converted to bytes for use in size calculations.
pub const KB_808: usize = 827392;

/// 809KB in bytes.
/// This constant represents 809 kb converted to bytes for use in size calculations.
pub const KB_809: usize = 828416;

/// 810KB in bytes.
/// This constant represents 810 kb converted to bytes for use in size calculations.
pub const KB_810: usize = 829440;

/// 811KB in bytes.
/// This constant represents 811 kb converted to bytes for use in size calculations.
pub const KB_811: usize = 830464;

/// 812KB in bytes.
/// This constant represents 812 kb converted to bytes for use in size calculations.
pub const KB_812: usize = 831488;

/// 813KB in bytes.
/// This constant represents 813 kb converted to bytes for use in size calculations.
pub const KB_813: usize = 832512;

/// 814KB in bytes.
/// This constant represents 814 kb converted to bytes for use in size calculations.
pub const KB_814: usize = 833536;

/// 815KB in bytes.
/// This constant represents 815 kb converted to bytes for use in size calculations.
pub const KB_815: usize = 834560;

/// 816KB in bytes.
/// This constant represents 816 kb converted to bytes for use in size calculations.
pub const KB_816: usize = 835584;

/// 817KB in bytes.
/// This constant represents 817 kb converted to bytes for use in size calculations.
pub const KB_817: usize = 836608;

/// 818KB in bytes.
/// This constant represents 818 kb converted to bytes for use in size calculations.
pub const KB_818: usize = 837632;

/// 819KB in bytes.
/// This constant represents 819 kb converted to bytes for use in size calculations.
pub const KB_819: usize = 838656;

/// 820KB in bytes.
/// This constant represents 820 kb converted to bytes for use in size calculations.
pub const KB_820: usize = 839680;

/// 821KB in bytes.
/// This constant represents 821 kb converted to bytes for use in size calculations.
pub const KB_821: usize = 840704;

/// 822KB in bytes.
/// This constant represents 822 kb converted to bytes for use in size calculations.
pub const KB_822: usize = 841728;

/// 823KB in bytes.
/// This constant represents 823 kb converted to bytes for use in size calculations.
pub const KB_823: usize = 842752;

/// 824KB in bytes.
/// This constant represents 824 kb converted to bytes for use in size calculations.
pub const KB_824: usize = 843776;

/// 825KB in bytes.
/// This constant represents 825 kb converted to bytes for use in size calculations.
pub const KB_825: usize = 844800;

/// 826KB in bytes.
/// This constant represents 826 kb converted to bytes for use in size calculations.
pub const KB_826: usize = 845824;

/// 827KB in bytes.
/// This constant represents 827 kb converted to bytes for use in size calculations.
pub const KB_827: usize = 846848;

/// 828KB in bytes.
/// This constant represents 828 kb converted to bytes for use in size calculations.
pub const KB_828: usize = 847872;

/// 829KB in bytes.
/// This constant represents 829 kb converted to bytes for use in size calculations.
pub const KB_829: usize = 848896;

/// 830KB in bytes.
/// This constant represents 830 kb converted to bytes for use in size calculations.
pub const KB_830: usize = 849920;

/// 831KB in bytes.
/// This constant represents 831 kb converted to bytes for use in size calculations.
pub const KB_831: usize = 850944;

/// 832KB in bytes.
/// This constant represents 832 kb converted to bytes for use in size calculations.
pub const KB_832: usize = 851968;

/// 833KB in bytes.
/// This constant represents 833 kb converted to bytes for use in size calculations.
pub const KB_833: usize = 852992;

/// 834KB in bytes.
/// This constant represents 834 kb converted to bytes for use in size calculations.
pub const KB_834: usize = 854016;

/// 835KB in bytes.
/// This constant represents 835 kb converted to bytes for use in size calculations.
pub const KB_835: usize = 855040;

/// 836KB in bytes.
/// This constant represents 836 kb converted to bytes for use in size calculations.
pub const KB_836: usize = 856064;

/// 837KB in bytes.
/// This constant represents 837 kb converted to bytes for use in size calculations.
pub const KB_837: usize = 857088;

/// 838KB in bytes.
/// This constant represents 838 kb converted to bytes for use in size calculations.
pub const KB_838: usize = 858112;

/// 839KB in bytes.
/// This constant represents 839 kb converted to bytes for use in size calculations.
pub const KB_839: usize = 859136;

/// 840KB in bytes.
/// This constant represents 840 kb converted to bytes for use in size calculations.
pub const KB_840: usize = 860160;

/// 841KB in bytes.
/// This constant represents 841 kb converted to bytes for use in size calculations.
pub const KB_841: usize = 861184;

/// 842KB in bytes.
/// This constant represents 842 kb converted to bytes for use in size calculations.
pub const KB_842: usize = 862208;

/// 843KB in bytes.
/// This constant represents 843 kb converted to bytes for use in size calculations.
pub const KB_843: usize = 863232;

/// 844KB in bytes.
/// This constant represents 844 kb converted to bytes for use in size calculations.
pub const KB_844: usize = 864256;

/// 845KB in bytes.
/// This constant represents 845 kb converted to bytes for use in size calculations.
pub const KB_845: usize = 865280;

/// 846KB in bytes.
/// This constant represents 846 kb converted to bytes for use in size calculations.
pub const KB_846: usize = 866304;

/// 847KB in bytes.
/// This constant represents 847 kb converted to bytes for use in size calculations.
pub const KB_847: usize = 867328;

/// 848KB in bytes.
/// This constant represents 848 kb converted to bytes for use in size calculations.
pub const KB_848: usize = 868352;

/// 849KB in bytes.
/// This constant represents 849 kb converted to bytes for use in size calculations.
pub const KB_849: usize = 869376;

/// 850KB in bytes.
/// This constant represents 850 kb converted to bytes for use in size calculations.
pub const KB_850: usize = 870400;

/// 851KB in bytes.
/// This constant represents 851 kb converted to bytes for use in size calculations.
pub const KB_851: usize = 871424;

/// 852KB in bytes.
/// This constant represents 852 kb converted to bytes for use in size calculations.
pub const KB_852: usize = 872448;

/// 853KB in bytes.
/// This constant represents 853 kb converted to bytes for use in size calculations.
pub const KB_853: usize = 873472;

/// 854KB in bytes.
/// This constant represents 854 kb converted to bytes for use in size calculations.
pub const KB_854: usize = 874496;

/// 855KB in bytes.
/// This constant represents 855 kb converted to bytes for use in size calculations.
pub const KB_855: usize = 875520;

/// 856KB in bytes.
/// This constant represents 856 kb converted to bytes for use in size calculations.
pub const KB_856: usize = 876544;

/// 857KB in bytes.
/// This constant represents 857 kb converted to bytes for use in size calculations.
pub const KB_857: usize = 877568;

/// 858KB in bytes.
/// This constant represents 858 kb converted to bytes for use in size calculations.
pub const KB_858: usize = 878592;

/// 859KB in bytes.
/// This constant represents 859 kb converted to bytes for use in size calculations.
pub const KB_859: usize = 879616;

/// 860KB in bytes.
/// This constant represents 860 kb converted to bytes for use in size calculations.
pub const KB_860: usize = 880640;

/// 861KB in bytes.
/// This constant represents 861 kb converted to bytes for use in size calculations.
pub const KB_861: usize = 881664;

/// 862KB in bytes.
/// This constant represents 862 kb converted to bytes for use in size calculations.
pub const KB_862: usize = 882688;

/// 863KB in bytes.
/// This constant represents 863 kb converted to bytes for use in size calculations.
pub const KB_863: usize = 883712;

/// 864KB in bytes.
/// This constant represents 864 kb converted to bytes for use in size calculations.
pub const KB_864: usize = 884736;

/// 865KB in bytes.
/// This constant represents 865 kb converted to bytes for use in size calculations.
pub const KB_865: usize = 885760;

/// 866KB in bytes.
/// This constant represents 866 kb converted to bytes for use in size calculations.
pub const KB_866: usize = 886784;

/// 867KB in bytes.
/// This constant represents 867 kb converted to bytes for use in size calculations.
pub const KB_867: usize = 887808;

/// 868KB in bytes.
/// This constant represents 868 kb converted to bytes for use in size calculations.
pub const KB_868: usize = 888832;

/// 869KB in bytes.
/// This constant represents 869 kb converted to bytes for use in size calculations.
pub const KB_869: usize = 889856;

/// 870KB in bytes.
/// This constant represents 870 kb converted to bytes for use in size calculations.
pub const KB_870: usize = 890880;

/// 871KB in bytes.
/// This constant represents 871 kb converted to bytes for use in size calculations.
pub const KB_871: usize = 891904;

/// 872KB in bytes.
/// This constant represents 872 kb converted to bytes for use in size calculations.
pub const KB_872: usize = 892928;

/// 873KB in bytes.
/// This constant represents 873 kb converted to bytes for use in size calculations.
pub const KB_873: usize = 893952;

/// 874KB in bytes.
/// This constant represents 874 kb converted to bytes for use in size calculations.
pub const KB_874: usize = 894976;

/// 875KB in bytes.
/// This constant represents 875 kb converted to bytes for use in size calculations.
pub const KB_875: usize = 896000;

/// 876KB in bytes.
/// This constant represents 876 kb converted to bytes for use in size calculations.
pub const KB_876: usize = 897024;

/// 877KB in bytes.
/// This constant represents 877 kb converted to bytes for use in size calculations.
pub const KB_877: usize = 898048;

/// 878KB in bytes.
/// This constant represents 878 kb converted to bytes for use in size calculations.
pub const KB_878: usize = 899072;

/// 879KB in bytes.
/// This constant represents 879 kb converted to bytes for use in size calculations.
pub const KB_879: usize = 900096;

/// 880KB in bytes.
/// This constant represents 880 kb converted to bytes for use in size calculations.
pub const KB_880: usize = 901120;

/// 881KB in bytes.
/// This constant represents 881 kb converted to bytes for use in size calculations.
pub const KB_881: usize = 902144;

/// 882KB in bytes.
/// This constant represents 882 kb converted to bytes for use in size calculations.
pub const KB_882: usize = 903168;

/// 883KB in bytes.
/// This constant represents 883 kb converted to bytes for use in size calculations.
pub const KB_883: usize = 904192;

/// 884KB in bytes.
/// This constant represents 884 kb converted to bytes for use in size calculations.
pub const KB_884: usize = 905216;

/// 885KB in bytes.
/// This constant represents 885 kb converted to bytes for use in size calculations.
pub const KB_885: usize = 906240;

/// 886KB in bytes.
/// This constant represents 886 kb converted to bytes for use in size calculations.
pub const KB_886: usize = 907264;

/// 887KB in bytes.
/// This constant represents 887 kb converted to bytes for use in size calculations.
pub const KB_887: usize = 908288;

/// 888KB in bytes.
/// This constant represents 888 kb converted to bytes for use in size calculations.
pub const KB_888: usize = 909312;

/// 889KB in bytes.
/// This constant represents 889 kb converted to bytes for use in size calculations.
pub const KB_889: usize = 910336;

/// 890KB in bytes.
/// This constant represents 890 kb converted to bytes for use in size calculations.
pub const KB_890: usize = 911360;

/// 891KB in bytes.
/// This constant represents 891 kb converted to bytes for use in size calculations.
pub const KB_891: usize = 912384;

/// 892KB in bytes.
/// This constant represents 892 kb converted to bytes for use in size calculations.
pub const KB_892: usize = 913408;

/// 893KB in bytes.
/// This constant represents 893 kb converted to bytes for use in size calculations.
pub const KB_893: usize = 914432;

/// 894KB in bytes.
/// This constant represents 894 kb converted to bytes for use in size calculations.
pub const KB_894: usize = 915456;

/// 895KB in bytes.
/// This constant represents 895 kb converted to bytes for use in size calculations.
pub const KB_895: usize = 916480;

/// 896KB in bytes.
/// This constant represents 896 kb converted to bytes for use in size calculations.
pub const KB_896: usize = 917504;

/// 897KB in bytes.
/// This constant represents 897 kb converted to bytes for use in size calculations.
pub const KB_897: usize = 918528;

/// 898KB in bytes.
/// This constant represents 898 kb converted to bytes for use in size calculations.
pub const KB_898: usize = 919552;

/// 899KB in bytes.
/// This constant represents 899 kb converted to bytes for use in size calculations.
pub const KB_899: usize = 920576;

/// 900KB in bytes.
/// This constant represents 900 kb converted to bytes for use in size calculations.
pub const KB_900: usize = 921600;

/// 901KB in bytes.
/// This constant represents 901 kb converted to bytes for use in size calculations.
pub const KB_901: usize = 922624;

/// 902KB in bytes.
/// This constant represents 902 kb converted to bytes for use in size calculations.
pub const KB_902: usize = 923648;

/// 903KB in bytes.
/// This constant represents 903 kb converted to bytes for use in size calculations.
pub const KB_903: usize = 924672;

/// 904KB in bytes.
/// This constant represents 904 kb converted to bytes for use in size calculations.
pub const KB_904: usize = 925696;

/// 905KB in bytes.
/// This constant represents 905 kb converted to bytes for use in size calculations.
pub const KB_905: usize = 926720;

/// 906KB in bytes.
/// This constant represents 906 kb converted to bytes for use in size calculations.
pub const KB_906: usize = 927744;

/// 907KB in bytes.
/// This constant represents 907 kb converted to bytes for use in size calculations.
pub const KB_907: usize = 928768;

/// 908KB in bytes.
/// This constant represents 908 kb converted to bytes for use in size calculations.
pub const KB_908: usize = 929792;

/// 909KB in bytes.
/// This constant represents 909 kb converted to bytes for use in size calculations.
pub const KB_909: usize = 930816;

/// 910KB in bytes.
/// This constant represents 910 kb converted to bytes for use in size calculations.
pub const KB_910: usize = 931840;

/// 911KB in bytes.
/// This constant represents 911 kb converted to bytes for use in size calculations.
pub const KB_911: usize = 932864;

/// 912KB in bytes.
/// This constant represents 912 kb converted to bytes for use in size calculations.
pub const KB_912: usize = 933888;

/// 913KB in bytes.
/// This constant represents 913 kb converted to bytes for use in size calculations.
pub const KB_913: usize = 934912;

/// 914KB in bytes.
/// This constant represents 914 kb converted to bytes for use in size calculations.
pub const KB_914: usize = 935936;

/// 915KB in bytes.
/// This constant represents 915 kb converted to bytes for use in size calculations.
pub const KB_915: usize = 936960;

/// 916KB in bytes.
/// This constant represents 916 kb converted to bytes for use in size calculations.
pub const KB_916: usize = 937984;

/// 917KB in bytes.
/// This constant represents 917 kb converted to bytes for use in size calculations.
pub const KB_917: usize = 939008;

/// 918KB in bytes.
/// This constant represents 918 kb converted to bytes for use in size calculations.
pub const KB_918: usize = 940032;

/// 919KB in bytes.
/// This constant represents 919 kb converted to bytes for use in size calculations.
pub const KB_919: usize = 941056;

/// 920KB in bytes.
/// This constant represents 920 kb converted to bytes for use in size calculations.
pub const KB_920: usize = 942080;

/// 921KB in bytes.
/// This constant represents 921 kb converted to bytes for use in size calculations.
pub const KB_921: usize = 943104;

/// 922KB in bytes.
/// This constant represents 922 kb converted to bytes for use in size calculations.
pub const KB_922: usize = 944128;

/// 923KB in bytes.
/// This constant represents 923 kb converted to bytes for use in size calculations.
pub const KB_923: usize = 945152;

/// 924KB in bytes.
/// This constant represents 924 kb converted to bytes for use in size calculations.
pub const KB_924: usize = 946176;

/// 925KB in bytes.
/// This constant represents 925 kb converted to bytes for use in size calculations.
pub const KB_925: usize = 947200;

/// 926KB in bytes.
/// This constant represents 926 kb converted to bytes for use in size calculations.
pub const KB_926: usize = 948224;

/// 927KB in bytes.
/// This constant represents 927 kb converted to bytes for use in size calculations.
pub const KB_927: usize = 949248;

/// 928KB in bytes.
/// This constant represents 928 kb converted to bytes for use in size calculations.
pub const KB_928: usize = 950272;

/// 929KB in bytes.
/// This constant represents 929 kb converted to bytes for use in size calculations.
pub const KB_929: usize = 951296;

/// 930KB in bytes.
/// This constant represents 930 kb converted to bytes for use in size calculations.
pub const KB_930: usize = 952320;

/// 931KB in bytes.
/// This constant represents 931 kb converted to bytes for use in size calculations.
pub const KB_931: usize = 953344;

/// 932KB in bytes.
/// This constant represents 932 kb converted to bytes for use in size calculations.
pub const KB_932: usize = 954368;

/// 933KB in bytes.
/// This constant represents 933 kb converted to bytes for use in size calculations.
pub const KB_933: usize = 955392;

/// 934KB in bytes.
/// This constant represents 934 kb converted to bytes for use in size calculations.
pub const KB_934: usize = 956416;

/// 935KB in bytes.
/// This constant represents 935 kb converted to bytes for use in size calculations.
pub const KB_935: usize = 957440;

/// 936KB in bytes.
/// This constant represents 936 kb converted to bytes for use in size calculations.
pub const KB_936: usize = 958464;

/// 937KB in bytes.
/// This constant represents 937 kb converted to bytes for use in size calculations.
pub const KB_937: usize = 959488;

/// 938KB in bytes.
/// This constant represents 938 kb converted to bytes for use in size calculations.
pub const KB_938: usize = 960512;

/// 939KB in bytes.
/// This constant represents 939 kb converted to bytes for use in size calculations.
pub const KB_939: usize = 961536;

/// 940KB in bytes.
/// This constant represents 940 kb converted to bytes for use in size calculations.
pub const KB_940: usize = 962560;

/// 941KB in bytes.
/// This constant represents 941 kb converted to bytes for use in size calculations.
pub const KB_941: usize = 963584;

/// 942KB in bytes.
/// This constant represents 942 kb converted to bytes for use in size calculations.
pub const KB_942: usize = 964608;

/// 943KB in bytes.
/// This constant represents 943 kb converted to bytes for use in size calculations.
pub const KB_943: usize = 965632;

/// 944KB in bytes.
/// This constant represents 944 kb converted to bytes for use in size calculations.
pub const KB_944: usize = 966656;

/// 945KB in bytes.
/// This constant represents 945 kb converted to bytes for use in size calculations.
pub const KB_945: usize = 967680;

/// 946KB in bytes.
/// This constant represents 946 kb converted to bytes for use in size calculations.
pub const KB_946: usize = 968704;

/// 947KB in bytes.
/// This constant represents 947 kb converted to bytes for use in size calculations.
pub const KB_947: usize = 969728;

/// 948KB in bytes.
/// This constant represents 948 kb converted to bytes for use in size calculations.
pub const KB_948: usize = 970752;

/// 949KB in bytes.
/// This constant represents 949 kb converted to bytes for use in size calculations.
pub const KB_949: usize = 971776;

/// 950KB in bytes.
/// This constant represents 950 kb converted to bytes for use in size calculations.
pub const KB_950: usize = 972800;

/// 951KB in bytes.
/// This constant represents 951 kb converted to bytes for use in size calculations.
pub const KB_951: usize = 973824;

/// 952KB in bytes.
/// This constant represents 952 kb converted to bytes for use in size calculations.
pub const KB_952: usize = 974848;

/// 953KB in bytes.
/// This constant represents 953 kb converted to bytes for use in size calculations.
pub const KB_953: usize = 975872;

/// 954KB in bytes.
/// This constant represents 954 kb converted to bytes for use in size calculations.
pub const KB_954: usize = 976896;

/// 955KB in bytes.
/// This constant represents 955 kb converted to bytes for use in size calculations.
pub const KB_955: usize = 977920;

/// 956KB in bytes.
/// This constant represents 956 kb converted to bytes for use in size calculations.
pub const KB_956: usize = 978944;

/// 957KB in bytes.
/// This constant represents 957 kb converted to bytes for use in size calculations.
pub const KB_957: usize = 979968;

/// 958KB in bytes.
/// This constant represents 958 kb converted to bytes for use in size calculations.
pub const KB_958: usize = 980992;

/// 959KB in bytes.
/// This constant represents 959 kb converted to bytes for use in size calculations.
pub const KB_959: usize = 982016;

/// 960KB in bytes.
/// This constant represents 960 kb converted to bytes for use in size calculations.
pub const KB_960: usize = 983040;

/// 961KB in bytes.
/// This constant represents 961 kb converted to bytes for use in size calculations.
pub const KB_961: usize = 984064;

/// 962KB in bytes.
/// This constant represents 962 kb converted to bytes for use in size calculations.
pub const KB_962: usize = 985088;

/// 963KB in bytes.
/// This constant represents 963 kb converted to bytes for use in size calculations.
pub const KB_963: usize = 986112;

/// 964KB in bytes.
/// This constant represents 964 kb converted to bytes for use in size calculations.
pub const KB_964: usize = 987136;

/// 965KB in bytes.
/// This constant represents 965 kb converted to bytes for use in size calculations.
pub const KB_965: usize = 988160;

/// 966KB in bytes.
/// This constant represents 966 kb converted to bytes for use in size calculations.
pub const KB_966: usize = 989184;

/// 967KB in bytes.
/// This constant represents 967 kb converted to bytes for use in size calculations.
pub const KB_967: usize = 990208;

/// 968KB in bytes.
/// This constant represents 968 kb converted to bytes for use in size calculations.
pub const KB_968: usize = 991232;

/// 969KB in bytes.
/// This constant represents 969 kb converted to bytes for use in size calculations.
pub const KB_969: usize = 992256;

/// 970KB in bytes.
/// This constant represents 970 kb converted to bytes for use in size calculations.
pub const KB_970: usize = 993280;

/// 971KB in bytes.
/// This constant represents 971 kb converted to bytes for use in size calculations.
pub const KB_971: usize = 994304;

/// 972KB in bytes.
/// This constant represents 972 kb converted to bytes for use in size calculations.
pub const KB_972: usize = 995328;

/// 973KB in bytes.
/// This constant represents 973 kb converted to bytes for use in size calculations.
pub const KB_973: usize = 996352;

/// 974KB in bytes.
/// This constant represents 974 kb converted to bytes for use in size calculations.
pub const KB_974: usize = 997376;

/// 975KB in bytes.
/// This constant represents 975 kb converted to bytes for use in size calculations.
pub const KB_975: usize = 998400;

/// 976KB in bytes.
/// This constant represents 976 kb converted to bytes for use in size calculations.
pub const KB_976: usize = 999424;

/// 977KB in bytes.
/// This constant represents 977 kb converted to bytes for use in size calculations.
pub const KB_977: usize = 1000448;

/// 978KB in bytes.
/// This constant represents 978 kb converted to bytes for use in size calculations.
pub const KB_978: usize = 1001472;

/// 979KB in bytes.
/// This constant represents 979 kb converted to bytes for use in size calculations.
pub const KB_979: usize = 1002496;

/// 980KB in bytes.
/// This constant represents 980 kb converted to bytes for use in size calculations.
pub const KB_980: usize = 1003520;

/// 981KB in bytes.
/// This constant represents 981 kb converted to bytes for use in size calculations.
pub const KB_981: usize = 1004544;

/// 982KB in bytes.
/// This constant represents 982 kb converted to bytes for use in size calculations.
pub const KB_982: usize = 1005568;

/// 983KB in bytes.
/// This constant represents 983 kb converted to bytes for use in size calculations.
pub const KB_983: usize = 1006592;

/// 984KB in bytes.
/// This constant represents 984 kb converted to bytes for use in size calculations.
pub const KB_984: usize = 1007616;

/// 985KB in bytes.
/// This constant represents 985 kb converted to bytes for use in size calculations.
pub const KB_985: usize = 1008640;

/// 986KB in bytes.
/// This constant represents 986 kb converted to bytes for use in size calculations.
pub const KB_986: usize = 1009664;

/// 987KB in bytes.
/// This constant represents 987 kb converted to bytes for use in size calculations.
pub const KB_987: usize = 1010688;

/// 988KB in bytes.
/// This constant represents 988 kb converted to bytes for use in size calculations.
pub const KB_988: usize = 1011712;

/// 989KB in bytes.
/// This constant represents 989 kb converted to bytes for use in size calculations.
pub const KB_989: usize = 1012736;

/// 990KB in bytes.
/// This constant represents 990 kb converted to bytes for use in size calculations.
pub const KB_990: usize = 1013760;

/// 991KB in bytes.
/// This constant represents 991 kb converted to bytes for use in size calculations.
pub const KB_991: usize = 1014784;

/// 992KB in bytes.
/// This constant represents 992 kb converted to bytes for use in size calculations.
pub const KB_992: usize = 1015808;

/// 993KB in bytes.
/// This constant represents 993 kb converted to bytes for use in size calculations.
pub const KB_993: usize = 1016832;

/// 994KB in bytes.
/// This constant represents 994 kb converted to bytes for use in size calculations.
pub const KB_994: usize = 1017856;

/// 995KB in bytes.
/// This constant represents 995 kb converted to bytes for use in size calculations.
pub const KB_995: usize = 1018880;

/// 996KB in bytes.
/// This constant represents 996 kb converted to bytes for use in size calculations.
pub const KB_996: usize = 1019904;

/// 997KB in bytes.
/// This constant represents 997 kb converted to bytes for use in size calculations.
pub const KB_997: usize = 1020928;

/// 998KB in bytes.
/// This constant represents 998 kb converted to bytes for use in size calculations.
pub const KB_998: usize = 1021952;

/// 999KB in bytes.
/// This constant represents 999 kb converted to bytes for use in size calculations.
pub const KB_999: usize = 1022976;

/// 1000KB in bytes.
/// This constant represents 1000 kb converted to bytes for use in size calculations.
pub const KB_1000: usize = 1024000;

/// 1001KB in bytes.
/// This constant represents 1001 kb converted to bytes for use in size calculations.
pub const KB_1001: usize = 1025024;

/// 1002KB in bytes.
/// This constant represents 1002 kb converted to bytes for use in size calculations.
pub const KB_1002: usize = 1026048;

/// 1003KB in bytes.
/// This constant represents 1003 kb converted to bytes for use in size calculations.
pub const KB_1003: usize = 1027072;

/// 1004KB in bytes.
/// This constant represents 1004 kb converted to bytes for use in size calculations.
pub const KB_1004: usize = 1028096;

/// 1005KB in bytes.
/// This constant represents 1005 kb converted to bytes for use in size calculations.
pub const KB_1005: usize = 1029120;

/// 1006KB in bytes.
/// This constant represents 1006 kb converted to bytes for use in size calculations.
pub const KB_1006: usize = 1030144;

/// 1007KB in bytes.
/// This constant represents 1007 kb converted to bytes for use in size calculations.
pub const KB_1007: usize = 1031168;

/// 1008KB in bytes.
/// This constant represents 1008 kb converted to bytes for use in size calculations.
pub const KB_1008: usize = 1032192;

/// 1009KB in bytes.
/// This constant represents 1009 kb converted to bytes for use in size calculations.
pub const KB_1009: usize = 1033216;

/// 1010KB in bytes.
/// This constant represents 1010 kb converted to bytes for use in size calculations.
pub const KB_1010: usize = 1034240;

/// 1011KB in bytes.
/// This constant represents 1011 kb converted to bytes for use in size calculations.
pub const KB_1011: usize = 1035264;

/// 1012KB in bytes.
/// This constant represents 1012 kb converted to bytes for use in size calculations.
pub const KB_1012: usize = 1036288;

/// 1013KB in bytes.
/// This constant represents 1013 kb converted to bytes for use in size calculations.
pub const KB_1013: usize = 1037312;

/// 1014KB in bytes.
/// This constant represents 1014 kb converted to bytes for use in size calculations.
pub const KB_1014: usize = 1038336;

/// 1015KB in bytes.
/// This constant represents 1015 kb converted to bytes for use in size calculations.
pub const KB_1015: usize = 1039360;

/// 1016KB in bytes.
/// This constant represents 1016 kb converted to bytes for use in size calculations.
pub const KB_1016: usize = 1040384;

/// 1017KB in bytes.
/// This constant represents 1017 kb converted to bytes for use in size calculations.
pub const KB_1017: usize = 1041408;

/// 1018KB in bytes.
/// This constant represents 1018 kb converted to bytes for use in size calculations.
pub const KB_1018: usize = 1042432;

/// 1019KB in bytes.
/// This constant represents 1019 kb converted to bytes for use in size calculations.
pub const KB_1019: usize = 1043456;

/// 1020KB in bytes.
/// This constant represents 1020 kb converted to bytes for use in size calculations.
pub const KB_1020: usize = 1044480;

/// 1021KB in bytes.
/// This constant represents 1021 kb converted to bytes for use in size calculations.
pub const KB_1021: usize = 1045504;

/// 1022KB in bytes.
/// This constant represents 1022 kb converted to bytes for use in size calculations.
pub const KB_1022: usize = 1046528;

/// 1023KB in bytes.
/// This constant represents 1023 kb converted to bytes for use in size calculations.
pub const KB_1023: usize = 1047552;

/// 1024KB in bytes.
/// This constant represents 1024 kb converted to bytes for use in size calculations.
pub const KB_1024: usize = 1048576;

// Storage unit constants from 1MB to 1024MB
/// 1MB in bytes.
/// This constant represents 1 mb converted to bytes for use in size calculations.
pub const MB_1: usize = 1048576;

/// 2MB in bytes.
/// This constant represents 2 mb converted to bytes for use in size calculations.
pub const MB_2: usize = 2097152;

/// 3MB in bytes.
/// This constant represents 3 mb converted to bytes for use in size calculations.
pub const MB_3: usize = 3145728;

/// 4MB in bytes.
/// This constant represents 4 mb converted to bytes for use in size calculations.
pub const MB_4: usize = 4194304;

/// 5MB in bytes.
/// This constant represents 5 mb converted to bytes for use in size calculations.
pub const MB_5: usize = 5242880;

/// 6MB in bytes.
/// This constant represents 6 mb converted to bytes for use in size calculations.
pub const MB_6: usize = 6291456;

/// 7MB in bytes.
/// This constant represents 7 mb converted to bytes for use in size calculations.
pub const MB_7: usize = 7340032;

/// 8MB in bytes.
/// This constant represents 8 mb converted to bytes for use in size calculations.
pub const MB_8: usize = 8388608;

/// 9MB in bytes.
/// This constant represents 9 mb converted to bytes for use in size calculations.
pub const MB_9: usize = 9437184;

/// 10MB in bytes.
/// This constant represents 10 mb converted to bytes for use in size calculations.
pub const MB_10: usize = 10485760;

/// 11MB in bytes.
/// This constant represents 11 mb converted to bytes for use in size calculations.
pub const MB_11: usize = 11534336;

/// 12MB in bytes.
/// This constant represents 12 mb converted to bytes for use in size calculations.
pub const MB_12: usize = 12582912;

/// 13MB in bytes.
/// This constant represents 13 mb converted to bytes for use in size calculations.
pub const MB_13: usize = 13631488;

/// 14MB in bytes.
/// This constant represents 14 mb converted to bytes for use in size calculations.
pub const MB_14: usize = 14680064;

/// 15MB in bytes.
/// This constant represents 15 mb converted to bytes for use in size calculations.
pub const MB_15: usize = 15728640;

/// 16MB in bytes.
/// This constant represents 16 mb converted to bytes for use in size calculations.
pub const MB_16: usize = 16777216;

/// 17MB in bytes.
/// This constant represents 17 mb converted to bytes for use in size calculations.
pub const MB_17: usize = 17825792;

/// 18MB in bytes.
/// This constant represents 18 mb converted to bytes for use in size calculations.
pub const MB_18: usize = 18874368;

/// 19MB in bytes.
/// This constant represents 19 mb converted to bytes for use in size calculations.
pub const MB_19: usize = 19922944;

/// 20MB in bytes.
/// This constant represents 20 mb converted to bytes for use in size calculations.
pub const MB_20: usize = 20971520;

/// 21MB in bytes.
/// This constant represents 21 mb converted to bytes for use in size calculations.
pub const MB_21: usize = 22020096;

/// 22MB in bytes.
/// This constant represents 22 mb converted to bytes for use in size calculations.
pub const MB_22: usize = 23068672;

/// 23MB in bytes.
/// This constant represents 23 mb converted to bytes for use in size calculations.
pub const MB_23: usize = 24117248;

/// 24MB in bytes.
/// This constant represents 24 mb converted to bytes for use in size calculations.
pub const MB_24: usize = 25165824;

/// 25MB in bytes.
/// This constant represents 25 mb converted to bytes for use in size calculations.
pub const MB_25: usize = 26214400;

/// 26MB in bytes.
/// This constant represents 26 mb converted to bytes for use in size calculations.
pub const MB_26: usize = 27262976;

/// 27MB in bytes.
/// This constant represents 27 mb converted to bytes for use in size calculations.
pub const MB_27: usize = 28311552;

/// 28MB in bytes.
/// This constant represents 28 mb converted to bytes for use in size calculations.
pub const MB_28: usize = 29360128;

/// 29MB in bytes.
/// This constant represents 29 mb converted to bytes for use in size calculations.
pub const MB_29: usize = 30408704;

/// 30MB in bytes.
/// This constant represents 30 mb converted to bytes for use in size calculations.
pub const MB_30: usize = 31457280;

/// 31MB in bytes.
/// This constant represents 31 mb converted to bytes for use in size calculations.
pub const MB_31: usize = 32505856;

/// 32MB in bytes.
/// This constant represents 32 mb converted to bytes for use in size calculations.
pub const MB_32: usize = 33554432;

/// 33MB in bytes.
/// This constant represents 33 mb converted to bytes for use in size calculations.
pub const MB_33: usize = 34603008;

/// 34MB in bytes.
/// This constant represents 34 mb converted to bytes for use in size calculations.
pub const MB_34: usize = 35651584;

/// 35MB in bytes.
/// This constant represents 35 mb converted to bytes for use in size calculations.
pub const MB_35: usize = 36700160;

/// 36MB in bytes.
/// This constant represents 36 mb converted to bytes for use in size calculations.
pub const MB_36: usize = 37748736;

/// 37MB in bytes.
/// This constant represents 37 mb converted to bytes for use in size calculations.
pub const MB_37: usize = 38797312;

/// 38MB in bytes.
/// This constant represents 38 mb converted to bytes for use in size calculations.
pub const MB_38: usize = 39845888;

/// 39MB in bytes.
/// This constant represents 39 mb converted to bytes for use in size calculations.
pub const MB_39: usize = 40894464;

/// 40MB in bytes.
/// This constant represents 40 mb converted to bytes for use in size calculations.
pub const MB_40: usize = 41943040;

/// 41MB in bytes.
/// This constant represents 41 mb converted to bytes for use in size calculations.
pub const MB_41: usize = 42991616;

/// 42MB in bytes.
/// This constant represents 42 mb converted to bytes for use in size calculations.
pub const MB_42: usize = 44040192;

/// 43MB in bytes.
/// This constant represents 43 mb converted to bytes for use in size calculations.
pub const MB_43: usize = 45088768;

/// 44MB in bytes.
/// This constant represents 44 mb converted to bytes for use in size calculations.
pub const MB_44: usize = 46137344;

/// 45MB in bytes.
/// This constant represents 45 mb converted to bytes for use in size calculations.
pub const MB_45: usize = 47185920;

/// 46MB in bytes.
/// This constant represents 46 mb converted to bytes for use in size calculations.
pub const MB_46: usize = 48234496;

/// 47MB in bytes.
/// This constant represents 47 mb converted to bytes for use in size calculations.
pub const MB_47: usize = 49283072;

/// 48MB in bytes.
/// This constant represents 48 mb converted to bytes for use in size calculations.
pub const MB_48: usize = 50331648;

/// 49MB in bytes.
/// This constant represents 49 mb converted to bytes for use in size calculations.
pub const MB_49: usize = 51380224;

/// 50MB in bytes.
/// This constant represents 50 mb converted to bytes for use in size calculations.
pub const MB_50: usize = 52428800;

/// 51MB in bytes.
/// This constant represents 51 mb converted to bytes for use in size calculations.
pub const MB_51: usize = 53477376;

/// 52MB in bytes.
/// This constant represents 52 mb converted to bytes for use in size calculations.
pub const MB_52: usize = 54525952;

/// 53MB in bytes.
/// This constant represents 53 mb converted to bytes for use in size calculations.
pub const MB_53: usize = 55574528;

/// 54MB in bytes.
/// This constant represents 54 mb converted to bytes for use in size calculations.
pub const MB_54: usize = 56623104;

/// 55MB in bytes.
/// This constant represents 55 mb converted to bytes for use in size calculations.
pub const MB_55: usize = 57671680;

/// 56MB in bytes.
/// This constant represents 56 mb converted to bytes for use in size calculations.
pub const MB_56: usize = 58720256;

/// 57MB in bytes.
/// This constant represents 57 mb converted to bytes for use in size calculations.
pub const MB_57: usize = 59768832;

/// 58MB in bytes.
/// This constant represents 58 mb converted to bytes for use in size calculations.
pub const MB_58: usize = 60817408;

/// 59MB in bytes.
/// This constant represents 59 mb converted to bytes for use in size calculations.
pub const MB_59: usize = 61865984;

/// 60MB in bytes.
/// This constant represents 60 mb converted to bytes for use in size calculations.
pub const MB_60: usize = 62914560;

/// 61MB in bytes.
/// This constant represents 61 mb converted to bytes for use in size calculations.
pub const MB_61: usize = 63963136;

/// 62MB in bytes.
/// This constant represents 62 mb converted to bytes for use in size calculations.
pub const MB_62: usize = 65011712;

/// 63MB in bytes.
/// This constant represents 63 mb converted to bytes for use in size calculations.
pub const MB_63: usize = 66060288;

/// 64MB in bytes.
/// This constant represents 64 mb converted to bytes for use in size calculations.
pub const MB_64: usize = 67108864;

/// 65MB in bytes.
/// This constant represents 65 mb converted to bytes for use in size calculations.
pub const MB_65: usize = 68157440;

/// 66MB in bytes.
/// This constant represents 66 mb converted to bytes for use in size calculations.
pub const MB_66: usize = 69206016;

/// 67MB in bytes.
/// This constant represents 67 mb converted to bytes for use in size calculations.
pub const MB_67: usize = 70254592;

/// 68MB in bytes.
/// This constant represents 68 mb converted to bytes for use in size calculations.
pub const MB_68: usize = 71303168;

/// 69MB in bytes.
/// This constant represents 69 mb converted to bytes for use in size calculations.
pub const MB_69: usize = 72351744;

/// 70MB in bytes.
/// This constant represents 70 mb converted to bytes for use in size calculations.
pub const MB_70: usize = 73400320;

/// 71MB in bytes.
/// This constant represents 71 mb converted to bytes for use in size calculations.
pub const MB_71: usize = 74448896;

/// 72MB in bytes.
/// This constant represents 72 mb converted to bytes for use in size calculations.
pub const MB_72: usize = 75497472;

/// 73MB in bytes.
/// This constant represents 73 mb converted to bytes for use in size calculations.
pub const MB_73: usize = 76546048;

/// 74MB in bytes.
/// This constant represents 74 mb converted to bytes for use in size calculations.
pub const MB_74: usize = 77594624;

/// 75MB in bytes.
/// This constant represents 75 mb converted to bytes for use in size calculations.
pub const MB_75: usize = 78643200;

/// 76MB in bytes.
/// This constant represents 76 mb converted to bytes for use in size calculations.
pub const MB_76: usize = 79691776;

/// 77MB in bytes.
/// This constant represents 77 mb converted to bytes for use in size calculations.
pub const MB_77: usize = 80740352;

/// 78MB in bytes.
/// This constant represents 78 mb converted to bytes for use in size calculations.
pub const MB_78: usize = 81788928;

/// 79MB in bytes.
/// This constant represents 79 mb converted to bytes for use in size calculations.
pub const MB_79: usize = 82837504;

/// 80MB in bytes.
/// This constant represents 80 mb converted to bytes for use in size calculations.
pub const MB_80: usize = 83886080;

/// 81MB in bytes.
/// This constant represents 81 mb converted to bytes for use in size calculations.
pub const MB_81: usize = 84934656;

/// 82MB in bytes.
/// This constant represents 82 mb converted to bytes for use in size calculations.
pub const MB_82: usize = 85983232;

/// 83MB in bytes.
/// This constant represents 83 mb converted to bytes for use in size calculations.
pub const MB_83: usize = 87031808;

/// 84MB in bytes.
/// This constant represents 84 mb converted to bytes for use in size calculations.
pub const MB_84: usize = 88080384;

/// 85MB in bytes.
/// This constant represents 85 mb converted to bytes for use in size calculations.
pub const MB_85: usize = 89128960;

/// 86MB in bytes.
/// This constant represents 86 mb converted to bytes for use in size calculations.
pub const MB_86: usize = 90177536;

/// 87MB in bytes.
/// This constant represents 87 mb converted to bytes for use in size calculations.
pub const MB_87: usize = 91226112;

/// 88MB in bytes.
/// This constant represents 88 mb converted to bytes for use in size calculations.
pub const MB_88: usize = 92274688;

/// 89MB in bytes.
/// This constant represents 89 mb converted to bytes for use in size calculations.
pub const MB_89: usize = 93323264;

/// 90MB in bytes.
/// This constant represents 90 mb converted to bytes for use in size calculations.
pub const MB_90: usize = 94371840;

/// 91MB in bytes.
/// This constant represents 91 mb converted to bytes for use in size calculations.
pub const MB_91: usize = 95420416;

/// 92MB in bytes.
/// This constant represents 92 mb converted to bytes for use in size calculations.
pub const MB_92: usize = 96468992;

/// 93MB in bytes.
/// This constant represents 93 mb converted to bytes for use in size calculations.
pub const MB_93: usize = 97517568;

/// 94MB in bytes.
/// This constant represents 94 mb converted to bytes for use in size calculations.
pub const MB_94: usize = 98566144;

/// 95MB in bytes.
/// This constant represents 95 mb converted to bytes for use in size calculations.
pub const MB_95: usize = 99614720;

/// 96MB in bytes.
/// This constant represents 96 mb converted to bytes for use in size calculations.
pub const MB_96: usize = 100663296;

/// 97MB in bytes.
/// This constant represents 97 mb converted to bytes for use in size calculations.
pub const MB_97: usize = 101711872;

/// 98MB in bytes.
/// This constant represents 98 mb converted to bytes for use in size calculations.
pub const MB_98: usize = 102760448;

/// 99MB in bytes.
/// This constant represents 99 mb converted to bytes for use in size calculations.
pub const MB_99: usize = 103809024;

/// 100MB in bytes.
/// This constant represents 100 mb converted to bytes for use in size calculations.
pub const MB_100: usize = 104857600;

/// 101MB in bytes.
/// This constant represents 101 mb converted to bytes for use in size calculations.
pub const MB_101: usize = 105906176;

/// 102MB in bytes.
/// This constant represents 102 mb converted to bytes for use in size calculations.
pub const MB_102: usize = 106954752;

/// 103MB in bytes.
/// This constant represents 103 mb converted to bytes for use in size calculations.
pub const MB_103: usize = 108003328;

/// 104MB in bytes.
/// This constant represents 104 mb converted to bytes for use in size calculations.
pub const MB_104: usize = 109051904;

/// 105MB in bytes.
/// This constant represents 105 mb converted to bytes for use in size calculations.
pub const MB_105: usize = 110100480;

/// 106MB in bytes.
/// This constant represents 106 mb converted to bytes for use in size calculations.
pub const MB_106: usize = 111149056;

/// 107MB in bytes.
/// This constant represents 107 mb converted to bytes for use in size calculations.
pub const MB_107: usize = 112197632;

/// 108MB in bytes.
/// This constant represents 108 mb converted to bytes for use in size calculations.
pub const MB_108: usize = 113246208;

/// 109MB in bytes.
/// This constant represents 109 mb converted to bytes for use in size calculations.
pub const MB_109: usize = 114294784;

/// 110MB in bytes.
/// This constant represents 110 mb converted to bytes for use in size calculations.
pub const MB_110: usize = 115343360;

/// 111MB in bytes.
/// This constant represents 111 mb converted to bytes for use in size calculations.
pub const MB_111: usize = 116391936;

/// 112MB in bytes.
/// This constant represents 112 mb converted to bytes for use in size calculations.
pub const MB_112: usize = 117440512;

/// 113MB in bytes.
/// This constant represents 113 mb converted to bytes for use in size calculations.
pub const MB_113: usize = 118489088;

/// 114MB in bytes.
/// This constant represents 114 mb converted to bytes for use in size calculations.
pub const MB_114: usize = 119537664;

/// 115MB in bytes.
/// This constant represents 115 mb converted to bytes for use in size calculations.
pub const MB_115: usize = 120586240;

/// 116MB in bytes.
/// This constant represents 116 mb converted to bytes for use in size calculations.
pub const MB_116: usize = 121634816;

/// 117MB in bytes.
/// This constant represents 117 mb converted to bytes for use in size calculations.
pub const MB_117: usize = 122683392;

/// 118MB in bytes.
/// This constant represents 118 mb converted to bytes for use in size calculations.
pub const MB_118: usize = 123731968;

/// 119MB in bytes.
/// This constant represents 119 mb converted to bytes for use in size calculations.
pub const MB_119: usize = 124780544;

/// 120MB in bytes.
/// This constant represents 120 mb converted to bytes for use in size calculations.
pub const MB_120: usize = 125829120;

/// 121MB in bytes.
/// This constant represents 121 mb converted to bytes for use in size calculations.
pub const MB_121: usize = 126877696;

/// 122MB in bytes.
/// This constant represents 122 mb converted to bytes for use in size calculations.
pub const MB_122: usize = 127926272;

/// 123MB in bytes.
/// This constant represents 123 mb converted to bytes for use in size calculations.
pub const MB_123: usize = 128974848;

/// 124MB in bytes.
/// This constant represents 124 mb converted to bytes for use in size calculations.
pub const MB_124: usize = 130023424;

/// 125MB in bytes.
/// This constant represents 125 mb converted to bytes for use in size calculations.
pub const MB_125: usize = 131072000;

/// 126MB in bytes.
/// This constant represents 126 mb converted to bytes for use in size calculations.
pub const MB_126: usize = 132120576;

/// 127MB in bytes.
/// This constant represents 127 mb converted to bytes for use in size calculations.
pub const MB_127: usize = 133169152;

/// 128MB in bytes.
/// This constant represents 128 mb converted to bytes for use in size calculations.
pub const MB_128: usize = 134217728;

/// 129MB in bytes.
/// This constant represents 129 mb converted to bytes for use in size calculations.
pub const MB_129: usize = 135266304;

/// 130MB in bytes.
/// This constant represents 130 mb converted to bytes for use in size calculations.
pub const MB_130: usize = 136314880;

/// 131MB in bytes.
/// This constant represents 131 mb converted to bytes for use in size calculations.
pub const MB_131: usize = 137363456;

/// 132MB in bytes.
/// This constant represents 132 mb converted to bytes for use in size calculations.
pub const MB_132: usize = 138412032;

/// 133MB in bytes.
/// This constant represents 133 mb converted to bytes for use in size calculations.
pub const MB_133: usize = 139460608;

/// 134MB in bytes.
/// This constant represents 134 mb converted to bytes for use in size calculations.
pub const MB_134: usize = 140509184;

/// 135MB in bytes.
/// This constant represents 135 mb converted to bytes for use in size calculations.
pub const MB_135: usize = 141557760;

/// 136MB in bytes.
/// This constant represents 136 mb converted to bytes for use in size calculations.
pub const MB_136: usize = 142606336;

/// 137MB in bytes.
/// This constant represents 137 mb converted to bytes for use in size calculations.
pub const MB_137: usize = 143654912;

/// 138MB in bytes.
/// This constant represents 138 mb converted to bytes for use in size calculations.
pub const MB_138: usize = 144703488;

/// 139MB in bytes.
/// This constant represents 139 mb converted to bytes for use in size calculations.
pub const MB_139: usize = 145752064;

/// 140MB in bytes.
/// This constant represents 140 mb converted to bytes for use in size calculations.
pub const MB_140: usize = 146800640;

/// 141MB in bytes.
/// This constant represents 141 mb converted to bytes for use in size calculations.
pub const MB_141: usize = 147849216;

/// 142MB in bytes.
/// This constant represents 142 mb converted to bytes for use in size calculations.
pub const MB_142: usize = 148897792;

/// 143MB in bytes.
/// This constant represents 143 mb converted to bytes for use in size calculations.
pub const MB_143: usize = 149946368;

/// 144MB in bytes.
/// This constant represents 144 mb converted to bytes for use in size calculations.
pub const MB_144: usize = 150994944;

/// 145MB in bytes.
/// This constant represents 145 mb converted to bytes for use in size calculations.
pub const MB_145: usize = 152043520;

/// 146MB in bytes.
/// This constant represents 146 mb converted to bytes for use in size calculations.
pub const MB_146: usize = 153092096;

/// 147MB in bytes.
/// This constant represents 147 mb converted to bytes for use in size calculations.
pub const MB_147: usize = 154140672;

/// 148MB in bytes.
/// This constant represents 148 mb converted to bytes for use in size calculations.
pub const MB_148: usize = 155189248;

/// 149MB in bytes.
/// This constant represents 149 mb converted to bytes for use in size calculations.
pub const MB_149: usize = 156237824;

/// 150MB in bytes.
/// This constant represents 150 mb converted to bytes for use in size calculations.
pub const MB_150: usize = 157286400;

/// 151MB in bytes.
/// This constant represents 151 mb converted to bytes for use in size calculations.
pub const MB_151: usize = 158334976;

/// 152MB in bytes.
/// This constant represents 152 mb converted to bytes for use in size calculations.
pub const MB_152: usize = 159383552;

/// 153MB in bytes.
/// This constant represents 153 mb converted to bytes for use in size calculations.
pub const MB_153: usize = 160432128;

/// 154MB in bytes.
/// This constant represents 154 mb converted to bytes for use in size calculations.
pub const MB_154: usize = 161480704;

/// 155MB in bytes.
/// This constant represents 155 mb converted to bytes for use in size calculations.
pub const MB_155: usize = 162529280;

/// 156MB in bytes.
/// This constant represents 156 mb converted to bytes for use in size calculations.
pub const MB_156: usize = 163577856;

/// 157MB in bytes.
/// This constant represents 157 mb converted to bytes for use in size calculations.
pub const MB_157: usize = 164626432;

/// 158MB in bytes.
/// This constant represents 158 mb converted to bytes for use in size calculations.
pub const MB_158: usize = 165675008;

/// 159MB in bytes.
/// This constant represents 159 mb converted to bytes for use in size calculations.
pub const MB_159: usize = 166723584;

/// 160MB in bytes.
/// This constant represents 160 mb converted to bytes for use in size calculations.
pub const MB_160: usize = 167772160;

/// 161MB in bytes.
/// This constant represents 161 mb converted to bytes for use in size calculations.
pub const MB_161: usize = 168820736;

/// 162MB in bytes.
/// This constant represents 162 mb converted to bytes for use in size calculations.
pub const MB_162: usize = 169869312;

/// 163MB in bytes.
/// This constant represents 163 mb converted to bytes for use in size calculations.
pub const MB_163: usize = 170917888;

/// 164MB in bytes.
/// This constant represents 164 mb converted to bytes for use in size calculations.
pub const MB_164: usize = 171966464;

/// 165MB in bytes.
/// This constant represents 165 mb converted to bytes for use in size calculations.
pub const MB_165: usize = 173015040;

/// 166MB in bytes.
/// This constant represents 166 mb converted to bytes for use in size calculations.
pub const MB_166: usize = 174063616;

/// 167MB in bytes.
/// This constant represents 167 mb converted to bytes for use in size calculations.
pub const MB_167: usize = 175112192;

/// 168MB in bytes.
/// This constant represents 168 mb converted to bytes for use in size calculations.
pub const MB_168: usize = 176160768;

/// 169MB in bytes.
/// This constant represents 169 mb converted to bytes for use in size calculations.
pub const MB_169: usize = 177209344;

/// 170MB in bytes.
/// This constant represents 170 mb converted to bytes for use in size calculations.
pub const MB_170: usize = 178257920;

/// 171MB in bytes.
/// This constant represents 171 mb converted to bytes for use in size calculations.
pub const MB_171: usize = 179306496;

/// 172MB in bytes.
/// This constant represents 172 mb converted to bytes for use in size calculations.
pub const MB_172: usize = 180355072;

/// 173MB in bytes.
/// This constant represents 173 mb converted to bytes for use in size calculations.
pub const MB_173: usize = 181403648;

/// 174MB in bytes.
/// This constant represents 174 mb converted to bytes for use in size calculations.
pub const MB_174: usize = 182452224;

/// 175MB in bytes.
/// This constant represents 175 mb converted to bytes for use in size calculations.
pub const MB_175: usize = 183500800;

/// 176MB in bytes.
/// This constant represents 176 mb converted to bytes for use in size calculations.
pub const MB_176: usize = 184549376;

/// 177MB in bytes.
/// This constant represents 177 mb converted to bytes for use in size calculations.
pub const MB_177: usize = 185597952;

/// 178MB in bytes.
/// This constant represents 178 mb converted to bytes for use in size calculations.
pub const MB_178: usize = 186646528;

/// 179MB in bytes.
/// This constant represents 179 mb converted to bytes for use in size calculations.
pub const MB_179: usize = 187695104;

/// 180MB in bytes.
/// This constant represents 180 mb converted to bytes for use in size calculations.
pub const MB_180: usize = 188743680;

/// 181MB in bytes.
/// This constant represents 181 mb converted to bytes for use in size calculations.
pub const MB_181: usize = 189792256;

/// 182MB in bytes.
/// This constant represents 182 mb converted to bytes for use in size calculations.
pub const MB_182: usize = 190840832;

/// 183MB in bytes.
/// This constant represents 183 mb converted to bytes for use in size calculations.
pub const MB_183: usize = 191889408;

/// 184MB in bytes.
/// This constant represents 184 mb converted to bytes for use in size calculations.
pub const MB_184: usize = 192937984;

/// 185MB in bytes.
/// This constant represents 185 mb converted to bytes for use in size calculations.
pub const MB_185: usize = 193986560;

/// 186MB in bytes.
/// This constant represents 186 mb converted to bytes for use in size calculations.
pub const MB_186: usize = 195035136;

/// 187MB in bytes.
/// This constant represents 187 mb converted to bytes for use in size calculations.
pub const MB_187: usize = 196083712;

/// 188MB in bytes.
/// This constant represents 188 mb converted to bytes for use in size calculations.
pub const MB_188: usize = 197132288;

/// 189MB in bytes.
/// This constant represents 189 mb converted to bytes for use in size calculations.
pub const MB_189: usize = 198180864;

/// 190MB in bytes.
/// This constant represents 190 mb converted to bytes for use in size calculations.
pub const MB_190: usize = 199229440;

/// 191MB in bytes.
/// This constant represents 191 mb converted to bytes for use in size calculations.
pub const MB_191: usize = 200278016;

/// 192MB in bytes.
/// This constant represents 192 mb converted to bytes for use in size calculations.
pub const MB_192: usize = 201326592;

/// 193MB in bytes.
/// This constant represents 193 mb converted to bytes for use in size calculations.
pub const MB_193: usize = 202375168;

/// 194MB in bytes.
/// This constant represents 194 mb converted to bytes for use in size calculations.
pub const MB_194: usize = 203423744;

/// 195MB in bytes.
/// This constant represents 195 mb converted to bytes for use in size calculations.
pub const MB_195: usize = 204472320;

/// 196MB in bytes.
/// This constant represents 196 mb converted to bytes for use in size calculations.
pub const MB_196: usize = 205520896;

/// 197MB in bytes.
/// This constant represents 197 mb converted to bytes for use in size calculations.
pub const MB_197: usize = 206569472;

/// 198MB in bytes.
/// This constant represents 198 mb converted to bytes for use in size calculations.
pub const MB_198: usize = 207618048;

/// 199MB in bytes.
/// This constant represents 199 mb converted to bytes for use in size calculations.
pub const MB_199: usize = 208666624;

/// 200MB in bytes.
/// This constant represents 200 mb converted to bytes for use in size calculations.
pub const MB_200: usize = 209715200;

/// 201MB in bytes.
/// This constant represents 201 mb converted to bytes for use in size calculations.
pub const MB_201: usize = 210763776;

/// 202MB in bytes.
/// This constant represents 202 mb converted to bytes for use in size calculations.
pub const MB_202: usize = 211812352;

/// 203MB in bytes.
/// This constant represents 203 mb converted to bytes for use in size calculations.
pub const MB_203: usize = 212860928;

/// 204MB in bytes.
/// This constant represents 204 mb converted to bytes for use in size calculations.
pub const MB_204: usize = 213909504;

/// 205MB in bytes.
/// This constant represents 205 mb converted to bytes for use in size calculations.
pub const MB_205: usize = 214958080;

/// 206MB in bytes.
/// This constant represents 206 mb converted to bytes for use in size calculations.
pub const MB_206: usize = 216006656;

/// 207MB in bytes.
/// This constant represents 207 mb converted to bytes for use in size calculations.
pub const MB_207: usize = 217055232;

/// 208MB in bytes.
/// This constant represents 208 mb converted to bytes for use in size calculations.
pub const MB_208: usize = 218103808;

/// 209MB in bytes.
/// This constant represents 209 mb converted to bytes for use in size calculations.
pub const MB_209: usize = 219152384;

/// 210MB in bytes.
/// This constant represents 210 mb converted to bytes for use in size calculations.
pub const MB_210: usize = 220200960;

/// 211MB in bytes.
/// This constant represents 211 mb converted to bytes for use in size calculations.
pub const MB_211: usize = 221249536;

/// 212MB in bytes.
/// This constant represents 212 mb converted to bytes for use in size calculations.
pub const MB_212: usize = 222298112;

/// 213MB in bytes.
/// This constant represents 213 mb converted to bytes for use in size calculations.
pub const MB_213: usize = 223346688;

/// 214MB in bytes.
/// This constant represents 214 mb converted to bytes for use in size calculations.
pub const MB_214: usize = 224395264;

/// 215MB in bytes.
/// This constant represents 215 mb converted to bytes for use in size calculations.
pub const MB_215: usize = 225443840;

/// 216MB in bytes.
/// This constant represents 216 mb converted to bytes for use in size calculations.
pub const MB_216: usize = 226492416;

/// 217MB in bytes.
/// This constant represents 217 mb converted to bytes for use in size calculations.
pub const MB_217: usize = 227540992;

/// 218MB in bytes.
/// This constant represents 218 mb converted to bytes for use in size calculations.
pub const MB_218: usize = 228589568;

/// 219MB in bytes.
/// This constant represents 219 mb converted to bytes for use in size calculations.
pub const MB_219: usize = 229638144;

/// 220MB in bytes.
/// This constant represents 220 mb converted to bytes for use in size calculations.
pub const MB_220: usize = 230686720;

/// 221MB in bytes.
/// This constant represents 221 mb converted to bytes for use in size calculations.
pub const MB_221: usize = 231735296;

/// 222MB in bytes.
/// This constant represents 222 mb converted to bytes for use in size calculations.
pub const MB_222: usize = 232783872;

/// 223MB in bytes.
/// This constant represents 223 mb converted to bytes for use in size calculations.
pub const MB_223: usize = 233832448;

/// 224MB in bytes.
/// This constant represents 224 mb converted to bytes for use in size calculations.
pub const MB_224: usize = 234881024;

/// 225MB in bytes.
/// This constant represents 225 mb converted to bytes for use in size calculations.
pub const MB_225: usize = 235929600;

/// 226MB in bytes.
/// This constant represents 226 mb converted to bytes for use in size calculations.
pub const MB_226: usize = 236978176;

/// 227MB in bytes.
/// This constant represents 227 mb converted to bytes for use in size calculations.
pub const MB_227: usize = 238026752;

/// 228MB in bytes.
/// This constant represents 228 mb converted to bytes for use in size calculations.
pub const MB_228: usize = 239075328;

/// 229MB in bytes.
/// This constant represents 229 mb converted to bytes for use in size calculations.
pub const MB_229: usize = 240123904;

/// 230MB in bytes.
/// This constant represents 230 mb converted to bytes for use in size calculations.
pub const MB_230: usize = 241172480;

/// 231MB in bytes.
/// This constant represents 231 mb converted to bytes for use in size calculations.
pub const MB_231: usize = 242221056;

/// 232MB in bytes.
/// This constant represents 232 mb converted to bytes for use in size calculations.
pub const MB_232: usize = 243269632;

/// 233MB in bytes.
/// This constant represents 233 mb converted to bytes for use in size calculations.
pub const MB_233: usize = 244318208;

/// 234MB in bytes.
/// This constant represents 234 mb converted to bytes for use in size calculations.
pub const MB_234: usize = 245366784;

/// 235MB in bytes.
/// This constant represents 235 mb converted to bytes for use in size calculations.
pub const MB_235: usize = 246415360;

/// 236MB in bytes.
/// This constant represents 236 mb converted to bytes for use in size calculations.
pub const MB_236: usize = 247463936;

/// 237MB in bytes.
/// This constant represents 237 mb converted to bytes for use in size calculations.
pub const MB_237: usize = 248512512;

/// 238MB in bytes.
/// This constant represents 238 mb converted to bytes for use in size calculations.
pub const MB_238: usize = 249561088;

/// 239MB in bytes.
/// This constant represents 239 mb converted to bytes for use in size calculations.
pub const MB_239: usize = 250609664;

/// 240MB in bytes.
/// This constant represents 240 mb converted to bytes for use in size calculations.
pub const MB_240: usize = 251658240;

/// 241MB in bytes.
/// This constant represents 241 mb converted to bytes for use in size calculations.
pub const MB_241: usize = 252706816;

/// 242MB in bytes.
/// This constant represents 242 mb converted to bytes for use in size calculations.
pub const MB_242: usize = 253755392;

/// 243MB in bytes.
/// This constant represents 243 mb converted to bytes for use in size calculations.
pub const MB_243: usize = 254803968;

/// 244MB in bytes.
/// This constant represents 244 mb converted to bytes for use in size calculations.
pub const MB_244: usize = 255852544;

/// 245MB in bytes.
/// This constant represents 245 mb converted to bytes for use in size calculations.
pub const MB_245: usize = 256901120;

/// 246MB in bytes.
/// This constant represents 246 mb converted to bytes for use in size calculations.
pub const MB_246: usize = 257949696;

/// 247MB in bytes.
/// This constant represents 247 mb converted to bytes for use in size calculations.
pub const MB_247: usize = 258998272;

/// 248MB in bytes.
/// This constant represents 248 mb converted to bytes for use in size calculations.
pub const MB_248: usize = 260046848;

/// 249MB in bytes.
/// This constant represents 249 mb converted to bytes for use in size calculations.
pub const MB_249: usize = 261095424;

/// 250MB in bytes.
/// This constant represents 250 mb converted to bytes for use in size calculations.
pub const MB_250: usize = 262144000;

/// 251MB in bytes.
/// This constant represents 251 mb converted to bytes for use in size calculations.
pub const MB_251: usize = 263192576;

/// 252MB in bytes.
/// This constant represents 252 mb converted to bytes for use in size calculations.
pub const MB_252: usize = 264241152;

/// 253MB in bytes.
/// This constant represents 253 mb converted to bytes for use in size calculations.
pub const MB_253: usize = 265289728;

/// 254MB in bytes.
/// This constant represents 254 mb converted to bytes for use in size calculations.
pub const MB_254: usize = 266338304;

/// 255MB in bytes.
/// This constant represents 255 mb converted to bytes for use in size calculations.
pub const MB_255: usize = 267386880;

/// 256MB in bytes.
/// This constant represents 256 mb converted to bytes for use in size calculations.
pub const MB_256: usize = 268435456;

/// 257MB in bytes.
/// This constant represents 257 mb converted to bytes for use in size calculations.
pub const MB_257: usize = 269484032;

/// 258MB in bytes.
/// This constant represents 258 mb converted to bytes for use in size calculations.
pub const MB_258: usize = 270532608;

/// 259MB in bytes.
/// This constant represents 259 mb converted to bytes for use in size calculations.
pub const MB_259: usize = 271581184;

/// 260MB in bytes.
/// This constant represents 260 mb converted to bytes for use in size calculations.
pub const MB_260: usize = 272629760;

/// 261MB in bytes.
/// This constant represents 261 mb converted to bytes for use in size calculations.
pub const MB_261: usize = 273678336;

/// 262MB in bytes.
/// This constant represents 262 mb converted to bytes for use in size calculations.
pub const MB_262: usize = 274726912;

/// 263MB in bytes.
/// This constant represents 263 mb converted to bytes for use in size calculations.
pub const MB_263: usize = 275775488;

/// 264MB in bytes.
/// This constant represents 264 mb converted to bytes for use in size calculations.
pub const MB_264: usize = 276824064;

/// 265MB in bytes.
/// This constant represents 265 mb converted to bytes for use in size calculations.
pub const MB_265: usize = 277872640;

/// 266MB in bytes.
/// This constant represents 266 mb converted to bytes for use in size calculations.
pub const MB_266: usize = 278921216;

/// 267MB in bytes.
/// This constant represents 267 mb converted to bytes for use in size calculations.
pub const MB_267: usize = 279969792;

/// 268MB in bytes.
/// This constant represents 268 mb converted to bytes for use in size calculations.
pub const MB_268: usize = 281018368;

/// 269MB in bytes.
/// This constant represents 269 mb converted to bytes for use in size calculations.
pub const MB_269: usize = 282066944;

/// 270MB in bytes.
/// This constant represents 270 mb converted to bytes for use in size calculations.
pub const MB_270: usize = 283115520;

/// 271MB in bytes.
/// This constant represents 271 mb converted to bytes for use in size calculations.
pub const MB_271: usize = 284164096;

/// 272MB in bytes.
/// This constant represents 272 mb converted to bytes for use in size calculations.
pub const MB_272: usize = 285212672;

/// 273MB in bytes.
/// This constant represents 273 mb converted to bytes for use in size calculations.
pub const MB_273: usize = 286261248;

/// 274MB in bytes.
/// This constant represents 274 mb converted to bytes for use in size calculations.
pub const MB_274: usize = 287309824;

/// 275MB in bytes.
/// This constant represents 275 mb converted to bytes for use in size calculations.
pub const MB_275: usize = 288358400;

/// 276MB in bytes.
/// This constant represents 276 mb converted to bytes for use in size calculations.
pub const MB_276: usize = 289406976;

/// 277MB in bytes.
/// This constant represents 277 mb converted to bytes for use in size calculations.
pub const MB_277: usize = 290455552;

/// 278MB in bytes.
/// This constant represents 278 mb converted to bytes for use in size calculations.
pub const MB_278: usize = 291504128;

/// 279MB in bytes.
/// This constant represents 279 mb converted to bytes for use in size calculations.
pub const MB_279: usize = 292552704;

/// 280MB in bytes.
/// This constant represents 280 mb converted to bytes for use in size calculations.
pub const MB_280: usize = 293601280;

/// 281MB in bytes.
/// This constant represents 281 mb converted to bytes for use in size calculations.
pub const MB_281: usize = 294649856;

/// 282MB in bytes.
/// This constant represents 282 mb converted to bytes for use in size calculations.
pub const MB_282: usize = 295698432;

/// 283MB in bytes.
/// This constant represents 283 mb converted to bytes for use in size calculations.
pub const MB_283: usize = 296747008;

/// 284MB in bytes.
/// This constant represents 284 mb converted to bytes for use in size calculations.
pub const MB_284: usize = 297795584;

/// 285MB in bytes.
/// This constant represents 285 mb converted to bytes for use in size calculations.
pub const MB_285: usize = 298844160;

/// 286MB in bytes.
/// This constant represents 286 mb converted to bytes for use in size calculations.
pub const MB_286: usize = 299892736;

/// 287MB in bytes.
/// This constant represents 287 mb converted to bytes for use in size calculations.
pub const MB_287: usize = 300941312;

/// 288MB in bytes.
/// This constant represents 288 mb converted to bytes for use in size calculations.
pub const MB_288: usize = 301989888;

/// 289MB in bytes.
/// This constant represents 289 mb converted to bytes for use in size calculations.
pub const MB_289: usize = 303038464;

/// 290MB in bytes.
/// This constant represents 290 mb converted to bytes for use in size calculations.
pub const MB_290: usize = 304087040;

/// 291MB in bytes.
/// This constant represents 291 mb converted to bytes for use in size calculations.
pub const MB_291: usize = 305135616;

/// 292MB in bytes.
/// This constant represents 292 mb converted to bytes for use in size calculations.
pub const MB_292: usize = 306184192;

/// 293MB in bytes.
/// This constant represents 293 mb converted to bytes for use in size calculations.
pub const MB_293: usize = 307232768;

/// 294MB in bytes.
/// This constant represents 294 mb converted to bytes for use in size calculations.
pub const MB_294: usize = 308281344;

/// 295MB in bytes.
/// This constant represents 295 mb converted to bytes for use in size calculations.
pub const MB_295: usize = 309329920;

/// 296MB in bytes.
/// This constant represents 296 mb converted to bytes for use in size calculations.
pub const MB_296: usize = 310378496;

/// 297MB in bytes.
/// This constant represents 297 mb converted to bytes for use in size calculations.
pub const MB_297: usize = 311427072;

/// 298MB in bytes.
/// This constant represents 298 mb converted to bytes for use in size calculations.
pub const MB_298: usize = 312475648;

/// 299MB in bytes.
/// This constant represents 299 mb converted to bytes for use in size calculations.
pub const MB_299: usize = 313524224;

/// 300MB in bytes.
/// This constant represents 300 mb converted to bytes for use in size calculations.
pub const MB_300: usize = 314572800;

/// 301MB in bytes.
/// This constant represents 301 mb converted to bytes for use in size calculations.
pub const MB_301: usize = 315621376;

/// 302MB in bytes.
/// This constant represents 302 mb converted to bytes for use in size calculations.
pub const MB_302: usize = 316669952;

/// 303MB in bytes.
/// This constant represents 303 mb converted to bytes for use in size calculations.
pub const MB_303: usize = 317718528;

/// 304MB in bytes.
/// This constant represents 304 mb converted to bytes for use in size calculations.
pub const MB_304: usize = 318767104;

/// 305MB in bytes.
/// This constant represents 305 mb converted to bytes for use in size calculations.
pub const MB_305: usize = 319815680;

/// 306MB in bytes.
/// This constant represents 306 mb converted to bytes for use in size calculations.
pub const MB_306: usize = 320864256;

/// 307MB in bytes.
/// This constant represents 307 mb converted to bytes for use in size calculations.
pub const MB_307: usize = 321912832;

/// 308MB in bytes.
/// This constant represents 308 mb converted to bytes for use in size calculations.
pub const MB_308: usize = 322961408;

/// 309MB in bytes.
/// This constant represents 309 mb converted to bytes for use in size calculations.
pub const MB_309: usize = 324009984;

/// 310MB in bytes.
/// This constant represents 310 mb converted to bytes for use in size calculations.
pub const MB_310: usize = 325058560;

/// 311MB in bytes.
/// This constant represents 311 mb converted to bytes for use in size calculations.
pub const MB_311: usize = 326107136;

/// 312MB in bytes.
/// This constant represents 312 mb converted to bytes for use in size calculations.
pub const MB_312: usize = 327155712;

/// 313MB in bytes.
/// This constant represents 313 mb converted to bytes for use in size calculations.
pub const MB_313: usize = 328204288;

/// 314MB in bytes.
/// This constant represents 314 mb converted to bytes for use in size calculations.
pub const MB_314: usize = 329252864;

/// 315MB in bytes.
/// This constant represents 315 mb converted to bytes for use in size calculations.
pub const MB_315: usize = 330301440;

/// 316MB in bytes.
/// This constant represents 316 mb converted to bytes for use in size calculations.
pub const MB_316: usize = 331350016;

/// 317MB in bytes.
/// This constant represents 317 mb converted to bytes for use in size calculations.
pub const MB_317: usize = 332398592;

/// 318MB in bytes.
/// This constant represents 318 mb converted to bytes for use in size calculations.
pub const MB_318: usize = 333447168;

/// 319MB in bytes.
/// This constant represents 319 mb converted to bytes for use in size calculations.
pub const MB_319: usize = 334495744;

/// 320MB in bytes.
/// This constant represents 320 mb converted to bytes for use in size calculations.
pub const MB_320: usize = 335544320;

/// 321MB in bytes.
/// This constant represents 321 mb converted to bytes for use in size calculations.
pub const MB_321: usize = 336592896;

/// 322MB in bytes.
/// This constant represents 322 mb converted to bytes for use in size calculations.
pub const MB_322: usize = 337641472;

/// 323MB in bytes.
/// This constant represents 323 mb converted to bytes for use in size calculations.
pub const MB_323: usize = 338690048;

/// 324MB in bytes.
/// This constant represents 324 mb converted to bytes for use in size calculations.
pub const MB_324: usize = 339738624;

/// 325MB in bytes.
/// This constant represents 325 mb converted to bytes for use in size calculations.
pub const MB_325: usize = 340787200;

/// 326MB in bytes.
/// This constant represents 326 mb converted to bytes for use in size calculations.
pub const MB_326: usize = 341835776;

/// 327MB in bytes.
/// This constant represents 327 mb converted to bytes for use in size calculations.
pub const MB_327: usize = 342884352;

/// 328MB in bytes.
/// This constant represents 328 mb converted to bytes for use in size calculations.
pub const MB_328: usize = 343932928;

/// 329MB in bytes.
/// This constant represents 329 mb converted to bytes for use in size calculations.
pub const MB_329: usize = 344981504;

/// 330MB in bytes.
/// This constant represents 330 mb converted to bytes for use in size calculations.
pub const MB_330: usize = 346030080;

/// 331MB in bytes.
/// This constant represents 331 mb converted to bytes for use in size calculations.
pub const MB_331: usize = 347078656;

/// 332MB in bytes.
/// This constant represents 332 mb converted to bytes for use in size calculations.
pub const MB_332: usize = 348127232;

/// 333MB in bytes.
/// This constant represents 333 mb converted to bytes for use in size calculations.
pub const MB_333: usize = 349175808;

/// 334MB in bytes.
/// This constant represents 334 mb converted to bytes for use in size calculations.
pub const MB_334: usize = 350224384;

/// 335MB in bytes.
/// This constant represents 335 mb converted to bytes for use in size calculations.
pub const MB_335: usize = 351272960;

/// 336MB in bytes.
/// This constant represents 336 mb converted to bytes for use in size calculations.
pub const MB_336: usize = 352321536;

/// 337MB in bytes.
/// This constant represents 337 mb converted to bytes for use in size calculations.
pub const MB_337: usize = 353370112;

/// 338MB in bytes.
/// This constant represents 338 mb converted to bytes for use in size calculations.
pub const MB_338: usize = 354418688;

/// 339MB in bytes.
/// This constant represents 339 mb converted to bytes for use in size calculations.
pub const MB_339: usize = 355467264;

/// 340MB in bytes.
/// This constant represents 340 mb converted to bytes for use in size calculations.
pub const MB_340: usize = 356515840;

/// 341MB in bytes.
/// This constant represents 341 mb converted to bytes for use in size calculations.
pub const MB_341: usize = 357564416;

/// 342MB in bytes.
/// This constant represents 342 mb converted to bytes for use in size calculations.
pub const MB_342: usize = 358612992;

/// 343MB in bytes.
/// This constant represents 343 mb converted to bytes for use in size calculations.
pub const MB_343: usize = 359661568;

/// 344MB in bytes.
/// This constant represents 344 mb converted to bytes for use in size calculations.
pub const MB_344: usize = 360710144;

/// 345MB in bytes.
/// This constant represents 345 mb converted to bytes for use in size calculations.
pub const MB_345: usize = 361758720;

/// 346MB in bytes.
/// This constant represents 346 mb converted to bytes for use in size calculations.
pub const MB_346: usize = 362807296;

/// 347MB in bytes.
/// This constant represents 347 mb converted to bytes for use in size calculations.
pub const MB_347: usize = 363855872;

/// 348MB in bytes.
/// This constant represents 348 mb converted to bytes for use in size calculations.
pub const MB_348: usize = 364904448;

/// 349MB in bytes.
/// This constant represents 349 mb converted to bytes for use in size calculations.
pub const MB_349: usize = 365953024;

/// 350MB in bytes.
/// This constant represents 350 mb converted to bytes for use in size calculations.
pub const MB_350: usize = 367001600;

/// 351MB in bytes.
/// This constant represents 351 mb converted to bytes for use in size calculations.
pub const MB_351: usize = 368050176;

/// 352MB in bytes.
/// This constant represents 352 mb converted to bytes for use in size calculations.
pub const MB_352: usize = 369098752;

/// 353MB in bytes.
/// This constant represents 353 mb converted to bytes for use in size calculations.
pub const MB_353: usize = 370147328;

/// 354MB in bytes.
/// This constant represents 354 mb converted to bytes for use in size calculations.
pub const MB_354: usize = 371195904;

/// 355MB in bytes.
/// This constant represents 355 mb converted to bytes for use in size calculations.
pub const MB_355: usize = 372244480;

/// 356MB in bytes.
/// This constant represents 356 mb converted to bytes for use in size calculations.
pub const MB_356: usize = 373293056;

/// 357MB in bytes.
/// This constant represents 357 mb converted to bytes for use in size calculations.
pub const MB_357: usize = 374341632;

/// 358MB in bytes.
/// This constant represents 358 mb converted to bytes for use in size calculations.
pub const MB_358: usize = 375390208;

/// 359MB in bytes.
/// This constant represents 359 mb converted to bytes for use in size calculations.
pub const MB_359: usize = 376438784;

/// 360MB in bytes.
/// This constant represents 360 mb converted to bytes for use in size calculations.
pub const MB_360: usize = 377487360;

/// 361MB in bytes.
/// This constant represents 361 mb converted to bytes for use in size calculations.
pub const MB_361: usize = 378535936;

/// 362MB in bytes.
/// This constant represents 362 mb converted to bytes for use in size calculations.
pub const MB_362: usize = 379584512;

/// 363MB in bytes.
/// This constant represents 363 mb converted to bytes for use in size calculations.
pub const MB_363: usize = 380633088;

/// 364MB in bytes.
/// This constant represents 364 mb converted to bytes for use in size calculations.
pub const MB_364: usize = 381681664;

/// 365MB in bytes.
/// This constant represents 365 mb converted to bytes for use in size calculations.
pub const MB_365: usize = 382730240;

/// 366MB in bytes.
/// This constant represents 366 mb converted to bytes for use in size calculations.
pub const MB_366: usize = 383778816;

/// 367MB in bytes.
/// This constant represents 367 mb converted to bytes for use in size calculations.
pub const MB_367: usize = 384827392;

/// 368MB in bytes.
/// This constant represents 368 mb converted to bytes for use in size calculations.
pub const MB_368: usize = 385875968;

/// 369MB in bytes.
/// This constant represents 369 mb converted to bytes for use in size calculations.
pub const MB_369: usize = 386924544;

/// 370MB in bytes.
/// This constant represents 370 mb converted to bytes for use in size calculations.
pub const MB_370: usize = 387973120;

/// 371MB in bytes.
/// This constant represents 371 mb converted to bytes for use in size calculations.
pub const MB_371: usize = 389021696;

/// 372MB in bytes.
/// This constant represents 372 mb converted to bytes for use in size calculations.
pub const MB_372: usize = 390070272;

/// 373MB in bytes.
/// This constant represents 373 mb converted to bytes for use in size calculations.
pub const MB_373: usize = 391118848;

/// 374MB in bytes.
/// This constant represents 374 mb converted to bytes for use in size calculations.
pub const MB_374: usize = 392167424;

/// 375MB in bytes.
/// This constant represents 375 mb converted to bytes for use in size calculations.
pub const MB_375: usize = 393216000;

/// 376MB in bytes.
/// This constant represents 376 mb converted to bytes for use in size calculations.
pub const MB_376: usize = 394264576;

/// 377MB in bytes.
/// This constant represents 377 mb converted to bytes for use in size calculations.
pub const MB_377: usize = 395313152;

/// 378MB in bytes.
/// This constant represents 378 mb converted to bytes for use in size calculations.
pub const MB_378: usize = 396361728;

/// 379MB in bytes.
/// This constant represents 379 mb converted to bytes for use in size calculations.
pub const MB_379: usize = 397410304;

/// 380MB in bytes.
/// This constant represents 380 mb converted to bytes for use in size calculations.
pub const MB_380: usize = 398458880;

/// 381MB in bytes.
/// This constant represents 381 mb converted to bytes for use in size calculations.
pub const MB_381: usize = 399507456;

/// 382MB in bytes.
/// This constant represents 382 mb converted to bytes for use in size calculations.
pub const MB_382: usize = 400556032;

/// 383MB in bytes.
/// This constant represents 383 mb converted to bytes for use in size calculations.
pub const MB_383: usize = 401604608;

/// 384MB in bytes.
/// This constant represents 384 mb converted to bytes for use in size calculations.
pub const MB_384: usize = 402653184;

/// 385MB in bytes.
/// This constant represents 385 mb converted to bytes for use in size calculations.
pub const MB_385: usize = 403701760;

/// 386MB in bytes.
/// This constant represents 386 mb converted to bytes for use in size calculations.
pub const MB_386: usize = 404750336;

/// 387MB in bytes.
/// This constant represents 387 mb converted to bytes for use in size calculations.
pub const MB_387: usize = 405798912;

/// 388MB in bytes.
/// This constant represents 388 mb converted to bytes for use in size calculations.
pub const MB_388: usize = 406847488;

/// 389MB in bytes.
/// This constant represents 389 mb converted to bytes for use in size calculations.
pub const MB_389: usize = 407896064;

/// 390MB in bytes.
/// This constant represents 390 mb converted to bytes for use in size calculations.
pub const MB_390: usize = 408944640;

/// 391MB in bytes.
/// This constant represents 391 mb converted to bytes for use in size calculations.
pub const MB_391: usize = 409993216;

/// 392MB in bytes.
/// This constant represents 392 mb converted to bytes for use in size calculations.
pub const MB_392: usize = 411041792;

/// 393MB in bytes.
/// This constant represents 393 mb converted to bytes for use in size calculations.
pub const MB_393: usize = 412090368;

/// 394MB in bytes.
/// This constant represents 394 mb converted to bytes for use in size calculations.
pub const MB_394: usize = 413138944;

/// 395MB in bytes.
/// This constant represents 395 mb converted to bytes for use in size calculations.
pub const MB_395: usize = 414187520;

/// 396MB in bytes.
/// This constant represents 396 mb converted to bytes for use in size calculations.
pub const MB_396: usize = 415236096;

/// 397MB in bytes.
/// This constant represents 397 mb converted to bytes for use in size calculations.
pub const MB_397: usize = 416284672;

/// 398MB in bytes.
/// This constant represents 398 mb converted to bytes for use in size calculations.
pub const MB_398: usize = 417333248;

/// 399MB in bytes.
/// This constant represents 399 mb converted to bytes for use in size calculations.
pub const MB_399: usize = 418381824;

/// 400MB in bytes.
/// This constant represents 400 mb converted to bytes for use in size calculations.
pub const MB_400: usize = 419430400;

/// 401MB in bytes.
/// This constant represents 401 mb converted to bytes for use in size calculations.
pub const MB_401: usize = 420478976;

/// 402MB in bytes.
/// This constant represents 402 mb converted to bytes for use in size calculations.
pub const MB_402: usize = 421527552;

/// 403MB in bytes.
/// This constant represents 403 mb converted to bytes for use in size calculations.
pub const MB_403: usize = 422576128;

/// 404MB in bytes.
/// This constant represents 404 mb converted to bytes for use in size calculations.
pub const MB_404: usize = 423624704;

/// 405MB in bytes.
/// This constant represents 405 mb converted to bytes for use in size calculations.
pub const MB_405: usize = 424673280;

/// 406MB in bytes.
/// This constant represents 406 mb converted to bytes for use in size calculations.
pub const MB_406: usize = 425721856;

/// 407MB in bytes.
/// This constant represents 407 mb converted to bytes for use in size calculations.
pub const MB_407: usize = 426770432;

/// 408MB in bytes.
/// This constant represents 408 mb converted to bytes for use in size calculations.
pub const MB_408: usize = 427819008;

/// 409MB in bytes.
/// This constant represents 409 mb converted to bytes for use in size calculations.
pub const MB_409: usize = 428867584;

/// 410MB in bytes.
/// This constant represents 410 mb converted to bytes for use in size calculations.
pub const MB_410: usize = 429916160;

/// 411MB in bytes.
/// This constant represents 411 mb converted to bytes for use in size calculations.
pub const MB_411: usize = 430964736;

/// 412MB in bytes.
/// This constant represents 412 mb converted to bytes for use in size calculations.
pub const MB_412: usize = 432013312;

/// 413MB in bytes.
/// This constant represents 413 mb converted to bytes for use in size calculations.
pub const MB_413: usize = 433061888;

/// 414MB in bytes.
/// This constant represents 414 mb converted to bytes for use in size calculations.
pub const MB_414: usize = 434110464;

/// 415MB in bytes.
/// This constant represents 415 mb converted to bytes for use in size calculations.
pub const MB_415: usize = 435159040;

/// 416MB in bytes.
/// This constant represents 416 mb converted to bytes for use in size calculations.
pub const MB_416: usize = 436207616;

/// 417MB in bytes.
/// This constant represents 417 mb converted to bytes for use in size calculations.
pub const MB_417: usize = 437256192;

/// 418MB in bytes.
/// This constant represents 418 mb converted to bytes for use in size calculations.
pub const MB_418: usize = 438304768;

/// 419MB in bytes.
/// This constant represents 419 mb converted to bytes for use in size calculations.
pub const MB_419: usize = 439353344;

/// 420MB in bytes.
/// This constant represents 420 mb converted to bytes for use in size calculations.
pub const MB_420: usize = 440401920;

/// 421MB in bytes.
/// This constant represents 421 mb converted to bytes for use in size calculations.
pub const MB_421: usize = 441450496;

/// 422MB in bytes.
/// This constant represents 422 mb converted to bytes for use in size calculations.
pub const MB_422: usize = 442499072;

/// 423MB in bytes.
/// This constant represents 423 mb converted to bytes for use in size calculations.
pub const MB_423: usize = 443547648;

/// 424MB in bytes.
/// This constant represents 424 mb converted to bytes for use in size calculations.
pub const MB_424: usize = 444596224;

/// 425MB in bytes.
/// This constant represents 425 mb converted to bytes for use in size calculations.
pub const MB_425: usize = 445644800;

/// 426MB in bytes.
/// This constant represents 426 mb converted to bytes for use in size calculations.
pub const MB_426: usize = 446693376;

/// 427MB in bytes.
/// This constant represents 427 mb converted to bytes for use in size calculations.
pub const MB_427: usize = 447741952;

/// 428MB in bytes.
/// This constant represents 428 mb converted to bytes for use in size calculations.
pub const MB_428: usize = 448790528;

/// 429MB in bytes.
/// This constant represents 429 mb converted to bytes for use in size calculations.
pub const MB_429: usize = 449839104;

/// 430MB in bytes.
/// This constant represents 430 mb converted to bytes for use in size calculations.
pub const MB_430: usize = 450887680;

/// 431MB in bytes.
/// This constant represents 431 mb converted to bytes for use in size calculations.
pub const MB_431: usize = 451936256;

/// 432MB in bytes.
/// This constant represents 432 mb converted to bytes for use in size calculations.
pub const MB_432: usize = 452984832;

/// 433MB in bytes.
/// This constant represents 433 mb converted to bytes for use in size calculations.
pub const MB_433: usize = 454033408;

/// 434MB in bytes.
/// This constant represents 434 mb converted to bytes for use in size calculations.
pub const MB_434: usize = 455081984;

/// 435MB in bytes.
/// This constant represents 435 mb converted to bytes for use in size calculations.
pub const MB_435: usize = 456130560;

/// 436MB in bytes.
/// This constant represents 436 mb converted to bytes for use in size calculations.
pub const MB_436: usize = 457179136;

/// 437MB in bytes.
/// This constant represents 437 mb converted to bytes for use in size calculations.
pub const MB_437: usize = 458227712;

/// 438MB in bytes.
/// This constant represents 438 mb converted to bytes for use in size calculations.
pub const MB_438: usize = 459276288;

/// 439MB in bytes.
/// This constant represents 439 mb converted to bytes for use in size calculations.
pub const MB_439: usize = 460324864;

/// 440MB in bytes.
/// This constant represents 440 mb converted to bytes for use in size calculations.
pub const MB_440: usize = 461373440;

/// 441MB in bytes.
/// This constant represents 441 mb converted to bytes for use in size calculations.
pub const MB_441: usize = 462422016;

/// 442MB in bytes.
/// This constant represents 442 mb converted to bytes for use in size calculations.
pub const MB_442: usize = 463470592;

/// 443MB in bytes.
/// This constant represents 443 mb converted to bytes for use in size calculations.
pub const MB_443: usize = 464519168;

/// 444MB in bytes.
/// This constant represents 444 mb converted to bytes for use in size calculations.
pub const MB_444: usize = 465567744;

/// 445MB in bytes.
/// This constant represents 445 mb converted to bytes for use in size calculations.
pub const MB_445: usize = 466616320;

/// 446MB in bytes.
/// This constant represents 446 mb converted to bytes for use in size calculations.
pub const MB_446: usize = 467664896;

/// 447MB in bytes.
/// This constant represents 447 mb converted to bytes for use in size calculations.
pub const MB_447: usize = 468713472;

/// 448MB in bytes.
/// This constant represents 448 mb converted to bytes for use in size calculations.
pub const MB_448: usize = 469762048;

/// 449MB in bytes.
/// This constant represents 449 mb converted to bytes for use in size calculations.
pub const MB_449: usize = 470810624;

/// 450MB in bytes.
/// This constant represents 450 mb converted to bytes for use in size calculations.
pub const MB_450: usize = 471859200;

/// 451MB in bytes.
/// This constant represents 451 mb converted to bytes for use in size calculations.
pub const MB_451: usize = 472907776;

/// 452MB in bytes.
/// This constant represents 452 mb converted to bytes for use in size calculations.
pub const MB_452: usize = 473956352;

/// 453MB in bytes.
/// This constant represents 453 mb converted to bytes for use in size calculations.
pub const MB_453: usize = 475004928;

/// 454MB in bytes.
/// This constant represents 454 mb converted to bytes for use in size calculations.
pub const MB_454: usize = 476053504;

/// 455MB in bytes.
/// This constant represents 455 mb converted to bytes for use in size calculations.
pub const MB_455: usize = 477102080;

/// 456MB in bytes.
/// This constant represents 456 mb converted to bytes for use in size calculations.
pub const MB_456: usize = 478150656;

/// 457MB in bytes.
/// This constant represents 457 mb converted to bytes for use in size calculations.
pub const MB_457: usize = 479199232;

/// 458MB in bytes.
/// This constant represents 458 mb converted to bytes for use in size calculations.
pub const MB_458: usize = 480247808;

/// 459MB in bytes.
/// This constant represents 459 mb converted to bytes for use in size calculations.
pub const MB_459: usize = 481296384;

/// 460MB in bytes.
/// This constant represents 460 mb converted to bytes for use in size calculations.
pub const MB_460: usize = 482344960;

/// 461MB in bytes.
/// This constant represents 461 mb converted to bytes for use in size calculations.
pub const MB_461: usize = 483393536;

/// 462MB in bytes.
/// This constant represents 462 mb converted to bytes for use in size calculations.
pub const MB_462: usize = 484442112;

/// 463MB in bytes.
/// This constant represents 463 mb converted to bytes for use in size calculations.
pub const MB_463: usize = 485490688;

/// 464MB in bytes.
/// This constant represents 464 mb converted to bytes for use in size calculations.
pub const MB_464: usize = 486539264;

/// 465MB in bytes.
/// This constant represents 465 mb converted to bytes for use in size calculations.
pub const MB_465: usize = 487587840;

/// 466MB in bytes.
/// This constant represents 466 mb converted to bytes for use in size calculations.
pub const MB_466: usize = 488636416;

/// 467MB in bytes.
/// This constant represents 467 mb converted to bytes for use in size calculations.
pub const MB_467: usize = 489684992;

/// 468MB in bytes.
/// This constant represents 468 mb converted to bytes for use in size calculations.
pub const MB_468: usize = 490733568;

/// 469MB in bytes.
/// This constant represents 469 mb converted to bytes for use in size calculations.
pub const MB_469: usize = 491782144;

/// 470MB in bytes.
/// This constant represents 470 mb converted to bytes for use in size calculations.
pub const MB_470: usize = 492830720;

/// 471MB in bytes.
/// This constant represents 471 mb converted to bytes for use in size calculations.
pub const MB_471: usize = 493879296;

/// 472MB in bytes.
/// This constant represents 472 mb converted to bytes for use in size calculations.
pub const MB_472: usize = 494927872;

/// 473MB in bytes.
/// This constant represents 473 mb converted to bytes for use in size calculations.
pub const MB_473: usize = 495976448;

/// 474MB in bytes.
/// This constant represents 474 mb converted to bytes for use in size calculations.
pub const MB_474: usize = 497025024;

/// 475MB in bytes.
/// This constant represents 475 mb converted to bytes for use in size calculations.
pub const MB_475: usize = 498073600;

/// 476MB in bytes.
/// This constant represents 476 mb converted to bytes for use in size calculations.
pub const MB_476: usize = 499122176;

/// 477MB in bytes.
/// This constant represents 477 mb converted to bytes for use in size calculations.
pub const MB_477: usize = 500170752;

/// 478MB in bytes.
/// This constant represents 478 mb converted to bytes for use in size calculations.
pub const MB_478: usize = 501219328;

/// 479MB in bytes.
/// This constant represents 479 mb converted to bytes for use in size calculations.
pub const MB_479: usize = 502267904;

/// 480MB in bytes.
/// This constant represents 480 mb converted to bytes for use in size calculations.
pub const MB_480: usize = 503316480;

/// 481MB in bytes.
/// This constant represents 481 mb converted to bytes for use in size calculations.
pub const MB_481: usize = 504365056;

/// 482MB in bytes.
/// This constant represents 482 mb converted to bytes for use in size calculations.
pub const MB_482: usize = 505413632;

/// 483MB in bytes.
/// This constant represents 483 mb converted to bytes for use in size calculations.
pub const MB_483: usize = 506462208;

/// 484MB in bytes.
/// This constant represents 484 mb converted to bytes for use in size calculations.
pub const MB_484: usize = 507510784;

/// 485MB in bytes.
/// This constant represents 485 mb converted to bytes for use in size calculations.
pub const MB_485: usize = 508559360;

/// 486MB in bytes.
/// This constant represents 486 mb converted to bytes for use in size calculations.
pub const MB_486: usize = 509607936;

/// 487MB in bytes.
/// This constant represents 487 mb converted to bytes for use in size calculations.
pub const MB_487: usize = 510656512;

/// 488MB in bytes.
/// This constant represents 488 mb converted to bytes for use in size calculations.
pub const MB_488: usize = 511705088;

/// 489MB in bytes.
/// This constant represents 489 mb converted to bytes for use in size calculations.
pub const MB_489: usize = 512753664;

/// 490MB in bytes.
/// This constant represents 490 mb converted to bytes for use in size calculations.
pub const MB_490: usize = 513802240;

/// 491MB in bytes.
/// This constant represents 491 mb converted to bytes for use in size calculations.
pub const MB_491: usize = 514850816;

/// 492MB in bytes.
/// This constant represents 492 mb converted to bytes for use in size calculations.
pub const MB_492: usize = 515899392;

/// 493MB in bytes.
/// This constant represents 493 mb converted to bytes for use in size calculations.
pub const MB_493: usize = 516947968;

/// 494MB in bytes.
/// This constant represents 494 mb converted to bytes for use in size calculations.
pub const MB_494: usize = 517996544;

/// 495MB in bytes.
/// This constant represents 495 mb converted to bytes for use in size calculations.
pub const MB_495: usize = 519045120;

/// 496MB in bytes.
/// This constant represents 496 mb converted to bytes for use in size calculations.
pub const MB_496: usize = 520093696;

/// 497MB in bytes.
/// This constant represents 497 mb converted to bytes for use in size calculations.
pub const MB_497: usize = 521142272;

/// 498MB in bytes.
/// This constant represents 498 mb converted to bytes for use in size calculations.
pub const MB_498: usize = 522190848;

/// 499MB in bytes.
/// This constant represents 499 mb converted to bytes for use in size calculations.
pub const MB_499: usize = 523239424;

/// 500MB in bytes.
/// This constant represents 500 mb converted to bytes for use in size calculations.
pub const MB_500: usize = 524288000;

/// 501MB in bytes.
/// This constant represents 501 mb converted to bytes for use in size calculations.
pub const MB_501: usize = 525336576;

/// 502MB in bytes.
/// This constant represents 502 mb converted to bytes for use in size calculations.
pub const MB_502: usize = 526385152;

/// 503MB in bytes.
/// This constant represents 503 mb converted to bytes for use in size calculations.
pub const MB_503: usize = 527433728;

/// 504MB in bytes.
/// This constant represents 504 mb converted to bytes for use in size calculations.
pub const MB_504: usize = 528482304;

/// 505MB in bytes.
/// This constant represents 505 mb converted to bytes for use in size calculations.
pub const MB_505: usize = 529530880;

/// 506MB in bytes.
/// This constant represents 506 mb converted to bytes for use in size calculations.
pub const MB_506: usize = 530579456;

/// 507MB in bytes.
/// This constant represents 507 mb converted to bytes for use in size calculations.
pub const MB_507: usize = 531628032;

/// 508MB in bytes.
/// This constant represents 508 mb converted to bytes for use in size calculations.
pub const MB_508: usize = 532676608;

/// 509MB in bytes.
/// This constant represents 509 mb converted to bytes for use in size calculations.
pub const MB_509: usize = 533725184;

/// 510MB in bytes.
/// This constant represents 510 mb converted to bytes for use in size calculations.
pub const MB_510: usize = 534773760;

/// 511MB in bytes.
/// This constant represents 511 mb converted to bytes for use in size calculations.
pub const MB_511: usize = 535822336;

/// 512MB in bytes.
/// This constant represents 512 mb converted to bytes for use in size calculations.
pub const MB_512: usize = 536870912;

/// 513MB in bytes.
/// This constant represents 513 mb converted to bytes for use in size calculations.
pub const MB_513: usize = 537919488;

/// 514MB in bytes.
/// This constant represents 514 mb converted to bytes for use in size calculations.
pub const MB_514: usize = 538968064;

/// 515MB in bytes.
/// This constant represents 515 mb converted to bytes for use in size calculations.
pub const MB_515: usize = 540016640;

/// 516MB in bytes.
/// This constant represents 516 mb converted to bytes for use in size calculations.
pub const MB_516: usize = 541065216;

/// 517MB in bytes.
/// This constant represents 517 mb converted to bytes for use in size calculations.
pub const MB_517: usize = 542113792;

/// 518MB in bytes.
/// This constant represents 518 mb converted to bytes for use in size calculations.
pub const MB_518: usize = 543162368;

/// 519MB in bytes.
/// This constant represents 519 mb converted to bytes for use in size calculations.
pub const MB_519: usize = 544210944;

/// 520MB in bytes.
/// This constant represents 520 mb converted to bytes for use in size calculations.
pub const MB_520: usize = 545259520;

/// 521MB in bytes.
/// This constant represents 521 mb converted to bytes for use in size calculations.
pub const MB_521: usize = 546308096;

/// 522MB in bytes.
/// This constant represents 522 mb converted to bytes for use in size calculations.
pub const MB_522: usize = 547356672;

/// 523MB in bytes.
/// This constant represents 523 mb converted to bytes for use in size calculations.
pub const MB_523: usize = 548405248;

/// 524MB in bytes.
/// This constant represents 524 mb converted to bytes for use in size calculations.
pub const MB_524: usize = 549453824;

/// 525MB in bytes.
/// This constant represents 525 mb converted to bytes for use in size calculations.
pub const MB_525: usize = 550502400;

/// 526MB in bytes.
/// This constant represents 526 mb converted to bytes for use in size calculations.
pub const MB_526: usize = 551550976;

/// 527MB in bytes.
/// This constant represents 527 mb converted to bytes for use in size calculations.
pub const MB_527: usize = 552599552;

/// 528MB in bytes.
/// This constant represents 528 mb converted to bytes for use in size calculations.
pub const MB_528: usize = 553648128;

/// 529MB in bytes.
/// This constant represents 529 mb converted to bytes for use in size calculations.
pub const MB_529: usize = 554696704;

/// 530MB in bytes.
/// This constant represents 530 mb converted to bytes for use in size calculations.
pub const MB_530: usize = 555745280;

/// 531MB in bytes.
/// This constant represents 531 mb converted to bytes for use in size calculations.
pub const MB_531: usize = 556793856;

/// 532MB in bytes.
/// This constant represents 532 mb converted to bytes for use in size calculations.
pub const MB_532: usize = 557842432;

/// 533MB in bytes.
/// This constant represents 533 mb converted to bytes for use in size calculations.
pub const MB_533: usize = 558891008;

/// 534MB in bytes.
/// This constant represents 534 mb converted to bytes for use in size calculations.
pub const MB_534: usize = 559939584;

/// 535MB in bytes.
/// This constant represents 535 mb converted to bytes for use in size calculations.
pub const MB_535: usize = 560988160;

/// 536MB in bytes.
/// This constant represents 536 mb converted to bytes for use in size calculations.
pub const MB_536: usize = 562036736;

/// 537MB in bytes.
/// This constant represents 537 mb converted to bytes for use in size calculations.
pub const MB_537: usize = 563085312;

/// 538MB in bytes.
/// This constant represents 538 mb converted to bytes for use in size calculations.
pub const MB_538: usize = 564133888;

/// 539MB in bytes.
/// This constant represents 539 mb converted to bytes for use in size calculations.
pub const MB_539: usize = 565182464;

/// 540MB in bytes.
/// This constant represents 540 mb converted to bytes for use in size calculations.
pub const MB_540: usize = 566231040;

/// 541MB in bytes.
/// This constant represents 541 mb converted to bytes for use in size calculations.
pub const MB_541: usize = 567279616;

/// 542MB in bytes.
/// This constant represents 542 mb converted to bytes for use in size calculations.
pub const MB_542: usize = 568328192;

/// 543MB in bytes.
/// This constant represents 543 mb converted to bytes for use in size calculations.
pub const MB_543: usize = 569376768;

/// 544MB in bytes.
/// This constant represents 544 mb converted to bytes for use in size calculations.
pub const MB_544: usize = 570425344;

/// 545MB in bytes.
/// This constant represents 545 mb converted to bytes for use in size calculations.
pub const MB_545: usize = 571473920;

/// 546MB in bytes.
/// This constant represents 546 mb converted to bytes for use in size calculations.
pub const MB_546: usize = 572522496;

/// 547MB in bytes.
/// This constant represents 547 mb converted to bytes for use in size calculations.
pub const MB_547: usize = 573571072;

/// 548MB in bytes.
/// This constant represents 548 mb converted to bytes for use in size calculations.
pub const MB_548: usize = 574619648;

/// 549MB in bytes.
/// This constant represents 549 mb converted to bytes for use in size calculations.
pub const MB_549: usize = 575668224;

/// 550MB in bytes.
/// This constant represents 550 mb converted to bytes for use in size calculations.
pub const MB_550: usize = 576716800;

/// 551MB in bytes.
/// This constant represents 551 mb converted to bytes for use in size calculations.
pub const MB_551: usize = 577765376;

/// 552MB in bytes.
/// This constant represents 552 mb converted to bytes for use in size calculations.
pub const MB_552: usize = 578813952;

/// 553MB in bytes.
/// This constant represents 553 mb converted to bytes for use in size calculations.
pub const MB_553: usize = 579862528;

/// 554MB in bytes.
/// This constant represents 554 mb converted to bytes for use in size calculations.
pub const MB_554: usize = 580911104;

/// 555MB in bytes.
/// This constant represents 555 mb converted to bytes for use in size calculations.
pub const MB_555: usize = 581959680;

/// 556MB in bytes.
/// This constant represents 556 mb converted to bytes for use in size calculations.
pub const MB_556: usize = 583008256;

/// 557MB in bytes.
/// This constant represents 557 mb converted to bytes for use in size calculations.
pub const MB_557: usize = 584056832;

/// 558MB in bytes.
/// This constant represents 558 mb converted to bytes for use in size calculations.
pub const MB_558: usize = 585105408;

/// 559MB in bytes.
/// This constant represents 559 mb converted to bytes for use in size calculations.
pub const MB_559: usize = 586153984;

/// 560MB in bytes.
/// This constant represents 560 mb converted to bytes for use in size calculations.
pub const MB_560: usize = 587202560;

/// 561MB in bytes.
/// This constant represents 561 mb converted to bytes for use in size calculations.
pub const MB_561: usize = 588251136;

/// 562MB in bytes.
/// This constant represents 562 mb converted to bytes for use in size calculations.
pub const MB_562: usize = 589299712;

/// 563MB in bytes.
/// This constant represents 563 mb converted to bytes for use in size calculations.
pub const MB_563: usize = 590348288;

/// 564MB in bytes.
/// This constant represents 564 mb converted to bytes for use in size calculations.
pub const MB_564: usize = 591396864;

/// 565MB in bytes.
/// This constant represents 565 mb converted to bytes for use in size calculations.
pub const MB_565: usize = 592445440;

/// 566MB in bytes.
/// This constant represents 566 mb converted to bytes for use in size calculations.
pub const MB_566: usize = 593494016;

/// 567MB in bytes.
/// This constant represents 567 mb converted to bytes for use in size calculations.
pub const MB_567: usize = 594542592;

/// 568MB in bytes.
/// This constant represents 568 mb converted to bytes for use in size calculations.
pub const MB_568: usize = 595591168;

/// 569MB in bytes.
/// This constant represents 569 mb converted to bytes for use in size calculations.
pub const MB_569: usize = 596639744;

/// 570MB in bytes.
/// This constant represents 570 mb converted to bytes for use in size calculations.
pub const MB_570: usize = 597688320;

/// 571MB in bytes.
/// This constant represents 571 mb converted to bytes for use in size calculations.
pub const MB_571: usize = 598736896;

/// 572MB in bytes.
/// This constant represents 572 mb converted to bytes for use in size calculations.
pub const MB_572: usize = 599785472;

/// 573MB in bytes.
/// This constant represents 573 mb converted to bytes for use in size calculations.
pub const MB_573: usize = 600834048;

/// 574MB in bytes.
/// This constant represents 574 mb converted to bytes for use in size calculations.
pub const MB_574: usize = 601882624;

/// 575MB in bytes.
/// This constant represents 575 mb converted to bytes for use in size calculations.
pub const MB_575: usize = 602931200;

/// 576MB in bytes.
/// This constant represents 576 mb converted to bytes for use in size calculations.
pub const MB_576: usize = 603979776;

/// 577MB in bytes.
/// This constant represents 577 mb converted to bytes for use in size calculations.
pub const MB_577: usize = 605028352;

/// 578MB in bytes.
/// This constant represents 578 mb converted to bytes for use in size calculations.
pub const MB_578: usize = 606076928;

/// 579MB in bytes.
/// This constant represents 579 mb converted to bytes for use in size calculations.
pub const MB_579: usize = 607125504;

/// 580MB in bytes.
/// This constant represents 580 mb converted to bytes for use in size calculations.
pub const MB_580: usize = 608174080;

/// 581MB in bytes.
/// This constant represents 581 mb converted to bytes for use in size calculations.
pub const MB_581: usize = 609222656;

/// 582MB in bytes.
/// This constant represents 582 mb converted to bytes for use in size calculations.
pub const MB_582: usize = 610271232;

/// 583MB in bytes.
/// This constant represents 583 mb converted to bytes for use in size calculations.
pub const MB_583: usize = 611319808;

/// 584MB in bytes.
/// This constant represents 584 mb converted to bytes for use in size calculations.
pub const MB_584: usize = 612368384;

/// 585MB in bytes.
/// This constant represents 585 mb converted to bytes for use in size calculations.
pub const MB_585: usize = 613416960;

/// 586MB in bytes.
/// This constant represents 586 mb converted to bytes for use in size calculations.
pub const MB_586: usize = 614465536;

/// 587MB in bytes.
/// This constant represents 587 mb converted to bytes for use in size calculations.
pub const MB_587: usize = 615514112;

/// 588MB in bytes.
/// This constant represents 588 mb converted to bytes for use in size calculations.
pub const MB_588: usize = 616562688;

/// 589MB in bytes.
/// This constant represents 589 mb converted to bytes for use in size calculations.
pub const MB_589: usize = 617611264;

/// 590MB in bytes.
/// This constant represents 590 mb converted to bytes for use in size calculations.
pub const MB_590: usize = 618659840;

/// 591MB in bytes.
/// This constant represents 591 mb converted to bytes for use in size calculations.
pub const MB_591: usize = 619708416;

/// 592MB in bytes.
/// This constant represents 592 mb converted to bytes for use in size calculations.
pub const MB_592: usize = 620756992;

/// 593MB in bytes.
/// This constant represents 593 mb converted to bytes for use in size calculations.
pub const MB_593: usize = 621805568;

/// 594MB in bytes.
/// This constant represents 594 mb converted to bytes for use in size calculations.
pub const MB_594: usize = 622854144;

/// 595MB in bytes.
/// This constant represents 595 mb converted to bytes for use in size calculations.
pub const MB_595: usize = 623902720;

/// 596MB in bytes.
/// This constant represents 596 mb converted to bytes for use in size calculations.
pub const MB_596: usize = 624951296;

/// 597MB in bytes.
/// This constant represents 597 mb converted to bytes for use in size calculations.
pub const MB_597: usize = 625999872;

/// 598MB in bytes.
/// This constant represents 598 mb converted to bytes for use in size calculations.
pub const MB_598: usize = 627048448;

/// 599MB in bytes.
/// This constant represents 599 mb converted to bytes for use in size calculations.
pub const MB_599: usize = 628097024;

/// 600MB in bytes.
/// This constant represents 600 mb converted to bytes for use in size calculations.
pub const MB_600: usize = 629145600;

/// 601MB in bytes.
/// This constant represents 601 mb converted to bytes for use in size calculations.
pub const MB_601: usize = 630194176;

/// 602MB in bytes.
/// This constant represents 602 mb converted to bytes for use in size calculations.
pub const MB_602: usize = 631242752;

/// 603MB in bytes.
/// This constant represents 603 mb converted to bytes for use in size calculations.
pub const MB_603: usize = 632291328;

/// 604MB in bytes.
/// This constant represents 604 mb converted to bytes for use in size calculations.
pub const MB_604: usize = 633339904;

/// 605MB in bytes.
/// This constant represents 605 mb converted to bytes for use in size calculations.
pub const MB_605: usize = 634388480;

/// 606MB in bytes.
/// This constant represents 606 mb converted to bytes for use in size calculations.
pub const MB_606: usize = 635437056;

/// 607MB in bytes.
/// This constant represents 607 mb converted to bytes for use in size calculations.
pub const MB_607: usize = 636485632;

/// 608MB in bytes.
/// This constant represents 608 mb converted to bytes for use in size calculations.
pub const MB_608: usize = 637534208;

/// 609MB in bytes.
/// This constant represents 609 mb converted to bytes for use in size calculations.
pub const MB_609: usize = 638582784;

/// 610MB in bytes.
/// This constant represents 610 mb converted to bytes for use in size calculations.
pub const MB_610: usize = 639631360;

/// 611MB in bytes.
/// This constant represents 611 mb converted to bytes for use in size calculations.
pub const MB_611: usize = 640679936;

/// 612MB in bytes.
/// This constant represents 612 mb converted to bytes for use in size calculations.
pub const MB_612: usize = 641728512;

/// 613MB in bytes.
/// This constant represents 613 mb converted to bytes for use in size calculations.
pub const MB_613: usize = 642777088;

/// 614MB in bytes.
/// This constant represents 614 mb converted to bytes for use in size calculations.
pub const MB_614: usize = 643825664;

/// 615MB in bytes.
/// This constant represents 615 mb converted to bytes for use in size calculations.
pub const MB_615: usize = 644874240;

/// 616MB in bytes.
/// This constant represents 616 mb converted to bytes for use in size calculations.
pub const MB_616: usize = 645922816;

/// 617MB in bytes.
/// This constant represents 617 mb converted to bytes for use in size calculations.
pub const MB_617: usize = 646971392;

/// 618MB in bytes.
/// This constant represents 618 mb converted to bytes for use in size calculations.
pub const MB_618: usize = 648019968;

/// 619MB in bytes.
/// This constant represents 619 mb converted to bytes for use in size calculations.
pub const MB_619: usize = 649068544;

/// 620MB in bytes.
/// This constant represents 620 mb converted to bytes for use in size calculations.
pub const MB_620: usize = 650117120;

/// 621MB in bytes.
/// This constant represents 621 mb converted to bytes for use in size calculations.
pub const MB_621: usize = 651165696;

/// 622MB in bytes.
/// This constant represents 622 mb converted to bytes for use in size calculations.
pub const MB_622: usize = 652214272;

/// 623MB in bytes.
/// This constant represents 623 mb converted to bytes for use in size calculations.
pub const MB_623: usize = 653262848;

/// 624MB in bytes.
/// This constant represents 624 mb converted to bytes for use in size calculations.
pub const MB_624: usize = 654311424;

/// 625MB in bytes.
/// This constant represents 625 mb converted to bytes for use in size calculations.
pub const MB_625: usize = 655360000;

/// 626MB in bytes.
/// This constant represents 626 mb converted to bytes for use in size calculations.
pub const MB_626: usize = 656408576;

/// 627MB in bytes.
/// This constant represents 627 mb converted to bytes for use in size calculations.
pub const MB_627: usize = 657457152;

/// 628MB in bytes.
/// This constant represents 628 mb converted to bytes for use in size calculations.
pub const MB_628: usize = 658505728;

/// 629MB in bytes.
/// This constant represents 629 mb converted to bytes for use in size calculations.
pub const MB_629: usize = 659554304;

/// 630MB in bytes.
/// This constant represents 630 mb converted to bytes for use in size calculations.
pub const MB_630: usize = 660602880;

/// 631MB in bytes.
/// This constant represents 631 mb converted to bytes for use in size calculations.
pub const MB_631: usize = 661651456;

/// 632MB in bytes.
/// This constant represents 632 mb converted to bytes for use in size calculations.
pub const MB_632: usize = 662700032;

/// 633MB in bytes.
/// This constant represents 633 mb converted to bytes for use in size calculations.
pub const MB_633: usize = 663748608;

/// 634MB in bytes.
/// This constant represents 634 mb converted to bytes for use in size calculations.
pub const MB_634: usize = 664797184;

/// 635MB in bytes.
/// This constant represents 635 mb converted to bytes for use in size calculations.
pub const MB_635: usize = 665845760;

/// 636MB in bytes.
/// This constant represents 636 mb converted to bytes for use in size calculations.
pub const MB_636: usize = 666894336;

/// 637MB in bytes.
/// This constant represents 637 mb converted to bytes for use in size calculations.
pub const MB_637: usize = 667942912;

/// 638MB in bytes.
/// This constant represents 638 mb converted to bytes for use in size calculations.
pub const MB_638: usize = 668991488;

/// 639MB in bytes.
/// This constant represents 639 mb converted to bytes for use in size calculations.
pub const MB_639: usize = 670040064;

/// 640MB in bytes.
/// This constant represents 640 mb converted to bytes for use in size calculations.
pub const MB_640: usize = 671088640;

/// 641MB in bytes.
/// This constant represents 641 mb converted to bytes for use in size calculations.
pub const MB_641: usize = 672137216;

/// 642MB in bytes.
/// This constant represents 642 mb converted to bytes for use in size calculations.
pub const MB_642: usize = 673185792;

/// 643MB in bytes.
/// This constant represents 643 mb converted to bytes for use in size calculations.
pub const MB_643: usize = 674234368;

/// 644MB in bytes.
/// This constant represents 644 mb converted to bytes for use in size calculations.
pub const MB_644: usize = 675282944;

/// 645MB in bytes.
/// This constant represents 645 mb converted to bytes for use in size calculations.
pub const MB_645: usize = 676331520;

/// 646MB in bytes.
/// This constant represents 646 mb converted to bytes for use in size calculations.
pub const MB_646: usize = 677380096;

/// 647MB in bytes.
/// This constant represents 647 mb converted to bytes for use in size calculations.
pub const MB_647: usize = 678428672;

/// 648MB in bytes.
/// This constant represents 648 mb converted to bytes for use in size calculations.
pub const MB_648: usize = 679477248;

/// 649MB in bytes.
/// This constant represents 649 mb converted to bytes for use in size calculations.
pub const MB_649: usize = 680525824;

/// 650MB in bytes.
/// This constant represents 650 mb converted to bytes for use in size calculations.
pub const MB_650: usize = 681574400;

/// 651MB in bytes.
/// This constant represents 651 mb converted to bytes for use in size calculations.
pub const MB_651: usize = 682622976;

/// 652MB in bytes.
/// This constant represents 652 mb converted to bytes for use in size calculations.
pub const MB_652: usize = 683671552;

/// 653MB in bytes.
/// This constant represents 653 mb converted to bytes for use in size calculations.
pub const MB_653: usize = 684720128;

/// 654MB in bytes.
/// This constant represents 654 mb converted to bytes for use in size calculations.
pub const MB_654: usize = 685768704;

/// 655MB in bytes.
/// This constant represents 655 mb converted to bytes for use in size calculations.
pub const MB_655: usize = 686817280;

/// 656MB in bytes.
/// This constant represents 656 mb converted to bytes for use in size calculations.
pub const MB_656: usize = 687865856;

/// 657MB in bytes.
/// This constant represents 657 mb converted to bytes for use in size calculations.
pub const MB_657: usize = 688914432;

/// 658MB in bytes.
/// This constant represents 658 mb converted to bytes for use in size calculations.
pub const MB_658: usize = 689963008;

/// 659MB in bytes.
/// This constant represents 659 mb converted to bytes for use in size calculations.
pub const MB_659: usize = 691011584;

/// 660MB in bytes.
/// This constant represents 660 mb converted to bytes for use in size calculations.
pub const MB_660: usize = 692060160;

/// 661MB in bytes.
/// This constant represents 661 mb converted to bytes for use in size calculations.
pub const MB_661: usize = 693108736;

/// 662MB in bytes.
/// This constant represents 662 mb converted to bytes for use in size calculations.
pub const MB_662: usize = 694157312;

/// 663MB in bytes.
/// This constant represents 663 mb converted to bytes for use in size calculations.
pub const MB_663: usize = 695205888;

/// 664MB in bytes.
/// This constant represents 664 mb converted to bytes for use in size calculations.
pub const MB_664: usize = 696254464;

/// 665MB in bytes.
/// This constant represents 665 mb converted to bytes for use in size calculations.
pub const MB_665: usize = 697303040;

/// 666MB in bytes.
/// This constant represents 666 mb converted to bytes for use in size calculations.
pub const MB_666: usize = 698351616;

/// 667MB in bytes.
/// This constant represents 667 mb converted to bytes for use in size calculations.
pub const MB_667: usize = 699400192;

/// 668MB in bytes.
/// This constant represents 668 mb converted to bytes for use in size calculations.
pub const MB_668: usize = 700448768;

/// 669MB in bytes.
/// This constant represents 669 mb converted to bytes for use in size calculations.
pub const MB_669: usize = 701497344;

/// 670MB in bytes.
/// This constant represents 670 mb converted to bytes for use in size calculations.
pub const MB_670: usize = 702545920;

/// 671MB in bytes.
/// This constant represents 671 mb converted to bytes for use in size calculations.
pub const MB_671: usize = 703594496;

/// 672MB in bytes.
/// This constant represents 672 mb converted to bytes for use in size calculations.
pub const MB_672: usize = 704643072;

/// 673MB in bytes.
/// This constant represents 673 mb converted to bytes for use in size calculations.
pub const MB_673: usize = 705691648;

/// 674MB in bytes.
/// This constant represents 674 mb converted to bytes for use in size calculations.
pub const MB_674: usize = 706740224;

/// 675MB in bytes.
/// This constant represents 675 mb converted to bytes for use in size calculations.
pub const MB_675: usize = 707788800;

/// 676MB in bytes.
/// This constant represents 676 mb converted to bytes for use in size calculations.
pub const MB_676: usize = 708837376;

/// 677MB in bytes.
/// This constant represents 677 mb converted to bytes for use in size calculations.
pub const MB_677: usize = 709885952;

/// 678MB in bytes.
/// This constant represents 678 mb converted to bytes for use in size calculations.
pub const MB_678: usize = 710934528;

/// 679MB in bytes.
/// This constant represents 679 mb converted to bytes for use in size calculations.
pub const MB_679: usize = 711983104;

/// 680MB in bytes.
/// This constant represents 680 mb converted to bytes for use in size calculations.
pub const MB_680: usize = 713031680;

/// 681MB in bytes.
/// This constant represents 681 mb converted to bytes for use in size calculations.
pub const MB_681: usize = 714080256;

/// 682MB in bytes.
/// This constant represents 682 mb converted to bytes for use in size calculations.
pub const MB_682: usize = 715128832;

/// 683MB in bytes.
/// This constant represents 683 mb converted to bytes for use in size calculations.
pub const MB_683: usize = 716177408;

/// 684MB in bytes.
/// This constant represents 684 mb converted to bytes for use in size calculations.
pub const MB_684: usize = 717225984;

/// 685MB in bytes.
/// This constant represents 685 mb converted to bytes for use in size calculations.
pub const MB_685: usize = 718274560;

/// 686MB in bytes.
/// This constant represents 686 mb converted to bytes for use in size calculations.
pub const MB_686: usize = 719323136;

/// 687MB in bytes.
/// This constant represents 687 mb converted to bytes for use in size calculations.
pub const MB_687: usize = 720371712;

/// 688MB in bytes.
/// This constant represents 688 mb converted to bytes for use in size calculations.
pub const MB_688: usize = 721420288;

/// 689MB in bytes.
/// This constant represents 689 mb converted to bytes for use in size calculations.
pub const MB_689: usize = 722468864;

/// 690MB in bytes.
/// This constant represents 690 mb converted to bytes for use in size calculations.
pub const MB_690: usize = 723517440;

/// 691MB in bytes.
/// This constant represents 691 mb converted to bytes for use in size calculations.
pub const MB_691: usize = 724566016;

/// 692MB in bytes.
/// This constant represents 692 mb converted to bytes for use in size calculations.
pub const MB_692: usize = 725614592;

/// 693MB in bytes.
/// This constant represents 693 mb converted to bytes for use in size calculations.
pub const MB_693: usize = 726663168;

/// 694MB in bytes.
/// This constant represents 694 mb converted to bytes for use in size calculations.
pub const MB_694: usize = 727711744;

/// 695MB in bytes.
/// This constant represents 695 mb converted to bytes for use in size calculations.
pub const MB_695: usize = 728760320;

/// 696MB in bytes.
/// This constant represents 696 mb converted to bytes for use in size calculations.
pub const MB_696: usize = 729808896;

/// 697MB in bytes.
/// This constant represents 697 mb converted to bytes for use in size calculations.
pub const MB_697: usize = 730857472;

/// 698MB in bytes.
/// This constant represents 698 mb converted to bytes for use in size calculations.
pub const MB_698: usize = 731906048;

/// 699MB in bytes.
/// This constant represents 699 mb converted to bytes for use in size calculations.
pub const MB_699: usize = 732954624;

/// 700MB in bytes.
/// This constant represents 700 mb converted to bytes for use in size calculations.
pub const MB_700: usize = 734003200;

/// 701MB in bytes.
/// This constant represents 701 mb converted to bytes for use in size calculations.
pub const MB_701: usize = 735051776;

/// 702MB in bytes.
/// This constant represents 702 mb converted to bytes for use in size calculations.
pub const MB_702: usize = 736100352;

/// 703MB in bytes.
/// This constant represents 703 mb converted to bytes for use in size calculations.
pub const MB_703: usize = 737148928;

/// 704MB in bytes.
/// This constant represents 704 mb converted to bytes for use in size calculations.
pub const MB_704: usize = 738197504;

/// 705MB in bytes.
/// This constant represents 705 mb converted to bytes for use in size calculations.
pub const MB_705: usize = 739246080;

/// 706MB in bytes.
/// This constant represents 706 mb converted to bytes for use in size calculations.
pub const MB_706: usize = 740294656;

/// 707MB in bytes.
/// This constant represents 707 mb converted to bytes for use in size calculations.
pub const MB_707: usize = 741343232;

/// 708MB in bytes.
/// This constant represents 708 mb converted to bytes for use in size calculations.
pub const MB_708: usize = 742391808;

/// 709MB in bytes.
/// This constant represents 709 mb converted to bytes for use in size calculations.
pub const MB_709: usize = 743440384;

/// 710MB in bytes.
/// This constant represents 710 mb converted to bytes for use in size calculations.
pub const MB_710: usize = 744488960;

/// 711MB in bytes.
/// This constant represents 711 mb converted to bytes for use in size calculations.
pub const MB_711: usize = 745537536;

/// 712MB in bytes.
/// This constant represents 712 mb converted to bytes for use in size calculations.
pub const MB_712: usize = 746586112;

/// 713MB in bytes.
/// This constant represents 713 mb converted to bytes for use in size calculations.
pub const MB_713: usize = 747634688;

/// 714MB in bytes.
/// This constant represents 714 mb converted to bytes for use in size calculations.
pub const MB_714: usize = 748683264;

/// 715MB in bytes.
/// This constant represents 715 mb converted to bytes for use in size calculations.
pub const MB_715: usize = 749731840;

/// 716MB in bytes.
/// This constant represents 716 mb converted to bytes for use in size calculations.
pub const MB_716: usize = 750780416;

/// 717MB in bytes.
/// This constant represents 717 mb converted to bytes for use in size calculations.
pub const MB_717: usize = 751828992;

/// 718MB in bytes.
/// This constant represents 718 mb converted to bytes for use in size calculations.
pub const MB_718: usize = 752877568;

/// 719MB in bytes.
/// This constant represents 719 mb converted to bytes for use in size calculations.
pub const MB_719: usize = 753926144;

/// 720MB in bytes.
/// This constant represents 720 mb converted to bytes for use in size calculations.
pub const MB_720: usize = 754974720;

/// 721MB in bytes.
/// This constant represents 721 mb converted to bytes for use in size calculations.
pub const MB_721: usize = 756023296;

/// 722MB in bytes.
/// This constant represents 722 mb converted to bytes for use in size calculations.
pub const MB_722: usize = 757071872;

/// 723MB in bytes.
/// This constant represents 723 mb converted to bytes for use in size calculations.
pub const MB_723: usize = 758120448;

/// 724MB in bytes.
/// This constant represents 724 mb converted to bytes for use in size calculations.
pub const MB_724: usize = 759169024;

/// 725MB in bytes.
/// This constant represents 725 mb converted to bytes for use in size calculations.
pub const MB_725: usize = 760217600;

/// 726MB in bytes.
/// This constant represents 726 mb converted to bytes for use in size calculations.
pub const MB_726: usize = 761266176;

/// 727MB in bytes.
/// This constant represents 727 mb converted to bytes for use in size calculations.
pub const MB_727: usize = 762314752;

/// 728MB in bytes.
/// This constant represents 728 mb converted to bytes for use in size calculations.
pub const MB_728: usize = 763363328;

/// 729MB in bytes.
/// This constant represents 729 mb converted to bytes for use in size calculations.
pub const MB_729: usize = 764411904;

/// 730MB in bytes.
/// This constant represents 730 mb converted to bytes for use in size calculations.
pub const MB_730: usize = 765460480;

/// 731MB in bytes.
/// This constant represents 731 mb converted to bytes for use in size calculations.
pub const MB_731: usize = 766509056;

/// 732MB in bytes.
/// This constant represents 732 mb converted to bytes for use in size calculations.
pub const MB_732: usize = 767557632;

/// 733MB in bytes.
/// This constant represents 733 mb converted to bytes for use in size calculations.
pub const MB_733: usize = 768606208;

/// 734MB in bytes.
/// This constant represents 734 mb converted to bytes for use in size calculations.
pub const MB_734: usize = 769654784;

/// 735MB in bytes.
/// This constant represents 735 mb converted to bytes for use in size calculations.
pub const MB_735: usize = 770703360;

/// 736MB in bytes.
/// This constant represents 736 mb converted to bytes for use in size calculations.
pub const MB_736: usize = 771751936;

/// 737MB in bytes.
/// This constant represents 737 mb converted to bytes for use in size calculations.
pub const MB_737: usize = 772800512;

/// 738MB in bytes.
/// This constant represents 738 mb converted to bytes for use in size calculations.
pub const MB_738: usize = 773849088;

/// 739MB in bytes.
/// This constant represents 739 mb converted to bytes for use in size calculations.
pub const MB_739: usize = 774897664;

/// 740MB in bytes.
/// This constant represents 740 mb converted to bytes for use in size calculations.
pub const MB_740: usize = 775946240;

/// 741MB in bytes.
/// This constant represents 741 mb converted to bytes for use in size calculations.
pub const MB_741: usize = 776994816;

/// 742MB in bytes.
/// This constant represents 742 mb converted to bytes for use in size calculations.
pub const MB_742: usize = 778043392;

/// 743MB in bytes.
/// This constant represents 743 mb converted to bytes for use in size calculations.
pub const MB_743: usize = 779091968;

/// 744MB in bytes.
/// This constant represents 744 mb converted to bytes for use in size calculations.
pub const MB_744: usize = 780140544;

/// 745MB in bytes.
/// This constant represents 745 mb converted to bytes for use in size calculations.
pub const MB_745: usize = 781189120;

/// 746MB in bytes.
/// This constant represents 746 mb converted to bytes for use in size calculations.
pub const MB_746: usize = 782237696;

/// 747MB in bytes.
/// This constant represents 747 mb converted to bytes for use in size calculations.
pub const MB_747: usize = 783286272;

/// 748MB in bytes.
/// This constant represents 748 mb converted to bytes for use in size calculations.
pub const MB_748: usize = 784334848;

/// 749MB in bytes.
/// This constant represents 749 mb converted to bytes for use in size calculations.
pub const MB_749: usize = 785383424;

/// 750MB in bytes.
/// This constant represents 750 mb converted to bytes for use in size calculations.
pub const MB_750: usize = 786432000;

/// 751MB in bytes.
/// This constant represents 751 mb converted to bytes for use in size calculations.
pub const MB_751: usize = 787480576;

/// 752MB in bytes.
/// This constant represents 752 mb converted to bytes for use in size calculations.
pub const MB_752: usize = 788529152;

/// 753MB in bytes.
/// This constant represents 753 mb converted to bytes for use in size calculations.
pub const MB_753: usize = 789577728;

/// 754MB in bytes.
/// This constant represents 754 mb converted to bytes for use in size calculations.
pub const MB_754: usize = 790626304;

/// 755MB in bytes.
/// This constant represents 755 mb converted to bytes for use in size calculations.
pub const MB_755: usize = 791674880;

/// 756MB in bytes.
/// This constant represents 756 mb converted to bytes for use in size calculations.
pub const MB_756: usize = 792723456;

/// 757MB in bytes.
/// This constant represents 757 mb converted to bytes for use in size calculations.
pub const MB_757: usize = 793772032;

/// 758MB in bytes.
/// This constant represents 758 mb converted to bytes for use in size calculations.
pub const MB_758: usize = 794820608;

/// 759MB in bytes.
/// This constant represents 759 mb converted to bytes for use in size calculations.
pub const MB_759: usize = 795869184;

/// 760MB in bytes.
/// This constant represents 760 mb converted to bytes for use in size calculations.
pub const MB_760: usize = 796917760;

/// 761MB in bytes.
/// This constant represents 761 mb converted to bytes for use in size calculations.
pub const MB_761: usize = 797966336;

/// 762MB in bytes.
/// This constant represents 762 mb converted to bytes for use in size calculations.
pub const MB_762: usize = 799014912;

/// 763MB in bytes.
/// This constant represents 763 mb converted to bytes for use in size calculations.
pub const MB_763: usize = 800063488;

/// 764MB in bytes.
/// This constant represents 764 mb converted to bytes for use in size calculations.
pub const MB_764: usize = 801112064;

/// 765MB in bytes.
/// This constant represents 765 mb converted to bytes for use in size calculations.
pub const MB_765: usize = 802160640;

/// 766MB in bytes.
/// This constant represents 766 mb converted to bytes for use in size calculations.
pub const MB_766: usize = 803209216;

/// 767MB in bytes.
/// This constant represents 767 mb converted to bytes for use in size calculations.
pub const MB_767: usize = 804257792;

/// 768MB in bytes.
/// This constant represents 768 mb converted to bytes for use in size calculations.
pub const MB_768: usize = 805306368;

/// 769MB in bytes.
/// This constant represents 769 mb converted to bytes for use in size calculations.
pub const MB_769: usize = 806354944;

/// 770MB in bytes.
/// This constant represents 770 mb converted to bytes for use in size calculations.
pub const MB_770: usize = 807403520;

/// 771MB in bytes.
/// This constant represents 771 mb converted to bytes for use in size calculations.
pub const MB_771: usize = 808452096;

/// 772MB in bytes.
/// This constant represents 772 mb converted to bytes for use in size calculations.
pub const MB_772: usize = 809500672;

/// 773MB in bytes.
/// This constant represents 773 mb converted to bytes for use in size calculations.
pub const MB_773: usize = 810549248;

/// 774MB in bytes.
/// This constant represents 774 mb converted to bytes for use in size calculations.
pub const MB_774: usize = 811597824;

/// 775MB in bytes.
/// This constant represents 775 mb converted to bytes for use in size calculations.
pub const MB_775: usize = 812646400;

/// 776MB in bytes.
/// This constant represents 776 mb converted to bytes for use in size calculations.
pub const MB_776: usize = 813694976;

/// 777MB in bytes.
/// This constant represents 777 mb converted to bytes for use in size calculations.
pub const MB_777: usize = 814743552;

/// 778MB in bytes.
/// This constant represents 778 mb converted to bytes for use in size calculations.
pub const MB_778: usize = 815792128;

/// 779MB in bytes.
/// This constant represents 779 mb converted to bytes for use in size calculations.
pub const MB_779: usize = 816840704;

/// 780MB in bytes.
/// This constant represents 780 mb converted to bytes for use in size calculations.
pub const MB_780: usize = 817889280;

/// 781MB in bytes.
/// This constant represents 781 mb converted to bytes for use in size calculations.
pub const MB_781: usize = 818937856;

/// 782MB in bytes.
/// This constant represents 782 mb converted to bytes for use in size calculations.
pub const MB_782: usize = 819986432;

/// 783MB in bytes.
/// This constant represents 783 mb converted to bytes for use in size calculations.
pub const MB_783: usize = 821035008;

/// 784MB in bytes.
/// This constant represents 784 mb converted to bytes for use in size calculations.
pub const MB_784: usize = 822083584;

/// 785MB in bytes.
/// This constant represents 785 mb converted to bytes for use in size calculations.
pub const MB_785: usize = 823132160;

/// 786MB in bytes.
/// This constant represents 786 mb converted to bytes for use in size calculations.
pub const MB_786: usize = 824180736;

/// 787MB in bytes.
/// This constant represents 787 mb converted to bytes for use in size calculations.
pub const MB_787: usize = 825229312;

/// 788MB in bytes.
/// This constant represents 788 mb converted to bytes for use in size calculations.
pub const MB_788: usize = 826277888;

/// 789MB in bytes.
/// This constant represents 789 mb converted to bytes for use in size calculations.
pub const MB_789: usize = 827326464;

/// 790MB in bytes.
/// This constant represents 790 mb converted to bytes for use in size calculations.
pub const MB_790: usize = 828375040;

/// 791MB in bytes.
/// This constant represents 791 mb converted to bytes for use in size calculations.
pub const MB_791: usize = 829423616;

/// 792MB in bytes.
/// This constant represents 792 mb converted to bytes for use in size calculations.
pub const MB_792: usize = 830472192;

/// 793MB in bytes.
/// This constant represents 793 mb converted to bytes for use in size calculations.
pub const MB_793: usize = 831520768;

/// 794MB in bytes.
/// This constant represents 794 mb converted to bytes for use in size calculations.
pub const MB_794: usize = 832569344;

/// 795MB in bytes.
/// This constant represents 795 mb converted to bytes for use in size calculations.
pub const MB_795: usize = 833617920;

/// 796MB in bytes.
/// This constant represents 796 mb converted to bytes for use in size calculations.
pub const MB_796: usize = 834666496;

/// 797MB in bytes.
/// This constant represents 797 mb converted to bytes for use in size calculations.
pub const MB_797: usize = 835715072;

/// 798MB in bytes.
/// This constant represents 798 mb converted to bytes for use in size calculations.
pub const MB_798: usize = 836763648;

/// 799MB in bytes.
/// This constant represents 799 mb converted to bytes for use in size calculations.
pub const MB_799: usize = 837812224;

/// 800MB in bytes.
/// This constant represents 800 mb converted to bytes for use in size calculations.
pub const MB_800: usize = 838860800;

/// 801MB in bytes.
/// This constant represents 801 mb converted to bytes for use in size calculations.
pub const MB_801: usize = 839909376;

/// 802MB in bytes.
/// This constant represents 802 mb converted to bytes for use in size calculations.
pub const MB_802: usize = 840957952;

/// 803MB in bytes.
/// This constant represents 803 mb converted to bytes for use in size calculations.
pub const MB_803: usize = 842006528;

/// 804MB in bytes.
/// This constant represents 804 mb converted to bytes for use in size calculations.
pub const MB_804: usize = 843055104;

/// 805MB in bytes.
/// This constant represents 805 mb converted to bytes for use in size calculations.
pub const MB_805: usize = 844103680;

/// 806MB in bytes.
/// This constant represents 806 mb converted to bytes for use in size calculations.
pub const MB_806: usize = 845152256;

/// 807MB in bytes.
/// This constant represents 807 mb converted to bytes for use in size calculations.
pub const MB_807: usize = 846200832;

/// 808MB in bytes.
/// This constant represents 808 mb converted to bytes for use in size calculations.
pub const MB_808: usize = 847249408;

/// 809MB in bytes.
/// This constant represents 809 mb converted to bytes for use in size calculations.
pub const MB_809: usize = 848297984;

/// 810MB in bytes.
/// This constant represents 810 mb converted to bytes for use in size calculations.
pub const MB_810: usize = 849346560;

/// 811MB in bytes.
/// This constant represents 811 mb converted to bytes for use in size calculations.
pub const MB_811: usize = 850395136;

/// 812MB in bytes.
/// This constant represents 812 mb converted to bytes for use in size calculations.
pub const MB_812: usize = 851443712;

/// 813MB in bytes.
/// This constant represents 813 mb converted to bytes for use in size calculations.
pub const MB_813: usize = 852492288;

/// 814MB in bytes.
/// This constant represents 814 mb converted to bytes for use in size calculations.
pub const MB_814: usize = 853540864;

/// 815MB in bytes.
/// This constant represents 815 mb converted to bytes for use in size calculations.
pub const MB_815: usize = 854589440;

/// 816MB in bytes.
/// This constant represents 816 mb converted to bytes for use in size calculations.
pub const MB_816: usize = 855638016;

/// 817MB in bytes.
/// This constant represents 817 mb converted to bytes for use in size calculations.
pub const MB_817: usize = 856686592;

/// 818MB in bytes.
/// This constant represents 818 mb converted to bytes for use in size calculations.
pub const MB_818: usize = 857735168;

/// 819MB in bytes.
/// This constant represents 819 mb converted to bytes for use in size calculations.
pub const MB_819: usize = 858783744;

/// 820MB in bytes.
/// This constant represents 820 mb converted to bytes for use in size calculations.
pub const MB_820: usize = 859832320;

/// 821MB in bytes.
/// This constant represents 821 mb converted to bytes for use in size calculations.
pub const MB_821: usize = 860880896;

/// 822MB in bytes.
/// This constant represents 822 mb converted to bytes for use in size calculations.
pub const MB_822: usize = 861929472;

/// 823MB in bytes.
/// This constant represents 823 mb converted to bytes for use in size calculations.
pub const MB_823: usize = 862978048;

/// 824MB in bytes.
/// This constant represents 824 mb converted to bytes for use in size calculations.
pub const MB_824: usize = 864026624;

/// 825MB in bytes.
/// This constant represents 825 mb converted to bytes for use in size calculations.
pub const MB_825: usize = 865075200;

/// 826MB in bytes.
/// This constant represents 826 mb converted to bytes for use in size calculations.
pub const MB_826: usize = 866123776;

/// 827MB in bytes.
/// This constant represents 827 mb converted to bytes for use in size calculations.
pub const MB_827: usize = 867172352;

/// 828MB in bytes.
/// This constant represents 828 mb converted to bytes for use in size calculations.
pub const MB_828: usize = 868220928;

/// 829MB in bytes.
/// This constant represents 829 mb converted to bytes for use in size calculations.
pub const MB_829: usize = 869269504;

/// 830MB in bytes.
/// This constant represents 830 mb converted to bytes for use in size calculations.
pub const MB_830: usize = 870318080;

/// 831MB in bytes.
/// This constant represents 831 mb converted to bytes for use in size calculations.
pub const MB_831: usize = 871366656;

/// 832MB in bytes.
/// This constant represents 832 mb converted to bytes for use in size calculations.
pub const MB_832: usize = 872415232;

/// 833MB in bytes.
/// This constant represents 833 mb converted to bytes for use in size calculations.
pub const MB_833: usize = 873463808;

/// 834MB in bytes.
/// This constant represents 834 mb converted to bytes for use in size calculations.
pub const MB_834: usize = 874512384;

/// 835MB in bytes.
/// This constant represents 835 mb converted to bytes for use in size calculations.
pub const MB_835: usize = 875560960;

/// 836MB in bytes.
/// This constant represents 836 mb converted to bytes for use in size calculations.
pub const MB_836: usize = 876609536;

/// 837MB in bytes.
/// This constant represents 837 mb converted to bytes for use in size calculations.
pub const MB_837: usize = 877658112;

/// 838MB in bytes.
/// This constant represents 838 mb converted to bytes for use in size calculations.
pub const MB_838: usize = 878706688;

/// 839MB in bytes.
/// This constant represents 839 mb converted to bytes for use in size calculations.
pub const MB_839: usize = 879755264;

/// 840MB in bytes.
/// This constant represents 840 mb converted to bytes for use in size calculations.
pub const MB_840: usize = 880803840;

/// 841MB in bytes.
/// This constant represents 841 mb converted to bytes for use in size calculations.
pub const MB_841: usize = 881852416;

/// 842MB in bytes.
/// This constant represents 842 mb converted to bytes for use in size calculations.
pub const MB_842: usize = 882900992;

/// 843MB in bytes.
/// This constant represents 843 mb converted to bytes for use in size calculations.
pub const MB_843: usize = 883949568;

/// 844MB in bytes.
/// This constant represents 844 mb converted to bytes for use in size calculations.
pub const MB_844: usize = 884998144;

/// 845MB in bytes.
/// This constant represents 845 mb converted to bytes for use in size calculations.
pub const MB_845: usize = 886046720;

/// 846MB in bytes.
/// This constant represents 846 mb converted to bytes for use in size calculations.
pub const MB_846: usize = 887095296;

/// 847MB in bytes.
/// This constant represents 847 mb converted to bytes for use in size calculations.
pub const MB_847: usize = 888143872;

/// 848MB in bytes.
/// This constant represents 848 mb converted to bytes for use in size calculations.
pub const MB_848: usize = 889192448;

/// 849MB in bytes.
/// This constant represents 849 mb converted to bytes for use in size calculations.
pub const MB_849: usize = 890241024;

/// 850MB in bytes.
/// This constant represents 850 mb converted to bytes for use in size calculations.
pub const MB_850: usize = 891289600;

/// 851MB in bytes.
/// This constant represents 851 mb converted to bytes for use in size calculations.
pub const MB_851: usize = 892338176;

/// 852MB in bytes.
/// This constant represents 852 mb converted to bytes for use in size calculations.
pub const MB_852: usize = 893386752;

/// 853MB in bytes.
/// This constant represents 853 mb converted to bytes for use in size calculations.
pub const MB_853: usize = 894435328;

/// 854MB in bytes.
/// This constant represents 854 mb converted to bytes for use in size calculations.
pub const MB_854: usize = 895483904;

/// 855MB in bytes.
/// This constant represents 855 mb converted to bytes for use in size calculations.
pub const MB_855: usize = 896532480;

/// 856MB in bytes.
/// This constant represents 856 mb converted to bytes for use in size calculations.
pub const MB_856: usize = 897581056;

/// 857MB in bytes.
/// This constant represents 857 mb converted to bytes for use in size calculations.
pub const MB_857: usize = 898629632;

/// 858MB in bytes.
/// This constant represents 858 mb converted to bytes for use in size calculations.
pub const MB_858: usize = 899678208;

/// 859MB in bytes.
/// This constant represents 859 mb converted to bytes for use in size calculations.
pub const MB_859: usize = 900726784;

/// 860MB in bytes.
/// This constant represents 860 mb converted to bytes for use in size calculations.
pub const MB_860: usize = 901775360;

/// 861MB in bytes.
/// This constant represents 861 mb converted to bytes for use in size calculations.
pub const MB_861: usize = 902823936;

/// 862MB in bytes.
/// This constant represents 862 mb converted to bytes for use in size calculations.
pub const MB_862: usize = 903872512;

/// 863MB in bytes.
/// This constant represents 863 mb converted to bytes for use in size calculations.
pub const MB_863: usize = 904921088;

/// 864MB in bytes.
/// This constant represents 864 mb converted to bytes for use in size calculations.
pub const MB_864: usize = 905969664;

/// 865MB in bytes.
/// This constant represents 865 mb converted to bytes for use in size calculations.
pub const MB_865: usize = 907018240;

/// 866MB in bytes.
/// This constant represents 866 mb converted to bytes for use in size calculations.
pub const MB_866: usize = 908066816;

/// 867MB in bytes.
/// This constant represents 867 mb converted to bytes for use in size calculations.
pub const MB_867: usize = 909115392;

/// 868MB in bytes.
/// This constant represents 868 mb converted to bytes for use in size calculations.
pub const MB_868: usize = 910163968;

/// 869MB in bytes.
/// This constant represents 869 mb converted to bytes for use in size calculations.
pub const MB_869: usize = 911212544;

/// 870MB in bytes.
/// This constant represents 870 mb converted to bytes for use in size calculations.
pub const MB_870: usize = 912261120;

/// 871MB in bytes.
/// This constant represents 871 mb converted to bytes for use in size calculations.
pub const MB_871: usize = 913309696;

/// 872MB in bytes.
/// This constant represents 872 mb converted to bytes for use in size calculations.
pub const MB_872: usize = 914358272;

/// 873MB in bytes.
/// This constant represents 873 mb converted to bytes for use in size calculations.
pub const MB_873: usize = 915406848;

/// 874MB in bytes.
/// This constant represents 874 mb converted to bytes for use in size calculations.
pub const MB_874: usize = 916455424;

/// 875MB in bytes.
/// This constant represents 875 mb converted to bytes for use in size calculations.
pub const MB_875: usize = 917504000;

/// 876MB in bytes.
/// This constant represents 876 mb converted to bytes for use in size calculations.
pub const MB_876: usize = 918552576;

/// 877MB in bytes.
/// This constant represents 877 mb converted to bytes for use in size calculations.
pub const MB_877: usize = 919601152;

/// 878MB in bytes.
/// This constant represents 878 mb converted to bytes for use in size calculations.
pub const MB_878: usize = 920649728;

/// 879MB in bytes.
/// This constant represents 879 mb converted to bytes for use in size calculations.
pub const MB_879: usize = 921698304;

/// 880MB in bytes.
/// This constant represents 880 mb converted to bytes for use in size calculations.
pub const MB_880: usize = 922746880;

/// 881MB in bytes.
/// This constant represents 881 mb converted to bytes for use in size calculations.
pub const MB_881: usize = 923795456;

/// 882MB in bytes.
/// This constant represents 882 mb converted to bytes for use in size calculations.
pub const MB_882: usize = 924844032;

/// 883MB in bytes.
/// This constant represents 883 mb converted to bytes for use in size calculations.
pub const MB_883: usize = 925892608;

/// 884MB in bytes.
/// This constant represents 884 mb converted to bytes for use in size calculations.
pub const MB_884: usize = 926941184;

/// 885MB in bytes.
/// This constant represents 885 mb converted to bytes for use in size calculations.
pub const MB_885: usize = 927989760;

/// 886MB in bytes.
/// This constant represents 886 mb converted to bytes for use in size calculations.
pub const MB_886: usize = 929038336;

/// 887MB in bytes.
/// This constant represents 887 mb converted to bytes for use in size calculations.
pub const MB_887: usize = 930086912;

/// 888MB in bytes.
/// This constant represents 888 mb converted to bytes for use in size calculations.
pub const MB_888: usize = 931135488;

/// 889MB in bytes.
/// This constant represents 889 mb converted to bytes for use in size calculations.
pub const MB_889: usize = 932184064;

/// 890MB in bytes.
/// This constant represents 890 mb converted to bytes for use in size calculations.
pub const MB_890: usize = 933232640;

/// 891MB in bytes.
/// This constant represents 891 mb converted to bytes for use in size calculations.
pub const MB_891: usize = 934281216;

/// 892MB in bytes.
/// This constant represents 892 mb converted to bytes for use in size calculations.
pub const MB_892: usize = 935329792;

/// 893MB in bytes.
/// This constant represents 893 mb converted to bytes for use in size calculations.
pub const MB_893: usize = 936378368;

/// 894MB in bytes.
/// This constant represents 894 mb converted to bytes for use in size calculations.
pub const MB_894: usize = 937426944;

/// 895MB in bytes.
/// This constant represents 895 mb converted to bytes for use in size calculations.
pub const MB_895: usize = 938475520;

/// 896MB in bytes.
/// This constant represents 896 mb converted to bytes for use in size calculations.
pub const MB_896: usize = 939524096;

/// 897MB in bytes.
/// This constant represents 897 mb converted to bytes for use in size calculations.
pub const MB_897: usize = 940572672;

/// 898MB in bytes.
/// This constant represents 898 mb converted to bytes for use in size calculations.
pub const MB_898: usize = 941621248;

/// 899MB in bytes.
/// This constant represents 899 mb converted to bytes for use in size calculations.
pub const MB_899: usize = 942669824;

/// 900MB in bytes.
/// This constant represents 900 mb converted to bytes for use in size calculations.
pub const MB_900: usize = 943718400;

/// 901MB in bytes.
/// This constant represents 901 mb converted to bytes for use in size calculations.
pub const MB_901: usize = 944766976;

/// 902MB in bytes.
/// This constant represents 902 mb converted to bytes for use in size calculations.
pub const MB_902: usize = 945815552;

/// 903MB in bytes.
/// This constant represents 903 mb converted to bytes for use in size calculations.
pub const MB_903: usize = 946864128;

/// 904MB in bytes.
/// This constant represents 904 mb converted to bytes for use in size calculations.
pub const MB_904: usize = 947912704;

/// 905MB in bytes.
/// This constant represents 905 mb converted to bytes for use in size calculations.
pub const MB_905: usize = 948961280;

/// 906MB in bytes.
/// This constant represents 906 mb converted to bytes for use in size calculations.
pub const MB_906: usize = 950009856;

/// 907MB in bytes.
/// This constant represents 907 mb converted to bytes for use in size calculations.
pub const MB_907: usize = 951058432;

/// 908MB in bytes.
/// This constant represents 908 mb converted to bytes for use in size calculations.
pub const MB_908: usize = 952107008;

/// 909MB in bytes.
/// This constant represents 909 mb converted to bytes for use in size calculations.
pub const MB_909: usize = 953155584;

/// 910MB in bytes.
/// This constant represents 910 mb converted to bytes for use in size calculations.
pub const MB_910: usize = 954204160;

/// 911MB in bytes.
/// This constant represents 911 mb converted to bytes for use in size calculations.
pub const MB_911: usize = 955252736;

/// 912MB in bytes.
/// This constant represents 912 mb converted to bytes for use in size calculations.
pub const MB_912: usize = 956301312;

/// 913MB in bytes.
/// This constant represents 913 mb converted to bytes for use in size calculations.
pub const MB_913: usize = 957349888;

/// 914MB in bytes.
/// This constant represents 914 mb converted to bytes for use in size calculations.
pub const MB_914: usize = 958398464;

/// 915MB in bytes.
/// This constant represents 915 mb converted to bytes for use in size calculations.
pub const MB_915: usize = 959447040;

/// 916MB in bytes.
/// This constant represents 916 mb converted to bytes for use in size calculations.
pub const MB_916: usize = 960495616;

/// 917MB in bytes.
/// This constant represents 917 mb converted to bytes for use in size calculations.
pub const MB_917: usize = 961544192;

/// 918MB in bytes.
/// This constant represents 918 mb converted to bytes for use in size calculations.
pub const MB_918: usize = 962592768;

/// 919MB in bytes.
/// This constant represents 919 mb converted to bytes for use in size calculations.
pub const MB_919: usize = 963641344;

/// 920MB in bytes.
/// This constant represents 920 mb converted to bytes for use in size calculations.
pub const MB_920: usize = 964689920;

/// 921MB in bytes.
/// This constant represents 921 mb converted to bytes for use in size calculations.
pub const MB_921: usize = 965738496;

/// 922MB in bytes.
/// This constant represents 922 mb converted to bytes for use in size calculations.
pub const MB_922: usize = 966787072;

/// 923MB in bytes.
/// This constant represents 923 mb converted to bytes for use in size calculations.
pub const MB_923: usize = 967835648;

/// 924MB in bytes.
/// This constant represents 924 mb converted to bytes for use in size calculations.
pub const MB_924: usize = 968884224;

/// 925MB in bytes.
/// This constant represents 925 mb converted to bytes for use in size calculations.
pub const MB_925: usize = 969932800;

/// 926MB in bytes.
/// This constant represents 926 mb converted to bytes for use in size calculations.
pub const MB_926: usize = 970981376;

/// 927MB in bytes.
/// This constant represents 927 mb converted to bytes for use in size calculations.
pub const MB_927: usize = 972029952;

/// 928MB in bytes.
/// This constant represents 928 mb converted to bytes for use in size calculations.
pub const MB_928: usize = 973078528;

/// 929MB in bytes.
/// This constant represents 929 mb converted to bytes for use in size calculations.
pub const MB_929: usize = 974127104;

/// 930MB in bytes.
/// This constant represents 930 mb converted to bytes for use in size calculations.
pub const MB_930: usize = 975175680;

/// 931MB in bytes.
/// This constant represents 931 mb converted to bytes for use in size calculations.
pub const MB_931: usize = 976224256;

/// 932MB in bytes.
/// This constant represents 932 mb converted to bytes for use in size calculations.
pub const MB_932: usize = 977272832;

/// 933MB in bytes.
/// This constant represents 933 mb converted to bytes for use in size calculations.
pub const MB_933: usize = 978321408;

/// 934MB in bytes.
/// This constant represents 934 mb converted to bytes for use in size calculations.
pub const MB_934: usize = 979369984;

/// 935MB in bytes.
/// This constant represents 935 mb converted to bytes for use in size calculations.
pub const MB_935: usize = 980418560;

/// 936MB in bytes.
/// This constant represents 936 mb converted to bytes for use in size calculations.
pub const MB_936: usize = 981467136;

/// 937MB in bytes.
/// This constant represents 937 mb converted to bytes for use in size calculations.
pub const MB_937: usize = 982515712;

/// 938MB in bytes.
/// This constant represents 938 mb converted to bytes for use in size calculations.
pub const MB_938: usize = 983564288;

/// 939MB in bytes.
/// This constant represents 939 mb converted to bytes for use in size calculations.
pub const MB_939: usize = 984612864;

/// 940MB in bytes.
/// This constant represents 940 mb converted to bytes for use in size calculations.
pub const MB_940: usize = 985661440;

/// 941MB in bytes.
/// This constant represents 941 mb converted to bytes for use in size calculations.
pub const MB_941: usize = 986710016;

/// 942MB in bytes.
/// This constant represents 942 mb converted to bytes for use in size calculations.
pub const MB_942: usize = 987758592;

/// 943MB in bytes.
/// This constant represents 943 mb converted to bytes for use in size calculations.
pub const MB_943: usize = 988807168;

/// 944MB in bytes.
/// This constant represents 944 mb converted to bytes for use in size calculations.
pub const MB_944: usize = 989855744;

/// 945MB in bytes.
/// This constant represents 945 mb converted to bytes for use in size calculations.
pub const MB_945: usize = 990904320;

/// 946MB in bytes.
/// This constant represents 946 mb converted to bytes for use in size calculations.
pub const MB_946: usize = 991952896;

/// 947MB in bytes.
/// This constant represents 947 mb converted to bytes for use in size calculations.
pub const MB_947: usize = 993001472;

/// 948MB in bytes.
/// This constant represents 948 mb converted to bytes for use in size calculations.
pub const MB_948: usize = 994050048;

/// 949MB in bytes.
/// This constant represents 949 mb converted to bytes for use in size calculations.
pub const MB_949: usize = 995098624;

/// 950MB in bytes.
/// This constant represents 950 mb converted to bytes for use in size calculations.
pub const MB_950: usize = 996147200;

/// 951MB in bytes.
/// This constant represents 951 mb converted to bytes for use in size calculations.
pub const MB_951: usize = 997195776;

/// 952MB in bytes.
/// This constant represents 952 mb converted to bytes for use in size calculations.
pub const MB_952: usize = 998244352;

/// 953MB in bytes.
/// This constant represents 953 mb converted to bytes for use in size calculations.
pub const MB_953: usize = 999292928;

/// 954MB in bytes.
/// This constant represents 954 mb converted to bytes for use in size calculations.
pub const MB_954: usize = 1000341504;

/// 955MB in bytes.
/// This constant represents 955 mb converted to bytes for use in size calculations.
pub const MB_955: usize = 1001390080;

/// 956MB in bytes.
/// This constant represents 956 mb converted to bytes for use in size calculations.
pub const MB_956: usize = 1002438656;

/// 957MB in bytes.
/// This constant represents 957 mb converted to bytes for use in size calculations.
pub const MB_957: usize = 1003487232;

/// 958MB in bytes.
/// This constant represents 958 mb converted to bytes for use in size calculations.
pub const MB_958: usize = 1004535808;

/// 959MB in bytes.
/// This constant represents 959 mb converted to bytes for use in size calculations.
pub const MB_959: usize = 1005584384;

/// 960MB in bytes.
/// This constant represents 960 mb converted to bytes for use in size calculations.
pub const MB_960: usize = 1006632960;

/// 961MB in bytes.
/// This constant represents 961 mb converted to bytes for use in size calculations.
pub const MB_961: usize = 1007681536;

/// 962MB in bytes.
/// This constant represents 962 mb converted to bytes for use in size calculations.
pub const MB_962: usize = 1008730112;

/// 963MB in bytes.
/// This constant represents 963 mb converted to bytes for use in size calculations.
pub const MB_963: usize = 1009778688;

/// 964MB in bytes.
/// This constant represents 964 mb converted to bytes for use in size calculations.
pub const MB_964: usize = 1010827264;

/// 965MB in bytes.
/// This constant represents 965 mb converted to bytes for use in size calculations.
pub const MB_965: usize = 1011875840;

/// 966MB in bytes.
/// This constant represents 966 mb converted to bytes for use in size calculations.
pub const MB_966: usize = 1012924416;

/// 967MB in bytes.
/// This constant represents 967 mb converted to bytes for use in size calculations.
pub const MB_967: usize = 1013972992;

/// 968MB in bytes.
/// This constant represents 968 mb converted to bytes for use in size calculations.
pub const MB_968: usize = 1015021568;

/// 969MB in bytes.
/// This constant represents 969 mb converted to bytes for use in size calculations.
pub const MB_969: usize = 1016070144;

/// 970MB in bytes.
/// This constant represents 970 mb converted to bytes for use in size calculations.
pub const MB_970: usize = 1017118720;

/// 971MB in bytes.
/// This constant represents 971 mb converted to bytes for use in size calculations.
pub const MB_971: usize = 1018167296;

/// 972MB in bytes.
/// This constant represents 972 mb converted to bytes for use in size calculations.
pub const MB_972: usize = 1019215872;

/// 973MB in bytes.
/// This constant represents 973 mb converted to bytes for use in size calculations.
pub const MB_973: usize = 1020264448;

/// 974MB in bytes.
/// This constant represents 974 mb converted to bytes for use in size calculations.
pub const MB_974: usize = 1021313024;

/// 975MB in bytes.
/// This constant represents 975 mb converted to bytes for use in size calculations.
pub const MB_975: usize = 1022361600;

/// 976MB in bytes.
/// This constant represents 976 mb converted to bytes for use in size calculations.
pub const MB_976: usize = 1023410176;

/// 977MB in bytes.
/// This constant represents 977 mb converted to bytes for use in size calculations.
pub const MB_977: usize = 1024458752;

/// 978MB in bytes.
/// This constant represents 978 mb converted to bytes for use in size calculations.
pub const MB_978: usize = 1025507328;

/// 979MB in bytes.
/// This constant represents 979 mb converted to bytes for use in size calculations.
pub const MB_979: usize = 1026555904;

/// 980MB in bytes.
/// This constant represents 980 mb converted to bytes for use in size calculations.
pub const MB_980: usize = 1027604480;

/// 981MB in bytes.
/// This constant represents 981 mb converted to bytes for use in size calculations.
pub const MB_981: usize = 1028653056;

/// 982MB in bytes.
/// This constant represents 982 mb converted to bytes for use in size calculations.
pub const MB_982: usize = 1029701632;

/// 983MB in bytes.
/// This constant represents 983 mb converted to bytes for use in size calculations.
pub const MB_983: usize = 1030750208;

/// 984MB in bytes.
/// This constant represents 984 mb converted to bytes for use in size calculations.
pub const MB_984: usize = 1031798784;

/// 985MB in bytes.
/// This constant represents 985 mb converted to bytes for use in size calculations.
pub const MB_985: usize = 1032847360;

/// 986MB in bytes.
/// This constant represents 986 mb converted to bytes for use in size calculations.
pub const MB_986: usize = 1033895936;

/// 987MB in bytes.
/// This constant represents 987 mb converted to bytes for use in size calculations.
pub const MB_987: usize = 1034944512;

/// 988MB in bytes.
/// This constant represents 988 mb converted to bytes for use in size calculations.
pub const MB_988: usize = 1035993088;

/// 989MB in bytes.
/// This constant represents 989 mb converted to bytes for use in size calculations.
pub const MB_989: usize = 1037041664;

/// 990MB in bytes.
/// This constant represents 990 mb converted to bytes for use in size calculations.
pub const MB_990: usize = 1038090240;

/// 991MB in bytes.
/// This constant represents 991 mb converted to bytes for use in size calculations.
pub const MB_991: usize = 1039138816;

/// 992MB in bytes.
/// This constant represents 992 mb converted to bytes for use in size calculations.
pub const MB_992: usize = 1040187392;

/// 993MB in bytes.
/// This constant represents 993 mb converted to bytes for use in size calculations.
pub const MB_993: usize = 1041235968;

/// 994MB in bytes.
/// This constant represents 994 mb converted to bytes for use in size calculations.
pub const MB_994: usize = 1042284544;

/// 995MB in bytes.
/// This constant represents 995 mb converted to bytes for use in size calculations.
pub const MB_995: usize = 1043333120;

/// 996MB in bytes.
/// This constant represents 996 mb converted to bytes for use in size calculations.
pub const MB_996: usize = 1044381696;

/// 997MB in bytes.
/// This constant represents 997 mb converted to bytes for use in size calculations.
pub const MB_997: usize = 1045430272;

/// 998MB in bytes.
/// This constant represents 998 mb converted to bytes for use in size calculations.
pub const MB_998: usize = 1046478848;

/// 999MB in bytes.
/// This constant represents 999 mb converted to bytes for use in size calculations.
pub const MB_999: usize = 1047527424;

/// 1000MB in bytes.
/// This constant represents 1000 mb converted to bytes for use in size calculations.
pub const MB_1000: usize = 1048576000;

/// 1001MB in bytes.
/// This constant represents 1001 mb converted to bytes for use in size calculations.
pub const MB_1001: usize = 1049624576;

/// 1002MB in bytes.
/// This constant represents 1002 mb converted to bytes for use in size calculations.
pub const MB_1002: usize = 1050673152;

/// 1003MB in bytes.
/// This constant represents 1003 mb converted to bytes for use in size calculations.
pub const MB_1003: usize = 1051721728;

/// 1004MB in bytes.
/// This constant represents 1004 mb converted to bytes for use in size calculations.
pub const MB_1004: usize = 1052770304;

/// 1005MB in bytes.
/// This constant represents 1005 mb converted to bytes for use in size calculations.
pub const MB_1005: usize = 1053818880;

/// 1006MB in bytes.
/// This constant represents 1006 mb converted to bytes for use in size calculations.
pub const MB_1006: usize = 1054867456;

/// 1007MB in bytes.
/// This constant represents 1007 mb converted to bytes for use in size calculations.
pub const MB_1007: usize = 1055916032;

/// 1008MB in bytes.
/// This constant represents 1008 mb converted to bytes for use in size calculations.
pub const MB_1008: usize = 1056964608;

/// 1009MB in bytes.
/// This constant represents 1009 mb converted to bytes for use in size calculations.
pub const MB_1009: usize = 1058013184;

/// 1010MB in bytes.
/// This constant represents 1010 mb converted to bytes for use in size calculations.
pub const MB_1010: usize = 1059061760;

/// 1011MB in bytes.
/// This constant represents 1011 mb converted to bytes for use in size calculations.
pub const MB_1011: usize = 1060110336;

/// 1012MB in bytes.
/// This constant represents 1012 mb converted to bytes for use in size calculations.
pub const MB_1012: usize = 1061158912;

/// 1013MB in bytes.
/// This constant represents 1013 mb converted to bytes for use in size calculations.
pub const MB_1013: usize = 1062207488;

/// 1014MB in bytes.
/// This constant represents 1014 mb converted to bytes for use in size calculations.
pub const MB_1014: usize = 1063256064;

/// 1015MB in bytes.
/// This constant represents 1015 mb converted to bytes for use in size calculations.
pub const MB_1015: usize = 1064304640;

/// 1016MB in bytes.
/// This constant represents 1016 mb converted to bytes for use in size calculations.
pub const MB_1016: usize = 1065353216;

/// 1017MB in bytes.
/// This constant represents 1017 mb converted to bytes for use in size calculations.
pub const MB_1017: usize = 1066401792;

/// 1018MB in bytes.
/// This constant represents 1018 mb converted to bytes for use in size calculations.
pub const MB_1018: usize = 1067450368;

/// 1019MB in bytes.
/// This constant represents 1019 mb converted to bytes for use in size calculations.
pub const MB_1019: usize = 1068498944;

/// 1020MB in bytes.
/// This constant represents 1020 mb converted to bytes for use in size calculations.
pub const MB_1020: usize = 1069547520;

/// 1021MB in bytes.
/// This constant represents 1021 mb converted to bytes for use in size calculations.
pub const MB_1021: usize = 1070596096;

/// 1022MB in bytes.
/// This constant represents 1022 mb converted to bytes for use in size calculations.
pub const MB_1022: usize = 1071644672;

/// 1023MB in bytes.
/// This constant represents 1023 mb converted to bytes for use in size calculations.
pub const MB_1023: usize = 1072693248;

/// 1024MB in bytes.
/// This constant represents 1024 mb converted to bytes for use in size calculations.
pub const MB_1024: usize = 1073741824;

// Storage unit constants from 1GB to 1024GB
/// 1GB in bytes.
/// This constant represents 1 gb converted to bytes for use in size calculations.
pub const GB_1: usize = 1073741824;

/// 2GB in bytes.
/// This constant represents 2 gb converted to bytes for use in size calculations.
pub const GB_2: usize = 2147483648;

/// 3GB in bytes.
/// This constant represents 3 gb converted to bytes for use in size calculations.
pub const GB_3: usize = 3221225472;

/// 4GB in bytes.
/// This constant represents 4 gb converted to bytes for use in size calculations.
pub const GB_4: usize = 4294967296;

/// 5GB in bytes.
/// This constant represents 5 gb converted to bytes for use in size calculations.
pub const GB_5: usize = 5368709120;

/// 6GB in bytes.
/// This constant represents 6 gb converted to bytes for use in size calculations.
pub const GB_6: usize = 6442450944;

/// 7GB in bytes.
/// This constant represents 7 gb converted to bytes for use in size calculations.
pub const GB_7: usize = 7516192768;

/// 8GB in bytes.
/// This constant represents 8 gb converted to bytes for use in size calculations.
pub const GB_8: usize = 8589934592;

/// 9GB in bytes.
/// This constant represents 9 gb converted to bytes for use in size calculations.
pub const GB_9: usize = 9663676416;

/// 10GB in bytes.
/// This constant represents 10 gb converted to bytes for use in size calculations.
pub const GB_10: usize = 10737418240;

/// 11GB in bytes.
/// This constant represents 11 gb converted to bytes for use in size calculations.
pub const GB_11: usize = 11811160064;

/// 12GB in bytes.
/// This constant represents 12 gb converted to bytes for use in size calculations.
pub const GB_12: usize = 12884901888;

/// 13GB in bytes.
/// This constant represents 13 gb converted to bytes for use in size calculations.
pub const GB_13: usize = 13958643712;

/// 14GB in bytes.
/// This constant represents 14 gb converted to bytes for use in size calculations.
pub const GB_14: usize = 15032385536;

/// 15GB in bytes.
/// This constant represents 15 gb converted to bytes for use in size calculations.
pub const GB_15: usize = 16106127360;

/// 16GB in bytes.
/// This constant represents 16 gb converted to bytes for use in size calculations.
pub const GB_16: usize = 17179869184;

/// 17GB in bytes.
/// This constant represents 17 gb converted to bytes for use in size calculations.
pub const GB_17: usize = 18253611008;

/// 18GB in bytes.
/// This constant represents 18 gb converted to bytes for use in size calculations.
pub const GB_18: usize = 19327352832;

/// 19GB in bytes.
/// This constant represents 19 gb converted to bytes for use in size calculations.
pub const GB_19: usize = 20401094656;

/// 20GB in bytes.
/// This constant represents 20 gb converted to bytes for use in size calculations.
pub const GB_20: usize = 21474836480;

/// 21GB in bytes.
/// This constant represents 21 gb converted to bytes for use in size calculations.
pub const GB_21: usize = 22548578304;

/// 22GB in bytes.
/// This constant represents 22 gb converted to bytes for use in size calculations.
pub const GB_22: usize = 23622320128;

/// 23GB in bytes.
/// This constant represents 23 gb converted to bytes for use in size calculations.
pub const GB_23: usize = 24696061952;

/// 24GB in bytes.
/// This constant represents 24 gb converted to bytes for use in size calculations.
pub const GB_24: usize = 25769803776;

/// 25GB in bytes.
/// This constant represents 25 gb converted to bytes for use in size calculations.
pub const GB_25: usize = 26843545600;

/// 26GB in bytes.
/// This constant represents 26 gb converted to bytes for use in size calculations.
pub const GB_26: usize = 27917287424;

/// 27GB in bytes.
/// This constant represents 27 gb converted to bytes for use in size calculations.
pub const GB_27: usize = 28991029248;

/// 28GB in bytes.
/// This constant represents 28 gb converted to bytes for use in size calculations.
pub const GB_28: usize = 30064771072;

/// 29GB in bytes.
/// This constant represents 29 gb converted to bytes for use in size calculations.
pub const GB_29: usize = 31138512896;

/// 30GB in bytes.
/// This constant represents 30 gb converted to bytes for use in size calculations.
pub const GB_30: usize = 32212254720;

/// 31GB in bytes.
/// This constant represents 31 gb converted to bytes for use in size calculations.
pub const GB_31: usize = 33285996544;

/// 32GB in bytes.
/// This constant represents 32 gb converted to bytes for use in size calculations.
pub const GB_32: usize = 34359738368;

/// 33GB in bytes.
/// This constant represents 33 gb converted to bytes for use in size calculations.
pub const GB_33: usize = 35433480192;

/// 34GB in bytes.
/// This constant represents 34 gb converted to bytes for use in size calculations.
pub const GB_34: usize = 36507222016;

/// 35GB in bytes.
/// This constant represents 35 gb converted to bytes for use in size calculations.
pub const GB_35: usize = 37580963840;

/// 36GB in bytes.
/// This constant represents 36 gb converted to bytes for use in size calculations.
pub const GB_36: usize = 38654705664;

/// 37GB in bytes.
/// This constant represents 37 gb converted to bytes for use in size calculations.
pub const GB_37: usize = 39728447488;

/// 38GB in bytes.
/// This constant represents 38 gb converted to bytes for use in size calculations.
pub const GB_38: usize = 40802189312;

/// 39GB in bytes.
/// This constant represents 39 gb converted to bytes for use in size calculations.
pub const GB_39: usize = 41875931136;

/// 40GB in bytes.
/// This constant represents 40 gb converted to bytes for use in size calculations.
pub const GB_40: usize = 42949672960;

/// 41GB in bytes.
/// This constant represents 41 gb converted to bytes for use in size calculations.
pub const GB_41: usize = 44023414784;

/// 42GB in bytes.
/// This constant represents 42 gb converted to bytes for use in size calculations.
pub const GB_42: usize = 45097156608;

/// 43GB in bytes.
/// This constant represents 43 gb converted to bytes for use in size calculations.
pub const GB_43: usize = 46170898432;

/// 44GB in bytes.
/// This constant represents 44 gb converted to bytes for use in size calculations.
pub const GB_44: usize = 47244640256;

/// 45GB in bytes.
/// This constant represents 45 gb converted to bytes for use in size calculations.
pub const GB_45: usize = 48318382080;

/// 46GB in bytes.
/// This constant represents 46 gb converted to bytes for use in size calculations.
pub const GB_46: usize = 49392123904;

/// 47GB in bytes.
/// This constant represents 47 gb converted to bytes for use in size calculations.
pub const GB_47: usize = 50465865728;

/// 48GB in bytes.
/// This constant represents 48 gb converted to bytes for use in size calculations.
pub const GB_48: usize = 51539607552;

/// 49GB in bytes.
/// This constant represents 49 gb converted to bytes for use in size calculations.
pub const GB_49: usize = 52613349376;

/// 50GB in bytes.
/// This constant represents 50 gb converted to bytes for use in size calculations.
pub const GB_50: usize = 53687091200;

/// 51GB in bytes.
/// This constant represents 51 gb converted to bytes for use in size calculations.
pub const GB_51: usize = 54760833024;

/// 52GB in bytes.
/// This constant represents 52 gb converted to bytes for use in size calculations.
pub const GB_52: usize = 55834574848;

/// 53GB in bytes.
/// This constant represents 53 gb converted to bytes for use in size calculations.
pub const GB_53: usize = 56908316672;

/// 54GB in bytes.
/// This constant represents 54 gb converted to bytes for use in size calculations.
pub const GB_54: usize = 57982058496;

/// 55GB in bytes.
/// This constant represents 55 gb converted to bytes for use in size calculations.
pub const GB_55: usize = 59055800320;

/// 56GB in bytes.
/// This constant represents 56 gb converted to bytes for use in size calculations.
pub const GB_56: usize = 60129542144;

/// 57GB in bytes.
/// This constant represents 57 gb converted to bytes for use in size calculations.
pub const GB_57: usize = 61203283968;

/// 58GB in bytes.
/// This constant represents 58 gb converted to bytes for use in size calculations.
pub const GB_58: usize = 62277025792;

/// 59GB in bytes.
/// This constant represents 59 gb converted to bytes for use in size calculations.
pub const GB_59: usize = 63350767616;

/// 60GB in bytes.
/// This constant represents 60 gb converted to bytes for use in size calculations.
pub const GB_60: usize = 64424509440;

/// 61GB in bytes.
/// This constant represents 61 gb converted to bytes for use in size calculations.
pub const GB_61: usize = 65498251264;

/// 62GB in bytes.
/// This constant represents 62 gb converted to bytes for use in size calculations.
pub const GB_62: usize = 66571993088;

/// 63GB in bytes.
/// This constant represents 63 gb converted to bytes for use in size calculations.
pub const GB_63: usize = 67645734912;

/// 64GB in bytes.
/// This constant represents 64 gb converted to bytes for use in size calculations.
pub const GB_64: usize = 68719476736;

/// 65GB in bytes.
/// This constant represents 65 gb converted to bytes for use in size calculations.
pub const GB_65: usize = 69793218560;

/// 66GB in bytes.
/// This constant represents 66 gb converted to bytes for use in size calculations.
pub const GB_66: usize = 70866960384;

/// 67GB in bytes.
/// This constant represents 67 gb converted to bytes for use in size calculations.
pub const GB_67: usize = 71940702208;

/// 68GB in bytes.
/// This constant represents 68 gb converted to bytes for use in size calculations.
pub const GB_68: usize = 73014444032;

/// 69GB in bytes.
/// This constant represents 69 gb converted to bytes for use in size calculations.
pub const GB_69: usize = 74088185856;

/// 70GB in bytes.
/// This constant represents 70 gb converted to bytes for use in size calculations.
pub const GB_70: usize = 75161927680;

/// 71GB in bytes.
/// This constant represents 71 gb converted to bytes for use in size calculations.
pub const GB_71: usize = 76235669504;

/// 72GB in bytes.
/// This constant represents 72 gb converted to bytes for use in size calculations.
pub const GB_72: usize = 77309411328;

/// 73GB in bytes.
/// This constant represents 73 gb converted to bytes for use in size calculations.
pub const GB_73: usize = 78383153152;

/// 74GB in bytes.
/// This constant represents 74 gb converted to bytes for use in size calculations.
pub const GB_74: usize = 79456894976;

/// 75GB in bytes.
/// This constant represents 75 gb converted to bytes for use in size calculations.
pub const GB_75: usize = 80530636800;

/// 76GB in bytes.
/// This constant represents 76 gb converted to bytes for use in size calculations.
pub const GB_76: usize = 81604378624;

/// 77GB in bytes.
/// This constant represents 77 gb converted to bytes for use in size calculations.
pub const GB_77: usize = 82678120448;

/// 78GB in bytes.
/// This constant represents 78 gb converted to bytes for use in size calculations.
pub const GB_78: usize = 83751862272;

/// 79GB in bytes.
/// This constant represents 79 gb converted to bytes for use in size calculations.
pub const GB_79: usize = 84825604096;

/// 80GB in bytes.
/// This constant represents 80 gb converted to bytes for use in size calculations.
pub const GB_80: usize = 85899345920;

/// 81GB in bytes.
/// This constant represents 81 gb converted to bytes for use in size calculations.
pub const GB_81: usize = 86973087744;

/// 82GB in bytes.
/// This constant represents 82 gb converted to bytes for use in size calculations.
pub const GB_82: usize = 88046829568;

/// 83GB in bytes.
/// This constant represents 83 gb converted to bytes for use in size calculations.
pub const GB_83: usize = 89120571392;

/// 84GB in bytes.
/// This constant represents 84 gb converted to bytes for use in size calculations.
pub const GB_84: usize = 90194313216;

/// 85GB in bytes.
/// This constant represents 85 gb converted to bytes for use in size calculations.
pub const GB_85: usize = 91268055040;

/// 86GB in bytes.
/// This constant represents 86 gb converted to bytes for use in size calculations.
pub const GB_86: usize = 92341796864;

/// 87GB in bytes.
/// This constant represents 87 gb converted to bytes for use in size calculations.
pub const GB_87: usize = 93415538688;

/// 88GB in bytes.
/// This constant represents 88 gb converted to bytes for use in size calculations.
pub const GB_88: usize = 94489280512;

/// 89GB in bytes.
/// This constant represents 89 gb converted to bytes for use in size calculations.
pub const GB_89: usize = 95563022336;

/// 90GB in bytes.
/// This constant represents 90 gb converted to bytes for use in size calculations.
pub const GB_90: usize = 96636764160;

/// 91GB in bytes.
/// This constant represents 91 gb converted to bytes for use in size calculations.
pub const GB_91: usize = 97710505984;

/// 92GB in bytes.
/// This constant represents 92 gb converted to bytes for use in size calculations.
pub const GB_92: usize = 98784247808;

/// 93GB in bytes.
/// This constant represents 93 gb converted to bytes for use in size calculations.
pub const GB_93: usize = 99857989632;

/// 94GB in bytes.
/// This constant represents 94 gb converted to bytes for use in size calculations.
pub const GB_94: usize = 100931731456;

/// 95GB in bytes.
/// This constant represents 95 gb converted to bytes for use in size calculations.
pub const GB_95: usize = 102005473280;

/// 96GB in bytes.
/// This constant represents 96 gb converted to bytes for use in size calculations.
pub const GB_96: usize = 103079215104;

/// 97GB in bytes.
/// This constant represents 97 gb converted to bytes for use in size calculations.
pub const GB_97: usize = 104152956928;

/// 98GB in bytes.
/// This constant represents 98 gb converted to bytes for use in size calculations.
pub const GB_98: usize = 105226698752;

/// 99GB in bytes.
/// This constant represents 99 gb converted to bytes for use in size calculations.
pub const GB_99: usize = 106300440576;

/// 100GB in bytes.
/// This constant represents 100 gb converted to bytes for use in size calculations.
pub const GB_100: usize = 107374182400;

/// 101GB in bytes.
/// This constant represents 101 gb converted to bytes for use in size calculations.
pub const GB_101: usize = 108447924224;

/// 102GB in bytes.
/// This constant represents 102 gb converted to bytes for use in size calculations.
pub const GB_102: usize = 109521666048;

/// 103GB in bytes.
/// This constant represents 103 gb converted to bytes for use in size calculations.
pub const GB_103: usize = 110595407872;

/// 104GB in bytes.
/// This constant represents 104 gb converted to bytes for use in size calculations.
pub const GB_104: usize = 111669149696;

/// 105GB in bytes.
/// This constant represents 105 gb converted to bytes for use in size calculations.
pub const GB_105: usize = 112742891520;

/// 106GB in bytes.
/// This constant represents 106 gb converted to bytes for use in size calculations.
pub const GB_106: usize = 113816633344;

/// 107GB in bytes.
/// This constant represents 107 gb converted to bytes for use in size calculations.
pub const GB_107: usize = 114890375168;

/// 108GB in bytes.
/// This constant represents 108 gb converted to bytes for use in size calculations.
pub const GB_108: usize = 115964116992;

/// 109GB in bytes.
/// This constant represents 109 gb converted to bytes for use in size calculations.
pub const GB_109: usize = 117037858816;

/// 110GB in bytes.
/// This constant represents 110 gb converted to bytes for use in size calculations.
pub const GB_110: usize = 118111600640;

/// 111GB in bytes.
/// This constant represents 111 gb converted to bytes for use in size calculations.
pub const GB_111: usize = 119185342464;

/// 112GB in bytes.
/// This constant represents 112 gb converted to bytes for use in size calculations.
pub const GB_112: usize = 120259084288;

/// 113GB in bytes.
/// This constant represents 113 gb converted to bytes for use in size calculations.
pub const GB_113: usize = 121332826112;

/// 114GB in bytes.
/// This constant represents 114 gb converted to bytes for use in size calculations.
pub const GB_114: usize = 122406567936;

/// 115GB in bytes.
/// This constant represents 115 gb converted to bytes for use in size calculations.
pub const GB_115: usize = 123480309760;

/// 116GB in bytes.
/// This constant represents 116 gb converted to bytes for use in size calculations.
pub const GB_116: usize = 124554051584;

/// 117GB in bytes.
/// This constant represents 117 gb converted to bytes for use in size calculations.
pub const GB_117: usize = 125627793408;

/// 118GB in bytes.
/// This constant represents 118 gb converted to bytes for use in size calculations.
pub const GB_118: usize = 126701535232;

/// 119GB in bytes.
/// This constant represents 119 gb converted to bytes for use in size calculations.
pub const GB_119: usize = 127775277056;

/// 120GB in bytes.
/// This constant represents 120 gb converted to bytes for use in size calculations.
pub const GB_120: usize = 128849018880;

/// 121GB in bytes.
/// This constant represents 121 gb converted to bytes for use in size calculations.
pub const GB_121: usize = 129922760704;

/// 122GB in bytes.
/// This constant represents 122 gb converted to bytes for use in size calculations.
pub const GB_122: usize = 130996502528;

/// 123GB in bytes.
/// This constant represents 123 gb converted to bytes for use in size calculations.
pub const GB_123: usize = 132070244352;

/// 124GB in bytes.
/// This constant represents 124 gb converted to bytes for use in size calculations.
pub const GB_124: usize = 133143986176;

/// 125GB in bytes.
/// This constant represents 125 gb converted to bytes for use in size calculations.
pub const GB_125: usize = 134217728000;

/// 126GB in bytes.
/// This constant represents 126 gb converted to bytes for use in size calculations.
pub const GB_126: usize = 135291469824;

/// 127GB in bytes.
/// This constant represents 127 gb converted to bytes for use in size calculations.
pub const GB_127: usize = 136365211648;

/// 128GB in bytes.
/// This constant represents 128 gb converted to bytes for use in size calculations.
pub const GB_128: usize = 137438953472;

/// 129GB in bytes.
/// This constant represents 129 gb converted to bytes for use in size calculations.
pub const GB_129: usize = 138512695296;

/// 130GB in bytes.
/// This constant represents 130 gb converted to bytes for use in size calculations.
pub const GB_130: usize = 139586437120;

/// 131GB in bytes.
/// This constant represents 131 gb converted to bytes for use in size calculations.
pub const GB_131: usize = 140660178944;

/// 132GB in bytes.
/// This constant represents 132 gb converted to bytes for use in size calculations.
pub const GB_132: usize = 141733920768;

/// 133GB in bytes.
/// This constant represents 133 gb converted to bytes for use in size calculations.
pub const GB_133: usize = 142807662592;

/// 134GB in bytes.
/// This constant represents 134 gb converted to bytes for use in size calculations.
pub const GB_134: usize = 143881404416;

/// 135GB in bytes.
/// This constant represents 135 gb converted to bytes for use in size calculations.
pub const GB_135: usize = 144955146240;

/// 136GB in bytes.
/// This constant represents 136 gb converted to bytes for use in size calculations.
pub const GB_136: usize = 146028888064;

/// 137GB in bytes.
/// This constant represents 137 gb converted to bytes for use in size calculations.
pub const GB_137: usize = 147102629888;

/// 138GB in bytes.
/// This constant represents 138 gb converted to bytes for use in size calculations.
pub const GB_138: usize = 148176371712;

/// 139GB in bytes.
/// This constant represents 139 gb converted to bytes for use in size calculations.
pub const GB_139: usize = 149250113536;

/// 140GB in bytes.
/// This constant represents 140 gb converted to bytes for use in size calculations.
pub const GB_140: usize = 150323855360;

/// 141GB in bytes.
/// This constant represents 141 gb converted to bytes for use in size calculations.
pub const GB_141: usize = 151397597184;

/// 142GB in bytes.
/// This constant represents 142 gb converted to bytes for use in size calculations.
pub const GB_142: usize = 152471339008;

/// 143GB in bytes.
/// This constant represents 143 gb converted to bytes for use in size calculations.
pub const GB_143: usize = 153545080832;

/// 144GB in bytes.
/// This constant represents 144 gb converted to bytes for use in size calculations.
pub const GB_144: usize = 154618822656;

/// 145GB in bytes.
/// This constant represents 145 gb converted to bytes for use in size calculations.
pub const GB_145: usize = 155692564480;

/// 146GB in bytes.
/// This constant represents 146 gb converted to bytes for use in size calculations.
pub const GB_146: usize = 156766306304;

/// 147GB in bytes.
/// This constant represents 147 gb converted to bytes for use in size calculations.
pub const GB_147: usize = 157840048128;

/// 148GB in bytes.
/// This constant represents 148 gb converted to bytes for use in size calculations.
pub const GB_148: usize = 158913789952;

/// 149GB in bytes.
/// This constant represents 149 gb converted to bytes for use in size calculations.
pub const GB_149: usize = 159987531776;

/// 150GB in bytes.
/// This constant represents 150 gb converted to bytes for use in size calculations.
pub const GB_150: usize = 161061273600;

/// 151GB in bytes.
/// This constant represents 151 gb converted to bytes for use in size calculations.
pub const GB_151: usize = 162135015424;

/// 152GB in bytes.
/// This constant represents 152 gb converted to bytes for use in size calculations.
pub const GB_152: usize = 163208757248;

/// 153GB in bytes.
/// This constant represents 153 gb converted to bytes for use in size calculations.
pub const GB_153: usize = 164282499072;

/// 154GB in bytes.
/// This constant represents 154 gb converted to bytes for use in size calculations.
pub const GB_154: usize = 165356240896;

/// 155GB in bytes.
/// This constant represents 155 gb converted to bytes for use in size calculations.
pub const GB_155: usize = 166429982720;

/// 156GB in bytes.
/// This constant represents 156 gb converted to bytes for use in size calculations.
pub const GB_156: usize = 167503724544;

/// 157GB in bytes.
/// This constant represents 157 gb converted to bytes for use in size calculations.
pub const GB_157: usize = 168577466368;

/// 158GB in bytes.
/// This constant represents 158 gb converted to bytes for use in size calculations.
pub const GB_158: usize = 169651208192;

/// 159GB in bytes.
/// This constant represents 159 gb converted to bytes for use in size calculations.
pub const GB_159: usize = 170724950016;

/// 160GB in bytes.
/// This constant represents 160 gb converted to bytes for use in size calculations.
pub const GB_160: usize = 171798691840;

/// 161GB in bytes.
/// This constant represents 161 gb converted to bytes for use in size calculations.
pub const GB_161: usize = 172872433664;

/// 162GB in bytes.
/// This constant represents 162 gb converted to bytes for use in size calculations.
pub const GB_162: usize = 173946175488;

/// 163GB in bytes.
/// This constant represents 163 gb converted to bytes for use in size calculations.
pub const GB_163: usize = 175019917312;

/// 164GB in bytes.
/// This constant represents 164 gb converted to bytes for use in size calculations.
pub const GB_164: usize = 176093659136;

/// 165GB in bytes.
/// This constant represents 165 gb converted to bytes for use in size calculations.
pub const GB_165: usize = 177167400960;

/// 166GB in bytes.
/// This constant represents 166 gb converted to bytes for use in size calculations.
pub const GB_166: usize = 178241142784;

/// 167GB in bytes.
/// This constant represents 167 gb converted to bytes for use in size calculations.
pub const GB_167: usize = 179314884608;

/// 168GB in bytes.
/// This constant represents 168 gb converted to bytes for use in size calculations.
pub const GB_168: usize = 180388626432;

/// 169GB in bytes.
/// This constant represents 169 gb converted to bytes for use in size calculations.
pub const GB_169: usize = 181462368256;

/// 170GB in bytes.
/// This constant represents 170 gb converted to bytes for use in size calculations.
pub const GB_170: usize = 182536110080;

/// 171GB in bytes.
/// This constant represents 171 gb converted to bytes for use in size calculations.
pub const GB_171: usize = 183609851904;

/// 172GB in bytes.
/// This constant represents 172 gb converted to bytes for use in size calculations.
pub const GB_172: usize = 184683593728;

/// 173GB in bytes.
/// This constant represents 173 gb converted to bytes for use in size calculations.
pub const GB_173: usize = 185757335552;

/// 174GB in bytes.
/// This constant represents 174 gb converted to bytes for use in size calculations.
pub const GB_174: usize = 186831077376;

/// 175GB in bytes.
/// This constant represents 175 gb converted to bytes for use in size calculations.
pub const GB_175: usize = 187904819200;

/// 176GB in bytes.
/// This constant represents 176 gb converted to bytes for use in size calculations.
pub const GB_176: usize = 188978561024;

/// 177GB in bytes.
/// This constant represents 177 gb converted to bytes for use in size calculations.
pub const GB_177: usize = 190052302848;

/// 178GB in bytes.
/// This constant represents 178 gb converted to bytes for use in size calculations.
pub const GB_178: usize = 191126044672;

/// 179GB in bytes.
/// This constant represents 179 gb converted to bytes for use in size calculations.
pub const GB_179: usize = 192199786496;

/// 180GB in bytes.
/// This constant represents 180 gb converted to bytes for use in size calculations.
pub const GB_180: usize = 193273528320;

/// 181GB in bytes.
/// This constant represents 181 gb converted to bytes for use in size calculations.
pub const GB_181: usize = 194347270144;

/// 182GB in bytes.
/// This constant represents 182 gb converted to bytes for use in size calculations.
pub const GB_182: usize = 195421011968;

/// 183GB in bytes.
/// This constant represents 183 gb converted to bytes for use in size calculations.
pub const GB_183: usize = 196494753792;

/// 184GB in bytes.
/// This constant represents 184 gb converted to bytes for use in size calculations.
pub const GB_184: usize = 197568495616;

/// 185GB in bytes.
/// This constant represents 185 gb converted to bytes for use in size calculations.
pub const GB_185: usize = 198642237440;

/// 186GB in bytes.
/// This constant represents 186 gb converted to bytes for use in size calculations.
pub const GB_186: usize = 199715979264;

/// 187GB in bytes.
/// This constant represents 187 gb converted to bytes for use in size calculations.
pub const GB_187: usize = 200789721088;

/// 188GB in bytes.
/// This constant represents 188 gb converted to bytes for use in size calculations.
pub const GB_188: usize = 201863462912;

/// 189GB in bytes.
/// This constant represents 189 gb converted to bytes for use in size calculations.
pub const GB_189: usize = 202937204736;

/// 190GB in bytes.
/// This constant represents 190 gb converted to bytes for use in size calculations.
pub const GB_190: usize = 204010946560;

/// 191GB in bytes.
/// This constant represents 191 gb converted to bytes for use in size calculations.
pub const GB_191: usize = 205084688384;

/// 192GB in bytes.
/// This constant represents 192 gb converted to bytes for use in size calculations.
pub const GB_192: usize = 206158430208;

/// 193GB in bytes.
/// This constant represents 193 gb converted to bytes for use in size calculations.
pub const GB_193: usize = 207232172032;

/// 194GB in bytes.
/// This constant represents 194 gb converted to bytes for use in size calculations.
pub const GB_194: usize = 208305913856;

/// 195GB in bytes.
/// This constant represents 195 gb converted to bytes for use in size calculations.
pub const GB_195: usize = 209379655680;

/// 196GB in bytes.
/// This constant represents 196 gb converted to bytes for use in size calculations.
pub const GB_196: usize = 210453397504;

/// 197GB in bytes.
/// This constant represents 197 gb converted to bytes for use in size calculations.
pub const GB_197: usize = 211527139328;

/// 198GB in bytes.
/// This constant represents 198 gb converted to bytes for use in size calculations.
pub const GB_198: usize = 212600881152;

/// 199GB in bytes.
/// This constant represents 199 gb converted to bytes for use in size calculations.
pub const GB_199: usize = 213674622976;

/// 200GB in bytes.
/// This constant represents 200 gb converted to bytes for use in size calculations.
pub const GB_200: usize = 214748364800;

/// 201GB in bytes.
/// This constant represents 201 gb converted to bytes for use in size calculations.
pub const GB_201: usize = 215822106624;

/// 202GB in bytes.
/// This constant represents 202 gb converted to bytes for use in size calculations.
pub const GB_202: usize = 216895848448;

/// 203GB in bytes.
/// This constant represents 203 gb converted to bytes for use in size calculations.
pub const GB_203: usize = 217969590272;

/// 204GB in bytes.
/// This constant represents 204 gb converted to bytes for use in size calculations.
pub const GB_204: usize = 219043332096;

/// 205GB in bytes.
/// This constant represents 205 gb converted to bytes for use in size calculations.
pub const GB_205: usize = 220117073920;

/// 206GB in bytes.
/// This constant represents 206 gb converted to bytes for use in size calculations.
pub const GB_206: usize = 221190815744;

/// 207GB in bytes.
/// This constant represents 207 gb converted to bytes for use in size calculations.
pub const GB_207: usize = 222264557568;

/// 208GB in bytes.
/// This constant represents 208 gb converted to bytes for use in size calculations.
pub const GB_208: usize = 223338299392;

/// 209GB in bytes.
/// This constant represents 209 gb converted to bytes for use in size calculations.
pub const GB_209: usize = 224412041216;

/// 210GB in bytes.
/// This constant represents 210 gb converted to bytes for use in size calculations.
pub const GB_210: usize = 225485783040;

/// 211GB in bytes.
/// This constant represents 211 gb converted to bytes for use in size calculations.
pub const GB_211: usize = 226559524864;

/// 212GB in bytes.
/// This constant represents 212 gb converted to bytes for use in size calculations.
pub const GB_212: usize = 227633266688;

/// 213GB in bytes.
/// This constant represents 213 gb converted to bytes for use in size calculations.
pub const GB_213: usize = 228707008512;

/// 214GB in bytes.
/// This constant represents 214 gb converted to bytes for use in size calculations.
pub const GB_214: usize = 229780750336;

/// 215GB in bytes.
/// This constant represents 215 gb converted to bytes for use in size calculations.
pub const GB_215: usize = 230854492160;

/// 216GB in bytes.
/// This constant represents 216 gb converted to bytes for use in size calculations.
pub const GB_216: usize = 231928233984;

/// 217GB in bytes.
/// This constant represents 217 gb converted to bytes for use in size calculations.
pub const GB_217: usize = 233001975808;

/// 218GB in bytes.
/// This constant represents 218 gb converted to bytes for use in size calculations.
pub const GB_218: usize = 234075717632;

/// 219GB in bytes.
/// This constant represents 219 gb converted to bytes for use in size calculations.
pub const GB_219: usize = 235149459456;

/// 220GB in bytes.
/// This constant represents 220 gb converted to bytes for use in size calculations.
pub const GB_220: usize = 236223201280;

/// 221GB in bytes.
/// This constant represents 221 gb converted to bytes for use in size calculations.
pub const GB_221: usize = 237296943104;

/// 222GB in bytes.
/// This constant represents 222 gb converted to bytes for use in size calculations.
pub const GB_222: usize = 238370684928;

/// 223GB in bytes.
/// This constant represents 223 gb converted to bytes for use in size calculations.
pub const GB_223: usize = 239444426752;

/// 224GB in bytes.
/// This constant represents 224 gb converted to bytes for use in size calculations.
pub const GB_224: usize = 240518168576;

/// 225GB in bytes.
/// This constant represents 225 gb converted to bytes for use in size calculations.
pub const GB_225: usize = 241591910400;

/// 226GB in bytes.
/// This constant represents 226 gb converted to bytes for use in size calculations.
pub const GB_226: usize = 242665652224;

/// 227GB in bytes.
/// This constant represents 227 gb converted to bytes for use in size calculations.
pub const GB_227: usize = 243739394048;

/// 228GB in bytes.
/// This constant represents 228 gb converted to bytes for use in size calculations.
pub const GB_228: usize = 244813135872;

/// 229GB in bytes.
/// This constant represents 229 gb converted to bytes for use in size calculations.
pub const GB_229: usize = 245886877696;

/// 230GB in bytes.
/// This constant represents 230 gb converted to bytes for use in size calculations.
pub const GB_230: usize = 246960619520;

/// 231GB in bytes.
/// This constant represents 231 gb converted to bytes for use in size calculations.
pub const GB_231: usize = 248034361344;

/// 232GB in bytes.
/// This constant represents 232 gb converted to bytes for use in size calculations.
pub const GB_232: usize = 249108103168;

/// 233GB in bytes.
/// This constant represents 233 gb converted to bytes for use in size calculations.
pub const GB_233: usize = 250181844992;

/// 234GB in bytes.
/// This constant represents 234 gb converted to bytes for use in size calculations.
pub const GB_234: usize = 251255586816;

/// 235GB in bytes.
/// This constant represents 235 gb converted to bytes for use in size calculations.
pub const GB_235: usize = 252329328640;

/// 236GB in bytes.
/// This constant represents 236 gb converted to bytes for use in size calculations.
pub const GB_236: usize = 253403070464;

/// 237GB in bytes.
/// This constant represents 237 gb converted to bytes for use in size calculations.
pub const GB_237: usize = 254476812288;

/// 238GB in bytes.
/// This constant represents 238 gb converted to bytes for use in size calculations.
pub const GB_238: usize = 255550554112;

/// 239GB in bytes.
/// This constant represents 239 gb converted to bytes for use in size calculations.
pub const GB_239: usize = 256624295936;

/// 240GB in bytes.
/// This constant represents 240 gb converted to bytes for use in size calculations.
pub const GB_240: usize = 257698037760;

/// 241GB in bytes.
/// This constant represents 241 gb converted to bytes for use in size calculations.
pub const GB_241: usize = 258771779584;

/// 242GB in bytes.
/// This constant represents 242 gb converted to bytes for use in size calculations.
pub const GB_242: usize = 259845521408;

/// 243GB in bytes.
/// This constant represents 243 gb converted to bytes for use in size calculations.
pub const GB_243: usize = 260919263232;

/// 244GB in bytes.
/// This constant represents 244 gb converted to bytes for use in size calculations.
pub const GB_244: usize = 261993005056;

/// 245GB in bytes.
/// This constant represents 245 gb converted to bytes for use in size calculations.
pub const GB_245: usize = 263066746880;

/// 246GB in bytes.
/// This constant represents 246 gb converted to bytes for use in size calculations.
pub const GB_246: usize = 264140488704;

/// 247GB in bytes.
/// This constant represents 247 gb converted to bytes for use in size calculations.
pub const GB_247: usize = 265214230528;

/// 248GB in bytes.
/// This constant represents 248 gb converted to bytes for use in size calculations.
pub const GB_248: usize = 266287972352;

/// 249GB in bytes.
/// This constant represents 249 gb converted to bytes for use in size calculations.
pub const GB_249: usize = 267361714176;

/// 250GB in bytes.
/// This constant represents 250 gb converted to bytes for use in size calculations.
pub const GB_250: usize = 268435456000;

/// 251GB in bytes.
/// This constant represents 251 gb converted to bytes for use in size calculations.
pub const GB_251: usize = 269509197824;

/// 252GB in bytes.
/// This constant represents 252 gb converted to bytes for use in size calculations.
pub const GB_252: usize = 270582939648;

/// 253GB in bytes.
/// This constant represents 253 gb converted to bytes for use in size calculations.
pub const GB_253: usize = 271656681472;

/// 254GB in bytes.
/// This constant represents 254 gb converted to bytes for use in size calculations.
pub const GB_254: usize = 272730423296;

/// 255GB in bytes.
/// This constant represents 255 gb converted to bytes for use in size calculations.
pub const GB_255: usize = 273804165120;

/// 256GB in bytes.
/// This constant represents 256 gb converted to bytes for use in size calculations.
pub const GB_256: usize = 274877906944;

/// 257GB in bytes.
/// This constant represents 257 gb converted to bytes for use in size calculations.
pub const GB_257: usize = 275951648768;

/// 258GB in bytes.
/// This constant represents 258 gb converted to bytes for use in size calculations.
pub const GB_258: usize = 277025390592;

/// 259GB in bytes.
/// This constant represents 259 gb converted to bytes for use in size calculations.
pub const GB_259: usize = 278099132416;

/// 260GB in bytes.
/// This constant represents 260 gb converted to bytes for use in size calculations.
pub const GB_260: usize = 279172874240;

/// 261GB in bytes.
/// This constant represents 261 gb converted to bytes for use in size calculations.
pub const GB_261: usize = 280246616064;

/// 262GB in bytes.
/// This constant represents 262 gb converted to bytes for use in size calculations.
pub const GB_262: usize = 281320357888;

/// 263GB in bytes.
/// This constant represents 263 gb converted to bytes for use in size calculations.
pub const GB_263: usize = 282394099712;

/// 264GB in bytes.
/// This constant represents 264 gb converted to bytes for use in size calculations.
pub const GB_264: usize = 283467841536;

/// 265GB in bytes.
/// This constant represents 265 gb converted to bytes for use in size calculations.
pub const GB_265: usize = 284541583360;

/// 266GB in bytes.
/// This constant represents 266 gb converted to bytes for use in size calculations.
pub const GB_266: usize = 285615325184;

/// 267GB in bytes.
/// This constant represents 267 gb converted to bytes for use in size calculations.
pub const GB_267: usize = 286689067008;

/// 268GB in bytes.
/// This constant represents 268 gb converted to bytes for use in size calculations.
pub const GB_268: usize = 287762808832;

/// 269GB in bytes.
/// This constant represents 269 gb converted to bytes for use in size calculations.
pub const GB_269: usize = 288836550656;

/// 270GB in bytes.
/// This constant represents 270 gb converted to bytes for use in size calculations.
pub const GB_270: usize = 289910292480;

/// 271GB in bytes.
/// This constant represents 271 gb converted to bytes for use in size calculations.
pub const GB_271: usize = 290984034304;

/// 272GB in bytes.
/// This constant represents 272 gb converted to bytes for use in size calculations.
pub const GB_272: usize = 292057776128;

/// 273GB in bytes.
/// This constant represents 273 gb converted to bytes for use in size calculations.
pub const GB_273: usize = 293131517952;

/// 274GB in bytes.
/// This constant represents 274 gb converted to bytes for use in size calculations.
pub const GB_274: usize = 294205259776;

/// 275GB in bytes.
/// This constant represents 275 gb converted to bytes for use in size calculations.
pub const GB_275: usize = 295279001600;

/// 276GB in bytes.
/// This constant represents 276 gb converted to bytes for use in size calculations.
pub const GB_276: usize = 296352743424;

/// 277GB in bytes.
/// This constant represents 277 gb converted to bytes for use in size calculations.
pub const GB_277: usize = 297426485248;

/// 278GB in bytes.
/// This constant represents 278 gb converted to bytes for use in size calculations.
pub const GB_278: usize = 298500227072;

/// 279GB in bytes.
/// This constant represents 279 gb converted to bytes for use in size calculations.
pub const GB_279: usize = 299573968896;

/// 280GB in bytes.
/// This constant represents 280 gb converted to bytes for use in size calculations.
pub const GB_280: usize = 300647710720;

/// 281GB in bytes.
/// This constant represents 281 gb converted to bytes for use in size calculations.
pub const GB_281: usize = 301721452544;

/// 282GB in bytes.
/// This constant represents 282 gb converted to bytes for use in size calculations.
pub const GB_282: usize = 302795194368;

/// 283GB in bytes.
/// This constant represents 283 gb converted to bytes for use in size calculations.
pub const GB_283: usize = 303868936192;

/// 284GB in bytes.
/// This constant represents 284 gb converted to bytes for use in size calculations.
pub const GB_284: usize = 304942678016;

/// 285GB in bytes.
/// This constant represents 285 gb converted to bytes for use in size calculations.
pub const GB_285: usize = 306016419840;

/// 286GB in bytes.
/// This constant represents 286 gb converted to bytes for use in size calculations.
pub const GB_286: usize = 307090161664;

/// 287GB in bytes.
/// This constant represents 287 gb converted to bytes for use in size calculations.
pub const GB_287: usize = 308163903488;

/// 288GB in bytes.
/// This constant represents 288 gb converted to bytes for use in size calculations.
pub const GB_288: usize = 309237645312;

/// 289GB in bytes.
/// This constant represents 289 gb converted to bytes for use in size calculations.
pub const GB_289: usize = 310311387136;

/// 290GB in bytes.
/// This constant represents 290 gb converted to bytes for use in size calculations.
pub const GB_290: usize = 311385128960;

/// 291GB in bytes.
/// This constant represents 291 gb converted to bytes for use in size calculations.
pub const GB_291: usize = 312458870784;

/// 292GB in bytes.
/// This constant represents 292 gb converted to bytes for use in size calculations.
pub const GB_292: usize = 313532612608;

/// 293GB in bytes.
/// This constant represents 293 gb converted to bytes for use in size calculations.
pub const GB_293: usize = 314606354432;

/// 294GB in bytes.
/// This constant represents 294 gb converted to bytes for use in size calculations.
pub const GB_294: usize = 315680096256;

/// 295GB in bytes.
/// This constant represents 295 gb converted to bytes for use in size calculations.
pub const GB_295: usize = 316753838080;

/// 296GB in bytes.
/// This constant represents 296 gb converted to bytes for use in size calculations.
pub const GB_296: usize = 317827579904;

/// 297GB in bytes.
/// This constant represents 297 gb converted to bytes for use in size calculations.
pub const GB_297: usize = 318901321728;

/// 298GB in bytes.
/// This constant represents 298 gb converted to bytes for use in size calculations.
pub const GB_298: usize = 319975063552;

/// 299GB in bytes.
/// This constant represents 299 gb converted to bytes for use in size calculations.
pub const GB_299: usize = 321048805376;

/// 300GB in bytes.
/// This constant represents 300 gb converted to bytes for use in size calculations.
pub const GB_300: usize = 322122547200;

/// 301GB in bytes.
/// This constant represents 301 gb converted to bytes for use in size calculations.
pub const GB_301: usize = 323196289024;

/// 302GB in bytes.
/// This constant represents 302 gb converted to bytes for use in size calculations.
pub const GB_302: usize = 324270030848;

/// 303GB in bytes.
/// This constant represents 303 gb converted to bytes for use in size calculations.
pub const GB_303: usize = 325343772672;

/// 304GB in bytes.
/// This constant represents 304 gb converted to bytes for use in size calculations.
pub const GB_304: usize = 326417514496;

/// 305GB in bytes.
/// This constant represents 305 gb converted to bytes for use in size calculations.
pub const GB_305: usize = 327491256320;

/// 306GB in bytes.
/// This constant represents 306 gb converted to bytes for use in size calculations.
pub const GB_306: usize = 328564998144;

/// 307GB in bytes.
/// This constant represents 307 gb converted to bytes for use in size calculations.
pub const GB_307: usize = 329638739968;

/// 308GB in bytes.
/// This constant represents 308 gb converted to bytes for use in size calculations.
pub const GB_308: usize = 330712481792;

/// 309GB in bytes.
/// This constant represents 309 gb converted to bytes for use in size calculations.
pub const GB_309: usize = 331786223616;

/// 310GB in bytes.
/// This constant represents 310 gb converted to bytes for use in size calculations.
pub const GB_310: usize = 332859965440;

/// 311GB in bytes.
/// This constant represents 311 gb converted to bytes for use in size calculations.
pub const GB_311: usize = 333933707264;

/// 312GB in bytes.
/// This constant represents 312 gb converted to bytes for use in size calculations.
pub const GB_312: usize = 335007449088;

/// 313GB in bytes.
/// This constant represents 313 gb converted to bytes for use in size calculations.
pub const GB_313: usize = 336081190912;

/// 314GB in bytes.
/// This constant represents 314 gb converted to bytes for use in size calculations.
pub const GB_314: usize = 337154932736;

/// 315GB in bytes.
/// This constant represents 315 gb converted to bytes for use in size calculations.
pub const GB_315: usize = 338228674560;

/// 316GB in bytes.
/// This constant represents 316 gb converted to bytes for use in size calculations.
pub const GB_316: usize = 339302416384;

/// 317GB in bytes.
/// This constant represents 317 gb converted to bytes for use in size calculations.
pub const GB_317: usize = 340376158208;

/// 318GB in bytes.
/// This constant represents 318 gb converted to bytes for use in size calculations.
pub const GB_318: usize = 341449900032;

/// 319GB in bytes.
/// This constant represents 319 gb converted to bytes for use in size calculations.
pub const GB_319: usize = 342523641856;

/// 320GB in bytes.
/// This constant represents 320 gb converted to bytes for use in size calculations.
pub const GB_320: usize = 343597383680;

/// 321GB in bytes.
/// This constant represents 321 gb converted to bytes for use in size calculations.
pub const GB_321: usize = 344671125504;

/// 322GB in bytes.
/// This constant represents 322 gb converted to bytes for use in size calculations.
pub const GB_322: usize = 345744867328;

/// 323GB in bytes.
/// This constant represents 323 gb converted to bytes for use in size calculations.
pub const GB_323: usize = 346818609152;

/// 324GB in bytes.
/// This constant represents 324 gb converted to bytes for use in size calculations.
pub const GB_324: usize = 347892350976;

/// 325GB in bytes.
/// This constant represents 325 gb converted to bytes for use in size calculations.
pub const GB_325: usize = 348966092800;

/// 326GB in bytes.
/// This constant represents 326 gb converted to bytes for use in size calculations.
pub const GB_326: usize = 350039834624;

/// 327GB in bytes.
/// This constant represents 327 gb converted to bytes for use in size calculations.
pub const GB_327: usize = 351113576448;

/// 328GB in bytes.
/// This constant represents 328 gb converted to bytes for use in size calculations.
pub const GB_328: usize = 352187318272;

/// 329GB in bytes.
/// This constant represents 329 gb converted to bytes for use in size calculations.
pub const GB_329: usize = 353261060096;

/// 330GB in bytes.
/// This constant represents 330 gb converted to bytes for use in size calculations.
pub const GB_330: usize = 354334801920;

/// 331GB in bytes.
/// This constant represents 331 gb converted to bytes for use in size calculations.
pub const GB_331: usize = 355408543744;

/// 332GB in bytes.
/// This constant represents 332 gb converted to bytes for use in size calculations.
pub const GB_332: usize = 356482285568;

/// 333GB in bytes.
/// This constant represents 333 gb converted to bytes for use in size calculations.
pub const GB_333: usize = 357556027392;

/// 334GB in bytes.
/// This constant represents 334 gb converted to bytes for use in size calculations.
pub const GB_334: usize = 358629769216;

/// 335GB in bytes.
/// This constant represents 335 gb converted to bytes for use in size calculations.
pub const GB_335: usize = 359703511040;

/// 336GB in bytes.
/// This constant represents 336 gb converted to bytes for use in size calculations.
pub const GB_336: usize = 360777252864;

/// 337GB in bytes.
/// This constant represents 337 gb converted to bytes for use in size calculations.
pub const GB_337: usize = 361850994688;

/// 338GB in bytes.
/// This constant represents 338 gb converted to bytes for use in size calculations.
pub const GB_338: usize = 362924736512;

/// 339GB in bytes.
/// This constant represents 339 gb converted to bytes for use in size calculations.
pub const GB_339: usize = 363998478336;

/// 340GB in bytes.
/// This constant represents 340 gb converted to bytes for use in size calculations.
pub const GB_340: usize = 365072220160;

/// 341GB in bytes.
/// This constant represents 341 gb converted to bytes for use in size calculations.
pub const GB_341: usize = 366145961984;

/// 342GB in bytes.
/// This constant represents 342 gb converted to bytes for use in size calculations.
pub const GB_342: usize = 367219703808;

/// 343GB in bytes.
/// This constant represents 343 gb converted to bytes for use in size calculations.
pub const GB_343: usize = 368293445632;

/// 344GB in bytes.
/// This constant represents 344 gb converted to bytes for use in size calculations.
pub const GB_344: usize = 369367187456;

/// 345GB in bytes.
/// This constant represents 345 gb converted to bytes for use in size calculations.
pub const GB_345: usize = 370440929280;

/// 346GB in bytes.
/// This constant represents 346 gb converted to bytes for use in size calculations.
pub const GB_346: usize = 371514671104;

/// 347GB in bytes.
/// This constant represents 347 gb converted to bytes for use in size calculations.
pub const GB_347: usize = 372588412928;

/// 348GB in bytes.
/// This constant represents 348 gb converted to bytes for use in size calculations.
pub const GB_348: usize = 373662154752;

/// 349GB in bytes.
/// This constant represents 349 gb converted to bytes for use in size calculations.
pub const GB_349: usize = 374735896576;

/// 350GB in bytes.
/// This constant represents 350 gb converted to bytes for use in size calculations.
pub const GB_350: usize = 375809638400;

/// 351GB in bytes.
/// This constant represents 351 gb converted to bytes for use in size calculations.
pub const GB_351: usize = 376883380224;

/// 352GB in bytes.
/// This constant represents 352 gb converted to bytes for use in size calculations.
pub const GB_352: usize = 377957122048;

/// 353GB in bytes.
/// This constant represents 353 gb converted to bytes for use in size calculations.
pub const GB_353: usize = 379030863872;

/// 354GB in bytes.
/// This constant represents 354 gb converted to bytes for use in size calculations.
pub const GB_354: usize = 380104605696;

/// 355GB in bytes.
/// This constant represents 355 gb converted to bytes for use in size calculations.
pub const GB_355: usize = 381178347520;

/// 356GB in bytes.
/// This constant represents 356 gb converted to bytes for use in size calculations.
pub const GB_356: usize = 382252089344;

/// 357GB in bytes.
/// This constant represents 357 gb converted to bytes for use in size calculations.
pub const GB_357: usize = 383325831168;

/// 358GB in bytes.
/// This constant represents 358 gb converted to bytes for use in size calculations.
pub const GB_358: usize = 384399572992;

/// 359GB in bytes.
/// This constant represents 359 gb converted to bytes for use in size calculations.
pub const GB_359: usize = 385473314816;

/// 360GB in bytes.
/// This constant represents 360 gb converted to bytes for use in size calculations.
pub const GB_360: usize = 386547056640;

/// 361GB in bytes.
/// This constant represents 361 gb converted to bytes for use in size calculations.
pub const GB_361: usize = 387620798464;

/// 362GB in bytes.
/// This constant represents 362 gb converted to bytes for use in size calculations.
pub const GB_362: usize = 388694540288;

/// 363GB in bytes.
/// This constant represents 363 gb converted to bytes for use in size calculations.
pub const GB_363: usize = 389768282112;

/// 364GB in bytes.
/// This constant represents 364 gb converted to bytes for use in size calculations.
pub const GB_364: usize = 390842023936;

/// 365GB in bytes.
/// This constant represents 365 gb converted to bytes for use in size calculations.
pub const GB_365: usize = 391915765760;

/// 366GB in bytes.
/// This constant represents 366 gb converted to bytes for use in size calculations.
pub const GB_366: usize = 392989507584;

/// 367GB in bytes.
/// This constant represents 367 gb converted to bytes for use in size calculations.
pub const GB_367: usize = 394063249408;

/// 368GB in bytes.
/// This constant represents 368 gb converted to bytes for use in size calculations.
pub const GB_368: usize = 395136991232;

/// 369GB in bytes.
/// This constant represents 369 gb converted to bytes for use in size calculations.
pub const GB_369: usize = 396210733056;

/// 370GB in bytes.
/// This constant represents 370 gb converted to bytes for use in size calculations.
pub const GB_370: usize = 397284474880;

/// 371GB in bytes.
/// This constant represents 371 gb converted to bytes for use in size calculations.
pub const GB_371: usize = 398358216704;

/// 372GB in bytes.
/// This constant represents 372 gb converted to bytes for use in size calculations.
pub const GB_372: usize = 399431958528;

/// 373GB in bytes.
/// This constant represents 373 gb converted to bytes for use in size calculations.
pub const GB_373: usize = 400505700352;

/// 374GB in bytes.
/// This constant represents 374 gb converted to bytes for use in size calculations.
pub const GB_374: usize = 401579442176;

/// 375GB in bytes.
/// This constant represents 375 gb converted to bytes for use in size calculations.
pub const GB_375: usize = 402653184000;

/// 376GB in bytes.
/// This constant represents 376 gb converted to bytes for use in size calculations.
pub const GB_376: usize = 403726925824;

/// 377GB in bytes.
/// This constant represents 377 gb converted to bytes for use in size calculations.
pub const GB_377: usize = 404800667648;

/// 378GB in bytes.
/// This constant represents 378 gb converted to bytes for use in size calculations.
pub const GB_378: usize = 405874409472;

/// 379GB in bytes.
/// This constant represents 379 gb converted to bytes for use in size calculations.
pub const GB_379: usize = 406948151296;

/// 380GB in bytes.
/// This constant represents 380 gb converted to bytes for use in size calculations.
pub const GB_380: usize = 408021893120;

/// 381GB in bytes.
/// This constant represents 381 gb converted to bytes for use in size calculations.
pub const GB_381: usize = 409095634944;

/// 382GB in bytes.
/// This constant represents 382 gb converted to bytes for use in size calculations.
pub const GB_382: usize = 410169376768;

/// 383GB in bytes.
/// This constant represents 383 gb converted to bytes for use in size calculations.
pub const GB_383: usize = 411243118592;

/// 384GB in bytes.
/// This constant represents 384 gb converted to bytes for use in size calculations.
pub const GB_384: usize = 412316860416;

/// 385GB in bytes.
/// This constant represents 385 gb converted to bytes for use in size calculations.
pub const GB_385: usize = 413390602240;

/// 386GB in bytes.
/// This constant represents 386 gb converted to bytes for use in size calculations.
pub const GB_386: usize = 414464344064;

/// 387GB in bytes.
/// This constant represents 387 gb converted to bytes for use in size calculations.
pub const GB_387: usize = 415538085888;

/// 388GB in bytes.
/// This constant represents 388 gb converted to bytes for use in size calculations.
pub const GB_388: usize = 416611827712;

/// 389GB in bytes.
/// This constant represents 389 gb converted to bytes for use in size calculations.
pub const GB_389: usize = 417685569536;

/// 390GB in bytes.
/// This constant represents 390 gb converted to bytes for use in size calculations.
pub const GB_390: usize = 418759311360;

/// 391GB in bytes.
/// This constant represents 391 gb converted to bytes for use in size calculations.
pub const GB_391: usize = 419833053184;

/// 392GB in bytes.
/// This constant represents 392 gb converted to bytes for use in size calculations.
pub const GB_392: usize = 420906795008;

/// 393GB in bytes.
/// This constant represents 393 gb converted to bytes for use in size calculations.
pub const GB_393: usize = 421980536832;

/// 394GB in bytes.
/// This constant represents 394 gb converted to bytes for use in size calculations.
pub const GB_394: usize = 423054278656;

/// 395GB in bytes.
/// This constant represents 395 gb converted to bytes for use in size calculations.
pub const GB_395: usize = 424128020480;

/// 396GB in bytes.
/// This constant represents 396 gb converted to bytes for use in size calculations.
pub const GB_396: usize = 425201762304;

/// 397GB in bytes.
/// This constant represents 397 gb converted to bytes for use in size calculations.
pub const GB_397: usize = 426275504128;

/// 398GB in bytes.
/// This constant represents 398 gb converted to bytes for use in size calculations.
pub const GB_398: usize = 427349245952;

/// 399GB in bytes.
/// This constant represents 399 gb converted to bytes for use in size calculations.
pub const GB_399: usize = 428422987776;

/// 400GB in bytes.
/// This constant represents 400 gb converted to bytes for use in size calculations.
pub const GB_400: usize = 429496729600;

/// 401GB in bytes.
/// This constant represents 401 gb converted to bytes for use in size calculations.
pub const GB_401: usize = 430570471424;

/// 402GB in bytes.
/// This constant represents 402 gb converted to bytes for use in size calculations.
pub const GB_402: usize = 431644213248;

/// 403GB in bytes.
/// This constant represents 403 gb converted to bytes for use in size calculations.
pub const GB_403: usize = 432717955072;

/// 404GB in bytes.
/// This constant represents 404 gb converted to bytes for use in size calculations.
pub const GB_404: usize = 433791696896;

/// 405GB in bytes.
/// This constant represents 405 gb converted to bytes for use in size calculations.
pub const GB_405: usize = 434865438720;

/// 406GB in bytes.
/// This constant represents 406 gb converted to bytes for use in size calculations.
pub const GB_406: usize = 435939180544;

/// 407GB in bytes.
/// This constant represents 407 gb converted to bytes for use in size calculations.
pub const GB_407: usize = 437012922368;

/// 408GB in bytes.
/// This constant represents 408 gb converted to bytes for use in size calculations.
pub const GB_408: usize = 438086664192;

/// 409GB in bytes.
/// This constant represents 409 gb converted to bytes for use in size calculations.
pub const GB_409: usize = 439160406016;

/// 410GB in bytes.
/// This constant represents 410 gb converted to bytes for use in size calculations.
pub const GB_410: usize = 440234147840;

/// 411GB in bytes.
/// This constant represents 411 gb converted to bytes for use in size calculations.
pub const GB_411: usize = 441307889664;

/// 412GB in bytes.
/// This constant represents 412 gb converted to bytes for use in size calculations.
pub const GB_412: usize = 442381631488;

/// 413GB in bytes.
/// This constant represents 413 gb converted to bytes for use in size calculations.
pub const GB_413: usize = 443455373312;

/// 414GB in bytes.
/// This constant represents 414 gb converted to bytes for use in size calculations.
pub const GB_414: usize = 444529115136;

/// 415GB in bytes.
/// This constant represents 415 gb converted to bytes for use in size calculations.
pub const GB_415: usize = 445602856960;

/// 416GB in bytes.
/// This constant represents 416 gb converted to bytes for use in size calculations.
pub const GB_416: usize = 446676598784;

/// 417GB in bytes.
/// This constant represents 417 gb converted to bytes for use in size calculations.
pub const GB_417: usize = 447750340608;

/// 418GB in bytes.
/// This constant represents 418 gb converted to bytes for use in size calculations.
pub const GB_418: usize = 448824082432;

/// 419GB in bytes.
/// This constant represents 419 gb converted to bytes for use in size calculations.
pub const GB_419: usize = 449897824256;

/// 420GB in bytes.
/// This constant represents 420 gb converted to bytes for use in size calculations.
pub const GB_420: usize = 450971566080;

/// 421GB in bytes.
/// This constant represents 421 gb converted to bytes for use in size calculations.
pub const GB_421: usize = 452045307904;

/// 422GB in bytes.
/// This constant represents 422 gb converted to bytes for use in size calculations.
pub const GB_422: usize = 453119049728;

/// 423GB in bytes.
/// This constant represents 423 gb converted to bytes for use in size calculations.
pub const GB_423: usize = 454192791552;

/// 424GB in bytes.
/// This constant represents 424 gb converted to bytes for use in size calculations.
pub const GB_424: usize = 455266533376;

/// 425GB in bytes.
/// This constant represents 425 gb converted to bytes for use in size calculations.
pub const GB_425: usize = 456340275200;

/// 426GB in bytes.
/// This constant represents 426 gb converted to bytes for use in size calculations.
pub const GB_426: usize = 457414017024;

/// 427GB in bytes.
/// This constant represents 427 gb converted to bytes for use in size calculations.
pub const GB_427: usize = 458487758848;

/// 428GB in bytes.
/// This constant represents 428 gb converted to bytes for use in size calculations.
pub const GB_428: usize = 459561500672;

/// 429GB in bytes.
/// This constant represents 429 gb converted to bytes for use in size calculations.
pub const GB_429: usize = 460635242496;

/// 430GB in bytes.
/// This constant represents 430 gb converted to bytes for use in size calculations.
pub const GB_430: usize = 461708984320;

/// 431GB in bytes.
/// This constant represents 431 gb converted to bytes for use in size calculations.
pub const GB_431: usize = 462782726144;

/// 432GB in bytes.
/// This constant represents 432 gb converted to bytes for use in size calculations.
pub const GB_432: usize = 463856467968;

/// 433GB in bytes.
/// This constant represents 433 gb converted to bytes for use in size calculations.
pub const GB_433: usize = 464930209792;

/// 434GB in bytes.
/// This constant represents 434 gb converted to bytes for use in size calculations.
pub const GB_434: usize = 466003951616;

/// 435GB in bytes.
/// This constant represents 435 gb converted to bytes for use in size calculations.
pub const GB_435: usize = 467077693440;

/// 436GB in bytes.
/// This constant represents 436 gb converted to bytes for use in size calculations.
pub const GB_436: usize = 468151435264;

/// 437GB in bytes.
/// This constant represents 437 gb converted to bytes for use in size calculations.
pub const GB_437: usize = 469225177088;

/// 438GB in bytes.
/// This constant represents 438 gb converted to bytes for use in size calculations.
pub const GB_438: usize = 470298918912;

/// 439GB in bytes.
/// This constant represents 439 gb converted to bytes for use in size calculations.
pub const GB_439: usize = 471372660736;

/// 440GB in bytes.
/// This constant represents 440 gb converted to bytes for use in size calculations.
pub const GB_440: usize = 472446402560;

/// 441GB in bytes.
/// This constant represents 441 gb converted to bytes for use in size calculations.
pub const GB_441: usize = 473520144384;

/// 442GB in bytes.
/// This constant represents 442 gb converted to bytes for use in size calculations.
pub const GB_442: usize = 474593886208;

/// 443GB in bytes.
/// This constant represents 443 gb converted to bytes for use in size calculations.
pub const GB_443: usize = 475667628032;

/// 444GB in bytes.
/// This constant represents 444 gb converted to bytes for use in size calculations.
pub const GB_444: usize = 476741369856;

/// 445GB in bytes.
/// This constant represents 445 gb converted to bytes for use in size calculations.
pub const GB_445: usize = 477815111680;

/// 446GB in bytes.
/// This constant represents 446 gb converted to bytes for use in size calculations.
pub const GB_446: usize = 478888853504;

/// 447GB in bytes.
/// This constant represents 447 gb converted to bytes for use in size calculations.
pub const GB_447: usize = 479962595328;

/// 448GB in bytes.
/// This constant represents 448 gb converted to bytes for use in size calculations.
pub const GB_448: usize = 481036337152;

/// 449GB in bytes.
/// This constant represents 449 gb converted to bytes for use in size calculations.
pub const GB_449: usize = 482110078976;

/// 450GB in bytes.
/// This constant represents 450 gb converted to bytes for use in size calculations.
pub const GB_450: usize = 483183820800;

/// 451GB in bytes.
/// This constant represents 451 gb converted to bytes for use in size calculations.
pub const GB_451: usize = 484257562624;

/// 452GB in bytes.
/// This constant represents 452 gb converted to bytes for use in size calculations.
pub const GB_452: usize = 485331304448;

/// 453GB in bytes.
/// This constant represents 453 gb converted to bytes for use in size calculations.
pub const GB_453: usize = 486405046272;

/// 454GB in bytes.
/// This constant represents 454 gb converted to bytes for use in size calculations.
pub const GB_454: usize = 487478788096;

/// 455GB in bytes.
/// This constant represents 455 gb converted to bytes for use in size calculations.
pub const GB_455: usize = 488552529920;

/// 456GB in bytes.
/// This constant represents 456 gb converted to bytes for use in size calculations.
pub const GB_456: usize = 489626271744;

/// 457GB in bytes.
/// This constant represents 457 gb converted to bytes for use in size calculations.
pub const GB_457: usize = 490700013568;

/// 458GB in bytes.
/// This constant represents 458 gb converted to bytes for use in size calculations.
pub const GB_458: usize = 491773755392;

/// 459GB in bytes.
/// This constant represents 459 gb converted to bytes for use in size calculations.
pub const GB_459: usize = 492847497216;

/// 460GB in bytes.
/// This constant represents 460 gb converted to bytes for use in size calculations.
pub const GB_460: usize = 493921239040;

/// 461GB in bytes.
/// This constant represents 461 gb converted to bytes for use in size calculations.
pub const GB_461: usize = 494994980864;

/// 462GB in bytes.
/// This constant represents 462 gb converted to bytes for use in size calculations.
pub const GB_462: usize = 496068722688;

/// 463GB in bytes.
/// This constant represents 463 gb converted to bytes for use in size calculations.
pub const GB_463: usize = 497142464512;

/// 464GB in bytes.
/// This constant represents 464 gb converted to bytes for use in size calculations.
pub const GB_464: usize = 498216206336;

/// 465GB in bytes.
/// This constant represents 465 gb converted to bytes for use in size calculations.
pub const GB_465: usize = 499289948160;

/// 466GB in bytes.
/// This constant represents 466 gb converted to bytes for use in size calculations.
pub const GB_466: usize = 500363689984;

/// 467GB in bytes.
/// This constant represents 467 gb converted to bytes for use in size calculations.
pub const GB_467: usize = 501437431808;

/// 468GB in bytes.
/// This constant represents 468 gb converted to bytes for use in size calculations.
pub const GB_468: usize = 502511173632;

/// 469GB in bytes.
/// This constant represents 469 gb converted to bytes for use in size calculations.
pub const GB_469: usize = 503584915456;

/// 470GB in bytes.
/// This constant represents 470 gb converted to bytes for use in size calculations.
pub const GB_470: usize = 504658657280;

/// 471GB in bytes.
/// This constant represents 471 gb converted to bytes for use in size calculations.
pub const GB_471: usize = 505732399104;

/// 472GB in bytes.
/// This constant represents 472 gb converted to bytes for use in size calculations.
pub const GB_472: usize = 506806140928;

/// 473GB in bytes.
/// This constant represents 473 gb converted to bytes for use in size calculations.
pub const GB_473: usize = 507879882752;

/// 474GB in bytes.
/// This constant represents 474 gb converted to bytes for use in size calculations.
pub const GB_474: usize = 508953624576;

/// 475GB in bytes.
/// This constant represents 475 gb converted to bytes for use in size calculations.
pub const GB_475: usize = 510027366400;

/// 476GB in bytes.
/// This constant represents 476 gb converted to bytes for use in size calculations.
pub const GB_476: usize = 511101108224;

/// 477GB in bytes.
/// This constant represents 477 gb converted to bytes for use in size calculations.
pub const GB_477: usize = 512174850048;

/// 478GB in bytes.
/// This constant represents 478 gb converted to bytes for use in size calculations.
pub const GB_478: usize = 513248591872;

/// 479GB in bytes.
/// This constant represents 479 gb converted to bytes for use in size calculations.
pub const GB_479: usize = 514322333696;

/// 480GB in bytes.
/// This constant represents 480 gb converted to bytes for use in size calculations.
pub const GB_480: usize = 515396075520;

/// 481GB in bytes.
/// This constant represents 481 gb converted to bytes for use in size calculations.
pub const GB_481: usize = 516469817344;

/// 482GB in bytes.
/// This constant represents 482 gb converted to bytes for use in size calculations.
pub const GB_482: usize = 517543559168;

/// 483GB in bytes.
/// This constant represents 483 gb converted to bytes for use in size calculations.
pub const GB_483: usize = 518617300992;

/// 484GB in bytes.
/// This constant represents 484 gb converted to bytes for use in size calculations.
pub const GB_484: usize = 519691042816;

/// 485GB in bytes.
/// This constant represents 485 gb converted to bytes for use in size calculations.
pub const GB_485: usize = 520764784640;

/// 486GB in bytes.
/// This constant represents 486 gb converted to bytes for use in size calculations.
pub const GB_486: usize = 521838526464;

/// 487GB in bytes.
/// This constant represents 487 gb converted to bytes for use in size calculations.
pub const GB_487: usize = 522912268288;

/// 488GB in bytes.
/// This constant represents 488 gb converted to bytes for use in size calculations.
pub const GB_488: usize = 523986010112;

/// 489GB in bytes.
/// This constant represents 489 gb converted to bytes for use in size calculations.
pub const GB_489: usize = 525059751936;

/// 490GB in bytes.
/// This constant represents 490 gb converted to bytes for use in size calculations.
pub const GB_490: usize = 526133493760;

/// 491GB in bytes.
/// This constant represents 491 gb converted to bytes for use in size calculations.
pub const GB_491: usize = 527207235584;

/// 492GB in bytes.
/// This constant represents 492 gb converted to bytes for use in size calculations.
pub const GB_492: usize = 528280977408;

/// 493GB in bytes.
/// This constant represents 493 gb converted to bytes for use in size calculations.
pub const GB_493: usize = 529354719232;

/// 494GB in bytes.
/// This constant represents 494 gb converted to bytes for use in size calculations.
pub const GB_494: usize = 530428461056;

/// 495GB in bytes.
/// This constant represents 495 gb converted to bytes for use in size calculations.
pub const GB_495: usize = 531502202880;

/// 496GB in bytes.
/// This constant represents 496 gb converted to bytes for use in size calculations.
pub const GB_496: usize = 532575944704;

/// 497GB in bytes.
/// This constant represents 497 gb converted to bytes for use in size calculations.
pub const GB_497: usize = 533649686528;

/// 498GB in bytes.
/// This constant represents 498 gb converted to bytes for use in size calculations.
pub const GB_498: usize = 534723428352;

/// 499GB in bytes.
/// This constant represents 499 gb converted to bytes for use in size calculations.
pub const GB_499: usize = 535797170176;

/// 500GB in bytes.
/// This constant represents 500 gb converted to bytes for use in size calculations.
pub const GB_500: usize = 536870912000;

/// 501GB in bytes.
/// This constant represents 501 gb converted to bytes for use in size calculations.
pub const GB_501: usize = 537944653824;

/// 502GB in bytes.
/// This constant represents 502 gb converted to bytes for use in size calculations.
pub const GB_502: usize = 539018395648;

/// 503GB in bytes.
/// This constant represents 503 gb converted to bytes for use in size calculations.
pub const GB_503: usize = 540092137472;

/// 504GB in bytes.
/// This constant represents 504 gb converted to bytes for use in size calculations.
pub const GB_504: usize = 541165879296;

/// 505GB in bytes.
/// This constant represents 505 gb converted to bytes for use in size calculations.
pub const GB_505: usize = 542239621120;

/// 506GB in bytes.
/// This constant represents 506 gb converted to bytes for use in size calculations.
pub const GB_506: usize = 543313362944;

/// 507GB in bytes.
/// This constant represents 507 gb converted to bytes for use in size calculations.
pub const GB_507: usize = 544387104768;

/// 508GB in bytes.
/// This constant represents 508 gb converted to bytes for use in size calculations.
pub const GB_508: usize = 545460846592;

/// 509GB in bytes.
/// This constant represents 509 gb converted to bytes for use in size calculations.
pub const GB_509: usize = 546534588416;

/// 510GB in bytes.
/// This constant represents 510 gb converted to bytes for use in size calculations.
pub const GB_510: usize = 547608330240;

/// 511GB in bytes.
/// This constant represents 511 gb converted to bytes for use in size calculations.
pub const GB_511: usize = 548682072064;

/// 512GB in bytes.
/// This constant represents 512 gb converted to bytes for use in size calculations.
pub const GB_512: usize = 549755813888;

/// 513GB in bytes.
/// This constant represents 513 gb converted to bytes for use in size calculations.
pub const GB_513: usize = 550829555712;

/// 514GB in bytes.
/// This constant represents 514 gb converted to bytes for use in size calculations.
pub const GB_514: usize = 551903297536;

/// 515GB in bytes.
/// This constant represents 515 gb converted to bytes for use in size calculations.
pub const GB_515: usize = 552977039360;

/// 516GB in bytes.
/// This constant represents 516 gb converted to bytes for use in size calculations.
pub const GB_516: usize = 554050781184;

/// 517GB in bytes.
/// This constant represents 517 gb converted to bytes for use in size calculations.
pub const GB_517: usize = 555124523008;

/// 518GB in bytes.
/// This constant represents 518 gb converted to bytes for use in size calculations.
pub const GB_518: usize = 556198264832;

/// 519GB in bytes.
/// This constant represents 519 gb converted to bytes for use in size calculations.
pub const GB_519: usize = 557272006656;

/// 520GB in bytes.
/// This constant represents 520 gb converted to bytes for use in size calculations.
pub const GB_520: usize = 558345748480;

/// 521GB in bytes.
/// This constant represents 521 gb converted to bytes for use in size calculations.
pub const GB_521: usize = 559419490304;

/// 522GB in bytes.
/// This constant represents 522 gb converted to bytes for use in size calculations.
pub const GB_522: usize = 560493232128;

/// 523GB in bytes.
/// This constant represents 523 gb converted to bytes for use in size calculations.
pub const GB_523: usize = 561566973952;

/// 524GB in bytes.
/// This constant represents 524 gb converted to bytes for use in size calculations.
pub const GB_524: usize = 562640715776;

/// 525GB in bytes.
/// This constant represents 525 gb converted to bytes for use in size calculations.
pub const GB_525: usize = 563714457600;

/// 526GB in bytes.
/// This constant represents 526 gb converted to bytes for use in size calculations.
pub const GB_526: usize = 564788199424;

/// 527GB in bytes.
/// This constant represents 527 gb converted to bytes for use in size calculations.
pub const GB_527: usize = 565861941248;

/// 528GB in bytes.
/// This constant represents 528 gb converted to bytes for use in size calculations.
pub const GB_528: usize = 566935683072;

/// 529GB in bytes.
/// This constant represents 529 gb converted to bytes for use in size calculations.
pub const GB_529: usize = 568009424896;

/// 530GB in bytes.
/// This constant represents 530 gb converted to bytes for use in size calculations.
pub const GB_530: usize = 569083166720;

/// 531GB in bytes.
/// This constant represents 531 gb converted to bytes for use in size calculations.
pub const GB_531: usize = 570156908544;

/// 532GB in bytes.
/// This constant represents 532 gb converted to bytes for use in size calculations.
pub const GB_532: usize = 571230650368;

/// 533GB in bytes.
/// This constant represents 533 gb converted to bytes for use in size calculations.
pub const GB_533: usize = 572304392192;

/// 534GB in bytes.
/// This constant represents 534 gb converted to bytes for use in size calculations.
pub const GB_534: usize = 573378134016;

/// 535GB in bytes.
/// This constant represents 535 gb converted to bytes for use in size calculations.
pub const GB_535: usize = 574451875840;

/// 536GB in bytes.
/// This constant represents 536 gb converted to bytes for use in size calculations.
pub const GB_536: usize = 575525617664;

/// 537GB in bytes.
/// This constant represents 537 gb converted to bytes for use in size calculations.
pub const GB_537: usize = 576599359488;

/// 538GB in bytes.
/// This constant represents 538 gb converted to bytes for use in size calculations.
pub const GB_538: usize = 577673101312;

/// 539GB in bytes.
/// This constant represents 539 gb converted to bytes for use in size calculations.
pub const GB_539: usize = 578746843136;

/// 540GB in bytes.
/// This constant represents 540 gb converted to bytes for use in size calculations.
pub const GB_540: usize = 579820584960;

/// 541GB in bytes.
/// This constant represents 541 gb converted to bytes for use in size calculations.
pub const GB_541: usize = 580894326784;

/// 542GB in bytes.
/// This constant represents 542 gb converted to bytes for use in size calculations.
pub const GB_542: usize = 581968068608;

/// 543GB in bytes.
/// This constant represents 543 gb converted to bytes for use in size calculations.
pub const GB_543: usize = 583041810432;

/// 544GB in bytes.
/// This constant represents 544 gb converted to bytes for use in size calculations.
pub const GB_544: usize = 584115552256;

/// 545GB in bytes.
/// This constant represents 545 gb converted to bytes for use in size calculations.
pub const GB_545: usize = 585189294080;

/// 546GB in bytes.
/// This constant represents 546 gb converted to bytes for use in size calculations.
pub const GB_546: usize = 586263035904;

/// 547GB in bytes.
/// This constant represents 547 gb converted to bytes for use in size calculations.
pub const GB_547: usize = 587336777728;

/// 548GB in bytes.
/// This constant represents 548 gb converted to bytes for use in size calculations.
pub const GB_548: usize = 588410519552;

/// 549GB in bytes.
/// This constant represents 549 gb converted to bytes for use in size calculations.
pub const GB_549: usize = 589484261376;

/// 550GB in bytes.
/// This constant represents 550 gb converted to bytes for use in size calculations.
pub const GB_550: usize = 590558003200;

/// 551GB in bytes.
/// This constant represents 551 gb converted to bytes for use in size calculations.
pub const GB_551: usize = 591631745024;

/// 552GB in bytes.
/// This constant represents 552 gb converted to bytes for use in size calculations.
pub const GB_552: usize = 592705486848;

/// 553GB in bytes.
/// This constant represents 553 gb converted to bytes for use in size calculations.
pub const GB_553: usize = 593779228672;

/// 554GB in bytes.
/// This constant represents 554 gb converted to bytes for use in size calculations.
pub const GB_554: usize = 594852970496;

/// 555GB in bytes.
/// This constant represents 555 gb converted to bytes for use in size calculations.
pub const GB_555: usize = 595926712320;

/// 556GB in bytes.
/// This constant represents 556 gb converted to bytes for use in size calculations.
pub const GB_556: usize = 597000454144;

/// 557GB in bytes.
/// This constant represents 557 gb converted to bytes for use in size calculations.
pub const GB_557: usize = 598074195968;

/// 558GB in bytes.
/// This constant represents 558 gb converted to bytes for use in size calculations.
pub const GB_558: usize = 599147937792;

/// 559GB in bytes.
/// This constant represents 559 gb converted to bytes for use in size calculations.
pub const GB_559: usize = 600221679616;

/// 560GB in bytes.
/// This constant represents 560 gb converted to bytes for use in size calculations.
pub const GB_560: usize = 601295421440;

/// 561GB in bytes.
/// This constant represents 561 gb converted to bytes for use in size calculations.
pub const GB_561: usize = 602369163264;

/// 562GB in bytes.
/// This constant represents 562 gb converted to bytes for use in size calculations.
pub const GB_562: usize = 603442905088;

/// 563GB in bytes.
/// This constant represents 563 gb converted to bytes for use in size calculations.
pub const GB_563: usize = 604516646912;

/// 564GB in bytes.
/// This constant represents 564 gb converted to bytes for use in size calculations.
pub const GB_564: usize = 605590388736;

/// 565GB in bytes.
/// This constant represents 565 gb converted to bytes for use in size calculations.
pub const GB_565: usize = 606664130560;

/// 566GB in bytes.
/// This constant represents 566 gb converted to bytes for use in size calculations.
pub const GB_566: usize = 607737872384;

/// 567GB in bytes.
/// This constant represents 567 gb converted to bytes for use in size calculations.
pub const GB_567: usize = 608811614208;

/// 568GB in bytes.
/// This constant represents 568 gb converted to bytes for use in size calculations.
pub const GB_568: usize = 609885356032;

/// 569GB in bytes.
/// This constant represents 569 gb converted to bytes for use in size calculations.
pub const GB_569: usize = 610959097856;

/// 570GB in bytes.
/// This constant represents 570 gb converted to bytes for use in size calculations.
pub const GB_570: usize = 612032839680;

/// 571GB in bytes.
/// This constant represents 571 gb converted to bytes for use in size calculations.
pub const GB_571: usize = 613106581504;

/// 572GB in bytes.
/// This constant represents 572 gb converted to bytes for use in size calculations.
pub const GB_572: usize = 614180323328;

/// 573GB in bytes.
/// This constant represents 573 gb converted to bytes for use in size calculations.
pub const GB_573: usize = 615254065152;

/// 574GB in bytes.
/// This constant represents 574 gb converted to bytes for use in size calculations.
pub const GB_574: usize = 616327806976;

/// 575GB in bytes.
/// This constant represents 575 gb converted to bytes for use in size calculations.
pub const GB_575: usize = 617401548800;

/// 576GB in bytes.
/// This constant represents 576 gb converted to bytes for use in size calculations.
pub const GB_576: usize = 618475290624;

/// 577GB in bytes.
/// This constant represents 577 gb converted to bytes for use in size calculations.
pub const GB_577: usize = 619549032448;

/// 578GB in bytes.
/// This constant represents 578 gb converted to bytes for use in size calculations.
pub const GB_578: usize = 620622774272;

/// 579GB in bytes.
/// This constant represents 579 gb converted to bytes for use in size calculations.
pub const GB_579: usize = 621696516096;

/// 580GB in bytes.
/// This constant represents 580 gb converted to bytes for use in size calculations.
pub const GB_580: usize = 622770257920;

/// 581GB in bytes.
/// This constant represents 581 gb converted to bytes for use in size calculations.
pub const GB_581: usize = 623843999744;

/// 582GB in bytes.
/// This constant represents 582 gb converted to bytes for use in size calculations.
pub const GB_582: usize = 624917741568;

/// 583GB in bytes.
/// This constant represents 583 gb converted to bytes for use in size calculations.
pub const GB_583: usize = 625991483392;

/// 584GB in bytes.
/// This constant represents 584 gb converted to bytes for use in size calculations.
pub const GB_584: usize = 627065225216;

/// 585GB in bytes.
/// This constant represents 585 gb converted to bytes for use in size calculations.
pub const GB_585: usize = 628138967040;

/// 586GB in bytes.
/// This constant represents 586 gb converted to bytes for use in size calculations.
pub const GB_586: usize = 629212708864;

/// 587GB in bytes.
/// This constant represents 587 gb converted to bytes for use in size calculations.
pub const GB_587: usize = 630286450688;

/// 588GB in bytes.
/// This constant represents 588 gb converted to bytes for use in size calculations.
pub const GB_588: usize = 631360192512;

/// 589GB in bytes.
/// This constant represents 589 gb converted to bytes for use in size calculations.
pub const GB_589: usize = 632433934336;

/// 590GB in bytes.
/// This constant represents 590 gb converted to bytes for use in size calculations.
pub const GB_590: usize = 633507676160;

/// 591GB in bytes.
/// This constant represents 591 gb converted to bytes for use in size calculations.
pub const GB_591: usize = 634581417984;

/// 592GB in bytes.
/// This constant represents 592 gb converted to bytes for use in size calculations.
pub const GB_592: usize = 635655159808;

/// 593GB in bytes.
/// This constant represents 593 gb converted to bytes for use in size calculations.
pub const GB_593: usize = 636728901632;

/// 594GB in bytes.
/// This constant represents 594 gb converted to bytes for use in size calculations.
pub const GB_594: usize = 637802643456;

/// 595GB in bytes.
/// This constant represents 595 gb converted to bytes for use in size calculations.
pub const GB_595: usize = 638876385280;

/// 596GB in bytes.
/// This constant represents 596 gb converted to bytes for use in size calculations.
pub const GB_596: usize = 639950127104;

/// 597GB in bytes.
/// This constant represents 597 gb converted to bytes for use in size calculations.
pub const GB_597: usize = 641023868928;

/// 598GB in bytes.
/// This constant represents 598 gb converted to bytes for use in size calculations.
pub const GB_598: usize = 642097610752;

/// 599GB in bytes.
/// This constant represents 599 gb converted to bytes for use in size calculations.
pub const GB_599: usize = 643171352576;

/// 600GB in bytes.
/// This constant represents 600 gb converted to bytes for use in size calculations.
pub const GB_600: usize = 644245094400;

/// 601GB in bytes.
/// This constant represents 601 gb converted to bytes for use in size calculations.
pub const GB_601: usize = 645318836224;

/// 602GB in bytes.
/// This constant represents 602 gb converted to bytes for use in size calculations.
pub const GB_602: usize = 646392578048;

/// 603GB in bytes.
/// This constant represents 603 gb converted to bytes for use in size calculations.
pub const GB_603: usize = 647466319872;

/// 604GB in bytes.
/// This constant represents 604 gb converted to bytes for use in size calculations.
pub const GB_604: usize = 648540061696;

/// 605GB in bytes.
/// This constant represents 605 gb converted to bytes for use in size calculations.
pub const GB_605: usize = 649613803520;

/// 606GB in bytes.
/// This constant represents 606 gb converted to bytes for use in size calculations.
pub const GB_606: usize = 650687545344;

/// 607GB in bytes.
/// This constant represents 607 gb converted to bytes for use in size calculations.
pub const GB_607: usize = 651761287168;

/// 608GB in bytes.
/// This constant represents 608 gb converted to bytes for use in size calculations.
pub const GB_608: usize = 652835028992;

/// 609GB in bytes.
/// This constant represents 609 gb converted to bytes for use in size calculations.
pub const GB_609: usize = 653908770816;

/// 610GB in bytes.
/// This constant represents 610 gb converted to bytes for use in size calculations.
pub const GB_610: usize = 654982512640;

/// 611GB in bytes.
/// This constant represents 611 gb converted to bytes for use in size calculations.
pub const GB_611: usize = 656056254464;

/// 612GB in bytes.
/// This constant represents 612 gb converted to bytes for use in size calculations.
pub const GB_612: usize = 657129996288;

/// 613GB in bytes.
/// This constant represents 613 gb converted to bytes for use in size calculations.
pub const GB_613: usize = 658203738112;

/// 614GB in bytes.
/// This constant represents 614 gb converted to bytes for use in size calculations.
pub const GB_614: usize = 659277479936;

/// 615GB in bytes.
/// This constant represents 615 gb converted to bytes for use in size calculations.
pub const GB_615: usize = 660351221760;

/// 616GB in bytes.
/// This constant represents 616 gb converted to bytes for use in size calculations.
pub const GB_616: usize = 661424963584;

/// 617GB in bytes.
/// This constant represents 617 gb converted to bytes for use in size calculations.
pub const GB_617: usize = 662498705408;

/// 618GB in bytes.
/// This constant represents 618 gb converted to bytes for use in size calculations.
pub const GB_618: usize = 663572447232;

/// 619GB in bytes.
/// This constant represents 619 gb converted to bytes for use in size calculations.
pub const GB_619: usize = 664646189056;

/// 620GB in bytes.
/// This constant represents 620 gb converted to bytes for use in size calculations.
pub const GB_620: usize = 665719930880;

/// 621GB in bytes.
/// This constant represents 621 gb converted to bytes for use in size calculations.
pub const GB_621: usize = 666793672704;

/// 622GB in bytes.
/// This constant represents 622 gb converted to bytes for use in size calculations.
pub const GB_622: usize = 667867414528;

/// 623GB in bytes.
/// This constant represents 623 gb converted to bytes for use in size calculations.
pub const GB_623: usize = 668941156352;

/// 624GB in bytes.
/// This constant represents 624 gb converted to bytes for use in size calculations.
pub const GB_624: usize = 670014898176;

/// 625GB in bytes.
/// This constant represents 625 gb converted to bytes for use in size calculations.
pub const GB_625: usize = 671088640000;

/// 626GB in bytes.
/// This constant represents 626 gb converted to bytes for use in size calculations.
pub const GB_626: usize = 672162381824;

/// 627GB in bytes.
/// This constant represents 627 gb converted to bytes for use in size calculations.
pub const GB_627: usize = 673236123648;

/// 628GB in bytes.
/// This constant represents 628 gb converted to bytes for use in size calculations.
pub const GB_628: usize = 674309865472;

/// 629GB in bytes.
/// This constant represents 629 gb converted to bytes for use in size calculations.
pub const GB_629: usize = 675383607296;

/// 630GB in bytes.
/// This constant represents 630 gb converted to bytes for use in size calculations.
pub const GB_630: usize = 676457349120;

/// 631GB in bytes.
/// This constant represents 631 gb converted to bytes for use in size calculations.
pub const GB_631: usize = 677531090944;

/// 632GB in bytes.
/// This constant represents 632 gb converted to bytes for use in size calculations.
pub const GB_632: usize = 678604832768;

/// 633GB in bytes.
/// This constant represents 633 gb converted to bytes for use in size calculations.
pub const GB_633: usize = 679678574592;

/// 634GB in bytes.
/// This constant represents 634 gb converted to bytes for use in size calculations.
pub const GB_634: usize = 680752316416;

/// 635GB in bytes.
/// This constant represents 635 gb converted to bytes for use in size calculations.
pub const GB_635: usize = 681826058240;

/// 636GB in bytes.
/// This constant represents 636 gb converted to bytes for use in size calculations.
pub const GB_636: usize = 682899800064;

/// 637GB in bytes.
/// This constant represents 637 gb converted to bytes for use in size calculations.
pub const GB_637: usize = 683973541888;

/// 638GB in bytes.
/// This constant represents 638 gb converted to bytes for use in size calculations.
pub const GB_638: usize = 685047283712;

/// 639GB in bytes.
/// This constant represents 639 gb converted to bytes for use in size calculations.
pub const GB_639: usize = 686121025536;

/// 640GB in bytes.
/// This constant represents 640 gb converted to bytes for use in size calculations.
pub const GB_640: usize = 687194767360;

/// 641GB in bytes.
/// This constant represents 641 gb converted to bytes for use in size calculations.
pub const GB_641: usize = 688268509184;

/// 642GB in bytes.
/// This constant represents 642 gb converted to bytes for use in size calculations.
pub const GB_642: usize = 689342251008;

/// 643GB in bytes.
/// This constant represents 643 gb converted to bytes for use in size calculations.
pub const GB_643: usize = 690415992832;

/// 644GB in bytes.
/// This constant represents 644 gb converted to bytes for use in size calculations.
pub const GB_644: usize = 691489734656;

/// 645GB in bytes.
/// This constant represents 645 gb converted to bytes for use in size calculations.
pub const GB_645: usize = 692563476480;

/// 646GB in bytes.
/// This constant represents 646 gb converted to bytes for use in size calculations.
pub const GB_646: usize = 693637218304;

/// 647GB in bytes.
/// This constant represents 647 gb converted to bytes for use in size calculations.
pub const GB_647: usize = 694710960128;

/// 648GB in bytes.
/// This constant represents 648 gb converted to bytes for use in size calculations.
pub const GB_648: usize = 695784701952;

/// 649GB in bytes.
/// This constant represents 649 gb converted to bytes for use in size calculations.
pub const GB_649: usize = 696858443776;

/// 650GB in bytes.
/// This constant represents 650 gb converted to bytes for use in size calculations.
pub const GB_650: usize = 697932185600;

/// 651GB in bytes.
/// This constant represents 651 gb converted to bytes for use in size calculations.
pub const GB_651: usize = 699005927424;

/// 652GB in bytes.
/// This constant represents 652 gb converted to bytes for use in size calculations.
pub const GB_652: usize = 700079669248;

/// 653GB in bytes.
/// This constant represents 653 gb converted to bytes for use in size calculations.
pub const GB_653: usize = 701153411072;

/// 654GB in bytes.
/// This constant represents 654 gb converted to bytes for use in size calculations.
pub const GB_654: usize = 702227152896;

/// 655GB in bytes.
/// This constant represents 655 gb converted to bytes for use in size calculations.
pub const GB_655: usize = 703300894720;

/// 656GB in bytes.
/// This constant represents 656 gb converted to bytes for use in size calculations.
pub const GB_656: usize = 704374636544;

/// 657GB in bytes.
/// This constant represents 657 gb converted to bytes for use in size calculations.
pub const GB_657: usize = 705448378368;

/// 658GB in bytes.
/// This constant represents 658 gb converted to bytes for use in size calculations.
pub const GB_658: usize = 706522120192;

/// 659GB in bytes.
/// This constant represents 659 gb converted to bytes for use in size calculations.
pub const GB_659: usize = 707595862016;

/// 660GB in bytes.
/// This constant represents 660 gb converted to bytes for use in size calculations.
pub const GB_660: usize = 708669603840;

/// 661GB in bytes.
/// This constant represents 661 gb converted to bytes for use in size calculations.
pub const GB_661: usize = 709743345664;

/// 662GB in bytes.
/// This constant represents 662 gb converted to bytes for use in size calculations.
pub const GB_662: usize = 710817087488;

/// 663GB in bytes.
/// This constant represents 663 gb converted to bytes for use in size calculations.
pub const GB_663: usize = 711890829312;

/// 664GB in bytes.
/// This constant represents 664 gb converted to bytes for use in size calculations.
pub const GB_664: usize = 712964571136;

/// 665GB in bytes.
/// This constant represents 665 gb converted to bytes for use in size calculations.
pub const GB_665: usize = 714038312960;

/// 666GB in bytes.
/// This constant represents 666 gb converted to bytes for use in size calculations.
pub const GB_666: usize = 715112054784;

/// 667GB in bytes.
/// This constant represents 667 gb converted to bytes for use in size calculations.
pub const GB_667: usize = 716185796608;

/// 668GB in bytes.
/// This constant represents 668 gb converted to bytes for use in size calculations.
pub const GB_668: usize = 717259538432;

/// 669GB in bytes.
/// This constant represents 669 gb converted to bytes for use in size calculations.
pub const GB_669: usize = 718333280256;

/// 670GB in bytes.
/// This constant represents 670 gb converted to bytes for use in size calculations.
pub const GB_670: usize = 719407022080;

/// 671GB in bytes.
/// This constant represents 671 gb converted to bytes for use in size calculations.
pub const GB_671: usize = 720480763904;

/// 672GB in bytes.
/// This constant represents 672 gb converted to bytes for use in size calculations.
pub const GB_672: usize = 721554505728;

/// 673GB in bytes.
/// This constant represents 673 gb converted to bytes for use in size calculations.
pub const GB_673: usize = 722628247552;

/// 674GB in bytes.
/// This constant represents 674 gb converted to bytes for use in size calculations.
pub const GB_674: usize = 723701989376;

/// 675GB in bytes.
/// This constant represents 675 gb converted to bytes for use in size calculations.
pub const GB_675: usize = 724775731200;

/// 676GB in bytes.
/// This constant represents 676 gb converted to bytes for use in size calculations.
pub const GB_676: usize = 725849473024;

/// 677GB in bytes.
/// This constant represents 677 gb converted to bytes for use in size calculations.
pub const GB_677: usize = 726923214848;

/// 678GB in bytes.
/// This constant represents 678 gb converted to bytes for use in size calculations.
pub const GB_678: usize = 727996956672;

/// 679GB in bytes.
/// This constant represents 679 gb converted to bytes for use in size calculations.
pub const GB_679: usize = 729070698496;

/// 680GB in bytes.
/// This constant represents 680 gb converted to bytes for use in size calculations.
pub const GB_680: usize = 730144440320;

/// 681GB in bytes.
/// This constant represents 681 gb converted to bytes for use in size calculations.
pub const GB_681: usize = 731218182144;

/// 682GB in bytes.
/// This constant represents 682 gb converted to bytes for use in size calculations.
pub const GB_682: usize = 732291923968;

/// 683GB in bytes.
/// This constant represents 683 gb converted to bytes for use in size calculations.
pub const GB_683: usize = 733365665792;

/// 684GB in bytes.
/// This constant represents 684 gb converted to bytes for use in size calculations.
pub const GB_684: usize = 734439407616;

/// 685GB in bytes.
/// This constant represents 685 gb converted to bytes for use in size calculations.
pub const GB_685: usize = 735513149440;

/// 686GB in bytes.
/// This constant represents 686 gb converted to bytes for use in size calculations.
pub const GB_686: usize = 736586891264;

/// 687GB in bytes.
/// This constant represents 687 gb converted to bytes for use in size calculations.
pub const GB_687: usize = 737660633088;

/// 688GB in bytes.
/// This constant represents 688 gb converted to bytes for use in size calculations.
pub const GB_688: usize = 738734374912;

/// 689GB in bytes.
/// This constant represents 689 gb converted to bytes for use in size calculations.
pub const GB_689: usize = 739808116736;

/// 690GB in bytes.
/// This constant represents 690 gb converted to bytes for use in size calculations.
pub const GB_690: usize = 740881858560;

/// 691GB in bytes.
/// This constant represents 691 gb converted to bytes for use in size calculations.
pub const GB_691: usize = 741955600384;

/// 692GB in bytes.
/// This constant represents 692 gb converted to bytes for use in size calculations.
pub const GB_692: usize = 743029342208;

/// 693GB in bytes.
/// This constant represents 693 gb converted to bytes for use in size calculations.
pub const GB_693: usize = 744103084032;

/// 694GB in bytes.
/// This constant represents 694 gb converted to bytes for use in size calculations.
pub const GB_694: usize = 745176825856;

/// 695GB in bytes.
/// This constant represents 695 gb converted to bytes for use in size calculations.
pub const GB_695: usize = 746250567680;

/// 696GB in bytes.
/// This constant represents 696 gb converted to bytes for use in size calculations.
pub const GB_696: usize = 747324309504;

/// 697GB in bytes.
/// This constant represents 697 gb converted to bytes for use in size calculations.
pub const GB_697: usize = 748398051328;

/// 698GB in bytes.
/// This constant represents 698 gb converted to bytes for use in size calculations.
pub const GB_698: usize = 749471793152;

/// 699GB in bytes.
/// This constant represents 699 gb converted to bytes for use in size calculations.
pub const GB_699: usize = 750545534976;

/// 700GB in bytes.
/// This constant represents 700 gb converted to bytes for use in size calculations.
pub const GB_700: usize = 751619276800;

/// 701GB in bytes.
/// This constant represents 701 gb converted to bytes for use in size calculations.
pub const GB_701: usize = 752693018624;

/// 702GB in bytes.
/// This constant represents 702 gb converted to bytes for use in size calculations.
pub const GB_702: usize = 753766760448;

/// 703GB in bytes.
/// This constant represents 703 gb converted to bytes for use in size calculations.
pub const GB_703: usize = 754840502272;

/// 704GB in bytes.
/// This constant represents 704 gb converted to bytes for use in size calculations.
pub const GB_704: usize = 755914244096;

/// 705GB in bytes.
/// This constant represents 705 gb converted to bytes for use in size calculations.
pub const GB_705: usize = 756987985920;

/// 706GB in bytes.
/// This constant represents 706 gb converted to bytes for use in size calculations.
pub const GB_706: usize = 758061727744;

/// 707GB in bytes.
/// This constant represents 707 gb converted to bytes for use in size calculations.
pub const GB_707: usize = 759135469568;

/// 708GB in bytes.
/// This constant represents 708 gb converted to bytes for use in size calculations.
pub const GB_708: usize = 760209211392;

/// 709GB in bytes.
/// This constant represents 709 gb converted to bytes for use in size calculations.
pub const GB_709: usize = 761282953216;

/// 710GB in bytes.
/// This constant represents 710 gb converted to bytes for use in size calculations.
pub const GB_710: usize = 762356695040;

/// 711GB in bytes.
/// This constant represents 711 gb converted to bytes for use in size calculations.
pub const GB_711: usize = 763430436864;

/// 712GB in bytes.
/// This constant represents 712 gb converted to bytes for use in size calculations.
pub const GB_712: usize = 764504178688;

/// 713GB in bytes.
/// This constant represents 713 gb converted to bytes for use in size calculations.
pub const GB_713: usize = 765577920512;

/// 714GB in bytes.
/// This constant represents 714 gb converted to bytes for use in size calculations.
pub const GB_714: usize = 766651662336;

/// 715GB in bytes.
/// This constant represents 715 gb converted to bytes for use in size calculations.
pub const GB_715: usize = 767725404160;

/// 716GB in bytes.
/// This constant represents 716 gb converted to bytes for use in size calculations.
pub const GB_716: usize = 768799145984;

/// 717GB in bytes.
/// This constant represents 717 gb converted to bytes for use in size calculations.
pub const GB_717: usize = 769872887808;

/// 718GB in bytes.
/// This constant represents 718 gb converted to bytes for use in size calculations.
pub const GB_718: usize = 770946629632;

/// 719GB in bytes.
/// This constant represents 719 gb converted to bytes for use in size calculations.
pub const GB_719: usize = 772020371456;

/// 720GB in bytes.
/// This constant represents 720 gb converted to bytes for use in size calculations.
pub const GB_720: usize = 773094113280;

/// 721GB in bytes.
/// This constant represents 721 gb converted to bytes for use in size calculations.
pub const GB_721: usize = 774167855104;

/// 722GB in bytes.
/// This constant represents 722 gb converted to bytes for use in size calculations.
pub const GB_722: usize = 775241596928;

/// 723GB in bytes.
/// This constant represents 723 gb converted to bytes for use in size calculations.
pub const GB_723: usize = 776315338752;

/// 724GB in bytes.
/// This constant represents 724 gb converted to bytes for use in size calculations.
pub const GB_724: usize = 777389080576;

/// 725GB in bytes.
/// This constant represents 725 gb converted to bytes for use in size calculations.
pub const GB_725: usize = 778462822400;

/// 726GB in bytes.
/// This constant represents 726 gb converted to bytes for use in size calculations.
pub const GB_726: usize = 779536564224;

/// 727GB in bytes.
/// This constant represents 727 gb converted to bytes for use in size calculations.
pub const GB_727: usize = 780610306048;

/// 728GB in bytes.
/// This constant represents 728 gb converted to bytes for use in size calculations.
pub const GB_728: usize = 781684047872;

/// 729GB in bytes.
/// This constant represents 729 gb converted to bytes for use in size calculations.
pub const GB_729: usize = 782757789696;

/// 730GB in bytes.
/// This constant represents 730 gb converted to bytes for use in size calculations.
pub const GB_730: usize = 783831531520;

/// 731GB in bytes.
/// This constant represents 731 gb converted to bytes for use in size calculations.
pub const GB_731: usize = 784905273344;

/// 732GB in bytes.
/// This constant represents 732 gb converted to bytes for use in size calculations.
pub const GB_732: usize = 785979015168;

/// 733GB in bytes.
/// This constant represents 733 gb converted to bytes for use in size calculations.
pub const GB_733: usize = 787052756992;

/// 734GB in bytes.
/// This constant represents 734 gb converted to bytes for use in size calculations.
pub const GB_734: usize = 788126498816;

/// 735GB in bytes.
/// This constant represents 735 gb converted to bytes for use in size calculations.
pub const GB_735: usize = 789200240640;

/// 736GB in bytes.
/// This constant represents 736 gb converted to bytes for use in size calculations.
pub const GB_736: usize = 790273982464;

/// 737GB in bytes.
/// This constant represents 737 gb converted to bytes for use in size calculations.
pub const GB_737: usize = 791347724288;

/// 738GB in bytes.
/// This constant represents 738 gb converted to bytes for use in size calculations.
pub const GB_738: usize = 792421466112;

/// 739GB in bytes.
/// This constant represents 739 gb converted to bytes for use in size calculations.
pub const GB_739: usize = 793495207936;

/// 740GB in bytes.
/// This constant represents 740 gb converted to bytes for use in size calculations.
pub const GB_740: usize = 794568949760;

/// 741GB in bytes.
/// This constant represents 741 gb converted to bytes for use in size calculations.
pub const GB_741: usize = 795642691584;

/// 742GB in bytes.
/// This constant represents 742 gb converted to bytes for use in size calculations.
pub const GB_742: usize = 796716433408;

/// 743GB in bytes.
/// This constant represents 743 gb converted to bytes for use in size calculations.
pub const GB_743: usize = 797790175232;

/// 744GB in bytes.
/// This constant represents 744 gb converted to bytes for use in size calculations.
pub const GB_744: usize = 798863917056;

/// 745GB in bytes.
/// This constant represents 745 gb converted to bytes for use in size calculations.
pub const GB_745: usize = 799937658880;

/// 746GB in bytes.
/// This constant represents 746 gb converted to bytes for use in size calculations.
pub const GB_746: usize = 801011400704;

/// 747GB in bytes.
/// This constant represents 747 gb converted to bytes for use in size calculations.
pub const GB_747: usize = 802085142528;

/// 748GB in bytes.
/// This constant represents 748 gb converted to bytes for use in size calculations.
pub const GB_748: usize = 803158884352;

/// 749GB in bytes.
/// This constant represents 749 gb converted to bytes for use in size calculations.
pub const GB_749: usize = 804232626176;

/// 750GB in bytes.
/// This constant represents 750 gb converted to bytes for use in size calculations.
pub const GB_750: usize = 805306368000;

/// 751GB in bytes.
/// This constant represents 751 gb converted to bytes for use in size calculations.
pub const GB_751: usize = 806380109824;

/// 752GB in bytes.
/// This constant represents 752 gb converted to bytes for use in size calculations.
pub const GB_752: usize = 807453851648;

/// 753GB in bytes.
/// This constant represents 753 gb converted to bytes for use in size calculations.
pub const GB_753: usize = 808527593472;

/// 754GB in bytes.
/// This constant represents 754 gb converted to bytes for use in size calculations.
pub const GB_754: usize = 809601335296;

/// 755GB in bytes.
/// This constant represents 755 gb converted to bytes for use in size calculations.
pub const GB_755: usize = 810675077120;

/// 756GB in bytes.
/// This constant represents 756 gb converted to bytes for use in size calculations.
pub const GB_756: usize = 811748818944;

/// 757GB in bytes.
/// This constant represents 757 gb converted to bytes for use in size calculations.
pub const GB_757: usize = 812822560768;

/// 758GB in bytes.
/// This constant represents 758 gb converted to bytes for use in size calculations.
pub const GB_758: usize = 813896302592;

/// 759GB in bytes.
/// This constant represents 759 gb converted to bytes for use in size calculations.
pub const GB_759: usize = 814970044416;

/// 760GB in bytes.
/// This constant represents 760 gb converted to bytes for use in size calculations.
pub const GB_760: usize = 816043786240;

/// 761GB in bytes.
/// This constant represents 761 gb converted to bytes for use in size calculations.
pub const GB_761: usize = 817117528064;

/// 762GB in bytes.
/// This constant represents 762 gb converted to bytes for use in size calculations.
pub const GB_762: usize = 818191269888;

/// 763GB in bytes.
/// This constant represents 763 gb converted to bytes for use in size calculations.
pub const GB_763: usize = 819265011712;

/// 764GB in bytes.
/// This constant represents 764 gb converted to bytes for use in size calculations.
pub const GB_764: usize = 820338753536;

/// 765GB in bytes.
/// This constant represents 765 gb converted to bytes for use in size calculations.
pub const GB_765: usize = 821412495360;

/// 766GB in bytes.
/// This constant represents 766 gb converted to bytes for use in size calculations.
pub const GB_766: usize = 822486237184;

/// 767GB in bytes.
/// This constant represents 767 gb converted to bytes for use in size calculations.
pub const GB_767: usize = 823559979008;

/// 768GB in bytes.
/// This constant represents 768 gb converted to bytes for use in size calculations.
pub const GB_768: usize = 824633720832;

/// 769GB in bytes.
/// This constant represents 769 gb converted to bytes for use in size calculations.
pub const GB_769: usize = 825707462656;

/// 770GB in bytes.
/// This constant represents 770 gb converted to bytes for use in size calculations.
pub const GB_770: usize = 826781204480;

/// 771GB in bytes.
/// This constant represents 771 gb converted to bytes for use in size calculations.
pub const GB_771: usize = 827854946304;

/// 772GB in bytes.
/// This constant represents 772 gb converted to bytes for use in size calculations.
pub const GB_772: usize = 828928688128;

/// 773GB in bytes.
/// This constant represents 773 gb converted to bytes for use in size calculations.
pub const GB_773: usize = 830002429952;

/// 774GB in bytes.
/// This constant represents 774 gb converted to bytes for use in size calculations.
pub const GB_774: usize = 831076171776;

/// 775GB in bytes.
/// This constant represents 775 gb converted to bytes for use in size calculations.
pub const GB_775: usize = 832149913600;

/// 776GB in bytes.
/// This constant represents 776 gb converted to bytes for use in size calculations.
pub const GB_776: usize = 833223655424;

/// 777GB in bytes.
/// This constant represents 777 gb converted to bytes for use in size calculations.
pub const GB_777: usize = 834297397248;

/// 778GB in bytes.
/// This constant represents 778 gb converted to bytes for use in size calculations.
pub const GB_778: usize = 835371139072;

/// 779GB in bytes.
/// This constant represents 779 gb converted to bytes for use in size calculations.
pub const GB_779: usize = 836444880896;

/// 780GB in bytes.
/// This constant represents 780 gb converted to bytes for use in size calculations.
pub const GB_780: usize = 837518622720;

/// 781GB in bytes.
/// This constant represents 781 gb converted to bytes for use in size calculations.
pub const GB_781: usize = 838592364544;

/// 782GB in bytes.
/// This constant represents 782 gb converted to bytes for use in size calculations.
pub const GB_782: usize = 839666106368;

/// 783GB in bytes.
/// This constant represents 783 gb converted to bytes for use in size calculations.
pub const GB_783: usize = 840739848192;

/// 784GB in bytes.
/// This constant represents 784 gb converted to bytes for use in size calculations.
pub const GB_784: usize = 841813590016;

/// 785GB in bytes.
/// This constant represents 785 gb converted to bytes for use in size calculations.
pub const GB_785: usize = 842887331840;

/// 786GB in bytes.
/// This constant represents 786 gb converted to bytes for use in size calculations.
pub const GB_786: usize = 843961073664;

/// 787GB in bytes.
/// This constant represents 787 gb converted to bytes for use in size calculations.
pub const GB_787: usize = 845034815488;

/// 788GB in bytes.
/// This constant represents 788 gb converted to bytes for use in size calculations.
pub const GB_788: usize = 846108557312;

/// 789GB in bytes.
/// This constant represents 789 gb converted to bytes for use in size calculations.
pub const GB_789: usize = 847182299136;

/// 790GB in bytes.
/// This constant represents 790 gb converted to bytes for use in size calculations.
pub const GB_790: usize = 848256040960;

/// 791GB in bytes.
/// This constant represents 791 gb converted to bytes for use in size calculations.
pub const GB_791: usize = 849329782784;

/// 792GB in bytes.
/// This constant represents 792 gb converted to bytes for use in size calculations.
pub const GB_792: usize = 850403524608;

/// 793GB in bytes.
/// This constant represents 793 gb converted to bytes for use in size calculations.
pub const GB_793: usize = 851477266432;

/// 794GB in bytes.
/// This constant represents 794 gb converted to bytes for use in size calculations.
pub const GB_794: usize = 852551008256;

/// 795GB in bytes.
/// This constant represents 795 gb converted to bytes for use in size calculations.
pub const GB_795: usize = 853624750080;

/// 796GB in bytes.
/// This constant represents 796 gb converted to bytes for use in size calculations.
pub const GB_796: usize = 854698491904;

/// 797GB in bytes.
/// This constant represents 797 gb converted to bytes for use in size calculations.
pub const GB_797: usize = 855772233728;

/// 798GB in bytes.
/// This constant represents 798 gb converted to bytes for use in size calculations.
pub const GB_798: usize = 856845975552;

/// 799GB in bytes.
/// This constant represents 799 gb converted to bytes for use in size calculations.
pub const GB_799: usize = 857919717376;

/// 800GB in bytes.
/// This constant represents 800 gb converted to bytes for use in size calculations.
pub const GB_800: usize = 858993459200;

/// 801GB in bytes.
/// This constant represents 801 gb converted to bytes for use in size calculations.
pub const GB_801: usize = 860067201024;

/// 802GB in bytes.
/// This constant represents 802 gb converted to bytes for use in size calculations.
pub const GB_802: usize = 861140942848;

/// 803GB in bytes.
/// This constant represents 803 gb converted to bytes for use in size calculations.
pub const GB_803: usize = 862214684672;

/// 804GB in bytes.
/// This constant represents 804 gb converted to bytes for use in size calculations.
pub const GB_804: usize = 863288426496;

/// 805GB in bytes.
/// This constant represents 805 gb converted to bytes for use in size calculations.
pub const GB_805: usize = 864362168320;

/// 806GB in bytes.
/// This constant represents 806 gb converted to bytes for use in size calculations.
pub const GB_806: usize = 865435910144;

/// 807GB in bytes.
/// This constant represents 807 gb converted to bytes for use in size calculations.
pub const GB_807: usize = 866509651968;

/// 808GB in bytes.
/// This constant represents 808 gb converted to bytes for use in size calculations.
pub const GB_808: usize = 867583393792;

/// 809GB in bytes.
/// This constant represents 809 gb converted to bytes for use in size calculations.
pub const GB_809: usize = 868657135616;

/// 810GB in bytes.
/// This constant represents 810 gb converted to bytes for use in size calculations.
pub const GB_810: usize = 869730877440;

/// 811GB in bytes.
/// This constant represents 811 gb converted to bytes for use in size calculations.
pub const GB_811: usize = 870804619264;

/// 812GB in bytes.
/// This constant represents 812 gb converted to bytes for use in size calculations.
pub const GB_812: usize = 871878361088;

/// 813GB in bytes.
/// This constant represents 813 gb converted to bytes for use in size calculations.
pub const GB_813: usize = 872952102912;

/// 814GB in bytes.
/// This constant represents 814 gb converted to bytes for use in size calculations.
pub const GB_814: usize = 874025844736;

/// 815GB in bytes.
/// This constant represents 815 gb converted to bytes for use in size calculations.
pub const GB_815: usize = 875099586560;

/// 816GB in bytes.
/// This constant represents 816 gb converted to bytes for use in size calculations.
pub const GB_816: usize = 876173328384;

/// 817GB in bytes.
/// This constant represents 817 gb converted to bytes for use in size calculations.
pub const GB_817: usize = 877247070208;

/// 818GB in bytes.
/// This constant represents 818 gb converted to bytes for use in size calculations.
pub const GB_818: usize = 878320812032;

/// 819GB in bytes.
/// This constant represents 819 gb converted to bytes for use in size calculations.
pub const GB_819: usize = 879394553856;

/// 820GB in bytes.
/// This constant represents 820 gb converted to bytes for use in size calculations.
pub const GB_820: usize = 880468295680;

/// 821GB in bytes.
/// This constant represents 821 gb converted to bytes for use in size calculations.
pub const GB_821: usize = 881542037504;

/// 822GB in bytes.
/// This constant represents 822 gb converted to bytes for use in size calculations.
pub const GB_822: usize = 882615779328;

/// 823GB in bytes.
/// This constant represents 823 gb converted to bytes for use in size calculations.
pub const GB_823: usize = 883689521152;

/// 824GB in bytes.
/// This constant represents 824 gb converted to bytes for use in size calculations.
pub const GB_824: usize = 884763262976;

/// 825GB in bytes.
/// This constant represents 825 gb converted to bytes for use in size calculations.
pub const GB_825: usize = 885837004800;

/// 826GB in bytes.
/// This constant represents 826 gb converted to bytes for use in size calculations.
pub const GB_826: usize = 886910746624;

/// 827GB in bytes.
/// This constant represents 827 gb converted to bytes for use in size calculations.
pub const GB_827: usize = 887984488448;

/// 828GB in bytes.
/// This constant represents 828 gb converted to bytes for use in size calculations.
pub const GB_828: usize = 889058230272;

/// 829GB in bytes.
/// This constant represents 829 gb converted to bytes for use in size calculations.
pub const GB_829: usize = 890131972096;

/// 830GB in bytes.
/// This constant represents 830 gb converted to bytes for use in size calculations.
pub const GB_830: usize = 891205713920;

/// 831GB in bytes.
/// This constant represents 831 gb converted to bytes for use in size calculations.
pub const GB_831: usize = 892279455744;

/// 832GB in bytes.
/// This constant represents 832 gb converted to bytes for use in size calculations.
pub const GB_832: usize = 893353197568;

/// 833GB in bytes.
/// This constant represents 833 gb converted to bytes for use in size calculations.
pub const GB_833: usize = 894426939392;

/// 834GB in bytes.
/// This constant represents 834 gb converted to bytes for use in size calculations.
pub const GB_834: usize = 895500681216;

/// 835GB in bytes.
/// This constant represents 835 gb converted to bytes for use in size calculations.
pub const GB_835: usize = 896574423040;

/// 836GB in bytes.
/// This constant represents 836 gb converted to bytes for use in size calculations.
pub const GB_836: usize = 897648164864;

/// 837GB in bytes.
/// This constant represents 837 gb converted to bytes for use in size calculations.
pub const GB_837: usize = 898721906688;

/// 838GB in bytes.
/// This constant represents 838 gb converted to bytes for use in size calculations.
pub const GB_838: usize = 899795648512;

/// 839GB in bytes.
/// This constant represents 839 gb converted to bytes for use in size calculations.
pub const GB_839: usize = 900869390336;

/// 840GB in bytes.
/// This constant represents 840 gb converted to bytes for use in size calculations.
pub const GB_840: usize = 901943132160;

/// 841GB in bytes.
/// This constant represents 841 gb converted to bytes for use in size calculations.
pub const GB_841: usize = 903016873984;

/// 842GB in bytes.
/// This constant represents 842 gb converted to bytes for use in size calculations.
pub const GB_842: usize = 904090615808;

/// 843GB in bytes.
/// This constant represents 843 gb converted to bytes for use in size calculations.
pub const GB_843: usize = 905164357632;

/// 844GB in bytes.
/// This constant represents 844 gb converted to bytes for use in size calculations.
pub const GB_844: usize = 906238099456;

/// 845GB in bytes.
/// This constant represents 845 gb converted to bytes for use in size calculations.
pub const GB_845: usize = 907311841280;

/// 846GB in bytes.
/// This constant represents 846 gb converted to bytes for use in size calculations.
pub const GB_846: usize = 908385583104;

/// 847GB in bytes.
/// This constant represents 847 gb converted to bytes for use in size calculations.
pub const GB_847: usize = 909459324928;

/// 848GB in bytes.
/// This constant represents 848 gb converted to bytes for use in size calculations.
pub const GB_848: usize = 910533066752;

/// 849GB in bytes.
/// This constant represents 849 gb converted to bytes for use in size calculations.
pub const GB_849: usize = 911606808576;

/// 850GB in bytes.
/// This constant represents 850 gb converted to bytes for use in size calculations.
pub const GB_850: usize = 912680550400;

/// 851GB in bytes.
/// This constant represents 851 gb converted to bytes for use in size calculations.
pub const GB_851: usize = 913754292224;

/// 852GB in bytes.
/// This constant represents 852 gb converted to bytes for use in size calculations.
pub const GB_852: usize = 914828034048;

/// 853GB in bytes.
/// This constant represents 853 gb converted to bytes for use in size calculations.
pub const GB_853: usize = 915901775872;

/// 854GB in bytes.
/// This constant represents 854 gb converted to bytes for use in size calculations.
pub const GB_854: usize = 916975517696;

/// 855GB in bytes.
/// This constant represents 855 gb converted to bytes for use in size calculations.
pub const GB_855: usize = 918049259520;

/// 856GB in bytes.
/// This constant represents 856 gb converted to bytes for use in size calculations.
pub const GB_856: usize = 919123001344;

/// 857GB in bytes.
/// This constant represents 857 gb converted to bytes for use in size calculations.
pub const GB_857: usize = 920196743168;

/// 858GB in bytes.
/// This constant represents 858 gb converted to bytes for use in size calculations.
pub const GB_858: usize = 921270484992;

/// 859GB in bytes.
/// This constant represents 859 gb converted to bytes for use in size calculations.
pub const GB_859: usize = 922344226816;

/// 860GB in bytes.
/// This constant represents 860 gb converted to bytes for use in size calculations.
pub const GB_860: usize = 923417968640;

/// 861GB in bytes.
/// This constant represents 861 gb converted to bytes for use in size calculations.
pub const GB_861: usize = 924491710464;

/// 862GB in bytes.
/// This constant represents 862 gb converted to bytes for use in size calculations.
pub const GB_862: usize = 925565452288;

/// 863GB in bytes.
/// This constant represents 863 gb converted to bytes for use in size calculations.
pub const GB_863: usize = 926639194112;

/// 864GB in bytes.
/// This constant represents 864 gb converted to bytes for use in size calculations.
pub const GB_864: usize = 927712935936;

/// 865GB in bytes.
/// This constant represents 865 gb converted to bytes for use in size calculations.
pub const GB_865: usize = 928786677760;

/// 866GB in bytes.
/// This constant represents 866 gb converted to bytes for use in size calculations.
pub const GB_866: usize = 929860419584;

/// 867GB in bytes.
/// This constant represents 867 gb converted to bytes for use in size calculations.
pub const GB_867: usize = 930934161408;

/// 868GB in bytes.
/// This constant represents 868 gb converted to bytes for use in size calculations.
pub const GB_868: usize = 932007903232;

/// 869GB in bytes.
/// This constant represents 869 gb converted to bytes for use in size calculations.
pub const GB_869: usize = 933081645056;

/// 870GB in bytes.
/// This constant represents 870 gb converted to bytes for use in size calculations.
pub const GB_870: usize = 934155386880;

/// 871GB in bytes.
/// This constant represents 871 gb converted to bytes for use in size calculations.
pub const GB_871: usize = 935229128704;

/// 872GB in bytes.
/// This constant represents 872 gb converted to bytes for use in size calculations.
pub const GB_872: usize = 936302870528;

/// 873GB in bytes.
/// This constant represents 873 gb converted to bytes for use in size calculations.
pub const GB_873: usize = 937376612352;

/// 874GB in bytes.
/// This constant represents 874 gb converted to bytes for use in size calculations.
pub const GB_874: usize = 938450354176;

/// 875GB in bytes.
/// This constant represents 875 gb converted to bytes for use in size calculations.
pub const GB_875: usize = 939524096000;

/// 876GB in bytes.
/// This constant represents 876 gb converted to bytes for use in size calculations.
pub const GB_876: usize = 940597837824;

/// 877GB in bytes.
/// This constant represents 877 gb converted to bytes for use in size calculations.
pub const GB_877: usize = 941671579648;

/// 878GB in bytes.
/// This constant represents 878 gb converted to bytes for use in size calculations.
pub const GB_878: usize = 942745321472;

/// 879GB in bytes.
/// This constant represents 879 gb converted to bytes for use in size calculations.
pub const GB_879: usize = 943819063296;

/// 880GB in bytes.
/// This constant represents 880 gb converted to bytes for use in size calculations.
pub const GB_880: usize = 944892805120;

/// 881GB in bytes.
/// This constant represents 881 gb converted to bytes for use in size calculations.
pub const GB_881: usize = 945966546944;

/// 882GB in bytes.
/// This constant represents 882 gb converted to bytes for use in size calculations.
pub const GB_882: usize = 947040288768;

/// 883GB in bytes.
/// This constant represents 883 gb converted to bytes for use in size calculations.
pub const GB_883: usize = 948114030592;

/// 884GB in bytes.
/// This constant represents 884 gb converted to bytes for use in size calculations.
pub const GB_884: usize = 949187772416;

/// 885GB in bytes.
/// This constant represents 885 gb converted to bytes for use in size calculations.
pub const GB_885: usize = 950261514240;

/// 886GB in bytes.
/// This constant represents 886 gb converted to bytes for use in size calculations.
pub const GB_886: usize = 951335256064;

/// 887GB in bytes.
/// This constant represents 887 gb converted to bytes for use in size calculations.
pub const GB_887: usize = 952408997888;

/// 888GB in bytes.
/// This constant represents 888 gb converted to bytes for use in size calculations.
pub const GB_888: usize = 953482739712;

/// 889GB in bytes.
/// This constant represents 889 gb converted to bytes for use in size calculations.
pub const GB_889: usize = 954556481536;

/// 890GB in bytes.
/// This constant represents 890 gb converted to bytes for use in size calculations.
pub const GB_890: usize = 955630223360;

/// 891GB in bytes.
/// This constant represents 891 gb converted to bytes for use in size calculations.
pub const GB_891: usize = 956703965184;

/// 892GB in bytes.
/// This constant represents 892 gb converted to bytes for use in size calculations.
pub const GB_892: usize = 957777707008;

/// 893GB in bytes.
/// This constant represents 893 gb converted to bytes for use in size calculations.
pub const GB_893: usize = 958851448832;

/// 894GB in bytes.
/// This constant represents 894 gb converted to bytes for use in size calculations.
pub const GB_894: usize = 959925190656;

/// 895GB in bytes.
/// This constant represents 895 gb converted to bytes for use in size calculations.
pub const GB_895: usize = 960998932480;

/// 896GB in bytes.
/// This constant represents 896 gb converted to bytes for use in size calculations.
pub const GB_896: usize = 962072674304;

/// 897GB in bytes.
/// This constant represents 897 gb converted to bytes for use in size calculations.
pub const GB_897: usize = 963146416128;

/// 898GB in bytes.
/// This constant represents 898 gb converted to bytes for use in size calculations.
pub const GB_898: usize = 964220157952;

/// 899GB in bytes.
/// This constant represents 899 gb converted to bytes for use in size calculations.
pub const GB_899: usize = 965293899776;

/// 900GB in bytes.
/// This constant represents 900 gb converted to bytes for use in size calculations.
pub const GB_900: usize = 966367641600;

/// 901GB in bytes.
/// This constant represents 901 gb converted to bytes for use in size calculations.
pub const GB_901: usize = 967441383424;

/// 902GB in bytes.
/// This constant represents 902 gb converted to bytes for use in size calculations.
pub const GB_902: usize = 968515125248;

/// 903GB in bytes.
/// This constant represents 903 gb converted to bytes for use in size calculations.
pub const GB_903: usize = 969588867072;

/// 904GB in bytes.
/// This constant represents 904 gb converted to bytes for use in size calculations.
pub const GB_904: usize = 970662608896;

/// 905GB in bytes.
/// This constant represents 905 gb converted to bytes for use in size calculations.
pub const GB_905: usize = 971736350720;

/// 906GB in bytes.
/// This constant represents 906 gb converted to bytes for use in size calculations.
pub const GB_906: usize = 972810092544;

/// 907GB in bytes.
/// This constant represents 907 gb converted to bytes for use in size calculations.
pub const GB_907: usize = 973883834368;

/// 908GB in bytes.
/// This constant represents 908 gb converted to bytes for use in size calculations.
pub const GB_908: usize = 974957576192;

/// 909GB in bytes.
/// This constant represents 909 gb converted to bytes for use in size calculations.
pub const GB_909: usize = 976031318016;

/// 910GB in bytes.
/// This constant represents 910 gb converted to bytes for use in size calculations.
pub const GB_910: usize = 977105059840;

/// 911GB in bytes.
/// This constant represents 911 gb converted to bytes for use in size calculations.
pub const GB_911: usize = 978178801664;

/// 912GB in bytes.
/// This constant represents 912 gb converted to bytes for use in size calculations.
pub const GB_912: usize = 979252543488;

/// 913GB in bytes.
/// This constant represents 913 gb converted to bytes for use in size calculations.
pub const GB_913: usize = 980326285312;

/// 914GB in bytes.
/// This constant represents 914 gb converted to bytes for use in size calculations.
pub const GB_914: usize = 981400027136;

/// 915GB in bytes.
/// This constant represents 915 gb converted to bytes for use in size calculations.
pub const GB_915: usize = 982473768960;

/// 916GB in bytes.
/// This constant represents 916 gb converted to bytes for use in size calculations.
pub const GB_916: usize = 983547510784;

/// 917GB in bytes.
/// This constant represents 917 gb converted to bytes for use in size calculations.
pub const GB_917: usize = 984621252608;

/// 918GB in bytes.
/// This constant represents 918 gb converted to bytes for use in size calculations.
pub const GB_918: usize = 985694994432;

/// 919GB in bytes.
/// This constant represents 919 gb converted to bytes for use in size calculations.
pub const GB_919: usize = 986768736256;

/// 920GB in bytes.
/// This constant represents 920 gb converted to bytes for use in size calculations.
pub const GB_920: usize = 987842478080;

/// 921GB in bytes.
/// This constant represents 921 gb converted to bytes for use in size calculations.
pub const GB_921: usize = 988916219904;

/// 922GB in bytes.
/// This constant represents 922 gb converted to bytes for use in size calculations.
pub const GB_922: usize = 989989961728;

/// 923GB in bytes.
/// This constant represents 923 gb converted to bytes for use in size calculations.
pub const GB_923: usize = 991063703552;

/// 924GB in bytes.
/// This constant represents 924 gb converted to bytes for use in size calculations.
pub const GB_924: usize = 992137445376;

/// 925GB in bytes.
/// This constant represents 925 gb converted to bytes for use in size calculations.
pub const GB_925: usize = 993211187200;

/// 926GB in bytes.
/// This constant represents 926 gb converted to bytes for use in size calculations.
pub const GB_926: usize = 994284929024;

/// 927GB in bytes.
/// This constant represents 927 gb converted to bytes for use in size calculations.
pub const GB_927: usize = 995358670848;

/// 928GB in bytes.
/// This constant represents 928 gb converted to bytes for use in size calculations.
pub const GB_928: usize = 996432412672;

/// 929GB in bytes.
/// This constant represents 929 gb converted to bytes for use in size calculations.
pub const GB_929: usize = 997506154496;

/// 930GB in bytes.
/// This constant represents 930 gb converted to bytes for use in size calculations.
pub const GB_930: usize = 998579896320;

/// 931GB in bytes.
/// This constant represents 931 gb converted to bytes for use in size calculations.
pub const GB_931: usize = 999653638144;

/// 932GB in bytes.
/// This constant represents 932 gb converted to bytes for use in size calculations.
pub const GB_932: usize = 1000727379968;

/// 933GB in bytes.
/// This constant represents 933 gb converted to bytes for use in size calculations.
pub const GB_933: usize = 1001801121792;

/// 934GB in bytes.
/// This constant represents 934 gb converted to bytes for use in size calculations.
pub const GB_934: usize = 1002874863616;

/// 935GB in bytes.
/// This constant represents 935 gb converted to bytes for use in size calculations.
pub const GB_935: usize = 1003948605440;

/// 936GB in bytes.
/// This constant represents 936 gb converted to bytes for use in size calculations.
pub const GB_936: usize = 1005022347264;

/// 937GB in bytes.
/// This constant represents 937 gb converted to bytes for use in size calculations.
pub const GB_937: usize = 1006096089088;

/// 938GB in bytes.
/// This constant represents 938 gb converted to bytes for use in size calculations.
pub const GB_938: usize = 1007169830912;

/// 939GB in bytes.
/// This constant represents 939 gb converted to bytes for use in size calculations.
pub const GB_939: usize = 1008243572736;

/// 940GB in bytes.
/// This constant represents 940 gb converted to bytes for use in size calculations.
pub const GB_940: usize = 1009317314560;

/// 941GB in bytes.
/// This constant represents 941 gb converted to bytes for use in size calculations.
pub const GB_941: usize = 1010391056384;

/// 942GB in bytes.
/// This constant represents 942 gb converted to bytes for use in size calculations.
pub const GB_942: usize = 1011464798208;

/// 943GB in bytes.
/// This constant represents 943 gb converted to bytes for use in size calculations.
pub const GB_943: usize = 1012538540032;

/// 944GB in bytes.
/// This constant represents 944 gb converted to bytes for use in size calculations.
pub const GB_944: usize = 1013612281856;

/// 945GB in bytes.
/// This constant represents 945 gb converted to bytes for use in size calculations.
pub const GB_945: usize = 1014686023680;

/// 946GB in bytes.
/// This constant represents 946 gb converted to bytes for use in size calculations.
pub const GB_946: usize = 1015759765504;

/// 947GB in bytes.
/// This constant represents 947 gb converted to bytes for use in size calculations.
pub const GB_947: usize = 1016833507328;

/// 948GB in bytes.
/// This constant represents 948 gb converted to bytes for use in size calculations.
pub const GB_948: usize = 1017907249152;

/// 949GB in bytes.
/// This constant represents 949 gb converted to bytes for use in size calculations.
pub const GB_949: usize = 1018980990976;

/// 950GB in bytes.
/// This constant represents 950 gb converted to bytes for use in size calculations.
pub const GB_950: usize = 1020054732800;

/// 951GB in bytes.
/// This constant represents 951 gb converted to bytes for use in size calculations.
pub const GB_951: usize = 1021128474624;

/// 952GB in bytes.
/// This constant represents 952 gb converted to bytes for use in size calculations.
pub const GB_952: usize = 1022202216448;

/// 953GB in bytes.
/// This constant represents 953 gb converted to bytes for use in size calculations.
pub const GB_953: usize = 1023275958272;

/// 954GB in bytes.
/// This constant represents 954 gb converted to bytes for use in size calculations.
pub const GB_954: usize = 1024349700096;

/// 955GB in bytes.
/// This constant represents 955 gb converted to bytes for use in size calculations.
pub const GB_955: usize = 1025423441920;

/// 956GB in bytes.
/// This constant represents 956 gb converted to bytes for use in size calculations.
pub const GB_956: usize = 1026497183744;

/// 957GB in bytes.
/// This constant represents 957 gb converted to bytes for use in size calculations.
pub const GB_957: usize = 1027570925568;

/// 958GB in bytes.
/// This constant represents 958 gb converted to bytes for use in size calculations.
pub const GB_958: usize = 1028644667392;

/// 959GB in bytes.
/// This constant represents 959 gb converted to bytes for use in size calculations.
pub const GB_959: usize = 1029718409216;

/// 960GB in bytes.
/// This constant represents 960 gb converted to bytes for use in size calculations.
pub const GB_960: usize = 1030792151040;

/// 961GB in bytes.
/// This constant represents 961 gb converted to bytes for use in size calculations.
pub const GB_961: usize = 1031865892864;

/// 962GB in bytes.
/// This constant represents 962 gb converted to bytes for use in size calculations.
pub const GB_962: usize = 1032939634688;

/// 963GB in bytes.
/// This constant represents 963 gb converted to bytes for use in size calculations.
pub const GB_963: usize = 1034013376512;

/// 964GB in bytes.
/// This constant represents 964 gb converted to bytes for use in size calculations.
pub const GB_964: usize = 1035087118336;

/// 965GB in bytes.
/// This constant represents 965 gb converted to bytes for use in size calculations.
pub const GB_965: usize = 1036160860160;

/// 966GB in bytes.
/// This constant represents 966 gb converted to bytes for use in size calculations.
pub const GB_966: usize = 1037234601984;

/// 967GB in bytes.
/// This constant represents 967 gb converted to bytes for use in size calculations.
pub const GB_967: usize = 1038308343808;

/// 968GB in bytes.
/// This constant represents 968 gb converted to bytes for use in size calculations.
pub const GB_968: usize = 1039382085632;

/// 969GB in bytes.
/// This constant represents 969 gb converted to bytes for use in size calculations.
pub const GB_969: usize = 1040455827456;

/// 970GB in bytes.
/// This constant represents 970 gb converted to bytes for use in size calculations.
pub const GB_970: usize = 1041529569280;

/// 971GB in bytes.
/// This constant represents 971 gb converted to bytes for use in size calculations.
pub const GB_971: usize = 1042603311104;

/// 972GB in bytes.
/// This constant represents 972 gb converted to bytes for use in size calculations.
pub const GB_972: usize = 1043677052928;

/// 973GB in bytes.
/// This constant represents 973 gb converted to bytes for use in size calculations.
pub const GB_973: usize = 1044750794752;

/// 974GB in bytes.
/// This constant represents 974 gb converted to bytes for use in size calculations.
pub const GB_974: usize = 1045824536576;

/// 975GB in bytes.
/// This constant represents 975 gb converted to bytes for use in size calculations.
pub const GB_975: usize = 1046898278400;

/// 976GB in bytes.
/// This constant represents 976 gb converted to bytes for use in size calculations.
pub const GB_976: usize = 1047972020224;

/// 977GB in bytes.
/// This constant represents 977 gb converted to bytes for use in size calculations.
pub const GB_977: usize = 1049045762048;

/// 978GB in bytes.
/// This constant represents 978 gb converted to bytes for use in size calculations.
pub const GB_978: usize = 1050119503872;

/// 979GB in bytes.
/// This constant represents 979 gb converted to bytes for use in size calculations.
pub const GB_979: usize = 1051193245696;

/// 980GB in bytes.
/// This constant represents 980 gb converted to bytes for use in size calculations.
pub const GB_980: usize = 1052266987520;

/// 981GB in bytes.
/// This constant represents 981 gb converted to bytes for use in size calculations.
pub const GB_981: usize = 1053340729344;

/// 982GB in bytes.
/// This constant represents 982 gb converted to bytes for use in size calculations.
pub const GB_982: usize = 1054414471168;

/// 983GB in bytes.
/// This constant represents 983 gb converted to bytes for use in size calculations.
pub const GB_983: usize = 1055488212992;

/// 984GB in bytes.
/// This constant represents 984 gb converted to bytes for use in size calculations.
pub const GB_984: usize = 1056561954816;

/// 985GB in bytes.
/// This constant represents 985 gb converted to bytes for use in size calculations.
pub const GB_985: usize = 1057635696640;

/// 986GB in bytes.
/// This constant represents 986 gb converted to bytes for use in size calculations.
pub const GB_986: usize = 1058709438464;

/// 987GB in bytes.
/// This constant represents 987 gb converted to bytes for use in size calculations.
pub const GB_987: usize = 1059783180288;

/// 988GB in bytes.
/// This constant represents 988 gb converted to bytes for use in size calculations.
pub const GB_988: usize = 1060856922112;

/// 989GB in bytes.
/// This constant represents 989 gb converted to bytes for use in size calculations.
pub const GB_989: usize = 1061930663936;

/// 990GB in bytes.
/// This constant represents 990 gb converted to bytes for use in size calculations.
pub const GB_990: usize = 1063004405760;

/// 991GB in bytes.
/// This constant represents 991 gb converted to bytes for use in size calculations.
pub const GB_991: usize = 1064078147584;

/// 992GB in bytes.
/// This constant represents 992 gb converted to bytes for use in size calculations.
pub const GB_992: usize = 1065151889408;

/// 993GB in bytes.
/// This constant represents 993 gb converted to bytes for use in size calculations.
pub const GB_993: usize = 1066225631232;

/// 994GB in bytes.
/// This constant represents 994 gb converted to bytes for use in size calculations.
pub const GB_994: usize = 1067299373056;

/// 995GB in bytes.
/// This constant represents 995 gb converted to bytes for use in size calculations.
pub const GB_995: usize = 1068373114880;

/// 996GB in bytes.
/// This constant represents 996 gb converted to bytes for use in size calculations.
pub const GB_996: usize = 1069446856704;

/// 997GB in bytes.
/// This constant represents 997 gb converted to bytes for use in size calculations.
pub const GB_997: usize = 1070520598528;

/// 998GB in bytes.
/// This constant represents 998 gb converted to bytes for use in size calculations.
pub const GB_998: usize = 1071594340352;

/// 999GB in bytes.
/// This constant represents 999 gb converted to bytes for use in size calculations.
pub const GB_999: usize = 1072668082176;

/// 1000GB in bytes.
/// This constant represents 1000 gb converted to bytes for use in size calculations.
pub const GB_1000: usize = 1073741824000;

/// 1001GB in bytes.
/// This constant represents 1001 gb converted to bytes for use in size calculations.
pub const GB_1001: usize = 1074815565824;

/// 1002GB in bytes.
/// This constant represents 1002 gb converted to bytes for use in size calculations.
pub const GB_1002: usize = 1075889307648;

/// 1003GB in bytes.
/// This constant represents 1003 gb converted to bytes for use in size calculations.
pub const GB_1003: usize = 1076963049472;

/// 1004GB in bytes.
/// This constant represents 1004 gb converted to bytes for use in size calculations.
pub const GB_1004: usize = 1078036791296;

/// 1005GB in bytes.
/// This constant represents 1005 gb converted to bytes for use in size calculations.
pub const GB_1005: usize = 1079110533120;

/// 1006GB in bytes.
/// This constant represents 1006 gb converted to bytes for use in size calculations.
pub const GB_1006: usize = 1080184274944;

/// 1007GB in bytes.
/// This constant represents 1007 gb converted to bytes for use in size calculations.
pub const GB_1007: usize = 1081258016768;

/// 1008GB in bytes.
/// This constant represents 1008 gb converted to bytes for use in size calculations.
pub const GB_1008: usize = 1082331758592;

/// 1009GB in bytes.
/// This constant represents 1009 gb converted to bytes for use in size calculations.
pub const GB_1009: usize = 1083405500416;

/// 1010GB in bytes.
/// This constant represents 1010 gb converted to bytes for use in size calculations.
pub const GB_1010: usize = 1084479242240;

/// 1011GB in bytes.
/// This constant represents 1011 gb converted to bytes for use in size calculations.
pub const GB_1011: usize = 1085552984064;

/// 1012GB in bytes.
/// This constant represents 1012 gb converted to bytes for use in size calculations.
pub const GB_1012: usize = 1086626725888;

/// 1013GB in bytes.
/// This constant represents 1013 gb converted to bytes for use in size calculations.
pub const GB_1013: usize = 1087700467712;

/// 1014GB in bytes.
/// This constant represents 1014 gb converted to bytes for use in size calculations.
pub const GB_1014: usize = 1088774209536;

/// 1015GB in bytes.
/// This constant represents 1015 gb converted to bytes for use in size calculations.
pub const GB_1015: usize = 1089847951360;

/// 1016GB in bytes.
/// This constant represents 1016 gb converted to bytes for use in size calculations.
pub const GB_1016: usize = 1090921693184;

/// 1017GB in bytes.
/// This constant represents 1017 gb converted to bytes for use in size calculations.
pub const GB_1017: usize = 1091995435008;

/// 1018GB in bytes.
/// This constant represents 1018 gb converted to bytes for use in size calculations.
pub const GB_1018: usize = 1093069176832;

/// 1019GB in bytes.
/// This constant represents 1019 gb converted to bytes for use in size calculations.
pub const GB_1019: usize = 1094142918656;

/// 1020GB in bytes.
/// This constant represents 1020 gb converted to bytes for use in size calculations.
pub const GB_1020: usize = 1095216660480;

/// 1021GB in bytes.
/// This constant represents 1021 gb converted to bytes for use in size calculations.
pub const GB_1021: usize = 1096290402304;

/// 1022GB in bytes.
/// This constant represents 1022 gb converted to bytes for use in size calculations.
pub const GB_1022: usize = 1097364144128;

/// 1023GB in bytes.
/// This constant represents 1023 gb converted to bytes for use in size calculations.
pub const GB_1023: usize = 1098437885952;

/// 1024GB in bytes.
/// This constant represents 1024 gb converted to bytes for use in size calculations.
pub const GB_1024: usize = 1099511627776;

// Storage unit constants from 1TB to 1024TB
/// 1TB in bytes.
/// This constant represents 1 tb converted to bytes for use in size calculations.
pub const TB_1: usize = 1099511627776;

/// 2TB in bytes.
/// This constant represents 2 tb converted to bytes for use in size calculations.
pub const TB_2: usize = 2199023255552;

/// 3TB in bytes.
/// This constant represents 3 tb converted to bytes for use in size calculations.
pub const TB_3: usize = 3298534883328;

/// 4TB in bytes.
/// This constant represents 4 tb converted to bytes for use in size calculations.
pub const TB_4: usize = 4398046511104;

/// 5TB in bytes.
/// This constant represents 5 tb converted to bytes for use in size calculations.
pub const TB_5: usize = 5497558138880;

/// 6TB in bytes.
/// This constant represents 6 tb converted to bytes for use in size calculations.
pub const TB_6: usize = 6597069766656;

/// 7TB in bytes.
/// This constant represents 7 tb converted to bytes for use in size calculations.
pub const TB_7: usize = 7696581394432;

/// 8TB in bytes.
/// This constant represents 8 tb converted to bytes for use in size calculations.
pub const TB_8: usize = 8796093022208;

/// 9TB in bytes.
/// This constant represents 9 tb converted to bytes for use in size calculations.
pub const TB_9: usize = 9895604649984;

/// 10TB in bytes.
/// This constant represents 10 tb converted to bytes for use in size calculations.
pub const TB_10: usize = 10995116277760;

/// 11TB in bytes.
/// This constant represents 11 tb converted to bytes for use in size calculations.
pub const TB_11: usize = 12094627905536;

/// 12TB in bytes.
/// This constant represents 12 tb converted to bytes for use in size calculations.
pub const TB_12: usize = 13194139533312;

/// 13TB in bytes.
/// This constant represents 13 tb converted to bytes for use in size calculations.
pub const TB_13: usize = 14293651161088;

/// 14TB in bytes.
/// This constant represents 14 tb converted to bytes for use in size calculations.
pub const TB_14: usize = 15393162788864;

/// 15TB in bytes.
/// This constant represents 15 tb converted to bytes for use in size calculations.
pub const TB_15: usize = 16492674416640;

/// 16TB in bytes.
/// This constant represents 16 tb converted to bytes for use in size calculations.
pub const TB_16: usize = 17592186044416;

/// 17TB in bytes.
/// This constant represents 17 tb converted to bytes for use in size calculations.
pub const TB_17: usize = 18691697672192;

/// 18TB in bytes.
/// This constant represents 18 tb converted to bytes for use in size calculations.
pub const TB_18: usize = 19791209299968;

/// 19TB in bytes.
/// This constant represents 19 tb converted to bytes for use in size calculations.
pub const TB_19: usize = 20890720927744;

/// 20TB in bytes.
/// This constant represents 20 tb converted to bytes for use in size calculations.
pub const TB_20: usize = 21990232555520;

/// 21TB in bytes.
/// This constant represents 21 tb converted to bytes for use in size calculations.
pub const TB_21: usize = 23089744183296;

/// 22TB in bytes.
/// This constant represents 22 tb converted to bytes for use in size calculations.
pub const TB_22: usize = 24189255811072;

/// 23TB in bytes.
/// This constant represents 23 tb converted to bytes for use in size calculations.
pub const TB_23: usize = 25288767438848;

/// 24TB in bytes.
/// This constant represents 24 tb converted to bytes for use in size calculations.
pub const TB_24: usize = 26388279066624;

/// 25TB in bytes.
/// This constant represents 25 tb converted to bytes for use in size calculations.
pub const TB_25: usize = 27487790694400;

/// 26TB in bytes.
/// This constant represents 26 tb converted to bytes for use in size calculations.
pub const TB_26: usize = 28587302322176;

/// 27TB in bytes.
/// This constant represents 27 tb converted to bytes for use in size calculations.
pub const TB_27: usize = 29686813949952;

/// 28TB in bytes.
/// This constant represents 28 tb converted to bytes for use in size calculations.
pub const TB_28: usize = 30786325577728;

/// 29TB in bytes.
/// This constant represents 29 tb converted to bytes for use in size calculations.
pub const TB_29: usize = 31885837205504;

/// 30TB in bytes.
/// This constant represents 30 tb converted to bytes for use in size calculations.
pub const TB_30: usize = 32985348833280;

/// 31TB in bytes.
/// This constant represents 31 tb converted to bytes for use in size calculations.
pub const TB_31: usize = 34084860461056;

/// 32TB in bytes.
/// This constant represents 32 tb converted to bytes for use in size calculations.
pub const TB_32: usize = 35184372088832;

/// 33TB in bytes.
/// This constant represents 33 tb converted to bytes for use in size calculations.
pub const TB_33: usize = 36283883716608;

/// 34TB in bytes.
/// This constant represents 34 tb converted to bytes for use in size calculations.
pub const TB_34: usize = 37383395344384;

/// 35TB in bytes.
/// This constant represents 35 tb converted to bytes for use in size calculations.
pub const TB_35: usize = 38482906972160;

/// 36TB in bytes.
/// This constant represents 36 tb converted to bytes for use in size calculations.
pub const TB_36: usize = 39582418599936;

/// 37TB in bytes.
/// This constant represents 37 tb converted to bytes for use in size calculations.
pub const TB_37: usize = 40681930227712;

/// 38TB in bytes.
/// This constant represents 38 tb converted to bytes for use in size calculations.
pub const TB_38: usize = 41781441855488;

/// 39TB in bytes.
/// This constant represents 39 tb converted to bytes for use in size calculations.
pub const TB_39: usize = 42880953483264;

/// 40TB in bytes.
/// This constant represents 40 tb converted to bytes for use in size calculations.
pub const TB_40: usize = 43980465111040;

/// 41TB in bytes.
/// This constant represents 41 tb converted to bytes for use in size calculations.
pub const TB_41: usize = 45079976738816;

/// 42TB in bytes.
/// This constant represents 42 tb converted to bytes for use in size calculations.
pub const TB_42: usize = 46179488366592;

/// 43TB in bytes.
/// This constant represents 43 tb converted to bytes for use in size calculations.
pub const TB_43: usize = 47278999994368;

/// 44TB in bytes.
/// This constant represents 44 tb converted to bytes for use in size calculations.
pub const TB_44: usize = 48378511622144;

/// 45TB in bytes.
/// This constant represents 45 tb converted to bytes for use in size calculations.
pub const TB_45: usize = 49478023249920;

/// 46TB in bytes.
/// This constant represents 46 tb converted to bytes for use in size calculations.
pub const TB_46: usize = 50577534877696;

/// 47TB in bytes.
/// This constant represents 47 tb converted to bytes for use in size calculations.
pub const TB_47: usize = 51677046505472;

/// 48TB in bytes.
/// This constant represents 48 tb converted to bytes for use in size calculations.
pub const TB_48: usize = 52776558133248;

/// 49TB in bytes.
/// This constant represents 49 tb converted to bytes for use in size calculations.
pub const TB_49: usize = 53876069761024;

/// 50TB in bytes.
/// This constant represents 50 tb converted to bytes for use in size calculations.
pub const TB_50: usize = 54975581388800;

/// 51TB in bytes.
/// This constant represents 51 tb converted to bytes for use in size calculations.
pub const TB_51: usize = 56075093016576;

/// 52TB in bytes.
/// This constant represents 52 tb converted to bytes for use in size calculations.
pub const TB_52: usize = 57174604644352;

/// 53TB in bytes.
/// This constant represents 53 tb converted to bytes for use in size calculations.
pub const TB_53: usize = 58274116272128;

/// 54TB in bytes.
/// This constant represents 54 tb converted to bytes for use in size calculations.
pub const TB_54: usize = 59373627899904;

/// 55TB in bytes.
/// This constant represents 55 tb converted to bytes for use in size calculations.
pub const TB_55: usize = 60473139527680;

/// 56TB in bytes.
/// This constant represents 56 tb converted to bytes for use in size calculations.
pub const TB_56: usize = 61572651155456;

/// 57TB in bytes.
/// This constant represents 57 tb converted to bytes for use in size calculations.
pub const TB_57: usize = 62672162783232;

/// 58TB in bytes.
/// This constant represents 58 tb converted to bytes for use in size calculations.
pub const TB_58: usize = 63771674411008;

/// 59TB in bytes.
/// This constant represents 59 tb converted to bytes for use in size calculations.
pub const TB_59: usize = 64871186038784;

/// 60TB in bytes.
/// This constant represents 60 tb converted to bytes for use in size calculations.
pub const TB_60: usize = 65970697666560;

/// 61TB in bytes.
/// This constant represents 61 tb converted to bytes for use in size calculations.
pub const TB_61: usize = 67070209294336;

/// 62TB in bytes.
/// This constant represents 62 tb converted to bytes for use in size calculations.
pub const TB_62: usize = 68169720922112;

/// 63TB in bytes.
/// This constant represents 63 tb converted to bytes for use in size calculations.
pub const TB_63: usize = 69269232549888;

/// 64TB in bytes.
/// This constant represents 64 tb converted to bytes for use in size calculations.
pub const TB_64: usize = 70368744177664;

/// 65TB in bytes.
/// This constant represents 65 tb converted to bytes for use in size calculations.
pub const TB_65: usize = 71468255805440;

/// 66TB in bytes.
/// This constant represents 66 tb converted to bytes for use in size calculations.
pub const TB_66: usize = 72567767433216;

/// 67TB in bytes.
/// This constant represents 67 tb converted to bytes for use in size calculations.
pub const TB_67: usize = 73667279060992;

/// 68TB in bytes.
/// This constant represents 68 tb converted to bytes for use in size calculations.
pub const TB_68: usize = 74766790688768;

/// 69TB in bytes.
/// This constant represents 69 tb converted to bytes for use in size calculations.
pub const TB_69: usize = 75866302316544;

/// 70TB in bytes.
/// This constant represents 70 tb converted to bytes for use in size calculations.
pub const TB_70: usize = 76965813944320;

/// 71TB in bytes.
/// This constant represents 71 tb converted to bytes for use in size calculations.
pub const TB_71: usize = 78065325572096;

/// 72TB in bytes.
/// This constant represents 72 tb converted to bytes for use in size calculations.
pub const TB_72: usize = 79164837199872;

/// 73TB in bytes.
/// This constant represents 73 tb converted to bytes for use in size calculations.
pub const TB_73: usize = 80264348827648;

/// 74TB in bytes.
/// This constant represents 74 tb converted to bytes for use in size calculations.
pub const TB_74: usize = 81363860455424;

/// 75TB in bytes.
/// This constant represents 75 tb converted to bytes for use in size calculations.
pub const TB_75: usize = 82463372083200;

/// 76TB in bytes.
/// This constant represents 76 tb converted to bytes for use in size calculations.
pub const TB_76: usize = 83562883710976;

/// 77TB in bytes.
/// This constant represents 77 tb converted to bytes for use in size calculations.
pub const TB_77: usize = 84662395338752;

/// 78TB in bytes.
/// This constant represents 78 tb converted to bytes for use in size calculations.
pub const TB_78: usize = 85761906966528;

/// 79TB in bytes.
/// This constant represents 79 tb converted to bytes for use in size calculations.
pub const TB_79: usize = 86861418594304;

/// 80TB in bytes.
/// This constant represents 80 tb converted to bytes for use in size calculations.
pub const TB_80: usize = 87960930222080;

/// 81TB in bytes.
/// This constant represents 81 tb converted to bytes for use in size calculations.
pub const TB_81: usize = 89060441849856;

/// 82TB in bytes.
/// This constant represents 82 tb converted to bytes for use in size calculations.
pub const TB_82: usize = 90159953477632;

/// 83TB in bytes.
/// This constant represents 83 tb converted to bytes for use in size calculations.
pub const TB_83: usize = 91259465105408;

/// 84TB in bytes.
/// This constant represents 84 tb converted to bytes for use in size calculations.
pub const TB_84: usize = 92358976733184;

/// 85TB in bytes.
/// This constant represents 85 tb converted to bytes for use in size calculations.
pub const TB_85: usize = 93458488360960;

/// 86TB in bytes.
/// This constant represents 86 tb converted to bytes for use in size calculations.
pub const TB_86: usize = 94557999988736;

/// 87TB in bytes.
/// This constant represents 87 tb converted to bytes for use in size calculations.
pub const TB_87: usize = 95657511616512;

/// 88TB in bytes.
/// This constant represents 88 tb converted to bytes for use in size calculations.
pub const TB_88: usize = 96757023244288;

/// 89TB in bytes.
/// This constant represents 89 tb converted to bytes for use in size calculations.
pub const TB_89: usize = 97856534872064;

/// 90TB in bytes.
/// This constant represents 90 tb converted to bytes for use in size calculations.
pub const TB_90: usize = 98956046499840;

/// 91TB in bytes.
/// This constant represents 91 tb converted to bytes for use in size calculations.
pub const TB_91: usize = 100055558127616;

/// 92TB in bytes.
/// This constant represents 92 tb converted to bytes for use in size calculations.
pub const TB_92: usize = 101155069755392;

/// 93TB in bytes.
/// This constant represents 93 tb converted to bytes for use in size calculations.
pub const TB_93: usize = 102254581383168;

/// 94TB in bytes.
/// This constant represents 94 tb converted to bytes for use in size calculations.
pub const TB_94: usize = 103354093010944;

/// 95TB in bytes.
/// This constant represents 95 tb converted to bytes for use in size calculations.
pub const TB_95: usize = 104453604638720;

/// 96TB in bytes.
/// This constant represents 96 tb converted to bytes for use in size calculations.
pub const TB_96: usize = 105553116266496;

/// 97TB in bytes.
/// This constant represents 97 tb converted to bytes for use in size calculations.
pub const TB_97: usize = 106652627894272;

/// 98TB in bytes.
/// This constant represents 98 tb converted to bytes for use in size calculations.
pub const TB_98: usize = 107752139522048;

/// 99TB in bytes.
/// This constant represents 99 tb converted to bytes for use in size calculations.
pub const TB_99: usize = 108851651149824;

/// 100TB in bytes.
/// This constant represents 100 tb converted to bytes for use in size calculations.
pub const TB_100: usize = 109951162777600;

/// 101TB in bytes.
/// This constant represents 101 tb converted to bytes for use in size calculations.
pub const TB_101: usize = 111050674405376;

/// 102TB in bytes.
/// This constant represents 102 tb converted to bytes for use in size calculations.
pub const TB_102: usize = 112150186033152;

/// 103TB in bytes.
/// This constant represents 103 tb converted to bytes for use in size calculations.
pub const TB_103: usize = 113249697660928;

/// 104TB in bytes.
/// This constant represents 104 tb converted to bytes for use in size calculations.
pub const TB_104: usize = 114349209288704;

/// 105TB in bytes.
/// This constant represents 105 tb converted to bytes for use in size calculations.
pub const TB_105: usize = 115448720916480;

/// 106TB in bytes.
/// This constant represents 106 tb converted to bytes for use in size calculations.
pub const TB_106: usize = 116548232544256;

/// 107TB in bytes.
/// This constant represents 107 tb converted to bytes for use in size calculations.
pub const TB_107: usize = 117647744172032;

/// 108TB in bytes.
/// This constant represents 108 tb converted to bytes for use in size calculations.
pub const TB_108: usize = 118747255799808;

/// 109TB in bytes.
/// This constant represents 109 tb converted to bytes for use in size calculations.
pub const TB_109: usize = 119846767427584;

/// 110TB in bytes.
/// This constant represents 110 tb converted to bytes for use in size calculations.
pub const TB_110: usize = 120946279055360;

/// 111TB in bytes.
/// This constant represents 111 tb converted to bytes for use in size calculations.
pub const TB_111: usize = 122045790683136;

/// 112TB in bytes.
/// This constant represents 112 tb converted to bytes for use in size calculations.
pub const TB_112: usize = 123145302310912;

/// 113TB in bytes.
/// This constant represents 113 tb converted to bytes for use in size calculations.
pub const TB_113: usize = 124244813938688;

/// 114TB in bytes.
/// This constant represents 114 tb converted to bytes for use in size calculations.
pub const TB_114: usize = 125344325566464;

/// 115TB in bytes.
/// This constant represents 115 tb converted to bytes for use in size calculations.
pub const TB_115: usize = 126443837194240;

/// 116TB in bytes.
/// This constant represents 116 tb converted to bytes for use in size calculations.
pub const TB_116: usize = 127543348822016;

/// 117TB in bytes.
/// This constant represents 117 tb converted to bytes for use in size calculations.
pub const TB_117: usize = 128642860449792;

/// 118TB in bytes.
/// This constant represents 118 tb converted to bytes for use in size calculations.
pub const TB_118: usize = 129742372077568;

/// 119TB in bytes.
/// This constant represents 119 tb converted to bytes for use in size calculations.
pub const TB_119: usize = 130841883705344;

/// 120TB in bytes.
/// This constant represents 120 tb converted to bytes for use in size calculations.
pub const TB_120: usize = 131941395333120;

/// 121TB in bytes.
/// This constant represents 121 tb converted to bytes for use in size calculations.
pub const TB_121: usize = 133040906960896;

/// 122TB in bytes.
/// This constant represents 122 tb converted to bytes for use in size calculations.
pub const TB_122: usize = 134140418588672;

/// 123TB in bytes.
/// This constant represents 123 tb converted to bytes for use in size calculations.
pub const TB_123: usize = 135239930216448;

/// 124TB in bytes.
/// This constant represents 124 tb converted to bytes for use in size calculations.
pub const TB_124: usize = 136339441844224;

/// 125TB in bytes.
/// This constant represents 125 tb converted to bytes for use in size calculations.
pub const TB_125: usize = 137438953472000;

/// 126TB in bytes.
/// This constant represents 126 tb converted to bytes for use in size calculations.
pub const TB_126: usize = 138538465099776;

/// 127TB in bytes.
/// This constant represents 127 tb converted to bytes for use in size calculations.
pub const TB_127: usize = 139637976727552;

/// 128TB in bytes.
/// This constant represents 128 tb converted to bytes for use in size calculations.
pub const TB_128: usize = 140737488355328;

/// 129TB in bytes.
/// This constant represents 129 tb converted to bytes for use in size calculations.
pub const TB_129: usize = 141836999983104;

/// 130TB in bytes.
/// This constant represents 130 tb converted to bytes for use in size calculations.
pub const TB_130: usize = 142936511610880;

/// 131TB in bytes.
/// This constant represents 131 tb converted to bytes for use in size calculations.
pub const TB_131: usize = 144036023238656;

/// 132TB in bytes.
/// This constant represents 132 tb converted to bytes for use in size calculations.
pub const TB_132: usize = 145135534866432;

/// 133TB in bytes.
/// This constant represents 133 tb converted to bytes for use in size calculations.
pub const TB_133: usize = 146235046494208;

/// 134TB in bytes.
/// This constant represents 134 tb converted to bytes for use in size calculations.
pub const TB_134: usize = 147334558121984;

/// 135TB in bytes.
/// This constant represents 135 tb converted to bytes for use in size calculations.
pub const TB_135: usize = 148434069749760;

/// 136TB in bytes.
/// This constant represents 136 tb converted to bytes for use in size calculations.
pub const TB_136: usize = 149533581377536;

/// 137TB in bytes.
/// This constant represents 137 tb converted to bytes for use in size calculations.
pub const TB_137: usize = 150633093005312;

/// 138TB in bytes.
/// This constant represents 138 tb converted to bytes for use in size calculations.
pub const TB_138: usize = 151732604633088;

/// 139TB in bytes.
/// This constant represents 139 tb converted to bytes for use in size calculations.
pub const TB_139: usize = 152832116260864;

/// 140TB in bytes.
/// This constant represents 140 tb converted to bytes for use in size calculations.
pub const TB_140: usize = 153931627888640;

/// 141TB in bytes.
/// This constant represents 141 tb converted to bytes for use in size calculations.
pub const TB_141: usize = 155031139516416;

/// 142TB in bytes.
/// This constant represents 142 tb converted to bytes for use in size calculations.
pub const TB_142: usize = 156130651144192;

/// 143TB in bytes.
/// This constant represents 143 tb converted to bytes for use in size calculations.
pub const TB_143: usize = 157230162771968;

/// 144TB in bytes.
/// This constant represents 144 tb converted to bytes for use in size calculations.
pub const TB_144: usize = 158329674399744;

/// 145TB in bytes.
/// This constant represents 145 tb converted to bytes for use in size calculations.
pub const TB_145: usize = 159429186027520;

/// 146TB in bytes.
/// This constant represents 146 tb converted to bytes for use in size calculations.
pub const TB_146: usize = 160528697655296;

/// 147TB in bytes.
/// This constant represents 147 tb converted to bytes for use in size calculations.
pub const TB_147: usize = 161628209283072;

/// 148TB in bytes.
/// This constant represents 148 tb converted to bytes for use in size calculations.
pub const TB_148: usize = 162727720910848;

/// 149TB in bytes.
/// This constant represents 149 tb converted to bytes for use in size calculations.
pub const TB_149: usize = 163827232538624;

/// 150TB in bytes.
/// This constant represents 150 tb converted to bytes for use in size calculations.
pub const TB_150: usize = 164926744166400;

/// 151TB in bytes.
/// This constant represents 151 tb converted to bytes for use in size calculations.
pub const TB_151: usize = 166026255794176;

/// 152TB in bytes.
/// This constant represents 152 tb converted to bytes for use in size calculations.
pub const TB_152: usize = 167125767421952;

/// 153TB in bytes.
/// This constant represents 153 tb converted to bytes for use in size calculations.
pub const TB_153: usize = 168225279049728;

/// 154TB in bytes.
/// This constant represents 154 tb converted to bytes for use in size calculations.
pub const TB_154: usize = 169324790677504;

/// 155TB in bytes.
/// This constant represents 155 tb converted to bytes for use in size calculations.
pub const TB_155: usize = 170424302305280;

/// 156TB in bytes.
/// This constant represents 156 tb converted to bytes for use in size calculations.
pub const TB_156: usize = 171523813933056;

/// 157TB in bytes.
/// This constant represents 157 tb converted to bytes for use in size calculations.
pub const TB_157: usize = 172623325560832;

/// 158TB in bytes.
/// This constant represents 158 tb converted to bytes for use in size calculations.
pub const TB_158: usize = 173722837188608;

/// 159TB in bytes.
/// This constant represents 159 tb converted to bytes for use in size calculations.
pub const TB_159: usize = 174822348816384;

/// 160TB in bytes.
/// This constant represents 160 tb converted to bytes for use in size calculations.
pub const TB_160: usize = 175921860444160;

/// 161TB in bytes.
/// This constant represents 161 tb converted to bytes for use in size calculations.
pub const TB_161: usize = 177021372071936;

/// 162TB in bytes.
/// This constant represents 162 tb converted to bytes for use in size calculations.
pub const TB_162: usize = 178120883699712;

/// 163TB in bytes.
/// This constant represents 163 tb converted to bytes for use in size calculations.
pub const TB_163: usize = 179220395327488;

/// 164TB in bytes.
/// This constant represents 164 tb converted to bytes for use in size calculations.
pub const TB_164: usize = 180319906955264;

/// 165TB in bytes.
/// This constant represents 165 tb converted to bytes for use in size calculations.
pub const TB_165: usize = 181419418583040;

/// 166TB in bytes.
/// This constant represents 166 tb converted to bytes for use in size calculations.
pub const TB_166: usize = 182518930210816;

/// 167TB in bytes.
/// This constant represents 167 tb converted to bytes for use in size calculations.
pub const TB_167: usize = 183618441838592;

/// 168TB in bytes.
/// This constant represents 168 tb converted to bytes for use in size calculations.
pub const TB_168: usize = 184717953466368;

/// 169TB in bytes.
/// This constant represents 169 tb converted to bytes for use in size calculations.
pub const TB_169: usize = 185817465094144;

/// 170TB in bytes.
/// This constant represents 170 tb converted to bytes for use in size calculations.
pub const TB_170: usize = 186916976721920;

/// 171TB in bytes.
/// This constant represents 171 tb converted to bytes for use in size calculations.
pub const TB_171: usize = 188016488349696;

/// 172TB in bytes.
/// This constant represents 172 tb converted to bytes for use in size calculations.
pub const TB_172: usize = 189115999977472;

/// 173TB in bytes.
/// This constant represents 173 tb converted to bytes for use in size calculations.
pub const TB_173: usize = 190215511605248;

/// 174TB in bytes.
/// This constant represents 174 tb converted to bytes for use in size calculations.
pub const TB_174: usize = 191315023233024;

/// 175TB in bytes.
/// This constant represents 175 tb converted to bytes for use in size calculations.
pub const TB_175: usize = 192414534860800;

/// 176TB in bytes.
/// This constant represents 176 tb converted to bytes for use in size calculations.
pub const TB_176: usize = 193514046488576;

/// 177TB in bytes.
/// This constant represents 177 tb converted to bytes for use in size calculations.
pub const TB_177: usize = 194613558116352;

/// 178TB in bytes.
/// This constant represents 178 tb converted to bytes for use in size calculations.
pub const TB_178: usize = 195713069744128;

/// 179TB in bytes.
/// This constant represents 179 tb converted to bytes for use in size calculations.
pub const TB_179: usize = 196812581371904;

/// 180TB in bytes.
/// This constant represents 180 tb converted to bytes for use in size calculations.
pub const TB_180: usize = 197912092999680;

/// 181TB in bytes.
/// This constant represents 181 tb converted to bytes for use in size calculations.
pub const TB_181: usize = 199011604627456;

/// 182TB in bytes.
/// This constant represents 182 tb converted to bytes for use in size calculations.
pub const TB_182: usize = 200111116255232;

/// 183TB in bytes.
/// This constant represents 183 tb converted to bytes for use in size calculations.
pub const TB_183: usize = 201210627883008;

/// 184TB in bytes.
/// This constant represents 184 tb converted to bytes for use in size calculations.
pub const TB_184: usize = 202310139510784;

/// 185TB in bytes.
/// This constant represents 185 tb converted to bytes for use in size calculations.
pub const TB_185: usize = 203409651138560;

/// 186TB in bytes.
/// This constant represents 186 tb converted to bytes for use in size calculations.
pub const TB_186: usize = 204509162766336;

/// 187TB in bytes.
/// This constant represents 187 tb converted to bytes for use in size calculations.
pub const TB_187: usize = 205608674394112;

/// 188TB in bytes.
/// This constant represents 188 tb converted to bytes for use in size calculations.
pub const TB_188: usize = 206708186021888;

/// 189TB in bytes.
/// This constant represents 189 tb converted to bytes for use in size calculations.
pub const TB_189: usize = 207807697649664;

/// 190TB in bytes.
/// This constant represents 190 tb converted to bytes for use in size calculations.
pub const TB_190: usize = 208907209277440;

/// 191TB in bytes.
/// This constant represents 191 tb converted to bytes for use in size calculations.
pub const TB_191: usize = 210006720905216;

/// 192TB in bytes.
/// This constant represents 192 tb converted to bytes for use in size calculations.
pub const TB_192: usize = 211106232532992;

/// 193TB in bytes.
/// This constant represents 193 tb converted to bytes for use in size calculations.
pub const TB_193: usize = 212205744160768;

/// 194TB in bytes.
/// This constant represents 194 tb converted to bytes for use in size calculations.
pub const TB_194: usize = 213305255788544;

/// 195TB in bytes.
/// This constant represents 195 tb converted to bytes for use in size calculations.
pub const TB_195: usize = 214404767416320;

/// 196TB in bytes.
/// This constant represents 196 tb converted to bytes for use in size calculations.
pub const TB_196: usize = 215504279044096;

/// 197TB in bytes.
/// This constant represents 197 tb converted to bytes for use in size calculations.
pub const TB_197: usize = 216603790671872;

/// 198TB in bytes.
/// This constant represents 198 tb converted to bytes for use in size calculations.
pub const TB_198: usize = 217703302299648;

/// 199TB in bytes.
/// This constant represents 199 tb converted to bytes for use in size calculations.
pub const TB_199: usize = 218802813927424;

/// 200TB in bytes.
/// This constant represents 200 tb converted to bytes for use in size calculations.
pub const TB_200: usize = 219902325555200;

/// 201TB in bytes.
/// This constant represents 201 tb converted to bytes for use in size calculations.
pub const TB_201: usize = 221001837182976;

/// 202TB in bytes.
/// This constant represents 202 tb converted to bytes for use in size calculations.
pub const TB_202: usize = 222101348810752;

/// 203TB in bytes.
/// This constant represents 203 tb converted to bytes for use in size calculations.
pub const TB_203: usize = 223200860438528;

/// 204TB in bytes.
/// This constant represents 204 tb converted to bytes for use in size calculations.
pub const TB_204: usize = 224300372066304;

/// 205TB in bytes.
/// This constant represents 205 tb converted to bytes for use in size calculations.
pub const TB_205: usize = 225399883694080;

/// 206TB in bytes.
/// This constant represents 206 tb converted to bytes for use in size calculations.
pub const TB_206: usize = 226499395321856;

/// 207TB in bytes.
/// This constant represents 207 tb converted to bytes for use in size calculations.
pub const TB_207: usize = 227598906949632;

/// 208TB in bytes.
/// This constant represents 208 tb converted to bytes for use in size calculations.
pub const TB_208: usize = 228698418577408;

/// 209TB in bytes.
/// This constant represents 209 tb converted to bytes for use in size calculations.
pub const TB_209: usize = 229797930205184;

/// 210TB in bytes.
/// This constant represents 210 tb converted to bytes for use in size calculations.
pub const TB_210: usize = 230897441832960;

/// 211TB in bytes.
/// This constant represents 211 tb converted to bytes for use in size calculations.
pub const TB_211: usize = 231996953460736;

/// 212TB in bytes.
/// This constant represents 212 tb converted to bytes for use in size calculations.
pub const TB_212: usize = 233096465088512;

/// 213TB in bytes.
/// This constant represents 213 tb converted to bytes for use in size calculations.
pub const TB_213: usize = 234195976716288;

/// 214TB in bytes.
/// This constant represents 214 tb converted to bytes for use in size calculations.
pub const TB_214: usize = 235295488344064;

/// 215TB in bytes.
/// This constant represents 215 tb converted to bytes for use in size calculations.
pub const TB_215: usize = 236394999971840;

/// 216TB in bytes.
/// This constant represents 216 tb converted to bytes for use in size calculations.
pub const TB_216: usize = 237494511599616;

/// 217TB in bytes.
/// This constant represents 217 tb converted to bytes for use in size calculations.
pub const TB_217: usize = 238594023227392;

/// 218TB in bytes.
/// This constant represents 218 tb converted to bytes for use in size calculations.
pub const TB_218: usize = 239693534855168;

/// 219TB in bytes.
/// This constant represents 219 tb converted to bytes for use in size calculations.
pub const TB_219: usize = 240793046482944;

/// 220TB in bytes.
/// This constant represents 220 tb converted to bytes for use in size calculations.
pub const TB_220: usize = 241892558110720;

/// 221TB in bytes.
/// This constant represents 221 tb converted to bytes for use in size calculations.
pub const TB_221: usize = 242992069738496;

/// 222TB in bytes.
/// This constant represents 222 tb converted to bytes for use in size calculations.
pub const TB_222: usize = 244091581366272;

/// 223TB in bytes.
/// This constant represents 223 tb converted to bytes for use in size calculations.
pub const TB_223: usize = 245191092994048;

/// 224TB in bytes.
/// This constant represents 224 tb converted to bytes for use in size calculations.
pub const TB_224: usize = 246290604621824;

/// 225TB in bytes.
/// This constant represents 225 tb converted to bytes for use in size calculations.
pub const TB_225: usize = 247390116249600;

/// 226TB in bytes.
/// This constant represents 226 tb converted to bytes for use in size calculations.
pub const TB_226: usize = 248489627877376;

/// 227TB in bytes.
/// This constant represents 227 tb converted to bytes for use in size calculations.
pub const TB_227: usize = 249589139505152;

/// 228TB in bytes.
/// This constant represents 228 tb converted to bytes for use in size calculations.
pub const TB_228: usize = 250688651132928;

/// 229TB in bytes.
/// This constant represents 229 tb converted to bytes for use in size calculations.
pub const TB_229: usize = 251788162760704;

/// 230TB in bytes.
/// This constant represents 230 tb converted to bytes for use in size calculations.
pub const TB_230: usize = 252887674388480;

/// 231TB in bytes.
/// This constant represents 231 tb converted to bytes for use in size calculations.
pub const TB_231: usize = 253987186016256;

/// 232TB in bytes.
/// This constant represents 232 tb converted to bytes for use in size calculations.
pub const TB_232: usize = 255086697644032;

/// 233TB in bytes.
/// This constant represents 233 tb converted to bytes for use in size calculations.
pub const TB_233: usize = 256186209271808;

/// 234TB in bytes.
/// This constant represents 234 tb converted to bytes for use in size calculations.
pub const TB_234: usize = 257285720899584;

/// 235TB in bytes.
/// This constant represents 235 tb converted to bytes for use in size calculations.
pub const TB_235: usize = 258385232527360;

/// 236TB in bytes.
/// This constant represents 236 tb converted to bytes for use in size calculations.
pub const TB_236: usize = 259484744155136;

/// 237TB in bytes.
/// This constant represents 237 tb converted to bytes for use in size calculations.
pub const TB_237: usize = 260584255782912;

/// 238TB in bytes.
/// This constant represents 238 tb converted to bytes for use in size calculations.
pub const TB_238: usize = 261683767410688;

/// 239TB in bytes.
/// This constant represents 239 tb converted to bytes for use in size calculations.
pub const TB_239: usize = 262783279038464;

/// 240TB in bytes.
/// This constant represents 240 tb converted to bytes for use in size calculations.
pub const TB_240: usize = 263882790666240;

/// 241TB in bytes.
/// This constant represents 241 tb converted to bytes for use in size calculations.
pub const TB_241: usize = 264982302294016;

/// 242TB in bytes.
/// This constant represents 242 tb converted to bytes for use in size calculations.
pub const TB_242: usize = 266081813921792;

/// 243TB in bytes.
/// This constant represents 243 tb converted to bytes for use in size calculations.
pub const TB_243: usize = 267181325549568;

/// 244TB in bytes.
/// This constant represents 244 tb converted to bytes for use in size calculations.
pub const TB_244: usize = 268280837177344;

/// 245TB in bytes.
/// This constant represents 245 tb converted to bytes for use in size calculations.
pub const TB_245: usize = 269380348805120;

/// 246TB in bytes.
/// This constant represents 246 tb converted to bytes for use in size calculations.
pub const TB_246: usize = 270479860432896;

/// 247TB in bytes.
/// This constant represents 247 tb converted to bytes for use in size calculations.
pub const TB_247: usize = 271579372060672;

/// 248TB in bytes.
/// This constant represents 248 tb converted to bytes for use in size calculations.
pub const TB_248: usize = 272678883688448;

/// 249TB in bytes.
/// This constant represents 249 tb converted to bytes for use in size calculations.
pub const TB_249: usize = 273778395316224;

/// 250TB in bytes.
/// This constant represents 250 tb converted to bytes for use in size calculations.
pub const TB_250: usize = 274877906944000;

/// 251TB in bytes.
/// This constant represents 251 tb converted to bytes for use in size calculations.
pub const TB_251: usize = 275977418571776;

/// 252TB in bytes.
/// This constant represents 252 tb converted to bytes for use in size calculations.
pub const TB_252: usize = 277076930199552;

/// 253TB in bytes.
/// This constant represents 253 tb converted to bytes for use in size calculations.
pub const TB_253: usize = 278176441827328;

/// 254TB in bytes.
/// This constant represents 254 tb converted to bytes for use in size calculations.
pub const TB_254: usize = 279275953455104;

/// 255TB in bytes.
/// This constant represents 255 tb converted to bytes for use in size calculations.
pub const TB_255: usize = 280375465082880;

/// 256TB in bytes.
/// This constant represents 256 tb converted to bytes for use in size calculations.
pub const TB_256: usize = 281474976710656;

/// 257TB in bytes.
/// This constant represents 257 tb converted to bytes for use in size calculations.
pub const TB_257: usize = 282574488338432;

/// 258TB in bytes.
/// This constant represents 258 tb converted to bytes for use in size calculations.
pub const TB_258: usize = 283673999966208;

/// 259TB in bytes.
/// This constant represents 259 tb converted to bytes for use in size calculations.
pub const TB_259: usize = 284773511593984;

/// 260TB in bytes.
/// This constant represents 260 tb converted to bytes for use in size calculations.
pub const TB_260: usize = 285873023221760;

/// 261TB in bytes.
/// This constant represents 261 tb converted to bytes for use in size calculations.
pub const TB_261: usize = 286972534849536;

/// 262TB in bytes.
/// This constant represents 262 tb converted to bytes for use in size calculations.
pub const TB_262: usize = 288072046477312;

/// 263TB in bytes.
/// This constant represents 263 tb converted to bytes for use in size calculations.
pub const TB_263: usize = 289171558105088;

/// 264TB in bytes.
/// This constant represents 264 tb converted to bytes for use in size calculations.
pub const TB_264: usize = 290271069732864;

/// 265TB in bytes.
/// This constant represents 265 tb converted to bytes for use in size calculations.
pub const TB_265: usize = 291370581360640;

/// 266TB in bytes.
/// This constant represents 266 tb converted to bytes for use in size calculations.
pub const TB_266: usize = 292470092988416;

/// 267TB in bytes.
/// This constant represents 267 tb converted to bytes for use in size calculations.
pub const TB_267: usize = 293569604616192;

/// 268TB in bytes.
/// This constant represents 268 tb converted to bytes for use in size calculations.
pub const TB_268: usize = 294669116243968;

/// 269TB in bytes.
/// This constant represents 269 tb converted to bytes for use in size calculations.
pub const TB_269: usize = 295768627871744;

/// 270TB in bytes.
/// This constant represents 270 tb converted to bytes for use in size calculations.
pub const TB_270: usize = 296868139499520;

/// 271TB in bytes.
/// This constant represents 271 tb converted to bytes for use in size calculations.
pub const TB_271: usize = 297967651127296;

/// 272TB in bytes.
/// This constant represents 272 tb converted to bytes for use in size calculations.
pub const TB_272: usize = 299067162755072;

/// 273TB in bytes.
/// This constant represents 273 tb converted to bytes for use in size calculations.
pub const TB_273: usize = 300166674382848;

/// 274TB in bytes.
/// This constant represents 274 tb converted to bytes for use in size calculations.
pub const TB_274: usize = 301266186010624;

/// 275TB in bytes.
/// This constant represents 275 tb converted to bytes for use in size calculations.
pub const TB_275: usize = 302365697638400;

/// 276TB in bytes.
/// This constant represents 276 tb converted to bytes for use in size calculations.
pub const TB_276: usize = 303465209266176;

/// 277TB in bytes.
/// This constant represents 277 tb converted to bytes for use in size calculations.
pub const TB_277: usize = 304564720893952;

/// 278TB in bytes.
/// This constant represents 278 tb converted to bytes for use in size calculations.
pub const TB_278: usize = 305664232521728;

/// 279TB in bytes.
/// This constant represents 279 tb converted to bytes for use in size calculations.
pub const TB_279: usize = 306763744149504;

/// 280TB in bytes.
/// This constant represents 280 tb converted to bytes for use in size calculations.
pub const TB_280: usize = 307863255777280;

/// 281TB in bytes.
/// This constant represents 281 tb converted to bytes for use in size calculations.
pub const TB_281: usize = 308962767405056;

/// 282TB in bytes.
/// This constant represents 282 tb converted to bytes for use in size calculations.
pub const TB_282: usize = 310062279032832;

/// 283TB in bytes.
/// This constant represents 283 tb converted to bytes for use in size calculations.
pub const TB_283: usize = 311161790660608;

/// 284TB in bytes.
/// This constant represents 284 tb converted to bytes for use in size calculations.
pub const TB_284: usize = 312261302288384;

/// 285TB in bytes.
/// This constant represents 285 tb converted to bytes for use in size calculations.
pub const TB_285: usize = 313360813916160;

/// 286TB in bytes.
/// This constant represents 286 tb converted to bytes for use in size calculations.
pub const TB_286: usize = 314460325543936;

/// 287TB in bytes.
/// This constant represents 287 tb converted to bytes for use in size calculations.
pub const TB_287: usize = 315559837171712;

/// 288TB in bytes.
/// This constant represents 288 tb converted to bytes for use in size calculations.
pub const TB_288: usize = 316659348799488;

/// 289TB in bytes.
/// This constant represents 289 tb converted to bytes for use in size calculations.
pub const TB_289: usize = 317758860427264;

/// 290TB in bytes.
/// This constant represents 290 tb converted to bytes for use in size calculations.
pub const TB_290: usize = 318858372055040;

/// 291TB in bytes.
/// This constant represents 291 tb converted to bytes for use in size calculations.
pub const TB_291: usize = 319957883682816;

/// 292TB in bytes.
/// This constant represents 292 tb converted to bytes for use in size calculations.
pub const TB_292: usize = 321057395310592;

/// 293TB in bytes.
/// This constant represents 293 tb converted to bytes for use in size calculations.
pub const TB_293: usize = 322156906938368;

/// 294TB in bytes.
/// This constant represents 294 tb converted to bytes for use in size calculations.
pub const TB_294: usize = 323256418566144;

/// 295TB in bytes.
/// This constant represents 295 tb converted to bytes for use in size calculations.
pub const TB_295: usize = 324355930193920;

/// 296TB in bytes.
/// This constant represents 296 tb converted to bytes for use in size calculations.
pub const TB_296: usize = 325455441821696;

/// 297TB in bytes.
/// This constant represents 297 tb converted to bytes for use in size calculations.
pub const TB_297: usize = 326554953449472;

/// 298TB in bytes.
/// This constant represents 298 tb converted to bytes for use in size calculations.
pub const TB_298: usize = 327654465077248;

/// 299TB in bytes.
/// This constant represents 299 tb converted to bytes for use in size calculations.
pub const TB_299: usize = 328753976705024;

/// 300TB in bytes.
/// This constant represents 300 tb converted to bytes for use in size calculations.
pub const TB_300: usize = 329853488332800;

/// 301TB in bytes.
/// This constant represents 301 tb converted to bytes for use in size calculations.
pub const TB_301: usize = 330952999960576;

/// 302TB in bytes.
/// This constant represents 302 tb converted to bytes for use in size calculations.
pub const TB_302: usize = 332052511588352;

/// 303TB in bytes.
/// This constant represents 303 tb converted to bytes for use in size calculations.
pub const TB_303: usize = 333152023216128;

/// 304TB in bytes.
/// This constant represents 304 tb converted to bytes for use in size calculations.
pub const TB_304: usize = 334251534843904;

/// 305TB in bytes.
/// This constant represents 305 tb converted to bytes for use in size calculations.
pub const TB_305: usize = 335351046471680;

/// 306TB in bytes.
/// This constant represents 306 tb converted to bytes for use in size calculations.
pub const TB_306: usize = 336450558099456;

/// 307TB in bytes.
/// This constant represents 307 tb converted to bytes for use in size calculations.
pub const TB_307: usize = 337550069727232;

/// 308TB in bytes.
/// This constant represents 308 tb converted to bytes for use in size calculations.
pub const TB_308: usize = 338649581355008;

/// 309TB in bytes.
/// This constant represents 309 tb converted to bytes for use in size calculations.
pub const TB_309: usize = 339749092982784;

/// 310TB in bytes.
/// This constant represents 310 tb converted to bytes for use in size calculations.
pub const TB_310: usize = 340848604610560;

/// 311TB in bytes.
/// This constant represents 311 tb converted to bytes for use in size calculations.
pub const TB_311: usize = 341948116238336;

/// 312TB in bytes.
/// This constant represents 312 tb converted to bytes for use in size calculations.
pub const TB_312: usize = 343047627866112;

/// 313TB in bytes.
/// This constant represents 313 tb converted to bytes for use in size calculations.
pub const TB_313: usize = 344147139493888;

/// 314TB in bytes.
/// This constant represents 314 tb converted to bytes for use in size calculations.
pub const TB_314: usize = 345246651121664;

/// 315TB in bytes.
/// This constant represents 315 tb converted to bytes for use in size calculations.
pub const TB_315: usize = 346346162749440;

/// 316TB in bytes.
/// This constant represents 316 tb converted to bytes for use in size calculations.
pub const TB_316: usize = 347445674377216;

/// 317TB in bytes.
/// This constant represents 317 tb converted to bytes for use in size calculations.
pub const TB_317: usize = 348545186004992;

/// 318TB in bytes.
/// This constant represents 318 tb converted to bytes for use in size calculations.
pub const TB_318: usize = 349644697632768;

/// 319TB in bytes.
/// This constant represents 319 tb converted to bytes for use in size calculations.
pub const TB_319: usize = 350744209260544;

/// 320TB in bytes.
/// This constant represents 320 tb converted to bytes for use in size calculations.
pub const TB_320: usize = 351843720888320;

/// 321TB in bytes.
/// This constant represents 321 tb converted to bytes for use in size calculations.
pub const TB_321: usize = 352943232516096;

/// 322TB in bytes.
/// This constant represents 322 tb converted to bytes for use in size calculations.
pub const TB_322: usize = 354042744143872;

/// 323TB in bytes.
/// This constant represents 323 tb converted to bytes for use in size calculations.
pub const TB_323: usize = 355142255771648;

/// 324TB in bytes.
/// This constant represents 324 tb converted to bytes for use in size calculations.
pub const TB_324: usize = 356241767399424;

/// 325TB in bytes.
/// This constant represents 325 tb converted to bytes for use in size calculations.
pub const TB_325: usize = 357341279027200;

/// 326TB in bytes.
/// This constant represents 326 tb converted to bytes for use in size calculations.
pub const TB_326: usize = 358440790654976;

/// 327TB in bytes.
/// This constant represents 327 tb converted to bytes for use in size calculations.
pub const TB_327: usize = 359540302282752;

/// 328TB in bytes.
/// This constant represents 328 tb converted to bytes for use in size calculations.
pub const TB_328: usize = 360639813910528;

/// 329TB in bytes.
/// This constant represents 329 tb converted to bytes for use in size calculations.
pub const TB_329: usize = 361739325538304;

/// 330TB in bytes.
/// This constant represents 330 tb converted to bytes for use in size calculations.
pub const TB_330: usize = 362838837166080;

/// 331TB in bytes.
/// This constant represents 331 tb converted to bytes for use in size calculations.
pub const TB_331: usize = 363938348793856;

/// 332TB in bytes.
/// This constant represents 332 tb converted to bytes for use in size calculations.
pub const TB_332: usize = 365037860421632;

/// 333TB in bytes.
/// This constant represents 333 tb converted to bytes for use in size calculations.
pub const TB_333: usize = 366137372049408;

/// 334TB in bytes.
/// This constant represents 334 tb converted to bytes for use in size calculations.
pub const TB_334: usize = 367236883677184;

/// 335TB in bytes.
/// This constant represents 335 tb converted to bytes for use in size calculations.
pub const TB_335: usize = 368336395304960;

/// 336TB in bytes.
/// This constant represents 336 tb converted to bytes for use in size calculations.
pub const TB_336: usize = 369435906932736;

/// 337TB in bytes.
/// This constant represents 337 tb converted to bytes for use in size calculations.
pub const TB_337: usize = 370535418560512;

/// 338TB in bytes.
/// This constant represents 338 tb converted to bytes for use in size calculations.
pub const TB_338: usize = 371634930188288;

/// 339TB in bytes.
/// This constant represents 339 tb converted to bytes for use in size calculations.
pub const TB_339: usize = 372734441816064;

/// 340TB in bytes.
/// This constant represents 340 tb converted to bytes for use in size calculations.
pub const TB_340: usize = 373833953443840;

/// 341TB in bytes.
/// This constant represents 341 tb converted to bytes for use in size calculations.
pub const TB_341: usize = 374933465071616;

/// 342TB in bytes.
/// This constant represents 342 tb converted to bytes for use in size calculations.
pub const TB_342: usize = 376032976699392;

/// 343TB in bytes.
/// This constant represents 343 tb converted to bytes for use in size calculations.
pub const TB_343: usize = 377132488327168;

/// 344TB in bytes.
/// This constant represents 344 tb converted to bytes for use in size calculations.
pub const TB_344: usize = 378231999954944;

/// 345TB in bytes.
/// This constant represents 345 tb converted to bytes for use in size calculations.
pub const TB_345: usize = 379331511582720;

/// 346TB in bytes.
/// This constant represents 346 tb converted to bytes for use in size calculations.
pub const TB_346: usize = 380431023210496;

/// 347TB in bytes.
/// This constant represents 347 tb converted to bytes for use in size calculations.
pub const TB_347: usize = 381530534838272;

/// 348TB in bytes.
/// This constant represents 348 tb converted to bytes for use in size calculations.
pub const TB_348: usize = 382630046466048;

/// 349TB in bytes.
/// This constant represents 349 tb converted to bytes for use in size calculations.
pub const TB_349: usize = 383729558093824;

/// 350TB in bytes.
/// This constant represents 350 tb converted to bytes for use in size calculations.
pub const TB_350: usize = 384829069721600;

/// 351TB in bytes.
/// This constant represents 351 tb converted to bytes for use in size calculations.
pub const TB_351: usize = 385928581349376;

/// 352TB in bytes.
/// This constant represents 352 tb converted to bytes for use in size calculations.
pub const TB_352: usize = 387028092977152;

/// 353TB in bytes.
/// This constant represents 353 tb converted to bytes for use in size calculations.
pub const TB_353: usize = 388127604604928;

/// 354TB in bytes.
/// This constant represents 354 tb converted to bytes for use in size calculations.
pub const TB_354: usize = 389227116232704;

/// 355TB in bytes.
/// This constant represents 355 tb converted to bytes for use in size calculations.
pub const TB_355: usize = 390326627860480;

/// 356TB in bytes.
/// This constant represents 356 tb converted to bytes for use in size calculations.
pub const TB_356: usize = 391426139488256;

/// 357TB in bytes.
/// This constant represents 357 tb converted to bytes for use in size calculations.
pub const TB_357: usize = 392525651116032;

/// 358TB in bytes.
/// This constant represents 358 tb converted to bytes for use in size calculations.
pub const TB_358: usize = 393625162743808;

/// 359TB in bytes.
/// This constant represents 359 tb converted to bytes for use in size calculations.
pub const TB_359: usize = 394724674371584;

/// 360TB in bytes.
/// This constant represents 360 tb converted to bytes for use in size calculations.
pub const TB_360: usize = 395824185999360;

/// 361TB in bytes.
/// This constant represents 361 tb converted to bytes for use in size calculations.
pub const TB_361: usize = 396923697627136;

/// 362TB in bytes.
/// This constant represents 362 tb converted to bytes for use in size calculations.
pub const TB_362: usize = 398023209254912;

/// 363TB in bytes.
/// This constant represents 363 tb converted to bytes for use in size calculations.
pub const TB_363: usize = 399122720882688;

/// 364TB in bytes.
/// This constant represents 364 tb converted to bytes for use in size calculations.
pub const TB_364: usize = 400222232510464;

/// 365TB in bytes.
/// This constant represents 365 tb converted to bytes for use in size calculations.
pub const TB_365: usize = 401321744138240;

/// 366TB in bytes.
/// This constant represents 366 tb converted to bytes for use in size calculations.
pub const TB_366: usize = 402421255766016;

/// 367TB in bytes.
/// This constant represents 367 tb converted to bytes for use in size calculations.
pub const TB_367: usize = 403520767393792;

/// 368TB in bytes.
/// This constant represents 368 tb converted to bytes for use in size calculations.
pub const TB_368: usize = 404620279021568;

/// 369TB in bytes.
/// This constant represents 369 tb converted to bytes for use in size calculations.
pub const TB_369: usize = 405719790649344;

/// 370TB in bytes.
/// This constant represents 370 tb converted to bytes for use in size calculations.
pub const TB_370: usize = 406819302277120;

/// 371TB in bytes.
/// This constant represents 371 tb converted to bytes for use in size calculations.
pub const TB_371: usize = 407918813904896;

/// 372TB in bytes.
/// This constant represents 372 tb converted to bytes for use in size calculations.
pub const TB_372: usize = 409018325532672;

/// 373TB in bytes.
/// This constant represents 373 tb converted to bytes for use in size calculations.
pub const TB_373: usize = 410117837160448;

/// 374TB in bytes.
/// This constant represents 374 tb converted to bytes for use in size calculations.
pub const TB_374: usize = 411217348788224;

/// 375TB in bytes.
/// This constant represents 375 tb converted to bytes for use in size calculations.
pub const TB_375: usize = 412316860416000;

/// 376TB in bytes.
/// This constant represents 376 tb converted to bytes for use in size calculations.
pub const TB_376: usize = 413416372043776;

/// 377TB in bytes.
/// This constant represents 377 tb converted to bytes for use in size calculations.
pub const TB_377: usize = 414515883671552;

/// 378TB in bytes.
/// This constant represents 378 tb converted to bytes for use in size calculations.
pub const TB_378: usize = 415615395299328;

/// 379TB in bytes.
/// This constant represents 379 tb converted to bytes for use in size calculations.
pub const TB_379: usize = 416714906927104;

/// 380TB in bytes.
/// This constant represents 380 tb converted to bytes for use in size calculations.
pub const TB_380: usize = 417814418554880;

/// 381TB in bytes.
/// This constant represents 381 tb converted to bytes for use in size calculations.
pub const TB_381: usize = 418913930182656;

/// 382TB in bytes.
/// This constant represents 382 tb converted to bytes for use in size calculations.
pub const TB_382: usize = 420013441810432;

/// 383TB in bytes.
/// This constant represents 383 tb converted to bytes for use in size calculations.
pub const TB_383: usize = 421112953438208;

/// 384TB in bytes.
/// This constant represents 384 tb converted to bytes for use in size calculations.
pub const TB_384: usize = 422212465065984;

/// 385TB in bytes.
/// This constant represents 385 tb converted to bytes for use in size calculations.
pub const TB_385: usize = 423311976693760;

/// 386TB in bytes.
/// This constant represents 386 tb converted to bytes for use in size calculations.
pub const TB_386: usize = 424411488321536;

/// 387TB in bytes.
/// This constant represents 387 tb converted to bytes for use in size calculations.
pub const TB_387: usize = 425510999949312;

/// 388TB in bytes.
/// This constant represents 388 tb converted to bytes for use in size calculations.
pub const TB_388: usize = 426610511577088;

/// 389TB in bytes.
/// This constant represents 389 tb converted to bytes for use in size calculations.
pub const TB_389: usize = 427710023204864;

/// 390TB in bytes.
/// This constant represents 390 tb converted to bytes for use in size calculations.
pub const TB_390: usize = 428809534832640;

/// 391TB in bytes.
/// This constant represents 391 tb converted to bytes for use in size calculations.
pub const TB_391: usize = 429909046460416;

/// 392TB in bytes.
/// This constant represents 392 tb converted to bytes for use in size calculations.
pub const TB_392: usize = 431008558088192;

/// 393TB in bytes.
/// This constant represents 393 tb converted to bytes for use in size calculations.
pub const TB_393: usize = 432108069715968;

/// 394TB in bytes.
/// This constant represents 394 tb converted to bytes for use in size calculations.
pub const TB_394: usize = 433207581343744;

/// 395TB in bytes.
/// This constant represents 395 tb converted to bytes for use in size calculations.
pub const TB_395: usize = 434307092971520;

/// 396TB in bytes.
/// This constant represents 396 tb converted to bytes for use in size calculations.
pub const TB_396: usize = 435406604599296;

/// 397TB in bytes.
/// This constant represents 397 tb converted to bytes for use in size calculations.
pub const TB_397: usize = 436506116227072;

/// 398TB in bytes.
/// This constant represents 398 tb converted to bytes for use in size calculations.
pub const TB_398: usize = 437605627854848;

/// 399TB in bytes.
/// This constant represents 399 tb converted to bytes for use in size calculations.
pub const TB_399: usize = 438705139482624;

/// 400TB in bytes.
/// This constant represents 400 tb converted to bytes for use in size calculations.
pub const TB_400: usize = 439804651110400;

/// 401TB in bytes.
/// This constant represents 401 tb converted to bytes for use in size calculations.
pub const TB_401: usize = 440904162738176;

/// 402TB in bytes.
/// This constant represents 402 tb converted to bytes for use in size calculations.
pub const TB_402: usize = 442003674365952;

/// 403TB in bytes.
/// This constant represents 403 tb converted to bytes for use in size calculations.
pub const TB_403: usize = 443103185993728;

/// 404TB in bytes.
/// This constant represents 404 tb converted to bytes for use in size calculations.
pub const TB_404: usize = 444202697621504;

/// 405TB in bytes.
/// This constant represents 405 tb converted to bytes for use in size calculations.
pub const TB_405: usize = 445302209249280;

/// 406TB in bytes.
/// This constant represents 406 tb converted to bytes for use in size calculations.
pub const TB_406: usize = 446401720877056;

/// 407TB in bytes.
/// This constant represents 407 tb converted to bytes for use in size calculations.
pub const TB_407: usize = 447501232504832;

/// 408TB in bytes.
/// This constant represents 408 tb converted to bytes for use in size calculations.
pub const TB_408: usize = 448600744132608;

/// 409TB in bytes.
/// This constant represents 409 tb converted to bytes for use in size calculations.
pub const TB_409: usize = 449700255760384;

/// 410TB in bytes.
/// This constant represents 410 tb converted to bytes for use in size calculations.
pub const TB_410: usize = 450799767388160;

/// 411TB in bytes.
/// This constant represents 411 tb converted to bytes for use in size calculations.
pub const TB_411: usize = 451899279015936;

/// 412TB in bytes.
/// This constant represents 412 tb converted to bytes for use in size calculations.
pub const TB_412: usize = 452998790643712;

/// 413TB in bytes.
/// This constant represents 413 tb converted to bytes for use in size calculations.
pub const TB_413: usize = 454098302271488;

/// 414TB in bytes.
/// This constant represents 414 tb converted to bytes for use in size calculations.
pub const TB_414: usize = 455197813899264;

/// 415TB in bytes.
/// This constant represents 415 tb converted to bytes for use in size calculations.
pub const TB_415: usize = 456297325527040;

/// 416TB in bytes.
/// This constant represents 416 tb converted to bytes for use in size calculations.
pub const TB_416: usize = 457396837154816;

/// 417TB in bytes.
/// This constant represents 417 tb converted to bytes for use in size calculations.
pub const TB_417: usize = 458496348782592;

/// 418TB in bytes.
/// This constant represents 418 tb converted to bytes for use in size calculations.
pub const TB_418: usize = 459595860410368;

/// 419TB in bytes.
/// This constant represents 419 tb converted to bytes for use in size calculations.
pub const TB_419: usize = 460695372038144;

/// 420TB in bytes.
/// This constant represents 420 tb converted to bytes for use in size calculations.
pub const TB_420: usize = 461794883665920;

/// 421TB in bytes.
/// This constant represents 421 tb converted to bytes for use in size calculations.
pub const TB_421: usize = 462894395293696;

/// 422TB in bytes.
/// This constant represents 422 tb converted to bytes for use in size calculations.
pub const TB_422: usize = 463993906921472;

/// 423TB in bytes.
/// This constant represents 423 tb converted to bytes for use in size calculations.
pub const TB_423: usize = 465093418549248;

/// 424TB in bytes.
/// This constant represents 424 tb converted to bytes for use in size calculations.
pub const TB_424: usize = 466192930177024;

/// 425TB in bytes.
/// This constant represents 425 tb converted to bytes for use in size calculations.
pub const TB_425: usize = 467292441804800;

/// 426TB in bytes.
/// This constant represents 426 tb converted to bytes for use in size calculations.
pub const TB_426: usize = 468391953432576;

/// 427TB in bytes.
/// This constant represents 427 tb converted to bytes for use in size calculations.
pub const TB_427: usize = 469491465060352;

/// 428TB in bytes.
/// This constant represents 428 tb converted to bytes for use in size calculations.
pub const TB_428: usize = 470590976688128;

/// 429TB in bytes.
/// This constant represents 429 tb converted to bytes for use in size calculations.
pub const TB_429: usize = 471690488315904;

/// 430TB in bytes.
/// This constant represents 430 tb converted to bytes for use in size calculations.
pub const TB_430: usize = 472789999943680;

/// 431TB in bytes.
/// This constant represents 431 tb converted to bytes for use in size calculations.
pub const TB_431: usize = 473889511571456;

/// 432TB in bytes.
/// This constant represents 432 tb converted to bytes for use in size calculations.
pub const TB_432: usize = 474989023199232;

/// 433TB in bytes.
/// This constant represents 433 tb converted to bytes for use in size calculations.
pub const TB_433: usize = 476088534827008;

/// 434TB in bytes.
/// This constant represents 434 tb converted to bytes for use in size calculations.
pub const TB_434: usize = 477188046454784;

/// 435TB in bytes.
/// This constant represents 435 tb converted to bytes for use in size calculations.
pub const TB_435: usize = 478287558082560;

/// 436TB in bytes.
/// This constant represents 436 tb converted to bytes for use in size calculations.
pub const TB_436: usize = 479387069710336;

/// 437TB in bytes.
/// This constant represents 437 tb converted to bytes for use in size calculations.
pub const TB_437: usize = 480486581338112;

/// 438TB in bytes.
/// This constant represents 438 tb converted to bytes for use in size calculations.
pub const TB_438: usize = 481586092965888;

/// 439TB in bytes.
/// This constant represents 439 tb converted to bytes for use in size calculations.
pub const TB_439: usize = 482685604593664;

/// 440TB in bytes.
/// This constant represents 440 tb converted to bytes for use in size calculations.
pub const TB_440: usize = 483785116221440;

/// 441TB in bytes.
/// This constant represents 441 tb converted to bytes for use in size calculations.
pub const TB_441: usize = 484884627849216;

/// 442TB in bytes.
/// This constant represents 442 tb converted to bytes for use in size calculations.
pub const TB_442: usize = 485984139476992;

/// 443TB in bytes.
/// This constant represents 443 tb converted to bytes for use in size calculations.
pub const TB_443: usize = 487083651104768;

/// 444TB in bytes.
/// This constant represents 444 tb converted to bytes for use in size calculations.
pub const TB_444: usize = 488183162732544;

/// 445TB in bytes.
/// This constant represents 445 tb converted to bytes for use in size calculations.
pub const TB_445: usize = 489282674360320;

/// 446TB in bytes.
/// This constant represents 446 tb converted to bytes for use in size calculations.
pub const TB_446: usize = 490382185988096;

/// 447TB in bytes.
/// This constant represents 447 tb converted to bytes for use in size calculations.
pub const TB_447: usize = 491481697615872;

/// 448TB in bytes.
/// This constant represents 448 tb converted to bytes for use in size calculations.
pub const TB_448: usize = 492581209243648;

/// 449TB in bytes.
/// This constant represents 449 tb converted to bytes for use in size calculations.
pub const TB_449: usize = 493680720871424;

/// 450TB in bytes.
/// This constant represents 450 tb converted to bytes for use in size calculations.
pub const TB_450: usize = 494780232499200;

/// 451TB in bytes.
/// This constant represents 451 tb converted to bytes for use in size calculations.
pub const TB_451: usize = 495879744126976;

/// 452TB in bytes.
/// This constant represents 452 tb converted to bytes for use in size calculations.
pub const TB_452: usize = 496979255754752;

/// 453TB in bytes.
/// This constant represents 453 tb converted to bytes for use in size calculations.
pub const TB_453: usize = 498078767382528;

/// 454TB in bytes.
/// This constant represents 454 tb converted to bytes for use in size calculations.
pub const TB_454: usize = 499178279010304;

/// 455TB in bytes.
/// This constant represents 455 tb converted to bytes for use in size calculations.
pub const TB_455: usize = 500277790638080;

/// 456TB in bytes.
/// This constant represents 456 tb converted to bytes for use in size calculations.
pub const TB_456: usize = 501377302265856;

/// 457TB in bytes.
/// This constant represents 457 tb converted to bytes for use in size calculations.
pub const TB_457: usize = 502476813893632;

/// 458TB in bytes.
/// This constant represents 458 tb converted to bytes for use in size calculations.
pub const TB_458: usize = 503576325521408;

/// 459TB in bytes.
/// This constant represents 459 tb converted to bytes for use in size calculations.
pub const TB_459: usize = 504675837149184;

/// 460TB in bytes.
/// This constant represents 460 tb converted to bytes for use in size calculations.
pub const TB_460: usize = 505775348776960;

/// 461TB in bytes.
/// This constant represents 461 tb converted to bytes for use in size calculations.
pub const TB_461: usize = 506874860404736;

/// 462TB in bytes.
/// This constant represents 462 tb converted to bytes for use in size calculations.
pub const TB_462: usize = 507974372032512;

/// 463TB in bytes.
/// This constant represents 463 tb converted to bytes for use in size calculations.
pub const TB_463: usize = 509073883660288;

/// 464TB in bytes.
/// This constant represents 464 tb converted to bytes for use in size calculations.
pub const TB_464: usize = 510173395288064;

/// 465TB in bytes.
/// This constant represents 465 tb converted to bytes for use in size calculations.
pub const TB_465: usize = 511272906915840;

/// 466TB in bytes.
/// This constant represents 466 tb converted to bytes for use in size calculations.
pub const TB_466: usize = 512372418543616;

/// 467TB in bytes.
/// This constant represents 467 tb converted to bytes for use in size calculations.
pub const TB_467: usize = 513471930171392;

/// 468TB in bytes.
/// This constant represents 468 tb converted to bytes for use in size calculations.
pub const TB_468: usize = 514571441799168;

/// 469TB in bytes.
/// This constant represents 469 tb converted to bytes for use in size calculations.
pub const TB_469: usize = 515670953426944;

/// 470TB in bytes.
/// This constant represents 470 tb converted to bytes for use in size calculations.
pub const TB_470: usize = 516770465054720;

/// 471TB in bytes.
/// This constant represents 471 tb converted to bytes for use in size calculations.
pub const TB_471: usize = 517869976682496;

/// 472TB in bytes.
/// This constant represents 472 tb converted to bytes for use in size calculations.
pub const TB_472: usize = 518969488310272;

/// 473TB in bytes.
/// This constant represents 473 tb converted to bytes for use in size calculations.
pub const TB_473: usize = 520068999938048;

/// 474TB in bytes.
/// This constant represents 474 tb converted to bytes for use in size calculations.
pub const TB_474: usize = 521168511565824;

/// 475TB in bytes.
/// This constant represents 475 tb converted to bytes for use in size calculations.
pub const TB_475: usize = 522268023193600;

/// 476TB in bytes.
/// This constant represents 476 tb converted to bytes for use in size calculations.
pub const TB_476: usize = 523367534821376;

/// 477TB in bytes.
/// This constant represents 477 tb converted to bytes for use in size calculations.
pub const TB_477: usize = 524467046449152;

/// 478TB in bytes.
/// This constant represents 478 tb converted to bytes for use in size calculations.
pub const TB_478: usize = 525566558076928;

/// 479TB in bytes.
/// This constant represents 479 tb converted to bytes for use in size calculations.
pub const TB_479: usize = 526666069704704;

/// 480TB in bytes.
/// This constant represents 480 tb converted to bytes for use in size calculations.
pub const TB_480: usize = 527765581332480;

/// 481TB in bytes.
/// This constant represents 481 tb converted to bytes for use in size calculations.
pub const TB_481: usize = 528865092960256;

/// 482TB in bytes.
/// This constant represents 482 tb converted to bytes for use in size calculations.
pub const TB_482: usize = 529964604588032;

/// 483TB in bytes.
/// This constant represents 483 tb converted to bytes for use in size calculations.
pub const TB_483: usize = 531064116215808;

/// 484TB in bytes.
/// This constant represents 484 tb converted to bytes for use in size calculations.
pub const TB_484: usize = 532163627843584;

/// 485TB in bytes.
/// This constant represents 485 tb converted to bytes for use in size calculations.
pub const TB_485: usize = 533263139471360;

/// 486TB in bytes.
/// This constant represents 486 tb converted to bytes for use in size calculations.
pub const TB_486: usize = 534362651099136;

/// 487TB in bytes.
/// This constant represents 487 tb converted to bytes for use in size calculations.
pub const TB_487: usize = 535462162726912;

/// 488TB in bytes.
/// This constant represents 488 tb converted to bytes for use in size calculations.
pub const TB_488: usize = 536561674354688;

/// 489TB in bytes.
/// This constant represents 489 tb converted to bytes for use in size calculations.
pub const TB_489: usize = 537661185982464;

/// 490TB in bytes.
/// This constant represents 490 tb converted to bytes for use in size calculations.
pub const TB_490: usize = 538760697610240;

/// 491TB in bytes.
/// This constant represents 491 tb converted to bytes for use in size calculations.
pub const TB_491: usize = 539860209238016;

/// 492TB in bytes.
/// This constant represents 492 tb converted to bytes for use in size calculations.
pub const TB_492: usize = 540959720865792;

/// 493TB in bytes.
/// This constant represents 493 tb converted to bytes for use in size calculations.
pub const TB_493: usize = 542059232493568;

/// 494TB in bytes.
/// This constant represents 494 tb converted to bytes for use in size calculations.
pub const TB_494: usize = 543158744121344;

/// 495TB in bytes.
/// This constant represents 495 tb converted to bytes for use in size calculations.
pub const TB_495: usize = 544258255749120;

/// 496TB in bytes.
/// This constant represents 496 tb converted to bytes for use in size calculations.
pub const TB_496: usize = 545357767376896;

/// 497TB in bytes.
/// This constant represents 497 tb converted to bytes for use in size calculations.
pub const TB_497: usize = 546457279004672;

/// 498TB in bytes.
/// This constant represents 498 tb converted to bytes for use in size calculations.
pub const TB_498: usize = 547556790632448;

/// 499TB in bytes.
/// This constant represents 499 tb converted to bytes for use in size calculations.
pub const TB_499: usize = 548656302260224;

/// 500TB in bytes.
/// This constant represents 500 tb converted to bytes for use in size calculations.
pub const TB_500: usize = 549755813888000;

/// 501TB in bytes.
/// This constant represents 501 tb converted to bytes for use in size calculations.
pub const TB_501: usize = 550855325515776;

/// 502TB in bytes.
/// This constant represents 502 tb converted to bytes for use in size calculations.
pub const TB_502: usize = 551954837143552;

/// 503TB in bytes.
/// This constant represents 503 tb converted to bytes for use in size calculations.
pub const TB_503: usize = 553054348771328;

/// 504TB in bytes.
/// This constant represents 504 tb converted to bytes for use in size calculations.
pub const TB_504: usize = 554153860399104;

/// 505TB in bytes.
/// This constant represents 505 tb converted to bytes for use in size calculations.
pub const TB_505: usize = 555253372026880;

/// 506TB in bytes.
/// This constant represents 506 tb converted to bytes for use in size calculations.
pub const TB_506: usize = 556352883654656;

/// 507TB in bytes.
/// This constant represents 507 tb converted to bytes for use in size calculations.
pub const TB_507: usize = 557452395282432;

/// 508TB in bytes.
/// This constant represents 508 tb converted to bytes for use in size calculations.
pub const TB_508: usize = 558551906910208;

/// 509TB in bytes.
/// This constant represents 509 tb converted to bytes for use in size calculations.
pub const TB_509: usize = 559651418537984;

/// 510TB in bytes.
/// This constant represents 510 tb converted to bytes for use in size calculations.
pub const TB_510: usize = 560750930165760;

/// 511TB in bytes.
/// This constant represents 511 tb converted to bytes for use in size calculations.
pub const TB_511: usize = 561850441793536;

/// 512TB in bytes.
/// This constant represents 512 tb converted to bytes for use in size calculations.
pub const TB_512: usize = 562949953421312;

/// 513TB in bytes.
/// This constant represents 513 tb converted to bytes for use in size calculations.
pub const TB_513: usize = 564049465049088;

/// 514TB in bytes.
/// This constant represents 514 tb converted to bytes for use in size calculations.
pub const TB_514: usize = 565148976676864;

/// 515TB in bytes.
/// This constant represents 515 tb converted to bytes for use in size calculations.
pub const TB_515: usize = 566248488304640;

/// 516TB in bytes.
/// This constant represents 516 tb converted to bytes for use in size calculations.
pub const TB_516: usize = 567347999932416;

/// 517TB in bytes.
/// This constant represents 517 tb converted to bytes for use in size calculations.
pub const TB_517: usize = 568447511560192;

/// 518TB in bytes.
/// This constant represents 518 tb converted to bytes for use in size calculations.
pub const TB_518: usize = 569547023187968;

/// 519TB in bytes.
/// This constant represents 519 tb converted to bytes for use in size calculations.
pub const TB_519: usize = 570646534815744;

/// 520TB in bytes.
/// This constant represents 520 tb converted to bytes for use in size calculations.
pub const TB_520: usize = 571746046443520;

/// 521TB in bytes.
/// This constant represents 521 tb converted to bytes for use in size calculations.
pub const TB_521: usize = 572845558071296;

/// 522TB in bytes.
/// This constant represents 522 tb converted to bytes for use in size calculations.
pub const TB_522: usize = 573945069699072;

/// 523TB in bytes.
/// This constant represents 523 tb converted to bytes for use in size calculations.
pub const TB_523: usize = 575044581326848;

/// 524TB in bytes.
/// This constant represents 524 tb converted to bytes for use in size calculations.
pub const TB_524: usize = 576144092954624;

/// 525TB in bytes.
/// This constant represents 525 tb converted to bytes for use in size calculations.
pub const TB_525: usize = 577243604582400;

/// 526TB in bytes.
/// This constant represents 526 tb converted to bytes for use in size calculations.
pub const TB_526: usize = 578343116210176;

/// 527TB in bytes.
/// This constant represents 527 tb converted to bytes for use in size calculations.
pub const TB_527: usize = 579442627837952;

/// 528TB in bytes.
/// This constant represents 528 tb converted to bytes for use in size calculations.
pub const TB_528: usize = 580542139465728;

/// 529TB in bytes.
/// This constant represents 529 tb converted to bytes for use in size calculations.
pub const TB_529: usize = 581641651093504;

/// 530TB in bytes.
/// This constant represents 530 tb converted to bytes for use in size calculations.
pub const TB_530: usize = 582741162721280;

/// 531TB in bytes.
/// This constant represents 531 tb converted to bytes for use in size calculations.
pub const TB_531: usize = 583840674349056;

/// 532TB in bytes.
/// This constant represents 532 tb converted to bytes for use in size calculations.
pub const TB_532: usize = 584940185976832;

/// 533TB in bytes.
/// This constant represents 533 tb converted to bytes for use in size calculations.
pub const TB_533: usize = 586039697604608;

/// 534TB in bytes.
/// This constant represents 534 tb converted to bytes for use in size calculations.
pub const TB_534: usize = 587139209232384;

/// 535TB in bytes.
/// This constant represents 535 tb converted to bytes for use in size calculations.
pub const TB_535: usize = 588238720860160;

/// 536TB in bytes.
/// This constant represents 536 tb converted to bytes for use in size calculations.
pub const TB_536: usize = 589338232487936;

/// 537TB in bytes.
/// This constant represents 537 tb converted to bytes for use in size calculations.
pub const TB_537: usize = 590437744115712;

/// 538TB in bytes.
/// This constant represents 538 tb converted to bytes for use in size calculations.
pub const TB_538: usize = 591537255743488;

/// 539TB in bytes.
/// This constant represents 539 tb converted to bytes for use in size calculations.
pub const TB_539: usize = 592636767371264;

/// 540TB in bytes.
/// This constant represents 540 tb converted to bytes for use in size calculations.
pub const TB_540: usize = 593736278999040;

/// 541TB in bytes.
/// This constant represents 541 tb converted to bytes for use in size calculations.
pub const TB_541: usize = 594835790626816;

/// 542TB in bytes.
/// This constant represents 542 tb converted to bytes for use in size calculations.
pub const TB_542: usize = 595935302254592;

/// 543TB in bytes.
/// This constant represents 543 tb converted to bytes for use in size calculations.
pub const TB_543: usize = 597034813882368;

/// 544TB in bytes.
/// This constant represents 544 tb converted to bytes for use in size calculations.
pub const TB_544: usize = 598134325510144;

/// 545TB in bytes.
/// This constant represents 545 tb converted to bytes for use in size calculations.
pub const TB_545: usize = 599233837137920;

/// 546TB in bytes.
/// This constant represents 546 tb converted to bytes for use in size calculations.
pub const TB_546: usize = 600333348765696;

/// 547TB in bytes.
/// This constant represents 547 tb converted to bytes for use in size calculations.
pub const TB_547: usize = 601432860393472;

/// 548TB in bytes.
/// This constant represents 548 tb converted to bytes for use in size calculations.
pub const TB_548: usize = 602532372021248;

/// 549TB in bytes.
/// This constant represents 549 tb converted to bytes for use in size calculations.
pub const TB_549: usize = 603631883649024;

/// 550TB in bytes.
/// This constant represents 550 tb converted to bytes for use in size calculations.
pub const TB_550: usize = 604731395276800;

/// 551TB in bytes.
/// This constant represents 551 tb converted to bytes for use in size calculations.
pub const TB_551: usize = 605830906904576;

/// 552TB in bytes.
/// This constant represents 552 tb converted to bytes for use in size calculations.
pub const TB_552: usize = 606930418532352;

/// 553TB in bytes.
/// This constant represents 553 tb converted to bytes for use in size calculations.
pub const TB_553: usize = 608029930160128;

/// 554TB in bytes.
/// This constant represents 554 tb converted to bytes for use in size calculations.
pub const TB_554: usize = 609129441787904;

/// 555TB in bytes.
/// This constant represents 555 tb converted to bytes for use in size calculations.
pub const TB_555: usize = 610228953415680;

/// 556TB in bytes.
/// This constant represents 556 tb converted to bytes for use in size calculations.
pub const TB_556: usize = 611328465043456;

/// 557TB in bytes.
/// This constant represents 557 tb converted to bytes for use in size calculations.
pub const TB_557: usize = 612427976671232;

/// 558TB in bytes.
/// This constant represents 558 tb converted to bytes for use in size calculations.
pub const TB_558: usize = 613527488299008;

/// 559TB in bytes.
/// This constant represents 559 tb converted to bytes for use in size calculations.
pub const TB_559: usize = 614626999926784;

/// 560TB in bytes.
/// This constant represents 560 tb converted to bytes for use in size calculations.
pub const TB_560: usize = 615726511554560;

/// 561TB in bytes.
/// This constant represents 561 tb converted to bytes for use in size calculations.
pub const TB_561: usize = 616826023182336;

/// 562TB in bytes.
/// This constant represents 562 tb converted to bytes for use in size calculations.
pub const TB_562: usize = 617925534810112;

/// 563TB in bytes.
/// This constant represents 563 tb converted to bytes for use in size calculations.
pub const TB_563: usize = 619025046437888;

/// 564TB in bytes.
/// This constant represents 564 tb converted to bytes for use in size calculations.
pub const TB_564: usize = 620124558065664;

/// 565TB in bytes.
/// This constant represents 565 tb converted to bytes for use in size calculations.
pub const TB_565: usize = 621224069693440;

/// 566TB in bytes.
/// This constant represents 566 tb converted to bytes for use in size calculations.
pub const TB_566: usize = 622323581321216;

/// 567TB in bytes.
/// This constant represents 567 tb converted to bytes for use in size calculations.
pub const TB_567: usize = 623423092948992;

/// 568TB in bytes.
/// This constant represents 568 tb converted to bytes for use in size calculations.
pub const TB_568: usize = 624522604576768;

/// 569TB in bytes.
/// This constant represents 569 tb converted to bytes for use in size calculations.
pub const TB_569: usize = 625622116204544;

/// 570TB in bytes.
/// This constant represents 570 tb converted to bytes for use in size calculations.
pub const TB_570: usize = 626721627832320;

/// 571TB in bytes.
/// This constant represents 571 tb converted to bytes for use in size calculations.
pub const TB_571: usize = 627821139460096;

/// 572TB in bytes.
/// This constant represents 572 tb converted to bytes for use in size calculations.
pub const TB_572: usize = 628920651087872;

/// 573TB in bytes.
/// This constant represents 573 tb converted to bytes for use in size calculations.
pub const TB_573: usize = 630020162715648;

/// 574TB in bytes.
/// This constant represents 574 tb converted to bytes for use in size calculations.
pub const TB_574: usize = 631119674343424;

/// 575TB in bytes.
/// This constant represents 575 tb converted to bytes for use in size calculations.
pub const TB_575: usize = 632219185971200;

/// 576TB in bytes.
/// This constant represents 576 tb converted to bytes for use in size calculations.
pub const TB_576: usize = 633318697598976;

/// 577TB in bytes.
/// This constant represents 577 tb converted to bytes for use in size calculations.
pub const TB_577: usize = 634418209226752;

/// 578TB in bytes.
/// This constant represents 578 tb converted to bytes for use in size calculations.
pub const TB_578: usize = 635517720854528;

/// 579TB in bytes.
/// This constant represents 579 tb converted to bytes for use in size calculations.
pub const TB_579: usize = 636617232482304;

/// 580TB in bytes.
/// This constant represents 580 tb converted to bytes for use in size calculations.
pub const TB_580: usize = 637716744110080;

/// 581TB in bytes.
/// This constant represents 581 tb converted to bytes for use in size calculations.
pub const TB_581: usize = 638816255737856;

/// 582TB in bytes.
/// This constant represents 582 tb converted to bytes for use in size calculations.
pub const TB_582: usize = 639915767365632;

/// 583TB in bytes.
/// This constant represents 583 tb converted to bytes for use in size calculations.
pub const TB_583: usize = 641015278993408;

/// 584TB in bytes.
/// This constant represents 584 tb converted to bytes for use in size calculations.
pub const TB_584: usize = 642114790621184;

/// 585TB in bytes.
/// This constant represents 585 tb converted to bytes for use in size calculations.
pub const TB_585: usize = 643214302248960;

/// 586TB in bytes.
/// This constant represents 586 tb converted to bytes for use in size calculations.
pub const TB_586: usize = 644313813876736;

/// 587TB in bytes.
/// This constant represents 587 tb converted to bytes for use in size calculations.
pub const TB_587: usize = 645413325504512;

/// 588TB in bytes.
/// This constant represents 588 tb converted to bytes for use in size calculations.
pub const TB_588: usize = 646512837132288;

/// 589TB in bytes.
/// This constant represents 589 tb converted to bytes for use in size calculations.
pub const TB_589: usize = 647612348760064;

/// 590TB in bytes.
/// This constant represents 590 tb converted to bytes for use in size calculations.
pub const TB_590: usize = 648711860387840;

/// 591TB in bytes.
/// This constant represents 591 tb converted to bytes for use in size calculations.
pub const TB_591: usize = 649811372015616;

/// 592TB in bytes.
/// This constant represents 592 tb converted to bytes for use in size calculations.
pub const TB_592: usize = 650910883643392;

/// 593TB in bytes.
/// This constant represents 593 tb converted to bytes for use in size calculations.
pub const TB_593: usize = 652010395271168;

/// 594TB in bytes.
/// This constant represents 594 tb converted to bytes for use in size calculations.
pub const TB_594: usize = 653109906898944;

/// 595TB in bytes.
/// This constant represents 595 tb converted to bytes for use in size calculations.
pub const TB_595: usize = 654209418526720;

/// 596TB in bytes.
/// This constant represents 596 tb converted to bytes for use in size calculations.
pub const TB_596: usize = 655308930154496;

/// 597TB in bytes.
/// This constant represents 597 tb converted to bytes for use in size calculations.
pub const TB_597: usize = 656408441782272;

/// 598TB in bytes.
/// This constant represents 598 tb converted to bytes for use in size calculations.
pub const TB_598: usize = 657507953410048;

/// 599TB in bytes.
/// This constant represents 599 tb converted to bytes for use in size calculations.
pub const TB_599: usize = 658607465037824;

/// 600TB in bytes.
/// This constant represents 600 tb converted to bytes for use in size calculations.
pub const TB_600: usize = 659706976665600;

/// 601TB in bytes.
/// This constant represents 601 tb converted to bytes for use in size calculations.
pub const TB_601: usize = 660806488293376;

/// 602TB in bytes.
/// This constant represents 602 tb converted to bytes for use in size calculations.
pub const TB_602: usize = 661905999921152;

/// 603TB in bytes.
/// This constant represents 603 tb converted to bytes for use in size calculations.
pub const TB_603: usize = 663005511548928;

/// 604TB in bytes.
/// This constant represents 604 tb converted to bytes for use in size calculations.
pub const TB_604: usize = 664105023176704;

/// 605TB in bytes.
/// This constant represents 605 tb converted to bytes for use in size calculations.
pub const TB_605: usize = 665204534804480;

/// 606TB in bytes.
/// This constant represents 606 tb converted to bytes for use in size calculations.
pub const TB_606: usize = 666304046432256;

/// 607TB in bytes.
/// This constant represents 607 tb converted to bytes for use in size calculations.
pub const TB_607: usize = 667403558060032;

/// 608TB in bytes.
/// This constant represents 608 tb converted to bytes for use in size calculations.
pub const TB_608: usize = 668503069687808;

/// 609TB in bytes.
/// This constant represents 609 tb converted to bytes for use in size calculations.
pub const TB_609: usize = 669602581315584;

/// 610TB in bytes.
/// This constant represents 610 tb converted to bytes for use in size calculations.
pub const TB_610: usize = 670702092943360;

/// 611TB in bytes.
/// This constant represents 611 tb converted to bytes for use in size calculations.
pub const TB_611: usize = 671801604571136;

/// 612TB in bytes.
/// This constant represents 612 tb converted to bytes for use in size calculations.
pub const TB_612: usize = 672901116198912;

/// 613TB in bytes.
/// This constant represents 613 tb converted to bytes for use in size calculations.
pub const TB_613: usize = 674000627826688;

/// 614TB in bytes.
/// This constant represents 614 tb converted to bytes for use in size calculations.
pub const TB_614: usize = 675100139454464;

/// 615TB in bytes.
/// This constant represents 615 tb converted to bytes for use in size calculations.
pub const TB_615: usize = 676199651082240;

/// 616TB in bytes.
/// This constant represents 616 tb converted to bytes for use in size calculations.
pub const TB_616: usize = 677299162710016;

/// 617TB in bytes.
/// This constant represents 617 tb converted to bytes for use in size calculations.
pub const TB_617: usize = 678398674337792;

/// 618TB in bytes.
/// This constant represents 618 tb converted to bytes for use in size calculations.
pub const TB_618: usize = 679498185965568;

/// 619TB in bytes.
/// This constant represents 619 tb converted to bytes for use in size calculations.
pub const TB_619: usize = 680597697593344;

/// 620TB in bytes.
/// This constant represents 620 tb converted to bytes for use in size calculations.
pub const TB_620: usize = 681697209221120;

/// 621TB in bytes.
/// This constant represents 621 tb converted to bytes for use in size calculations.
pub const TB_621: usize = 682796720848896;

/// 622TB in bytes.
/// This constant represents 622 tb converted to bytes for use in size calculations.
pub const TB_622: usize = 683896232476672;

/// 623TB in bytes.
/// This constant represents 623 tb converted to bytes for use in size calculations.
pub const TB_623: usize = 684995744104448;

/// 624TB in bytes.
/// This constant represents 624 tb converted to bytes for use in size calculations.
pub const TB_624: usize = 686095255732224;

/// 625TB in bytes.
/// This constant represents 625 tb converted to bytes for use in size calculations.
pub const TB_625: usize = 687194767360000;

/// 626TB in bytes.
/// This constant represents 626 tb converted to bytes for use in size calculations.
pub const TB_626: usize = 688294278987776;

/// 627TB in bytes.
/// This constant represents 627 tb converted to bytes for use in size calculations.
pub const TB_627: usize = 689393790615552;

/// 628TB in bytes.
/// This constant represents 628 tb converted to bytes for use in size calculations.
pub const TB_628: usize = 690493302243328;

/// 629TB in bytes.
/// This constant represents 629 tb converted to bytes for use in size calculations.
pub const TB_629: usize = 691592813871104;

/// 630TB in bytes.
/// This constant represents 630 tb converted to bytes for use in size calculations.
pub const TB_630: usize = 692692325498880;

/// 631TB in bytes.
/// This constant represents 631 tb converted to bytes for use in size calculations.
pub const TB_631: usize = 693791837126656;

/// 632TB in bytes.
/// This constant represents 632 tb converted to bytes for use in size calculations.
pub const TB_632: usize = 694891348754432;

/// 633TB in bytes.
/// This constant represents 633 tb converted to bytes for use in size calculations.
pub const TB_633: usize = 695990860382208;

/// 634TB in bytes.
/// This constant represents 634 tb converted to bytes for use in size calculations.
pub const TB_634: usize = 697090372009984;

/// 635TB in bytes.
/// This constant represents 635 tb converted to bytes for use in size calculations.
pub const TB_635: usize = 698189883637760;

/// 636TB in bytes.
/// This constant represents 636 tb converted to bytes for use in size calculations.
pub const TB_636: usize = 699289395265536;

/// 637TB in bytes.
/// This constant represents 637 tb converted to bytes for use in size calculations.
pub const TB_637: usize = 700388906893312;

/// 638TB in bytes.
/// This constant represents 638 tb converted to bytes for use in size calculations.
pub const TB_638: usize = 701488418521088;

/// 639TB in bytes.
/// This constant represents 639 tb converted to bytes for use in size calculations.
pub const TB_639: usize = 702587930148864;

/// 640TB in bytes.
/// This constant represents 640 tb converted to bytes for use in size calculations.
pub const TB_640: usize = 703687441776640;

/// 641TB in bytes.
/// This constant represents 641 tb converted to bytes for use in size calculations.
pub const TB_641: usize = 704786953404416;

/// 642TB in bytes.
/// This constant represents 642 tb converted to bytes for use in size calculations.
pub const TB_642: usize = 705886465032192;

/// 643TB in bytes.
/// This constant represents 643 tb converted to bytes for use in size calculations.
pub const TB_643: usize = 706985976659968;

/// 644TB in bytes.
/// This constant represents 644 tb converted to bytes for use in size calculations.
pub const TB_644: usize = 708085488287744;

/// 645TB in bytes.
/// This constant represents 645 tb converted to bytes for use in size calculations.
pub const TB_645: usize = 709184999915520;

/// 646TB in bytes.
/// This constant represents 646 tb converted to bytes for use in size calculations.
pub const TB_646: usize = 710284511543296;

/// 647TB in bytes.
/// This constant represents 647 tb converted to bytes for use in size calculations.
pub const TB_647: usize = 711384023171072;

/// 648TB in bytes.
/// This constant represents 648 tb converted to bytes for use in size calculations.
pub const TB_648: usize = 712483534798848;

/// 649TB in bytes.
/// This constant represents 649 tb converted to bytes for use in size calculations.
pub const TB_649: usize = 713583046426624;

/// 650TB in bytes.
/// This constant represents 650 tb converted to bytes for use in size calculations.
pub const TB_650: usize = 714682558054400;

/// 651TB in bytes.
/// This constant represents 651 tb converted to bytes for use in size calculations.
pub const TB_651: usize = 715782069682176;

/// 652TB in bytes.
/// This constant represents 652 tb converted to bytes for use in size calculations.
pub const TB_652: usize = 716881581309952;

/// 653TB in bytes.
/// This constant represents 653 tb converted to bytes for use in size calculations.
pub const TB_653: usize = 717981092937728;

/// 654TB in bytes.
/// This constant represents 654 tb converted to bytes for use in size calculations.
pub const TB_654: usize = 719080604565504;

/// 655TB in bytes.
/// This constant represents 655 tb converted to bytes for use in size calculations.
pub const TB_655: usize = 720180116193280;

/// 656TB in bytes.
/// This constant represents 656 tb converted to bytes for use in size calculations.
pub const TB_656: usize = 721279627821056;

/// 657TB in bytes.
/// This constant represents 657 tb converted to bytes for use in size calculations.
pub const TB_657: usize = 722379139448832;

/// 658TB in bytes.
/// This constant represents 658 tb converted to bytes for use in size calculations.
pub const TB_658: usize = 723478651076608;

/// 659TB in bytes.
/// This constant represents 659 tb converted to bytes for use in size calculations.
pub const TB_659: usize = 724578162704384;

/// 660TB in bytes.
/// This constant represents 660 tb converted to bytes for use in size calculations.
pub const TB_660: usize = 725677674332160;

/// 661TB in bytes.
/// This constant represents 661 tb converted to bytes for use in size calculations.
pub const TB_661: usize = 726777185959936;

/// 662TB in bytes.
/// This constant represents 662 tb converted to bytes for use in size calculations.
pub const TB_662: usize = 727876697587712;

/// 663TB in bytes.
/// This constant represents 663 tb converted to bytes for use in size calculations.
pub const TB_663: usize = 728976209215488;

/// 664TB in bytes.
/// This constant represents 664 tb converted to bytes for use in size calculations.
pub const TB_664: usize = 730075720843264;

/// 665TB in bytes.
/// This constant represents 665 tb converted to bytes for use in size calculations.
pub const TB_665: usize = 731175232471040;

/// 666TB in bytes.
/// This constant represents 666 tb converted to bytes for use in size calculations.
pub const TB_666: usize = 732274744098816;

/// 667TB in bytes.
/// This constant represents 667 tb converted to bytes for use in size calculations.
pub const TB_667: usize = 733374255726592;

/// 668TB in bytes.
/// This constant represents 668 tb converted to bytes for use in size calculations.
pub const TB_668: usize = 734473767354368;

/// 669TB in bytes.
/// This constant represents 669 tb converted to bytes for use in size calculations.
pub const TB_669: usize = 735573278982144;

/// 670TB in bytes.
/// This constant represents 670 tb converted to bytes for use in size calculations.
pub const TB_670: usize = 736672790609920;

/// 671TB in bytes.
/// This constant represents 671 tb converted to bytes for use in size calculations.
pub const TB_671: usize = 737772302237696;

/// 672TB in bytes.
/// This constant represents 672 tb converted to bytes for use in size calculations.
pub const TB_672: usize = 738871813865472;

/// 673TB in bytes.
/// This constant represents 673 tb converted to bytes for use in size calculations.
pub const TB_673: usize = 739971325493248;

/// 674TB in bytes.
/// This constant represents 674 tb converted to bytes for use in size calculations.
pub const TB_674: usize = 741070837121024;

/// 675TB in bytes.
/// This constant represents 675 tb converted to bytes for use in size calculations.
pub const TB_675: usize = 742170348748800;

/// 676TB in bytes.
/// This constant represents 676 tb converted to bytes for use in size calculations.
pub const TB_676: usize = 743269860376576;

/// 677TB in bytes.
/// This constant represents 677 tb converted to bytes for use in size calculations.
pub const TB_677: usize = 744369372004352;

/// 678TB in bytes.
/// This constant represents 678 tb converted to bytes for use in size calculations.
pub const TB_678: usize = 745468883632128;

/// 679TB in bytes.
/// This constant represents 679 tb converted to bytes for use in size calculations.
pub const TB_679: usize = 746568395259904;

/// 680TB in bytes.
/// This constant represents 680 tb converted to bytes for use in size calculations.
pub const TB_680: usize = 747667906887680;

/// 681TB in bytes.
/// This constant represents 681 tb converted to bytes for use in size calculations.
pub const TB_681: usize = 748767418515456;

/// 682TB in bytes.
/// This constant represents 682 tb converted to bytes for use in size calculations.
pub const TB_682: usize = 749866930143232;

/// 683TB in bytes.
/// This constant represents 683 tb converted to bytes for use in size calculations.
pub const TB_683: usize = 750966441771008;

/// 684TB in bytes.
/// This constant represents 684 tb converted to bytes for use in size calculations.
pub const TB_684: usize = 752065953398784;

/// 685TB in bytes.
/// This constant represents 685 tb converted to bytes for use in size calculations.
pub const TB_685: usize = 753165465026560;

/// 686TB in bytes.
/// This constant represents 686 tb converted to bytes for use in size calculations.
pub const TB_686: usize = 754264976654336;

/// 687TB in bytes.
/// This constant represents 687 tb converted to bytes for use in size calculations.
pub const TB_687: usize = 755364488282112;

/// 688TB in bytes.
/// This constant represents 688 tb converted to bytes for use in size calculations.
pub const TB_688: usize = 756463999909888;

/// 689TB in bytes.
/// This constant represents 689 tb converted to bytes for use in size calculations.
pub const TB_689: usize = 757563511537664;

/// 690TB in bytes.
/// This constant represents 690 tb converted to bytes for use in size calculations.
pub const TB_690: usize = 758663023165440;

/// 691TB in bytes.
/// This constant represents 691 tb converted to bytes for use in size calculations.
pub const TB_691: usize = 759762534793216;

/// 692TB in bytes.
/// This constant represents 692 tb converted to bytes for use in size calculations.
pub const TB_692: usize = 760862046420992;

/// 693TB in bytes.
/// This constant represents 693 tb converted to bytes for use in size calculations.
pub const TB_693: usize = 761961558048768;

/// 694TB in bytes.
/// This constant represents 694 tb converted to bytes for use in size calculations.
pub const TB_694: usize = 763061069676544;

/// 695TB in bytes.
/// This constant represents 695 tb converted to bytes for use in size calculations.
pub const TB_695: usize = 764160581304320;

/// 696TB in bytes.
/// This constant represents 696 tb converted to bytes for use in size calculations.
pub const TB_696: usize = 765260092932096;

/// 697TB in bytes.
/// This constant represents 697 tb converted to bytes for use in size calculations.
pub const TB_697: usize = 766359604559872;

/// 698TB in bytes.
/// This constant represents 698 tb converted to bytes for use in size calculations.
pub const TB_698: usize = 767459116187648;

/// 699TB in bytes.
/// This constant represents 699 tb converted to bytes for use in size calculations.
pub const TB_699: usize = 768558627815424;

/// 700TB in bytes.
/// This constant represents 700 tb converted to bytes for use in size calculations.
pub const TB_700: usize = 769658139443200;

/// 701TB in bytes.
/// This constant represents 701 tb converted to bytes for use in size calculations.
pub const TB_701: usize = 770757651070976;

/// 702TB in bytes.
/// This constant represents 702 tb converted to bytes for use in size calculations.
pub const TB_702: usize = 771857162698752;

/// 703TB in bytes.
/// This constant represents 703 tb converted to bytes for use in size calculations.
pub const TB_703: usize = 772956674326528;

/// 704TB in bytes.
/// This constant represents 704 tb converted to bytes for use in size calculations.
pub const TB_704: usize = 774056185954304;

/// 705TB in bytes.
/// This constant represents 705 tb converted to bytes for use in size calculations.
pub const TB_705: usize = 775155697582080;

/// 706TB in bytes.
/// This constant represents 706 tb converted to bytes for use in size calculations.
pub const TB_706: usize = 776255209209856;

/// 707TB in bytes.
/// This constant represents 707 tb converted to bytes for use in size calculations.
pub const TB_707: usize = 777354720837632;

/// 708TB in bytes.
/// This constant represents 708 tb converted to bytes for use in size calculations.
pub const TB_708: usize = 778454232465408;

/// 709TB in bytes.
/// This constant represents 709 tb converted to bytes for use in size calculations.
pub const TB_709: usize = 779553744093184;

/// 710TB in bytes.
/// This constant represents 710 tb converted to bytes for use in size calculations.
pub const TB_710: usize = 780653255720960;

/// 711TB in bytes.
/// This constant represents 711 tb converted to bytes for use in size calculations.
pub const TB_711: usize = 781752767348736;

/// 712TB in bytes.
/// This constant represents 712 tb converted to bytes for use in size calculations.
pub const TB_712: usize = 782852278976512;

/// 713TB in bytes.
/// This constant represents 713 tb converted to bytes for use in size calculations.
pub const TB_713: usize = 783951790604288;

/// 714TB in bytes.
/// This constant represents 714 tb converted to bytes for use in size calculations.
pub const TB_714: usize = 785051302232064;

/// 715TB in bytes.
/// This constant represents 715 tb converted to bytes for use in size calculations.
pub const TB_715: usize = 786150813859840;

/// 716TB in bytes.
/// This constant represents 716 tb converted to bytes for use in size calculations.
pub const TB_716: usize = 787250325487616;

/// 717TB in bytes.
/// This constant represents 717 tb converted to bytes for use in size calculations.
pub const TB_717: usize = 788349837115392;

/// 718TB in bytes.
/// This constant represents 718 tb converted to bytes for use in size calculations.
pub const TB_718: usize = 789449348743168;

/// 719TB in bytes.
/// This constant represents 719 tb converted to bytes for use in size calculations.
pub const TB_719: usize = 790548860370944;

/// 720TB in bytes.
/// This constant represents 720 tb converted to bytes for use in size calculations.
pub const TB_720: usize = 791648371998720;

/// 721TB in bytes.
/// This constant represents 721 tb converted to bytes for use in size calculations.
pub const TB_721: usize = 792747883626496;

/// 722TB in bytes.
/// This constant represents 722 tb converted to bytes for use in size calculations.
pub const TB_722: usize = 793847395254272;

/// 723TB in bytes.
/// This constant represents 723 tb converted to bytes for use in size calculations.
pub const TB_723: usize = 794946906882048;

/// 724TB in bytes.
/// This constant represents 724 tb converted to bytes for use in size calculations.
pub const TB_724: usize = 796046418509824;

/// 725TB in bytes.
/// This constant represents 725 tb converted to bytes for use in size calculations.
pub const TB_725: usize = 797145930137600;

/// 726TB in bytes.
/// This constant represents 726 tb converted to bytes for use in size calculations.
pub const TB_726: usize = 798245441765376;

/// 727TB in bytes.
/// This constant represents 727 tb converted to bytes for use in size calculations.
pub const TB_727: usize = 799344953393152;

/// 728TB in bytes.
/// This constant represents 728 tb converted to bytes for use in size calculations.
pub const TB_728: usize = 800444465020928;

/// 729TB in bytes.
/// This constant represents 729 tb converted to bytes for use in size calculations.
pub const TB_729: usize = 801543976648704;

/// 730TB in bytes.
/// This constant represents 730 tb converted to bytes for use in size calculations.
pub const TB_730: usize = 802643488276480;

/// 731TB in bytes.
/// This constant represents 731 tb converted to bytes for use in size calculations.
pub const TB_731: usize = 803742999904256;

/// 732TB in bytes.
/// This constant represents 732 tb converted to bytes for use in size calculations.
pub const TB_732: usize = 804842511532032;

/// 733TB in bytes.
/// This constant represents 733 tb converted to bytes for use in size calculations.
pub const TB_733: usize = 805942023159808;

/// 734TB in bytes.
/// This constant represents 734 tb converted to bytes for use in size calculations.
pub const TB_734: usize = 807041534787584;

/// 735TB in bytes.
/// This constant represents 735 tb converted to bytes for use in size calculations.
pub const TB_735: usize = 808141046415360;

/// 736TB in bytes.
/// This constant represents 736 tb converted to bytes for use in size calculations.
pub const TB_736: usize = 809240558043136;

/// 737TB in bytes.
/// This constant represents 737 tb converted to bytes for use in size calculations.
pub const TB_737: usize = 810340069670912;

/// 738TB in bytes.
/// This constant represents 738 tb converted to bytes for use in size calculations.
pub const TB_738: usize = 811439581298688;

/// 739TB in bytes.
/// This constant represents 739 tb converted to bytes for use in size calculations.
pub const TB_739: usize = 812539092926464;

/// 740TB in bytes.
/// This constant represents 740 tb converted to bytes for use in size calculations.
pub const TB_740: usize = 813638604554240;

/// 741TB in bytes.
/// This constant represents 741 tb converted to bytes for use in size calculations.
pub const TB_741: usize = 814738116182016;

/// 742TB in bytes.
/// This constant represents 742 tb converted to bytes for use in size calculations.
pub const TB_742: usize = 815837627809792;

/// 743TB in bytes.
/// This constant represents 743 tb converted to bytes for use in size calculations.
pub const TB_743: usize = 816937139437568;

/// 744TB in bytes.
/// This constant represents 744 tb converted to bytes for use in size calculations.
pub const TB_744: usize = 818036651065344;

/// 745TB in bytes.
/// This constant represents 745 tb converted to bytes for use in size calculations.
pub const TB_745: usize = 819136162693120;

/// 746TB in bytes.
/// This constant represents 746 tb converted to bytes for use in size calculations.
pub const TB_746: usize = 820235674320896;

/// 747TB in bytes.
/// This constant represents 747 tb converted to bytes for use in size calculations.
pub const TB_747: usize = 821335185948672;

/// 748TB in bytes.
/// This constant represents 748 tb converted to bytes for use in size calculations.
pub const TB_748: usize = 822434697576448;

/// 749TB in bytes.
/// This constant represents 749 tb converted to bytes for use in size calculations.
pub const TB_749: usize = 823534209204224;

/// 750TB in bytes.
/// This constant represents 750 tb converted to bytes for use in size calculations.
pub const TB_750: usize = 824633720832000;

/// 751TB in bytes.
/// This constant represents 751 tb converted to bytes for use in size calculations.
pub const TB_751: usize = 825733232459776;

/// 752TB in bytes.
/// This constant represents 752 tb converted to bytes for use in size calculations.
pub const TB_752: usize = 826832744087552;

/// 753TB in bytes.
/// This constant represents 753 tb converted to bytes for use in size calculations.
pub const TB_753: usize = 827932255715328;

/// 754TB in bytes.
/// This constant represents 754 tb converted to bytes for use in size calculations.
pub const TB_754: usize = 829031767343104;

/// 755TB in bytes.
/// This constant represents 755 tb converted to bytes for use in size calculations.
pub const TB_755: usize = 830131278970880;

/// 756TB in bytes.
/// This constant represents 756 tb converted to bytes for use in size calculations.
pub const TB_756: usize = 831230790598656;

/// 757TB in bytes.
/// This constant represents 757 tb converted to bytes for use in size calculations.
pub const TB_757: usize = 832330302226432;

/// 758TB in bytes.
/// This constant represents 758 tb converted to bytes for use in size calculations.
pub const TB_758: usize = 833429813854208;

/// 759TB in bytes.
/// This constant represents 759 tb converted to bytes for use in size calculations.
pub const TB_759: usize = 834529325481984;

/// 760TB in bytes.
/// This constant represents 760 tb converted to bytes for use in size calculations.
pub const TB_760: usize = 835628837109760;

/// 761TB in bytes.
/// This constant represents 761 tb converted to bytes for use in size calculations.
pub const TB_761: usize = 836728348737536;

/// 762TB in bytes.
/// This constant represents 762 tb converted to bytes for use in size calculations.
pub const TB_762: usize = 837827860365312;

/// 763TB in bytes.
/// This constant represents 763 tb converted to bytes for use in size calculations.
pub const TB_763: usize = 838927371993088;

/// 764TB in bytes.
/// This constant represents 764 tb converted to bytes for use in size calculations.
pub const TB_764: usize = 840026883620864;

/// 765TB in bytes.
/// This constant represents 765 tb converted to bytes for use in size calculations.
pub const TB_765: usize = 841126395248640;

/// 766TB in bytes.
/// This constant represents 766 tb converted to bytes for use in size calculations.
pub const TB_766: usize = 842225906876416;

/// 767TB in bytes.
/// This constant represents 767 tb converted to bytes for use in size calculations.
pub const TB_767: usize = 843325418504192;

/// 768TB in bytes.
/// This constant represents 768 tb converted to bytes for use in size calculations.
pub const TB_768: usize = 844424930131968;

/// 769TB in bytes.
/// This constant represents 769 tb converted to bytes for use in size calculations.
pub const TB_769: usize = 845524441759744;

/// 770TB in bytes.
/// This constant represents 770 tb converted to bytes for use in size calculations.
pub const TB_770: usize = 846623953387520;

/// 771TB in bytes.
/// This constant represents 771 tb converted to bytes for use in size calculations.
pub const TB_771: usize = 847723465015296;

/// 772TB in bytes.
/// This constant represents 772 tb converted to bytes for use in size calculations.
pub const TB_772: usize = 848822976643072;

/// 773TB in bytes.
/// This constant represents 773 tb converted to bytes for use in size calculations.
pub const TB_773: usize = 849922488270848;

/// 774TB in bytes.
/// This constant represents 774 tb converted to bytes for use in size calculations.
pub const TB_774: usize = 851021999898624;

/// 775TB in bytes.
/// This constant represents 775 tb converted to bytes for use in size calculations.
pub const TB_775: usize = 852121511526400;

/// 776TB in bytes.
/// This constant represents 776 tb converted to bytes for use in size calculations.
pub const TB_776: usize = 853221023154176;

/// 777TB in bytes.
/// This constant represents 777 tb converted to bytes for use in size calculations.
pub const TB_777: usize = 854320534781952;

/// 778TB in bytes.
/// This constant represents 778 tb converted to bytes for use in size calculations.
pub const TB_778: usize = 855420046409728;

/// 779TB in bytes.
/// This constant represents 779 tb converted to bytes for use in size calculations.
pub const TB_779: usize = 856519558037504;

/// 780TB in bytes.
/// This constant represents 780 tb converted to bytes for use in size calculations.
pub const TB_780: usize = 857619069665280;

/// 781TB in bytes.
/// This constant represents 781 tb converted to bytes for use in size calculations.
pub const TB_781: usize = 858718581293056;

/// 782TB in bytes.
/// This constant represents 782 tb converted to bytes for use in size calculations.
pub const TB_782: usize = 859818092920832;

/// 783TB in bytes.
/// This constant represents 783 tb converted to bytes for use in size calculations.
pub const TB_783: usize = 860917604548608;

/// 784TB in bytes.
/// This constant represents 784 tb converted to bytes for use in size calculations.
pub const TB_784: usize = 862017116176384;

/// 785TB in bytes.
/// This constant represents 785 tb converted to bytes for use in size calculations.
pub const TB_785: usize = 863116627804160;

/// 786TB in bytes.
/// This constant represents 786 tb converted to bytes for use in size calculations.
pub const TB_786: usize = 864216139431936;

/// 787TB in bytes.
/// This constant represents 787 tb converted to bytes for use in size calculations.
pub const TB_787: usize = 865315651059712;

/// 788TB in bytes.
/// This constant represents 788 tb converted to bytes for use in size calculations.
pub const TB_788: usize = 866415162687488;

/// 789TB in bytes.
/// This constant represents 789 tb converted to bytes for use in size calculations.
pub const TB_789: usize = 867514674315264;

/// 790TB in bytes.
/// This constant represents 790 tb converted to bytes for use in size calculations.
pub const TB_790: usize = 868614185943040;

/// 791TB in bytes.
/// This constant represents 791 tb converted to bytes for use in size calculations.
pub const TB_791: usize = 869713697570816;

/// 792TB in bytes.
/// This constant represents 792 tb converted to bytes for use in size calculations.
pub const TB_792: usize = 870813209198592;

/// 793TB in bytes.
/// This constant represents 793 tb converted to bytes for use in size calculations.
pub const TB_793: usize = 871912720826368;

/// 794TB in bytes.
/// This constant represents 794 tb converted to bytes for use in size calculations.
pub const TB_794: usize = 873012232454144;

/// 795TB in bytes.
/// This constant represents 795 tb converted to bytes for use in size calculations.
pub const TB_795: usize = 874111744081920;

/// 796TB in bytes.
/// This constant represents 796 tb converted to bytes for use in size calculations.
pub const TB_796: usize = 875211255709696;

/// 797TB in bytes.
/// This constant represents 797 tb converted to bytes for use in size calculations.
pub const TB_797: usize = 876310767337472;

/// 798TB in bytes.
/// This constant represents 798 tb converted to bytes for use in size calculations.
pub const TB_798: usize = 877410278965248;

/// 799TB in bytes.
/// This constant represents 799 tb converted to bytes for use in size calculations.
pub const TB_799: usize = 878509790593024;

/// 800TB in bytes.
/// This constant represents 800 tb converted to bytes for use in size calculations.
pub const TB_800: usize = 879609302220800;

/// 801TB in bytes.
/// This constant represents 801 tb converted to bytes for use in size calculations.
pub const TB_801: usize = 880708813848576;

/// 802TB in bytes.
/// This constant represents 802 tb converted to bytes for use in size calculations.
pub const TB_802: usize = 881808325476352;

/// 803TB in bytes.
/// This constant represents 803 tb converted to bytes for use in size calculations.
pub const TB_803: usize = 882907837104128;

/// 804TB in bytes.
/// This constant represents 804 tb converted to bytes for use in size calculations.
pub const TB_804: usize = 884007348731904;

/// 805TB in bytes.
/// This constant represents 805 tb converted to bytes for use in size calculations.
pub const TB_805: usize = 885106860359680;

/// 806TB in bytes.
/// This constant represents 806 tb converted to bytes for use in size calculations.
pub const TB_806: usize = 886206371987456;

/// 807TB in bytes.
/// This constant represents 807 tb converted to bytes for use in size calculations.
pub const TB_807: usize = 887305883615232;

/// 808TB in bytes.
/// This constant represents 808 tb converted to bytes for use in size calculations.
pub const TB_808: usize = 888405395243008;

/// 809TB in bytes.
/// This constant represents 809 tb converted to bytes for use in size calculations.
pub const TB_809: usize = 889504906870784;

/// 810TB in bytes.
/// This constant represents 810 tb converted to bytes for use in size calculations.
pub const TB_810: usize = 890604418498560;

/// 811TB in bytes.
/// This constant represents 811 tb converted to bytes for use in size calculations.
pub const TB_811: usize = 891703930126336;

/// 812TB in bytes.
/// This constant represents 812 tb converted to bytes for use in size calculations.
pub const TB_812: usize = 892803441754112;

/// 813TB in bytes.
/// This constant represents 813 tb converted to bytes for use in size calculations.
pub const TB_813: usize = 893902953381888;

/// 814TB in bytes.
/// This constant represents 814 tb converted to bytes for use in size calculations.
pub const TB_814: usize = 895002465009664;

/// 815TB in bytes.
/// This constant represents 815 tb converted to bytes for use in size calculations.
pub const TB_815: usize = 896101976637440;

/// 816TB in bytes.
/// This constant represents 816 tb converted to bytes for use in size calculations.
pub const TB_816: usize = 897201488265216;

/// 817TB in bytes.
/// This constant represents 817 tb converted to bytes for use in size calculations.
pub const TB_817: usize = 898300999892992;

/// 818TB in bytes.
/// This constant represents 818 tb converted to bytes for use in size calculations.
pub const TB_818: usize = 899400511520768;

/// 819TB in bytes.
/// This constant represents 819 tb converted to bytes for use in size calculations.
pub const TB_819: usize = 900500023148544;

/// 820TB in bytes.
/// This constant represents 820 tb converted to bytes for use in size calculations.
pub const TB_820: usize = 901599534776320;

/// 821TB in bytes.
/// This constant represents 821 tb converted to bytes for use in size calculations.
pub const TB_821: usize = 902699046404096;

/// 822TB in bytes.
/// This constant represents 822 tb converted to bytes for use in size calculations.
pub const TB_822: usize = 903798558031872;

/// 823TB in bytes.
/// This constant represents 823 tb converted to bytes for use in size calculations.
pub const TB_823: usize = 904898069659648;

/// 824TB in bytes.
/// This constant represents 824 tb converted to bytes for use in size calculations.
pub const TB_824: usize = 905997581287424;

/// 825TB in bytes.
/// This constant represents 825 tb converted to bytes for use in size calculations.
pub const TB_825: usize = 907097092915200;

/// 826TB in bytes.
/// This constant represents 826 tb converted to bytes for use in size calculations.
pub const TB_826: usize = 908196604542976;

/// 827TB in bytes.
/// This constant represents 827 tb converted to bytes for use in size calculations.
pub const TB_827: usize = 909296116170752;

/// 828TB in bytes.
/// This constant represents 828 tb converted to bytes for use in size calculations.
pub const TB_828: usize = 910395627798528;

/// 829TB in bytes.
/// This constant represents 829 tb converted to bytes for use in size calculations.
pub const TB_829: usize = 911495139426304;

/// 830TB in bytes.
/// This constant represents 830 tb converted to bytes for use in size calculations.
pub const TB_830: usize = 912594651054080;

/// 831TB in bytes.
/// This constant represents 831 tb converted to bytes for use in size calculations.
pub const TB_831: usize = 913694162681856;

/// 832TB in bytes.
/// This constant represents 832 tb converted to bytes for use in size calculations.
pub const TB_832: usize = 914793674309632;

/// 833TB in bytes.
/// This constant represents 833 tb converted to bytes for use in size calculations.
pub const TB_833: usize = 915893185937408;

/// 834TB in bytes.
/// This constant represents 834 tb converted to bytes for use in size calculations.
pub const TB_834: usize = 916992697565184;

/// 835TB in bytes.
/// This constant represents 835 tb converted to bytes for use in size calculations.
pub const TB_835: usize = 918092209192960;

/// 836TB in bytes.
/// This constant represents 836 tb converted to bytes for use in size calculations.
pub const TB_836: usize = 919191720820736;

/// 837TB in bytes.
/// This constant represents 837 tb converted to bytes for use in size calculations.
pub const TB_837: usize = 920291232448512;

/// 838TB in bytes.
/// This constant represents 838 tb converted to bytes for use in size calculations.
pub const TB_838: usize = 921390744076288;

/// 839TB in bytes.
/// This constant represents 839 tb converted to bytes for use in size calculations.
pub const TB_839: usize = 922490255704064;

/// 840TB in bytes.
/// This constant represents 840 tb converted to bytes for use in size calculations.
pub const TB_840: usize = 923589767331840;

/// 841TB in bytes.
/// This constant represents 841 tb converted to bytes for use in size calculations.
pub const TB_841: usize = 924689278959616;

/// 842TB in bytes.
/// This constant represents 842 tb converted to bytes for use in size calculations.
pub const TB_842: usize = 925788790587392;

/// 843TB in bytes.
/// This constant represents 843 tb converted to bytes for use in size calculations.
pub const TB_843: usize = 926888302215168;

/// 844TB in bytes.
/// This constant represents 844 tb converted to bytes for use in size calculations.
pub const TB_844: usize = 927987813842944;

/// 845TB in bytes.
/// This constant represents 845 tb converted to bytes for use in size calculations.
pub const TB_845: usize = 929087325470720;

/// 846TB in bytes.
/// This constant represents 846 tb converted to bytes for use in size calculations.
pub const TB_846: usize = 930186837098496;

/// 847TB in bytes.
/// This constant represents 847 tb converted to bytes for use in size calculations.
pub const TB_847: usize = 931286348726272;

/// 848TB in bytes.
/// This constant represents 848 tb converted to bytes for use in size calculations.
pub const TB_848: usize = 932385860354048;

/// 849TB in bytes.
/// This constant represents 849 tb converted to bytes for use in size calculations.
pub const TB_849: usize = 933485371981824;

/// 850TB in bytes.
/// This constant represents 850 tb converted to bytes for use in size calculations.
pub const TB_850: usize = 934584883609600;

/// 851TB in bytes.
/// This constant represents 851 tb converted to bytes for use in size calculations.
pub const TB_851: usize = 935684395237376;

/// 852TB in bytes.
/// This constant represents 852 tb converted to bytes for use in size calculations.
pub const TB_852: usize = 936783906865152;

/// 853TB in bytes.
/// This constant represents 853 tb converted to bytes for use in size calculations.
pub const TB_853: usize = 937883418492928;

/// 854TB in bytes.
/// This constant represents 854 tb converted to bytes for use in size calculations.
pub const TB_854: usize = 938982930120704;

/// 855TB in bytes.
/// This constant represents 855 tb converted to bytes for use in size calculations.
pub const TB_855: usize = 940082441748480;

/// 856TB in bytes.
/// This constant represents 856 tb converted to bytes for use in size calculations.
pub const TB_856: usize = 941181953376256;

/// 857TB in bytes.
/// This constant represents 857 tb converted to bytes for use in size calculations.
pub const TB_857: usize = 942281465004032;

/// 858TB in bytes.
/// This constant represents 858 tb converted to bytes for use in size calculations.
pub const TB_858: usize = 943380976631808;

/// 859TB in bytes.
/// This constant represents 859 tb converted to bytes for use in size calculations.
pub const TB_859: usize = 944480488259584;

/// 860TB in bytes.
/// This constant represents 860 tb converted to bytes for use in size calculations.
pub const TB_860: usize = 945579999887360;

/// 861TB in bytes.
/// This constant represents 861 tb converted to bytes for use in size calculations.
pub const TB_861: usize = 946679511515136;

/// 862TB in bytes.
/// This constant represents 862 tb converted to bytes for use in size calculations.
pub const TB_862: usize = 947779023142912;

/// 863TB in bytes.
/// This constant represents 863 tb converted to bytes for use in size calculations.
pub const TB_863: usize = 948878534770688;

/// 864TB in bytes.
/// This constant represents 864 tb converted to bytes for use in size calculations.
pub const TB_864: usize = 949978046398464;

/// 865TB in bytes.
/// This constant represents 865 tb converted to bytes for use in size calculations.
pub const TB_865: usize = 951077558026240;

/// 866TB in bytes.
/// This constant represents 866 tb converted to bytes for use in size calculations.
pub const TB_866: usize = 952177069654016;

/// 867TB in bytes.
/// This constant represents 867 tb converted to bytes for use in size calculations.
pub const TB_867: usize = 953276581281792;

/// 868TB in bytes.
/// This constant represents 868 tb converted to bytes for use in size calculations.
pub const TB_868: usize = 954376092909568;

/// 869TB in bytes.
/// This constant represents 869 tb converted to bytes for use in size calculations.
pub const TB_869: usize = 955475604537344;

/// 870TB in bytes.
/// This constant represents 870 tb converted to bytes for use in size calculations.
pub const TB_870: usize = 956575116165120;

/// 871TB in bytes.
/// This constant represents 871 tb converted to bytes for use in size calculations.
pub const TB_871: usize = 957674627792896;

/// 872TB in bytes.
/// This constant represents 872 tb converted to bytes for use in size calculations.
pub const TB_872: usize = 958774139420672;

/// 873TB in bytes.
/// This constant represents 873 tb converted to bytes for use in size calculations.
pub const TB_873: usize = 959873651048448;

/// 874TB in bytes.
/// This constant represents 874 tb converted to bytes for use in size calculations.
pub const TB_874: usize = 960973162676224;

/// 875TB in bytes.
/// This constant represents 875 tb converted to bytes for use in size calculations.
pub const TB_875: usize = 962072674304000;

/// 876TB in bytes.
/// This constant represents 876 tb converted to bytes for use in size calculations.
pub const TB_876: usize = 963172185931776;

/// 877TB in bytes.
/// This constant represents 877 tb converted to bytes for use in size calculations.
pub const TB_877: usize = 964271697559552;

/// 878TB in bytes.
/// This constant represents 878 tb converted to bytes for use in size calculations.
pub const TB_878: usize = 965371209187328;

/// 879TB in bytes.
/// This constant represents 879 tb converted to bytes for use in size calculations.
pub const TB_879: usize = 966470720815104;

/// 880TB in bytes.
/// This constant represents 880 tb converted to bytes for use in size calculations.
pub const TB_880: usize = 967570232442880;

/// 881TB in bytes.
/// This constant represents 881 tb converted to bytes for use in size calculations.
pub const TB_881: usize = 968669744070656;

/// 882TB in bytes.
/// This constant represents 882 tb converted to bytes for use in size calculations.
pub const TB_882: usize = 969769255698432;

/// 883TB in bytes.
/// This constant represents 883 tb converted to bytes for use in size calculations.
pub const TB_883: usize = 970868767326208;

/// 884TB in bytes.
/// This constant represents 884 tb converted to bytes for use in size calculations.
pub const TB_884: usize = 971968278953984;

/// 885TB in bytes.
/// This constant represents 885 tb converted to bytes for use in size calculations.
pub const TB_885: usize = 973067790581760;

/// 886TB in bytes.
/// This constant represents 886 tb converted to bytes for use in size calculations.
pub const TB_886: usize = 974167302209536;

/// 887TB in bytes.
/// This constant represents 887 tb converted to bytes for use in size calculations.
pub const TB_887: usize = 975266813837312;

/// 888TB in bytes.
/// This constant represents 888 tb converted to bytes for use in size calculations.
pub const TB_888: usize = 976366325465088;

/// 889TB in bytes.
/// This constant represents 889 tb converted to bytes for use in size calculations.
pub const TB_889: usize = 977465837092864;

/// 890TB in bytes.
/// This constant represents 890 tb converted to bytes for use in size calculations.
pub const TB_890: usize = 978565348720640;

/// 891TB in bytes.
/// This constant represents 891 tb converted to bytes for use in size calculations.
pub const TB_891: usize = 979664860348416;

/// 892TB in bytes.
/// This constant represents 892 tb converted to bytes for use in size calculations.
pub const TB_892: usize = 980764371976192;

/// 893TB in bytes.
/// This constant represents 893 tb converted to bytes for use in size calculations.
pub const TB_893: usize = 981863883603968;

/// 894TB in bytes.
/// This constant represents 894 tb converted to bytes for use in size calculations.
pub const TB_894: usize = 982963395231744;

/// 895TB in bytes.
/// This constant represents 895 tb converted to bytes for use in size calculations.
pub const TB_895: usize = 984062906859520;

/// 896TB in bytes.
/// This constant represents 896 tb converted to bytes for use in size calculations.
pub const TB_896: usize = 985162418487296;

/// 897TB in bytes.
/// This constant represents 897 tb converted to bytes for use in size calculations.
pub const TB_897: usize = 986261930115072;

/// 898TB in bytes.
/// This constant represents 898 tb converted to bytes for use in size calculations.
pub const TB_898: usize = 987361441742848;

/// 899TB in bytes.
/// This constant represents 899 tb converted to bytes for use in size calculations.
pub const TB_899: usize = 988460953370624;

/// 900TB in bytes.
/// This constant represents 900 tb converted to bytes for use in size calculations.
pub const TB_900: usize = 989560464998400;

/// 901TB in bytes.
/// This constant represents 901 tb converted to bytes for use in size calculations.
pub const TB_901: usize = 990659976626176;

/// 902TB in bytes.
/// This constant represents 902 tb converted to bytes for use in size calculations.
pub const TB_902: usize = 991759488253952;

/// 903TB in bytes.
/// This constant represents 903 tb converted to bytes for use in size calculations.
pub const TB_903: usize = 992858999881728;

/// 904TB in bytes.
/// This constant represents 904 tb converted to bytes for use in size calculations.
pub const TB_904: usize = 993958511509504;

/// 905TB in bytes.
/// This constant represents 905 tb converted to bytes for use in size calculations.
pub const TB_905: usize = 995058023137280;

/// 906TB in bytes.
/// This constant represents 906 tb converted to bytes for use in size calculations.
pub const TB_906: usize = 996157534765056;

/// 907TB in bytes.
/// This constant represents 907 tb converted to bytes for use in size calculations.
pub const TB_907: usize = 997257046392832;

/// 908TB in bytes.
/// This constant represents 908 tb converted to bytes for use in size calculations.
pub const TB_908: usize = 998356558020608;

/// 909TB in bytes.
/// This constant represents 909 tb converted to bytes for use in size calculations.
pub const TB_909: usize = 999456069648384;

/// 910TB in bytes.
/// This constant represents 910 tb converted to bytes for use in size calculations.
pub const TB_910: usize = 1000555581276160;

/// 911TB in bytes.
/// This constant represents 911 tb converted to bytes for use in size calculations.
pub const TB_911: usize = 1001655092903936;

/// 912TB in bytes.
/// This constant represents 912 tb converted to bytes for use in size calculations.
pub const TB_912: usize = 1002754604531712;

/// 913TB in bytes.
/// This constant represents 913 tb converted to bytes for use in size calculations.
pub const TB_913: usize = 1003854116159488;

/// 914TB in bytes.
/// This constant represents 914 tb converted to bytes for use in size calculations.
pub const TB_914: usize = 1004953627787264;

/// 915TB in bytes.
/// This constant represents 915 tb converted to bytes for use in size calculations.
pub const TB_915: usize = 1006053139415040;

/// 916TB in bytes.
/// This constant represents 916 tb converted to bytes for use in size calculations.
pub const TB_916: usize = 1007152651042816;

/// 917TB in bytes.
/// This constant represents 917 tb converted to bytes for use in size calculations.
pub const TB_917: usize = 1008252162670592;

/// 918TB in bytes.
/// This constant represents 918 tb converted to bytes for use in size calculations.
pub const TB_918: usize = 1009351674298368;

/// 919TB in bytes.
/// This constant represents 919 tb converted to bytes for use in size calculations.
pub const TB_919: usize = 1010451185926144;

/// 920TB in bytes.
/// This constant represents 920 tb converted to bytes for use in size calculations.
pub const TB_920: usize = 1011550697553920;

/// 921TB in bytes.
/// This constant represents 921 tb converted to bytes for use in size calculations.
pub const TB_921: usize = 1012650209181696;

/// 922TB in bytes.
/// This constant represents 922 tb converted to bytes for use in size calculations.
pub const TB_922: usize = 1013749720809472;

/// 923TB in bytes.
/// This constant represents 923 tb converted to bytes for use in size calculations.
pub const TB_923: usize = 1014849232437248;

/// 924TB in bytes.
/// This constant represents 924 tb converted to bytes for use in size calculations.
pub const TB_924: usize = 1015948744065024;

/// 925TB in bytes.
/// This constant represents 925 tb converted to bytes for use in size calculations.
pub const TB_925: usize = 1017048255692800;

/// 926TB in bytes.
/// This constant represents 926 tb converted to bytes for use in size calculations.
pub const TB_926: usize = 1018147767320576;

/// 927TB in bytes.
/// This constant represents 927 tb converted to bytes for use in size calculations.
pub const TB_927: usize = 1019247278948352;

/// 928TB in bytes.
/// This constant represents 928 tb converted to bytes for use in size calculations.
pub const TB_928: usize = 1020346790576128;

/// 929TB in bytes.
/// This constant represents 929 tb converted to bytes for use in size calculations.
pub const TB_929: usize = 1021446302203904;

/// 930TB in bytes.
/// This constant represents 930 tb converted to bytes for use in size calculations.
pub const TB_930: usize = 1022545813831680;

/// 931TB in bytes.
/// This constant represents 931 tb converted to bytes for use in size calculations.
pub const TB_931: usize = 1023645325459456;

/// 932TB in bytes.
/// This constant represents 932 tb converted to bytes for use in size calculations.
pub const TB_932: usize = 1024744837087232;

/// 933TB in bytes.
/// This constant represents 933 tb converted to bytes for use in size calculations.
pub const TB_933: usize = 1025844348715008;

/// 934TB in bytes.
/// This constant represents 934 tb converted to bytes for use in size calculations.
pub const TB_934: usize = 1026943860342784;

/// 935TB in bytes.
/// This constant represents 935 tb converted to bytes for use in size calculations.
pub const TB_935: usize = 1028043371970560;

/// 936TB in bytes.
/// This constant represents 936 tb converted to bytes for use in size calculations.
pub const TB_936: usize = 1029142883598336;

/// 937TB in bytes.
/// This constant represents 937 tb converted to bytes for use in size calculations.
pub const TB_937: usize = 1030242395226112;

/// 938TB in bytes.
/// This constant represents 938 tb converted to bytes for use in size calculations.
pub const TB_938: usize = 1031341906853888;

/// 939TB in bytes.
/// This constant represents 939 tb converted to bytes for use in size calculations.
pub const TB_939: usize = 1032441418481664;

/// 940TB in bytes.
/// This constant represents 940 tb converted to bytes for use in size calculations.
pub const TB_940: usize = 1033540930109440;

/// 941TB in bytes.
/// This constant represents 941 tb converted to bytes for use in size calculations.
pub const TB_941: usize = 1034640441737216;

/// 942TB in bytes.
/// This constant represents 942 tb converted to bytes for use in size calculations.
pub const TB_942: usize = 1035739953364992;

/// 943TB in bytes.
/// This constant represents 943 tb converted to bytes for use in size calculations.
pub const TB_943: usize = 1036839464992768;

/// 944TB in bytes.
/// This constant represents 944 tb converted to bytes for use in size calculations.
pub const TB_944: usize = 1037938976620544;

/// 945TB in bytes.
/// This constant represents 945 tb converted to bytes for use in size calculations.
pub const TB_945: usize = 1039038488248320;

/// 946TB in bytes.
/// This constant represents 946 tb converted to bytes for use in size calculations.
pub const TB_946: usize = 1040137999876096;

/// 947TB in bytes.
/// This constant represents 947 tb converted to bytes for use in size calculations.
pub const TB_947: usize = 1041237511503872;

/// 948TB in bytes.
/// This constant represents 948 tb converted to bytes for use in size calculations.
pub const TB_948: usize = 1042337023131648;

/// 949TB in bytes.
/// This constant represents 949 tb converted to bytes for use in size calculations.
pub const TB_949: usize = 1043436534759424;

/// 950TB in bytes.
/// This constant represents 950 tb converted to bytes for use in size calculations.
pub const TB_950: usize = 1044536046387200;

/// 951TB in bytes.
/// This constant represents 951 tb converted to bytes for use in size calculations.
pub const TB_951: usize = 1045635558014976;

/// 952TB in bytes.
/// This constant represents 952 tb converted to bytes for use in size calculations.
pub const TB_952: usize = 1046735069642752;

/// 953TB in bytes.
/// This constant represents 953 tb converted to bytes for use in size calculations.
pub const TB_953: usize = 1047834581270528;

/// 954TB in bytes.
/// This constant represents 954 tb converted to bytes for use in size calculations.
pub const TB_954: usize = 1048934092898304;

/// 955TB in bytes.
/// This constant represents 955 tb converted to bytes for use in size calculations.
pub const TB_955: usize = 1050033604526080;

/// 956TB in bytes.
/// This constant represents 956 tb converted to bytes for use in size calculations.
pub const TB_956: usize = 1051133116153856;

/// 957TB in bytes.
/// This constant represents 957 tb converted to bytes for use in size calculations.
pub const TB_957: usize = 1052232627781632;

/// 958TB in bytes.
/// This constant represents 958 tb converted to bytes for use in size calculations.
pub const TB_958: usize = 1053332139409408;

/// 959TB in bytes.
/// This constant represents 959 tb converted to bytes for use in size calculations.
pub const TB_959: usize = 1054431651037184;

/// 960TB in bytes.
/// This constant represents 960 tb converted to bytes for use in size calculations.
pub const TB_960: usize = 1055531162664960;

/// 961TB in bytes.
/// This constant represents 961 tb converted to bytes for use in size calculations.
pub const TB_961: usize = 1056630674292736;

/// 962TB in bytes.
/// This constant represents 962 tb converted to bytes for use in size calculations.
pub const TB_962: usize = 1057730185920512;

/// 963TB in bytes.
/// This constant represents 963 tb converted to bytes for use in size calculations.
pub const TB_963: usize = 1058829697548288;

/// 964TB in bytes.
/// This constant represents 964 tb converted to bytes for use in size calculations.
pub const TB_964: usize = 1059929209176064;

/// 965TB in bytes.
/// This constant represents 965 tb converted to bytes for use in size calculations.
pub const TB_965: usize = 1061028720803840;

/// 966TB in bytes.
/// This constant represents 966 tb converted to bytes for use in size calculations.
pub const TB_966: usize = 1062128232431616;

/// 967TB in bytes.
/// This constant represents 967 tb converted to bytes for use in size calculations.
pub const TB_967: usize = 1063227744059392;

/// 968TB in bytes.
/// This constant represents 968 tb converted to bytes for use in size calculations.
pub const TB_968: usize = 1064327255687168;

/// 969TB in bytes.
/// This constant represents 969 tb converted to bytes for use in size calculations.
pub const TB_969: usize = 1065426767314944;

/// 970TB in bytes.
/// This constant represents 970 tb converted to bytes for use in size calculations.
pub const TB_970: usize = 1066526278942720;

/// 971TB in bytes.
/// This constant represents 971 tb converted to bytes for use in size calculations.
pub const TB_971: usize = 1067625790570496;

/// 972TB in bytes.
/// This constant represents 972 tb converted to bytes for use in size calculations.
pub const TB_972: usize = 1068725302198272;

/// 973TB in bytes.
/// This constant represents 973 tb converted to bytes for use in size calculations.
pub const TB_973: usize = 1069824813826048;

/// 974TB in bytes.
/// This constant represents 974 tb converted to bytes for use in size calculations.
pub const TB_974: usize = 1070924325453824;

/// 975TB in bytes.
/// This constant represents 975 tb converted to bytes for use in size calculations.
pub const TB_975: usize = 1072023837081600;

/// 976TB in bytes.
/// This constant represents 976 tb converted to bytes for use in size calculations.
pub const TB_976: usize = 1073123348709376;

/// 977TB in bytes.
/// This constant represents 977 tb converted to bytes for use in size calculations.
pub const TB_977: usize = 1074222860337152;

/// 978TB in bytes.
/// This constant represents 978 tb converted to bytes for use in size calculations.
pub const TB_978: usize = 1075322371964928;

/// 979TB in bytes.
/// This constant represents 979 tb converted to bytes for use in size calculations.
pub const TB_979: usize = 1076421883592704;

/// 980TB in bytes.
/// This constant represents 980 tb converted to bytes for use in size calculations.
pub const TB_980: usize = 1077521395220480;

/// 981TB in bytes.
/// This constant represents 981 tb converted to bytes for use in size calculations.
pub const TB_981: usize = 1078620906848256;

/// 982TB in bytes.
/// This constant represents 982 tb converted to bytes for use in size calculations.
pub const TB_982: usize = 1079720418476032;

/// 983TB in bytes.
/// This constant represents 983 tb converted to bytes for use in size calculations.
pub const TB_983: usize = 1080819930103808;

/// 984TB in bytes.
/// This constant represents 984 tb converted to bytes for use in size calculations.
pub const TB_984: usize = 1081919441731584;

/// 985TB in bytes.
/// This constant represents 985 tb converted to bytes for use in size calculations.
pub const TB_985: usize = 1083018953359360;

/// 986TB in bytes.
/// This constant represents 986 tb converted to bytes for use in size calculations.
pub const TB_986: usize = 1084118464987136;

/// 987TB in bytes.
/// This constant represents 987 tb converted to bytes for use in size calculations.
pub const TB_987: usize = 1085217976614912;

/// 988TB in bytes.
/// This constant represents 988 tb converted to bytes for use in size calculations.
pub const TB_988: usize = 1086317488242688;

/// 989TB in bytes.
/// This constant represents 989 tb converted to bytes for use in size calculations.
pub const TB_989: usize = 1087416999870464;

/// 990TB in bytes.
/// This constant represents 990 tb converted to bytes for use in size calculations.
pub const TB_990: usize = 1088516511498240;

/// 991TB in bytes.
/// This constant represents 991 tb converted to bytes for use in size calculations.
pub const TB_991: usize = 1089616023126016;

/// 992TB in bytes.
/// This constant represents 992 tb converted to bytes for use in size calculations.
pub const TB_992: usize = 1090715534753792;

/// 993TB in bytes.
/// This constant represents 993 tb converted to bytes for use in size calculations.
pub const TB_993: usize = 1091815046381568;

/// 994TB in bytes.
/// This constant represents 994 tb converted to bytes for use in size calculations.
pub const TB_994: usize = 1092914558009344;

/// 995TB in bytes.
/// This constant represents 995 tb converted to bytes for use in size calculations.
pub const TB_995: usize = 1094014069637120;

/// 996TB in bytes.
/// This constant represents 996 tb converted to bytes for use in size calculations.
pub const TB_996: usize = 1095113581264896;

/// 997TB in bytes.
/// This constant represents 997 tb converted to bytes for use in size calculations.
pub const TB_997: usize = 1096213092892672;

/// 998TB in bytes.
/// This constant represents 998 tb converted to bytes for use in size calculations.
pub const TB_998: usize = 1097312604520448;

/// 999TB in bytes.
/// This constant represents 999 tb converted to bytes for use in size calculations.
pub const TB_999: usize = 1098412116148224;

/// 1000TB in bytes.
/// This constant represents 1000 tb converted to bytes for use in size calculations.
pub const TB_1000: usize = 1099511627776000;

/// 1001TB in bytes.
/// This constant represents 1001 tb converted to bytes for use in size calculations.
pub const TB_1001: usize = 1100611139403776;

/// 1002TB in bytes.
/// This constant represents 1002 tb converted to bytes for use in size calculations.
pub const TB_1002: usize = 1101710651031552;

/// 1003TB in bytes.
/// This constant represents 1003 tb converted to bytes for use in size calculations.
pub const TB_1003: usize = 1102810162659328;

/// 1004TB in bytes.
/// This constant represents 1004 tb converted to bytes for use in size calculations.
pub const TB_1004: usize = 1103909674287104;

/// 1005TB in bytes.
/// This constant represents 1005 tb converted to bytes for use in size calculations.
pub const TB_1005: usize = 1105009185914880;

/// 1006TB in bytes.
/// This constant represents 1006 tb converted to bytes for use in size calculations.
pub const TB_1006: usize = 1106108697542656;

/// 1007TB in bytes.
/// This constant represents 1007 tb converted to bytes for use in size calculations.
pub const TB_1007: usize = 1107208209170432;

/// 1008TB in bytes.
/// This constant represents 1008 tb converted to bytes for use in size calculations.
pub const TB_1008: usize = 1108307720798208;

/// 1009TB in bytes.
/// This constant represents 1009 tb converted to bytes for use in size calculations.
pub const TB_1009: usize = 1109407232425984;

/// 1010TB in bytes.
/// This constant represents 1010 tb converted to bytes for use in size calculations.
pub const TB_1010: usize = 1110506744053760;

/// 1011TB in bytes.
/// This constant represents 1011 tb converted to bytes for use in size calculations.
pub const TB_1011: usize = 1111606255681536;

/// 1012TB in bytes.
/// This constant represents 1012 tb converted to bytes for use in size calculations.
pub const TB_1012: usize = 1112705767309312;

/// 1013TB in bytes.
/// This constant represents 1013 tb converted to bytes for use in size calculations.
pub const TB_1013: usize = 1113805278937088;

/// 1014TB in bytes.
/// This constant represents 1014 tb converted to bytes for use in size calculations.
pub const TB_1014: usize = 1114904790564864;

/// 1015TB in bytes.
/// This constant represents 1015 tb converted to bytes for use in size calculations.
pub const TB_1015: usize = 1116004302192640;

/// 1016TB in bytes.
/// This constant represents 1016 tb converted to bytes for use in size calculations.
pub const TB_1016: usize = 1117103813820416;

/// 1017TB in bytes.
/// This constant represents 1017 tb converted to bytes for use in size calculations.
pub const TB_1017: usize = 1118203325448192;

/// 1018TB in bytes.
/// This constant represents 1018 tb converted to bytes for use in size calculations.
pub const TB_1018: usize = 1119302837075968;

/// 1019TB in bytes.
/// This constant represents 1019 tb converted to bytes for use in size calculations.
pub const TB_1019: usize = 1120402348703744;

/// 1020TB in bytes.
/// This constant represents 1020 tb converted to bytes for use in size calculations.
pub const TB_1020: usize = 1121501860331520;

/// 1021TB in bytes.
/// This constant represents 1021 tb converted to bytes for use in size calculations.
pub const TB_1021: usize = 1122601371959296;

/// 1022TB in bytes.
/// This constant represents 1022 tb converted to bytes for use in size calculations.
pub const TB_1022: usize = 1123700883587072;

/// 1023TB in bytes.
/// This constant represents 1023 tb converted to bytes for use in size calculations.
pub const TB_1023: usize = 1124800395214848;

/// 1024TB in bytes.
/// This constant represents 1024 tb converted to bytes for use in size calculations.
pub const TB_1024: usize = 1125899906842624;

// Storage unit constants from 1PB to 1024PB
/// 1PB in bytes.
/// This constant represents 1 pb converted to bytes for use in size calculations.
pub const PB_1: usize = 1125899906842624;

/// 2PB in bytes.
/// This constant represents 2 pb converted to bytes for use in size calculations.
pub const PB_2: usize = 2251799813685248;

/// 3PB in bytes.
/// This constant represents 3 pb converted to bytes for use in size calculations.
pub const PB_3: usize = 3377699720527872;

/// 4PB in bytes.
/// This constant represents 4 pb converted to bytes for use in size calculations.
pub const PB_4: usize = 4503599627370496;

/// 5PB in bytes.
/// This constant represents 5 pb converted to bytes for use in size calculations.
pub const PB_5: usize = 5629499534213120;

/// 6PB in bytes.
/// This constant represents 6 pb converted to bytes for use in size calculations.
pub const PB_6: usize = 6755399441055744;

/// 7PB in bytes.
/// This constant represents 7 pb converted to bytes for use in size calculations.
pub const PB_7: usize = 7881299347898368;

/// 8PB in bytes.
/// This constant represents 8 pb converted to bytes for use in size calculations.
pub const PB_8: usize = 9007199254740992;

/// 9PB in bytes.
/// This constant represents 9 pb converted to bytes for use in size calculations.
pub const PB_9: usize = 10133099161583616;

/// 10PB in bytes.
/// This constant represents 10 pb converted to bytes for use in size calculations.
pub const PB_10: usize = 11258999068426240;

/// 11PB in bytes.
/// This constant represents 11 pb converted to bytes for use in size calculations.
pub const PB_11: usize = 12384898975268864;

/// 12PB in bytes.
/// This constant represents 12 pb converted to bytes for use in size calculations.
pub const PB_12: usize = 13510798882111488;

/// 13PB in bytes.
/// This constant represents 13 pb converted to bytes for use in size calculations.
pub const PB_13: usize = 14636698788954112;

/// 14PB in bytes.
/// This constant represents 14 pb converted to bytes for use in size calculations.
pub const PB_14: usize = 15762598695796736;

/// 15PB in bytes.
/// This constant represents 15 pb converted to bytes for use in size calculations.
pub const PB_15: usize = 16888498602639360;

/// 16PB in bytes.
/// This constant represents 16 pb converted to bytes for use in size calculations.
pub const PB_16: usize = 18014398509481984;

/// 17PB in bytes.
/// This constant represents 17 pb converted to bytes for use in size calculations.
pub const PB_17: usize = 19140298416324608;

/// 18PB in bytes.
/// This constant represents 18 pb converted to bytes for use in size calculations.
pub const PB_18: usize = 20266198323167232;

/// 19PB in bytes.
/// This constant represents 19 pb converted to bytes for use in size calculations.
pub const PB_19: usize = 21392098230009856;

/// 20PB in bytes.
/// This constant represents 20 pb converted to bytes for use in size calculations.
pub const PB_20: usize = 22517998136852480;

/// 21PB in bytes.
/// This constant represents 21 pb converted to bytes for use in size calculations.
pub const PB_21: usize = 23643898043695104;

/// 22PB in bytes.
/// This constant represents 22 pb converted to bytes for use in size calculations.
pub const PB_22: usize = 24769797950537728;

/// 23PB in bytes.
/// This constant represents 23 pb converted to bytes for use in size calculations.
pub const PB_23: usize = 25895697857380352;

/// 24PB in bytes.
/// This constant represents 24 pb converted to bytes for use in size calculations.
pub const PB_24: usize = 27021597764222976;

/// 25PB in bytes.
/// This constant represents 25 pb converted to bytes for use in size calculations.
pub const PB_25: usize = 28147497671065600;

/// 26PB in bytes.
/// This constant represents 26 pb converted to bytes for use in size calculations.
pub const PB_26: usize = 29273397577908224;

/// 27PB in bytes.
/// This constant represents 27 pb converted to bytes for use in size calculations.
pub const PB_27: usize = 30399297484750848;

/// 28PB in bytes.
/// This constant represents 28 pb converted to bytes for use in size calculations.
pub const PB_28: usize = 31525197391593472;

/// 29PB in bytes.
/// This constant represents 29 pb converted to bytes for use in size calculations.
pub const PB_29: usize = 32651097298436096;

/// 30PB in bytes.
/// This constant represents 30 pb converted to bytes for use in size calculations.
pub const PB_30: usize = 33776997205278720;

/// 31PB in bytes.
/// This constant represents 31 pb converted to bytes for use in size calculations.
pub const PB_31: usize = 34902897112121344;

/// 32PB in bytes.
/// This constant represents 32 pb converted to bytes for use in size calculations.
pub const PB_32: usize = 36028797018963968;

/// 33PB in bytes.
/// This constant represents 33 pb converted to bytes for use in size calculations.
pub const PB_33: usize = 37154696925806592;

/// 34PB in bytes.
/// This constant represents 34 pb converted to bytes for use in size calculations.
pub const PB_34: usize = 38280596832649216;

/// 35PB in bytes.
/// This constant represents 35 pb converted to bytes for use in size calculations.
pub const PB_35: usize = 39406496739491840;

/// 36PB in bytes.
/// This constant represents 36 pb converted to bytes for use in size calculations.
pub const PB_36: usize = 40532396646334464;

/// 37PB in bytes.
/// This constant represents 37 pb converted to bytes for use in size calculations.
pub const PB_37: usize = 41658296553177088;

/// 38PB in bytes.
/// This constant represents 38 pb converted to bytes for use in size calculations.
pub const PB_38: usize = 42784196460019712;

/// 39PB in bytes.
/// This constant represents 39 pb converted to bytes for use in size calculations.
pub const PB_39: usize = 43910096366862336;

/// 40PB in bytes.
/// This constant represents 40 pb converted to bytes for use in size calculations.
pub const PB_40: usize = 45035996273704960;

/// 41PB in bytes.
/// This constant represents 41 pb converted to bytes for use in size calculations.
pub const PB_41: usize = 46161896180547584;

/// 42PB in bytes.
/// This constant represents 42 pb converted to bytes for use in size calculations.
pub const PB_42: usize = 47287796087390208;

/// 43PB in bytes.
/// This constant represents 43 pb converted to bytes for use in size calculations.
pub const PB_43: usize = 48413695994232832;

/// 44PB in bytes.
/// This constant represents 44 pb converted to bytes for use in size calculations.
pub const PB_44: usize = 49539595901075456;

/// 45PB in bytes.
/// This constant represents 45 pb converted to bytes for use in size calculations.
pub const PB_45: usize = 50665495807918080;

/// 46PB in bytes.
/// This constant represents 46 pb converted to bytes for use in size calculations.
pub const PB_46: usize = 51791395714760704;

/// 47PB in bytes.
/// This constant represents 47 pb converted to bytes for use in size calculations.
pub const PB_47: usize = 52917295621603328;

/// 48PB in bytes.
/// This constant represents 48 pb converted to bytes for use in size calculations.
pub const PB_48: usize = 54043195528445952;

/// 49PB in bytes.
/// This constant represents 49 pb converted to bytes for use in size calculations.
pub const PB_49: usize = 55169095435288576;

/// 50PB in bytes.
/// This constant represents 50 pb converted to bytes for use in size calculations.
pub const PB_50: usize = 56294995342131200;

/// 51PB in bytes.
/// This constant represents 51 pb converted to bytes for use in size calculations.
pub const PB_51: usize = 57420895248973824;

/// 52PB in bytes.
/// This constant represents 52 pb converted to bytes for use in size calculations.
pub const PB_52: usize = 58546795155816448;

/// 53PB in bytes.
/// This constant represents 53 pb converted to bytes for use in size calculations.
pub const PB_53: usize = 59672695062659072;

/// 54PB in bytes.
/// This constant represents 54 pb converted to bytes for use in size calculations.
pub const PB_54: usize = 60798594969501696;

/// 55PB in bytes.
/// This constant represents 55 pb converted to bytes for use in size calculations.
pub const PB_55: usize = 61924494876344320;

/// 56PB in bytes.
/// This constant represents 56 pb converted to bytes for use in size calculations.
pub const PB_56: usize = 63050394783186944;

/// 57PB in bytes.
/// This constant represents 57 pb converted to bytes for use in size calculations.
pub const PB_57: usize = 64176294690029568;

/// 58PB in bytes.
/// This constant represents 58 pb converted to bytes for use in size calculations.
pub const PB_58: usize = 65302194596872192;

/// 59PB in bytes.
/// This constant represents 59 pb converted to bytes for use in size calculations.
pub const PB_59: usize = 66428094503714816;

/// 60PB in bytes.
/// This constant represents 60 pb converted to bytes for use in size calculations.
pub const PB_60: usize = 67553994410557440;

/// 61PB in bytes.
/// This constant represents 61 pb converted to bytes for use in size calculations.
pub const PB_61: usize = 68679894317400064;

/// 62PB in bytes.
/// This constant represents 62 pb converted to bytes for use in size calculations.
pub const PB_62: usize = 69805794224242688;

/// 63PB in bytes.
/// This constant represents 63 pb converted to bytes for use in size calculations.
pub const PB_63: usize = 70931694131085312;

/// 64PB in bytes.
/// This constant represents 64 pb converted to bytes for use in size calculations.
pub const PB_64: usize = 72057594037927936;

/// 65PB in bytes.
/// This constant represents 65 pb converted to bytes for use in size calculations.
pub const PB_65: usize = 73183493944770560;

/// 66PB in bytes.
/// This constant represents 66 pb converted to bytes for use in size calculations.
pub const PB_66: usize = 74309393851613184;

/// 67PB in bytes.
/// This constant represents 67 pb converted to bytes for use in size calculations.
pub const PB_67: usize = 75435293758455808;

/// 68PB in bytes.
/// This constant represents 68 pb converted to bytes for use in size calculations.
pub const PB_68: usize = 76561193665298432;

/// 69PB in bytes.
/// This constant represents 69 pb converted to bytes for use in size calculations.
pub const PB_69: usize = 77687093572141056;

/// 70PB in bytes.
/// This constant represents 70 pb converted to bytes for use in size calculations.
pub const PB_70: usize = 78812993478983680;

/// 71PB in bytes.
/// This constant represents 71 pb converted to bytes for use in size calculations.
pub const PB_71: usize = 79938893385826304;

/// 72PB in bytes.
/// This constant represents 72 pb converted to bytes for use in size calculations.
pub const PB_72: usize = 81064793292668928;

/// 73PB in bytes.
/// This constant represents 73 pb converted to bytes for use in size calculations.
pub const PB_73: usize = 82190693199511552;

/// 74PB in bytes.
/// This constant represents 74 pb converted to bytes for use in size calculations.
pub const PB_74: usize = 83316593106354176;

/// 75PB in bytes.
/// This constant represents 75 pb converted to bytes for use in size calculations.
pub const PB_75: usize = 84442493013196800;

/// 76PB in bytes.
/// This constant represents 76 pb converted to bytes for use in size calculations.
pub const PB_76: usize = 85568392920039424;

/// 77PB in bytes.
/// This constant represents 77 pb converted to bytes for use in size calculations.
pub const PB_77: usize = 86694292826882048;

/// 78PB in bytes.
/// This constant represents 78 pb converted to bytes for use in size calculations.
pub const PB_78: usize = 87820192733724672;

/// 79PB in bytes.
/// This constant represents 79 pb converted to bytes for use in size calculations.
pub const PB_79: usize = 88946092640567296;

/// 80PB in bytes.
/// This constant represents 80 pb converted to bytes for use in size calculations.
pub const PB_80: usize = 90071992547409920;

/// 81PB in bytes.
/// This constant represents 81 pb converted to bytes for use in size calculations.
pub const PB_81: usize = 91197892454252544;

/// 82PB in bytes.
/// This constant represents 82 pb converted to bytes for use in size calculations.
pub const PB_82: usize = 92323792361095168;

/// 83PB in bytes.
/// This constant represents 83 pb converted to bytes for use in size calculations.
pub const PB_83: usize = 93449692267937792;

/// 84PB in bytes.
/// This constant represents 84 pb converted to bytes for use in size calculations.
pub const PB_84: usize = 94575592174780416;

/// 85PB in bytes.
/// This constant represents 85 pb converted to bytes for use in size calculations.
pub const PB_85: usize = 95701492081623040;

/// 86PB in bytes.
/// This constant represents 86 pb converted to bytes for use in size calculations.
pub const PB_86: usize = 96827391988465664;

/// 87PB in bytes.
/// This constant represents 87 pb converted to bytes for use in size calculations.
pub const PB_87: usize = 97953291895308288;

/// 88PB in bytes.
/// This constant represents 88 pb converted to bytes for use in size calculations.
pub const PB_88: usize = 99079191802150912;

/// 89PB in bytes.
/// This constant represents 89 pb converted to bytes for use in size calculations.
pub const PB_89: usize = 100205091708993536;

/// 90PB in bytes.
/// This constant represents 90 pb converted to bytes for use in size calculations.
pub const PB_90: usize = 101330991615836160;

/// 91PB in bytes.
/// This constant represents 91 pb converted to bytes for use in size calculations.
pub const PB_91: usize = 102456891522678784;

/// 92PB in bytes.
/// This constant represents 92 pb converted to bytes for use in size calculations.
pub const PB_92: usize = 103582791429521408;

/// 93PB in bytes.
/// This constant represents 93 pb converted to bytes for use in size calculations.
pub const PB_93: usize = 104708691336364032;

/// 94PB in bytes.
/// This constant represents 94 pb converted to bytes for use in size calculations.
pub const PB_94: usize = 105834591243206656;

/// 95PB in bytes.
/// This constant represents 95 pb converted to bytes for use in size calculations.
pub const PB_95: usize = 106960491150049280;

/// 96PB in bytes.
/// This constant represents 96 pb converted to bytes for use in size calculations.
pub const PB_96: usize = 108086391056891904;

/// 97PB in bytes.
/// This constant represents 97 pb converted to bytes for use in size calculations.
pub const PB_97: usize = 109212290963734528;

/// 98PB in bytes.
/// This constant represents 98 pb converted to bytes for use in size calculations.
pub const PB_98: usize = 110338190870577152;

/// 99PB in bytes.
/// This constant represents 99 pb converted to bytes for use in size calculations.
pub const PB_99: usize = 111464090777419776;

/// 100PB in bytes.
/// This constant represents 100 pb converted to bytes for use in size calculations.
pub const PB_100: usize = 112589990684262400;

/// 101PB in bytes.
/// This constant represents 101 pb converted to bytes for use in size calculations.
pub const PB_101: usize = 113715890591105024;

/// 102PB in bytes.
/// This constant represents 102 pb converted to bytes for use in size calculations.
pub const PB_102: usize = 114841790497947648;

/// 103PB in bytes.
/// This constant represents 103 pb converted to bytes for use in size calculations.
pub const PB_103: usize = 115967690404790272;

/// 104PB in bytes.
/// This constant represents 104 pb converted to bytes for use in size calculations.
pub const PB_104: usize = 117093590311632896;

/// 105PB in bytes.
/// This constant represents 105 pb converted to bytes for use in size calculations.
pub const PB_105: usize = 118219490218475520;

/// 106PB in bytes.
/// This constant represents 106 pb converted to bytes for use in size calculations.
pub const PB_106: usize = 119345390125318144;

/// 107PB in bytes.
/// This constant represents 107 pb converted to bytes for use in size calculations.
pub const PB_107: usize = 120471290032160768;

/// 108PB in bytes.
/// This constant represents 108 pb converted to bytes for use in size calculations.
pub const PB_108: usize = 121597189939003392;

/// 109PB in bytes.
/// This constant represents 109 pb converted to bytes for use in size calculations.
pub const PB_109: usize = 122723089845846016;

/// 110PB in bytes.
/// This constant represents 110 pb converted to bytes for use in size calculations.
pub const PB_110: usize = 123848989752688640;

/// 111PB in bytes.
/// This constant represents 111 pb converted to bytes for use in size calculations.
pub const PB_111: usize = 124974889659531264;

/// 112PB in bytes.
/// This constant represents 112 pb converted to bytes for use in size calculations.
pub const PB_112: usize = 126100789566373888;

/// 113PB in bytes.
/// This constant represents 113 pb converted to bytes for use in size calculations.
pub const PB_113: usize = 127226689473216512;

/// 114PB in bytes.
/// This constant represents 114 pb converted to bytes for use in size calculations.
pub const PB_114: usize = 128352589380059136;

/// 115PB in bytes.
/// This constant represents 115 pb converted to bytes for use in size calculations.
pub const PB_115: usize = 129478489286901760;

/// 116PB in bytes.
/// This constant represents 116 pb converted to bytes for use in size calculations.
pub const PB_116: usize = 130604389193744384;

/// 117PB in bytes.
/// This constant represents 117 pb converted to bytes for use in size calculations.
pub const PB_117: usize = 131730289100587008;

/// 118PB in bytes.
/// This constant represents 118 pb converted to bytes for use in size calculations.
pub const PB_118: usize = 132856189007429632;

/// 119PB in bytes.
/// This constant represents 119 pb converted to bytes for use in size calculations.
pub const PB_119: usize = 133982088914272256;

/// 120PB in bytes.
/// This constant represents 120 pb converted to bytes for use in size calculations.
pub const PB_120: usize = 135107988821114880;

/// 121PB in bytes.
/// This constant represents 121 pb converted to bytes for use in size calculations.
pub const PB_121: usize = 136233888727957504;

/// 122PB in bytes.
/// This constant represents 122 pb converted to bytes for use in size calculations.
pub const PB_122: usize = 137359788634800128;

/// 123PB in bytes.
/// This constant represents 123 pb converted to bytes for use in size calculations.
pub const PB_123: usize = 138485688541642752;

/// 124PB in bytes.
/// This constant represents 124 pb converted to bytes for use in size calculations.
pub const PB_124: usize = 139611588448485376;

/// 125PB in bytes.
/// This constant represents 125 pb converted to bytes for use in size calculations.
pub const PB_125: usize = 140737488355328000;

/// 126PB in bytes.
/// This constant represents 126 pb converted to bytes for use in size calculations.
pub const PB_126: usize = 141863388262170624;

/// 127PB in bytes.
/// This constant represents 127 pb converted to bytes for use in size calculations.
pub const PB_127: usize = 142989288169013248;

/// 128PB in bytes.
/// This constant represents 128 pb converted to bytes for use in size calculations.
pub const PB_128: usize = 144115188075855872;

/// 129PB in bytes.
/// This constant represents 129 pb converted to bytes for use in size calculations.
pub const PB_129: usize = 145241087982698496;

/// 130PB in bytes.
/// This constant represents 130 pb converted to bytes for use in size calculations.
pub const PB_130: usize = 146366987889541120;

/// 131PB in bytes.
/// This constant represents 131 pb converted to bytes for use in size calculations.
pub const PB_131: usize = 147492887796383744;

/// 132PB in bytes.
/// This constant represents 132 pb converted to bytes for use in size calculations.
pub const PB_132: usize = 148618787703226368;

/// 133PB in bytes.
/// This constant represents 133 pb converted to bytes for use in size calculations.
pub const PB_133: usize = 149744687610068992;

/// 134PB in bytes.
/// This constant represents 134 pb converted to bytes for use in size calculations.
pub const PB_134: usize = 150870587516911616;

/// 135PB in bytes.
/// This constant represents 135 pb converted to bytes for use in size calculations.
pub const PB_135: usize = 151996487423754240;

/// 136PB in bytes.
/// This constant represents 136 pb converted to bytes for use in size calculations.
pub const PB_136: usize = 153122387330596864;

/// 137PB in bytes.
/// This constant represents 137 pb converted to bytes for use in size calculations.
pub const PB_137: usize = 154248287237439488;

/// 138PB in bytes.
/// This constant represents 138 pb converted to bytes for use in size calculations.
pub const PB_138: usize = 155374187144282112;

/// 139PB in bytes.
/// This constant represents 139 pb converted to bytes for use in size calculations.
pub const PB_139: usize = 156500087051124736;

/// 140PB in bytes.
/// This constant represents 140 pb converted to bytes for use in size calculations.
pub const PB_140: usize = 157625986957967360;

/// 141PB in bytes.
/// This constant represents 141 pb converted to bytes for use in size calculations.
pub const PB_141: usize = 158751886864809984;

/// 142PB in bytes.
/// This constant represents 142 pb converted to bytes for use in size calculations.
pub const PB_142: usize = 159877786771652608;

/// 143PB in bytes.
/// This constant represents 143 pb converted to bytes for use in size calculations.
pub const PB_143: usize = 161003686678495232;

/// 144PB in bytes.
/// This constant represents 144 pb converted to bytes for use in size calculations.
pub const PB_144: usize = 162129586585337856;

/// 145PB in bytes.
/// This constant represents 145 pb converted to bytes for use in size calculations.
pub const PB_145: usize = 163255486492180480;

/// 146PB in bytes.
/// This constant represents 146 pb converted to bytes for use in size calculations.
pub const PB_146: usize = 164381386399023104;

/// 147PB in bytes.
/// This constant represents 147 pb converted to bytes for use in size calculations.
pub const PB_147: usize = 165507286305865728;

/// 148PB in bytes.
/// This constant represents 148 pb converted to bytes for use in size calculations.
pub const PB_148: usize = 166633186212708352;

/// 149PB in bytes.
/// This constant represents 149 pb converted to bytes for use in size calculations.
pub const PB_149: usize = 167759086119550976;

/// 150PB in bytes.
/// This constant represents 150 pb converted to bytes for use in size calculations.
pub const PB_150: usize = 168884986026393600;

/// 151PB in bytes.
/// This constant represents 151 pb converted to bytes for use in size calculations.
pub const PB_151: usize = 170010885933236224;

/// 152PB in bytes.
/// This constant represents 152 pb converted to bytes for use in size calculations.
pub const PB_152: usize = 171136785840078848;

/// 153PB in bytes.
/// This constant represents 153 pb converted to bytes for use in size calculations.
pub const PB_153: usize = 172262685746921472;

/// 154PB in bytes.
/// This constant represents 154 pb converted to bytes for use in size calculations.
pub const PB_154: usize = 173388585653764096;

/// 155PB in bytes.
/// This constant represents 155 pb converted to bytes for use in size calculations.
pub const PB_155: usize = 174514485560606720;

/// 156PB in bytes.
/// This constant represents 156 pb converted to bytes for use in size calculations.
pub const PB_156: usize = 175640385467449344;

/// 157PB in bytes.
/// This constant represents 157 pb converted to bytes for use in size calculations.
pub const PB_157: usize = 176766285374291968;

/// 158PB in bytes.
/// This constant represents 158 pb converted to bytes for use in size calculations.
pub const PB_158: usize = 177892185281134592;

/// 159PB in bytes.
/// This constant represents 159 pb converted to bytes for use in size calculations.
pub const PB_159: usize = 179018085187977216;

/// 160PB in bytes.
/// This constant represents 160 pb converted to bytes for use in size calculations.
pub const PB_160: usize = 180143985094819840;

/// 161PB in bytes.
/// This constant represents 161 pb converted to bytes for use in size calculations.
pub const PB_161: usize = 181269885001662464;

/// 162PB in bytes.
/// This constant represents 162 pb converted to bytes for use in size calculations.
pub const PB_162: usize = 182395784908505088;

/// 163PB in bytes.
/// This constant represents 163 pb converted to bytes for use in size calculations.
pub const PB_163: usize = 183521684815347712;

/// 164PB in bytes.
/// This constant represents 164 pb converted to bytes for use in size calculations.
pub const PB_164: usize = 184647584722190336;

/// 165PB in bytes.
/// This constant represents 165 pb converted to bytes for use in size calculations.
pub const PB_165: usize = 185773484629032960;

/// 166PB in bytes.
/// This constant represents 166 pb converted to bytes for use in size calculations.
pub const PB_166: usize = 186899384535875584;

/// 167PB in bytes.
/// This constant represents 167 pb converted to bytes for use in size calculations.
pub const PB_167: usize = 188025284442718208;

/// 168PB in bytes.
/// This constant represents 168 pb converted to bytes for use in size calculations.
pub const PB_168: usize = 189151184349560832;

/// 169PB in bytes.
/// This constant represents 169 pb converted to bytes for use in size calculations.
pub const PB_169: usize = 190277084256403456;

/// 170PB in bytes.
/// This constant represents 170 pb converted to bytes for use in size calculations.
pub const PB_170: usize = 191402984163246080;

/// 171PB in bytes.
/// This constant represents 171 pb converted to bytes for use in size calculations.
pub const PB_171: usize = 192528884070088704;

/// 172PB in bytes.
/// This constant represents 172 pb converted to bytes for use in size calculations.
pub const PB_172: usize = 193654783976931328;

/// 173PB in bytes.
/// This constant represents 173 pb converted to bytes for use in size calculations.
pub const PB_173: usize = 194780683883773952;

/// 174PB in bytes.
/// This constant represents 174 pb converted to bytes for use in size calculations.
pub const PB_174: usize = 195906583790616576;

/// 175PB in bytes.
/// This constant represents 175 pb converted to bytes for use in size calculations.
pub const PB_175: usize = 197032483697459200;

/// 176PB in bytes.
/// This constant represents 176 pb converted to bytes for use in size calculations.
pub const PB_176: usize = 198158383604301824;

/// 177PB in bytes.
/// This constant represents 177 pb converted to bytes for use in size calculations.
pub const PB_177: usize = 199284283511144448;

/// 178PB in bytes.
/// This constant represents 178 pb converted to bytes for use in size calculations.
pub const PB_178: usize = 200410183417987072;

/// 179PB in bytes.
/// This constant represents 179 pb converted to bytes for use in size calculations.
pub const PB_179: usize = 201536083324829696;

/// 180PB in bytes.
/// This constant represents 180 pb converted to bytes for use in size calculations.
pub const PB_180: usize = 202661983231672320;

/// 181PB in bytes.
/// This constant represents 181 pb converted to bytes for use in size calculations.
pub const PB_181: usize = 203787883138514944;

/// 182PB in bytes.
/// This constant represents 182 pb converted to bytes for use in size calculations.
pub const PB_182: usize = 204913783045357568;

/// 183PB in bytes.
/// This constant represents 183 pb converted to bytes for use in size calculations.
pub const PB_183: usize = 206039682952200192;

/// 184PB in bytes.
/// This constant represents 184 pb converted to bytes for use in size calculations.
pub const PB_184: usize = 207165582859042816;

/// 185PB in bytes.
/// This constant represents 185 pb converted to bytes for use in size calculations.
pub const PB_185: usize = 208291482765885440;

/// 186PB in bytes.
/// This constant represents 186 pb converted to bytes for use in size calculations.
pub const PB_186: usize = 209417382672728064;

/// 187PB in bytes.
/// This constant represents 187 pb converted to bytes for use in size calculations.
pub const PB_187: usize = 210543282579570688;

/// 188PB in bytes.
/// This constant represents 188 pb converted to bytes for use in size calculations.
pub const PB_188: usize = 211669182486413312;

/// 189PB in bytes.
/// This constant represents 189 pb converted to bytes for use in size calculations.
pub const PB_189: usize = 212795082393255936;

/// 190PB in bytes.
/// This constant represents 190 pb converted to bytes for use in size calculations.
pub const PB_190: usize = 213920982300098560;

/// 191PB in bytes.
/// This constant represents 191 pb converted to bytes for use in size calculations.
pub const PB_191: usize = 215046882206941184;

/// 192PB in bytes.
/// This constant represents 192 pb converted to bytes for use in size calculations.
pub const PB_192: usize = 216172782113783808;

/// 193PB in bytes.
/// This constant represents 193 pb converted to bytes for use in size calculations.
pub const PB_193: usize = 217298682020626432;

/// 194PB in bytes.
/// This constant represents 194 pb converted to bytes for use in size calculations.
pub const PB_194: usize = 218424581927469056;

/// 195PB in bytes.
/// This constant represents 195 pb converted to bytes for use in size calculations.
pub const PB_195: usize = 219550481834311680;

/// 196PB in bytes.
/// This constant represents 196 pb converted to bytes for use in size calculations.
pub const PB_196: usize = 220676381741154304;

/// 197PB in bytes.
/// This constant represents 197 pb converted to bytes for use in size calculations.
pub const PB_197: usize = 221802281647996928;

/// 198PB in bytes.
/// This constant represents 198 pb converted to bytes for use in size calculations.
pub const PB_198: usize = 222928181554839552;

/// 199PB in bytes.
/// This constant represents 199 pb converted to bytes for use in size calculations.
pub const PB_199: usize = 224054081461682176;

/// 200PB in bytes.
/// This constant represents 200 pb converted to bytes for use in size calculations.
pub const PB_200: usize = 225179981368524800;

/// 201PB in bytes.
/// This constant represents 201 pb converted to bytes for use in size calculations.
pub const PB_201: usize = 226305881275367424;

/// 202PB in bytes.
/// This constant represents 202 pb converted to bytes for use in size calculations.
pub const PB_202: usize = 227431781182210048;

/// 203PB in bytes.
/// This constant represents 203 pb converted to bytes for use in size calculations.
pub const PB_203: usize = 228557681089052672;

/// 204PB in bytes.
/// This constant represents 204 pb converted to bytes for use in size calculations.
pub const PB_204: usize = 229683580995895296;

/// 205PB in bytes.
/// This constant represents 205 pb converted to bytes for use in size calculations.
pub const PB_205: usize = 230809480902737920;

/// 206PB in bytes.
/// This constant represents 206 pb converted to bytes for use in size calculations.
pub const PB_206: usize = 231935380809580544;

/// 207PB in bytes.
/// This constant represents 207 pb converted to bytes for use in size calculations.
pub const PB_207: usize = 233061280716423168;

/// 208PB in bytes.
/// This constant represents 208 pb converted to bytes for use in size calculations.
pub const PB_208: usize = 234187180623265792;

/// 209PB in bytes.
/// This constant represents 209 pb converted to bytes for use in size calculations.
pub const PB_209: usize = 235313080530108416;

/// 210PB in bytes.
/// This constant represents 210 pb converted to bytes for use in size calculations.
pub const PB_210: usize = 236438980436951040;

/// 211PB in bytes.
/// This constant represents 211 pb converted to bytes for use in size calculations.
pub const PB_211: usize = 237564880343793664;

/// 212PB in bytes.
/// This constant represents 212 pb converted to bytes for use in size calculations.
pub const PB_212: usize = 238690780250636288;

/// 213PB in bytes.
/// This constant represents 213 pb converted to bytes for use in size calculations.
pub const PB_213: usize = 239816680157478912;

/// 214PB in bytes.
/// This constant represents 214 pb converted to bytes for use in size calculations.
pub const PB_214: usize = 240942580064321536;

/// 215PB in bytes.
/// This constant represents 215 pb converted to bytes for use in size calculations.
pub const PB_215: usize = 242068479971164160;

/// 216PB in bytes.
/// This constant represents 216 pb converted to bytes for use in size calculations.
pub const PB_216: usize = 243194379878006784;

/// 217PB in bytes.
/// This constant represents 217 pb converted to bytes for use in size calculations.
pub const PB_217: usize = 244320279784849408;

/// 218PB in bytes.
/// This constant represents 218 pb converted to bytes for use in size calculations.
pub const PB_218: usize = 245446179691692032;

/// 219PB in bytes.
/// This constant represents 219 pb converted to bytes for use in size calculations.
pub const PB_219: usize = 246572079598534656;

/// 220PB in bytes.
/// This constant represents 220 pb converted to bytes for use in size calculations.
pub const PB_220: usize = 247697979505377280;

/// 221PB in bytes.
/// This constant represents 221 pb converted to bytes for use in size calculations.
pub const PB_221: usize = 248823879412219904;

/// 222PB in bytes.
/// This constant represents 222 pb converted to bytes for use in size calculations.
pub const PB_222: usize = 249949779319062528;

/// 223PB in bytes.
/// This constant represents 223 pb converted to bytes for use in size calculations.
pub const PB_223: usize = 251075679225905152;

/// 224PB in bytes.
/// This constant represents 224 pb converted to bytes for use in size calculations.
pub const PB_224: usize = 252201579132747776;

/// 225PB in bytes.
/// This constant represents 225 pb converted to bytes for use in size calculations.
pub const PB_225: usize = 253327479039590400;

/// 226PB in bytes.
/// This constant represents 226 pb converted to bytes for use in size calculations.
pub const PB_226: usize = 254453378946433024;

/// 227PB in bytes.
/// This constant represents 227 pb converted to bytes for use in size calculations.
pub const PB_227: usize = 255579278853275648;

/// 228PB in bytes.
/// This constant represents 228 pb converted to bytes for use in size calculations.
pub const PB_228: usize = 256705178760118272;

/// 229PB in bytes.
/// This constant represents 229 pb converted to bytes for use in size calculations.
pub const PB_229: usize = 257831078666960896;

/// 230PB in bytes.
/// This constant represents 230 pb converted to bytes for use in size calculations.
pub const PB_230: usize = 258956978573803520;

/// 231PB in bytes.
/// This constant represents 231 pb converted to bytes for use in size calculations.
pub const PB_231: usize = 260082878480646144;

/// 232PB in bytes.
/// This constant represents 232 pb converted to bytes for use in size calculations.
pub const PB_232: usize = 261208778387488768;

/// 233PB in bytes.
/// This constant represents 233 pb converted to bytes for use in size calculations.
pub const PB_233: usize = 262334678294331392;

/// 234PB in bytes.
/// This constant represents 234 pb converted to bytes for use in size calculations.
pub const PB_234: usize = 263460578201174016;

/// 235PB in bytes.
/// This constant represents 235 pb converted to bytes for use in size calculations.
pub const PB_235: usize = 264586478108016640;

/// 236PB in bytes.
/// This constant represents 236 pb converted to bytes for use in size calculations.
pub const PB_236: usize = 265712378014859264;

/// 237PB in bytes.
/// This constant represents 237 pb converted to bytes for use in size calculations.
pub const PB_237: usize = 266838277921701888;

/// 238PB in bytes.
/// This constant represents 238 pb converted to bytes for use in size calculations.
pub const PB_238: usize = 267964177828544512;

/// 239PB in bytes.
/// This constant represents 239 pb converted to bytes for use in size calculations.
pub const PB_239: usize = 269090077735387136;

/// 240PB in bytes.
/// This constant represents 240 pb converted to bytes for use in size calculations.
pub const PB_240: usize = 270215977642229760;

/// 241PB in bytes.
/// This constant represents 241 pb converted to bytes for use in size calculations.
pub const PB_241: usize = 271341877549072384;

/// 242PB in bytes.
/// This constant represents 242 pb converted to bytes for use in size calculations.
pub const PB_242: usize = 272467777455915008;

/// 243PB in bytes.
/// This constant represents 243 pb converted to bytes for use in size calculations.
pub const PB_243: usize = 273593677362757632;

/// 244PB in bytes.
/// This constant represents 244 pb converted to bytes for use in size calculations.
pub const PB_244: usize = 274719577269600256;

/// 245PB in bytes.
/// This constant represents 245 pb converted to bytes for use in size calculations.
pub const PB_245: usize = 275845477176442880;

/// 246PB in bytes.
/// This constant represents 246 pb converted to bytes for use in size calculations.
pub const PB_246: usize = 276971377083285504;

/// 247PB in bytes.
/// This constant represents 247 pb converted to bytes for use in size calculations.
pub const PB_247: usize = 278097276990128128;

/// 248PB in bytes.
/// This constant represents 248 pb converted to bytes for use in size calculations.
pub const PB_248: usize = 279223176896970752;

/// 249PB in bytes.
/// This constant represents 249 pb converted to bytes for use in size calculations.
pub const PB_249: usize = 280349076803813376;

/// 250PB in bytes.
/// This constant represents 250 pb converted to bytes for use in size calculations.
pub const PB_250: usize = 281474976710656000;

/// 251PB in bytes.
/// This constant represents 251 pb converted to bytes for use in size calculations.
pub const PB_251: usize = 282600876617498624;

/// 252PB in bytes.
/// This constant represents 252 pb converted to bytes for use in size calculations.
pub const PB_252: usize = 283726776524341248;

/// 253PB in bytes.
/// This constant represents 253 pb converted to bytes for use in size calculations.
pub const PB_253: usize = 284852676431183872;

/// 254PB in bytes.
/// This constant represents 254 pb converted to bytes for use in size calculations.
pub const PB_254: usize = 285978576338026496;

/// 255PB in bytes.
/// This constant represents 255 pb converted to bytes for use in size calculations.
pub const PB_255: usize = 287104476244869120;

/// 256PB in bytes.
/// This constant represents 256 pb converted to bytes for use in size calculations.
pub const PB_256: usize = 288230376151711744;

/// 257PB in bytes.
/// This constant represents 257 pb converted to bytes for use in size calculations.
pub const PB_257: usize = 289356276058554368;

/// 258PB in bytes.
/// This constant represents 258 pb converted to bytes for use in size calculations.
pub const PB_258: usize = 290482175965396992;

/// 259PB in bytes.
/// This constant represents 259 pb converted to bytes for use in size calculations.
pub const PB_259: usize = 291608075872239616;

/// 260PB in bytes.
/// This constant represents 260 pb converted to bytes for use in size calculations.
pub const PB_260: usize = 292733975779082240;

/// 261PB in bytes.
/// This constant represents 261 pb converted to bytes for use in size calculations.
pub const PB_261: usize = 293859875685924864;

/// 262PB in bytes.
/// This constant represents 262 pb converted to bytes for use in size calculations.
pub const PB_262: usize = 294985775592767488;

/// 263PB in bytes.
/// This constant represents 263 pb converted to bytes for use in size calculations.
pub const PB_263: usize = 296111675499610112;

/// 264PB in bytes.
/// This constant represents 264 pb converted to bytes for use in size calculations.
pub const PB_264: usize = 297237575406452736;

/// 265PB in bytes.
/// This constant represents 265 pb converted to bytes for use in size calculations.
pub const PB_265: usize = 298363475313295360;

/// 266PB in bytes.
/// This constant represents 266 pb converted to bytes for use in size calculations.
pub const PB_266: usize = 299489375220137984;

/// 267PB in bytes.
/// This constant represents 267 pb converted to bytes for use in size calculations.
pub const PB_267: usize = 300615275126980608;

/// 268PB in bytes.
/// This constant represents 268 pb converted to bytes for use in size calculations.
pub const PB_268: usize = 301741175033823232;

/// 269PB in bytes.
/// This constant represents 269 pb converted to bytes for use in size calculations.
pub const PB_269: usize = 302867074940665856;

/// 270PB in bytes.
/// This constant represents 270 pb converted to bytes for use in size calculations.
pub const PB_270: usize = 303992974847508480;

/// 271PB in bytes.
/// This constant represents 271 pb converted to bytes for use in size calculations.
pub const PB_271: usize = 305118874754351104;

/// 272PB in bytes.
/// This constant represents 272 pb converted to bytes for use in size calculations.
pub const PB_272: usize = 306244774661193728;

/// 273PB in bytes.
/// This constant represents 273 pb converted to bytes for use in size calculations.
pub const PB_273: usize = 307370674568036352;

/// 274PB in bytes.
/// This constant represents 274 pb converted to bytes for use in size calculations.
pub const PB_274: usize = 308496574474878976;

/// 275PB in bytes.
/// This constant represents 275 pb converted to bytes for use in size calculations.
pub const PB_275: usize = 309622474381721600;

/// 276PB in bytes.
/// This constant represents 276 pb converted to bytes for use in size calculations.
pub const PB_276: usize = 310748374288564224;

/// 277PB in bytes.
/// This constant represents 277 pb converted to bytes for use in size calculations.
pub const PB_277: usize = 311874274195406848;

/// 278PB in bytes.
/// This constant represents 278 pb converted to bytes for use in size calculations.
pub const PB_278: usize = 313000174102249472;

/// 279PB in bytes.
/// This constant represents 279 pb converted to bytes for use in size calculations.
pub const PB_279: usize = 314126074009092096;

/// 280PB in bytes.
/// This constant represents 280 pb converted to bytes for use in size calculations.
pub const PB_280: usize = 315251973915934720;

/// 281PB in bytes.
/// This constant represents 281 pb converted to bytes for use in size calculations.
pub const PB_281: usize = 316377873822777344;

/// 282PB in bytes.
/// This constant represents 282 pb converted to bytes for use in size calculations.
pub const PB_282: usize = 317503773729619968;

/// 283PB in bytes.
/// This constant represents 283 pb converted to bytes for use in size calculations.
pub const PB_283: usize = 318629673636462592;

/// 284PB in bytes.
/// This constant represents 284 pb converted to bytes for use in size calculations.
pub const PB_284: usize = 319755573543305216;

/// 285PB in bytes.
/// This constant represents 285 pb converted to bytes for use in size calculations.
pub const PB_285: usize = 320881473450147840;

/// 286PB in bytes.
/// This constant represents 286 pb converted to bytes for use in size calculations.
pub const PB_286: usize = 322007373356990464;

/// 287PB in bytes.
/// This constant represents 287 pb converted to bytes for use in size calculations.
pub const PB_287: usize = 323133273263833088;

/// 288PB in bytes.
/// This constant represents 288 pb converted to bytes for use in size calculations.
pub const PB_288: usize = 324259173170675712;

/// 289PB in bytes.
/// This constant represents 289 pb converted to bytes for use in size calculations.
pub const PB_289: usize = 325385073077518336;

/// 290PB in bytes.
/// This constant represents 290 pb converted to bytes for use in size calculations.
pub const PB_290: usize = 326510972984360960;

/// 291PB in bytes.
/// This constant represents 291 pb converted to bytes for use in size calculations.
pub const PB_291: usize = 327636872891203584;

/// 292PB in bytes.
/// This constant represents 292 pb converted to bytes for use in size calculations.
pub const PB_292: usize = 328762772798046208;

/// 293PB in bytes.
/// This constant represents 293 pb converted to bytes for use in size calculations.
pub const PB_293: usize = 329888672704888832;

/// 294PB in bytes.
/// This constant represents 294 pb converted to bytes for use in size calculations.
pub const PB_294: usize = 331014572611731456;

/// 295PB in bytes.
/// This constant represents 295 pb converted to bytes for use in size calculations.
pub const PB_295: usize = 332140472518574080;

/// 296PB in bytes.
/// This constant represents 296 pb converted to bytes for use in size calculations.
pub const PB_296: usize = 333266372425416704;

/// 297PB in bytes.
/// This constant represents 297 pb converted to bytes for use in size calculations.
pub const PB_297: usize = 334392272332259328;

/// 298PB in bytes.
/// This constant represents 298 pb converted to bytes for use in size calculations.
pub const PB_298: usize = 335518172239101952;

/// 299PB in bytes.
/// This constant represents 299 pb converted to bytes for use in size calculations.
pub const PB_299: usize = 336644072145944576;

/// 300PB in bytes.
/// This constant represents 300 pb converted to bytes for use in size calculations.
pub const PB_300: usize = 337769972052787200;

/// 301PB in bytes.
/// This constant represents 301 pb converted to bytes for use in size calculations.
pub const PB_301: usize = 338895871959629824;

/// 302PB in bytes.
/// This constant represents 302 pb converted to bytes for use in size calculations.
pub const PB_302: usize = 340021771866472448;

/// 303PB in bytes.
/// This constant represents 303 pb converted to bytes for use in size calculations.
pub const PB_303: usize = 341147671773315072;

/// 304PB in bytes.
/// This constant represents 304 pb converted to bytes for use in size calculations.
pub const PB_304: usize = 342273571680157696;

/// 305PB in bytes.
/// This constant represents 305 pb converted to bytes for use in size calculations.
pub const PB_305: usize = 343399471587000320;

/// 306PB in bytes.
/// This constant represents 306 pb converted to bytes for use in size calculations.
pub const PB_306: usize = 344525371493842944;

/// 307PB in bytes.
/// This constant represents 307 pb converted to bytes for use in size calculations.
pub const PB_307: usize = 345651271400685568;

/// 308PB in bytes.
/// This constant represents 308 pb converted to bytes for use in size calculations.
pub const PB_308: usize = 346777171307528192;

/// 309PB in bytes.
/// This constant represents 309 pb converted to bytes for use in size calculations.
pub const PB_309: usize = 347903071214370816;

/// 310PB in bytes.
/// This constant represents 310 pb converted to bytes for use in size calculations.
pub const PB_310: usize = 349028971121213440;

/// 311PB in bytes.
/// This constant represents 311 pb converted to bytes for use in size calculations.
pub const PB_311: usize = 350154871028056064;

/// 312PB in bytes.
/// This constant represents 312 pb converted to bytes for use in size calculations.
pub const PB_312: usize = 351280770934898688;

/// 313PB in bytes.
/// This constant represents 313 pb converted to bytes for use in size calculations.
pub const PB_313: usize = 352406670841741312;

/// 314PB in bytes.
/// This constant represents 314 pb converted to bytes for use in size calculations.
pub const PB_314: usize = 353532570748583936;

/// 315PB in bytes.
/// This constant represents 315 pb converted to bytes for use in size calculations.
pub const PB_315: usize = 354658470655426560;

/// 316PB in bytes.
/// This constant represents 316 pb converted to bytes for use in size calculations.
pub const PB_316: usize = 355784370562269184;

/// 317PB in bytes.
/// This constant represents 317 pb converted to bytes for use in size calculations.
pub const PB_317: usize = 356910270469111808;

/// 318PB in bytes.
/// This constant represents 318 pb converted to bytes for use in size calculations.
pub const PB_318: usize = 358036170375954432;

/// 319PB in bytes.
/// This constant represents 319 pb converted to bytes for use in size calculations.
pub const PB_319: usize = 359162070282797056;

/// 320PB in bytes.
/// This constant represents 320 pb converted to bytes for use in size calculations.
pub const PB_320: usize = 360287970189639680;

/// 321PB in bytes.
/// This constant represents 321 pb converted to bytes for use in size calculations.
pub const PB_321: usize = 361413870096482304;

/// 322PB in bytes.
/// This constant represents 322 pb converted to bytes for use in size calculations.
pub const PB_322: usize = 362539770003324928;

/// 323PB in bytes.
/// This constant represents 323 pb converted to bytes for use in size calculations.
pub const PB_323: usize = 363665669910167552;

/// 324PB in bytes.
/// This constant represents 324 pb converted to bytes for use in size calculations.
pub const PB_324: usize = 364791569817010176;

/// 325PB in bytes.
/// This constant represents 325 pb converted to bytes for use in size calculations.
pub const PB_325: usize = 365917469723852800;

/// 326PB in bytes.
/// This constant represents 326 pb converted to bytes for use in size calculations.
pub const PB_326: usize = 367043369630695424;

/// 327PB in bytes.
/// This constant represents 327 pb converted to bytes for use in size calculations.
pub const PB_327: usize = 368169269537538048;

/// 328PB in bytes.
/// This constant represents 328 pb converted to bytes for use in size calculations.
pub const PB_328: usize = 369295169444380672;

/// 329PB in bytes.
/// This constant represents 329 pb converted to bytes for use in size calculations.
pub const PB_329: usize = 370421069351223296;

/// 330PB in bytes.
/// This constant represents 330 pb converted to bytes for use in size calculations.
pub const PB_330: usize = 371546969258065920;

/// 331PB in bytes.
/// This constant represents 331 pb converted to bytes for use in size calculations.
pub const PB_331: usize = 372672869164908544;

/// 332PB in bytes.
/// This constant represents 332 pb converted to bytes for use in size calculations.
pub const PB_332: usize = 373798769071751168;

/// 333PB in bytes.
/// This constant represents 333 pb converted to bytes for use in size calculations.
pub const PB_333: usize = 374924668978593792;

/// 334PB in bytes.
/// This constant represents 334 pb converted to bytes for use in size calculations.
pub const PB_334: usize = 376050568885436416;

/// 335PB in bytes.
/// This constant represents 335 pb converted to bytes for use in size calculations.
pub const PB_335: usize = 377176468792279040;

/// 336PB in bytes.
/// This constant represents 336 pb converted to bytes for use in size calculations.
pub const PB_336: usize = 378302368699121664;

/// 337PB in bytes.
/// This constant represents 337 pb converted to bytes for use in size calculations.
pub const PB_337: usize = 379428268605964288;

/// 338PB in bytes.
/// This constant represents 338 pb converted to bytes for use in size calculations.
pub const PB_338: usize = 380554168512806912;

/// 339PB in bytes.
/// This constant represents 339 pb converted to bytes for use in size calculations.
pub const PB_339: usize = 381680068419649536;

/// 340PB in bytes.
/// This constant represents 340 pb converted to bytes for use in size calculations.
pub const PB_340: usize = 382805968326492160;

/// 341PB in bytes.
/// This constant represents 341 pb converted to bytes for use in size calculations.
pub const PB_341: usize = 383931868233334784;

/// 342PB in bytes.
/// This constant represents 342 pb converted to bytes for use in size calculations.
pub const PB_342: usize = 385057768140177408;

/// 343PB in bytes.
/// This constant represents 343 pb converted to bytes for use in size calculations.
pub const PB_343: usize = 386183668047020032;

/// 344PB in bytes.
/// This constant represents 344 pb converted to bytes for use in size calculations.
pub const PB_344: usize = 387309567953862656;

/// 345PB in bytes.
/// This constant represents 345 pb converted to bytes for use in size calculations.
pub const PB_345: usize = 388435467860705280;

/// 346PB in bytes.
/// This constant represents 346 pb converted to bytes for use in size calculations.
pub const PB_346: usize = 389561367767547904;

/// 347PB in bytes.
/// This constant represents 347 pb converted to bytes for use in size calculations.
pub const PB_347: usize = 390687267674390528;

/// 348PB in bytes.
/// This constant represents 348 pb converted to bytes for use in size calculations.
pub const PB_348: usize = 391813167581233152;

/// 349PB in bytes.
/// This constant represents 349 pb converted to bytes for use in size calculations.
pub const PB_349: usize = 392939067488075776;

/// 350PB in bytes.
/// This constant represents 350 pb converted to bytes for use in size calculations.
pub const PB_350: usize = 394064967394918400;

/// 351PB in bytes.
/// This constant represents 351 pb converted to bytes for use in size calculations.
pub const PB_351: usize = 395190867301761024;

/// 352PB in bytes.
/// This constant represents 352 pb converted to bytes for use in size calculations.
pub const PB_352: usize = 396316767208603648;

/// 353PB in bytes.
/// This constant represents 353 pb converted to bytes for use in size calculations.
pub const PB_353: usize = 397442667115446272;

/// 354PB in bytes.
/// This constant represents 354 pb converted to bytes for use in size calculations.
pub const PB_354: usize = 398568567022288896;

/// 355PB in bytes.
/// This constant represents 355 pb converted to bytes for use in size calculations.
pub const PB_355: usize = 399694466929131520;

/// 356PB in bytes.
/// This constant represents 356 pb converted to bytes for use in size calculations.
pub const PB_356: usize = 400820366835974144;

/// 357PB in bytes.
/// This constant represents 357 pb converted to bytes for use in size calculations.
pub const PB_357: usize = 401946266742816768;

/// 358PB in bytes.
/// This constant represents 358 pb converted to bytes for use in size calculations.
pub const PB_358: usize = 403072166649659392;

/// 359PB in bytes.
/// This constant represents 359 pb converted to bytes for use in size calculations.
pub const PB_359: usize = 404198066556502016;

/// 360PB in bytes.
/// This constant represents 360 pb converted to bytes for use in size calculations.
pub const PB_360: usize = 405323966463344640;

/// 361PB in bytes.
/// This constant represents 361 pb converted to bytes for use in size calculations.
pub const PB_361: usize = 406449866370187264;

/// 362PB in bytes.
/// This constant represents 362 pb converted to bytes for use in size calculations.
pub const PB_362: usize = 407575766277029888;

/// 363PB in bytes.
/// This constant represents 363 pb converted to bytes for use in size calculations.
pub const PB_363: usize = 408701666183872512;

/// 364PB in bytes.
/// This constant represents 364 pb converted to bytes for use in size calculations.
pub const PB_364: usize = 409827566090715136;

/// 365PB in bytes.
/// This constant represents 365 pb converted to bytes for use in size calculations.
pub const PB_365: usize = 410953465997557760;

/// 366PB in bytes.
/// This constant represents 366 pb converted to bytes for use in size calculations.
pub const PB_366: usize = 412079365904400384;

/// 367PB in bytes.
/// This constant represents 367 pb converted to bytes for use in size calculations.
pub const PB_367: usize = 413205265811243008;

/// 368PB in bytes.
/// This constant represents 368 pb converted to bytes for use in size calculations.
pub const PB_368: usize = 414331165718085632;

/// 369PB in bytes.
/// This constant represents 369 pb converted to bytes for use in size calculations.
pub const PB_369: usize = 415457065624928256;

/// 370PB in bytes.
/// This constant represents 370 pb converted to bytes for use in size calculations.
pub const PB_370: usize = 416582965531770880;

/// 371PB in bytes.
/// This constant represents 371 pb converted to bytes for use in size calculations.
pub const PB_371: usize = 417708865438613504;

/// 372PB in bytes.
/// This constant represents 372 pb converted to bytes for use in size calculations.
pub const PB_372: usize = 418834765345456128;

/// 373PB in bytes.
/// This constant represents 373 pb converted to bytes for use in size calculations.
pub const PB_373: usize = 419960665252298752;

/// 374PB in bytes.
/// This constant represents 374 pb converted to bytes for use in size calculations.
pub const PB_374: usize = 421086565159141376;

/// 375PB in bytes.
/// This constant represents 375 pb converted to bytes for use in size calculations.
pub const PB_375: usize = 422212465065984000;

/// 376PB in bytes.
/// This constant represents 376 pb converted to bytes for use in size calculations.
pub const PB_376: usize = 423338364972826624;

/// 377PB in bytes.
/// This constant represents 377 pb converted to bytes for use in size calculations.
pub const PB_377: usize = 424464264879669248;

/// 378PB in bytes.
/// This constant represents 378 pb converted to bytes for use in size calculations.
pub const PB_378: usize = 425590164786511872;

/// 379PB in bytes.
/// This constant represents 379 pb converted to bytes for use in size calculations.
pub const PB_379: usize = 426716064693354496;

/// 380PB in bytes.
/// This constant represents 380 pb converted to bytes for use in size calculations.
pub const PB_380: usize = 427841964600197120;

/// 381PB in bytes.
/// This constant represents 381 pb converted to bytes for use in size calculations.
pub const PB_381: usize = 428967864507039744;

/// 382PB in bytes.
/// This constant represents 382 pb converted to bytes for use in size calculations.
pub const PB_382: usize = 430093764413882368;

/// 383PB in bytes.
/// This constant represents 383 pb converted to bytes for use in size calculations.
pub const PB_383: usize = 431219664320724992;

/// 384PB in bytes.
/// This constant represents 384 pb converted to bytes for use in size calculations.
pub const PB_384: usize = 432345564227567616;

/// 385PB in bytes.
/// This constant represents 385 pb converted to bytes for use in size calculations.
pub const PB_385: usize = 433471464134410240;

/// 386PB in bytes.
/// This constant represents 386 pb converted to bytes for use in size calculations.
pub const PB_386: usize = 434597364041252864;

/// 387PB in bytes.
/// This constant represents 387 pb converted to bytes for use in size calculations.
pub const PB_387: usize = 435723263948095488;

/// 388PB in bytes.
/// This constant represents 388 pb converted to bytes for use in size calculations.
pub const PB_388: usize = 436849163854938112;

/// 389PB in bytes.
/// This constant represents 389 pb converted to bytes for use in size calculations.
pub const PB_389: usize = 437975063761780736;

/// 390PB in bytes.
/// This constant represents 390 pb converted to bytes for use in size calculations.
pub const PB_390: usize = 439100963668623360;

/// 391PB in bytes.
/// This constant represents 391 pb converted to bytes for use in size calculations.
pub const PB_391: usize = 440226863575465984;

/// 392PB in bytes.
/// This constant represents 392 pb converted to bytes for use in size calculations.
pub const PB_392: usize = 441352763482308608;

/// 393PB in bytes.
/// This constant represents 393 pb converted to bytes for use in size calculations.
pub const PB_393: usize = 442478663389151232;

/// 394PB in bytes.
/// This constant represents 394 pb converted to bytes for use in size calculations.
pub const PB_394: usize = 443604563295993856;

/// 395PB in bytes.
/// This constant represents 395 pb converted to bytes for use in size calculations.
pub const PB_395: usize = 444730463202836480;

/// 396PB in bytes.
/// This constant represents 396 pb converted to bytes for use in size calculations.
pub const PB_396: usize = 445856363109679104;

/// 397PB in bytes.
/// This constant represents 397 pb converted to bytes for use in size calculations.
pub const PB_397: usize = 446982263016521728;

/// 398PB in bytes.
/// This constant represents 398 pb converted to bytes for use in size calculations.
pub const PB_398: usize = 448108162923364352;

/// 399PB in bytes.
/// This constant represents 399 pb converted to bytes for use in size calculations.
pub const PB_399: usize = 449234062830206976;

/// 400PB in bytes.
/// This constant represents 400 pb converted to bytes for use in size calculations.
pub const PB_400: usize = 450359962737049600;

/// 401PB in bytes.
/// This constant represents 401 pb converted to bytes for use in size calculations.
pub const PB_401: usize = 451485862643892224;

/// 402PB in bytes.
/// This constant represents 402 pb converted to bytes for use in size calculations.
pub const PB_402: usize = 452611762550734848;

/// 403PB in bytes.
/// This constant represents 403 pb converted to bytes for use in size calculations.
pub const PB_403: usize = 453737662457577472;

/// 404PB in bytes.
/// This constant represents 404 pb converted to bytes for use in size calculations.
pub const PB_404: usize = 454863562364420096;

/// 405PB in bytes.
/// This constant represents 405 pb converted to bytes for use in size calculations.
pub const PB_405: usize = 455989462271262720;

/// 406PB in bytes.
/// This constant represents 406 pb converted to bytes for use in size calculations.
pub const PB_406: usize = 457115362178105344;

/// 407PB in bytes.
/// This constant represents 407 pb converted to bytes for use in size calculations.
pub const PB_407: usize = 458241262084947968;

/// 408PB in bytes.
/// This constant represents 408 pb converted to bytes for use in size calculations.
pub const PB_408: usize = 459367161991790592;

/// 409PB in bytes.
/// This constant represents 409 pb converted to bytes for use in size calculations.
pub const PB_409: usize = 460493061898633216;

/// 410PB in bytes.
/// This constant represents 410 pb converted to bytes for use in size calculations.
pub const PB_410: usize = 461618961805475840;

/// 411PB in bytes.
/// This constant represents 411 pb converted to bytes for use in size calculations.
pub const PB_411: usize = 462744861712318464;

/// 412PB in bytes.
/// This constant represents 412 pb converted to bytes for use in size calculations.
pub const PB_412: usize = 463870761619161088;

/// 413PB in bytes.
/// This constant represents 413 pb converted to bytes for use in size calculations.
pub const PB_413: usize = 464996661526003712;

/// 414PB in bytes.
/// This constant represents 414 pb converted to bytes for use in size calculations.
pub const PB_414: usize = 466122561432846336;

/// 415PB in bytes.
/// This constant represents 415 pb converted to bytes for use in size calculations.
pub const PB_415: usize = 467248461339688960;

/// 416PB in bytes.
/// This constant represents 416 pb converted to bytes for use in size calculations.
pub const PB_416: usize = 468374361246531584;

/// 417PB in bytes.
/// This constant represents 417 pb converted to bytes for use in size calculations.
pub const PB_417: usize = 469500261153374208;

/// 418PB in bytes.
/// This constant represents 418 pb converted to bytes for use in size calculations.
pub const PB_418: usize = 470626161060216832;

/// 419PB in bytes.
/// This constant represents 419 pb converted to bytes for use in size calculations.
pub const PB_419: usize = 471752060967059456;

/// 420PB in bytes.
/// This constant represents 420 pb converted to bytes for use in size calculations.
pub const PB_420: usize = 472877960873902080;

/// 421PB in bytes.
/// This constant represents 421 pb converted to bytes for use in size calculations.
pub const PB_421: usize = 474003860780744704;

/// 422PB in bytes.
/// This constant represents 422 pb converted to bytes for use in size calculations.
pub const PB_422: usize = 475129760687587328;

/// 423PB in bytes.
/// This constant represents 423 pb converted to bytes for use in size calculations.
pub const PB_423: usize = 476255660594429952;

/// 424PB in bytes.
/// This constant represents 424 pb converted to bytes for use in size calculations.
pub const PB_424: usize = 477381560501272576;

/// 425PB in bytes.
/// This constant represents 425 pb converted to bytes for use in size calculations.
pub const PB_425: usize = 478507460408115200;

/// 426PB in bytes.
/// This constant represents 426 pb converted to bytes for use in size calculations.
pub const PB_426: usize = 479633360314957824;

/// 427PB in bytes.
/// This constant represents 427 pb converted to bytes for use in size calculations.
pub const PB_427: usize = 480759260221800448;

/// 428PB in bytes.
/// This constant represents 428 pb converted to bytes for use in size calculations.
pub const PB_428: usize = 481885160128643072;

/// 429PB in bytes.
/// This constant represents 429 pb converted to bytes for use in size calculations.
pub const PB_429: usize = 483011060035485696;

/// 430PB in bytes.
/// This constant represents 430 pb converted to bytes for use in size calculations.
pub const PB_430: usize = 484136959942328320;

/// 431PB in bytes.
/// This constant represents 431 pb converted to bytes for use in size calculations.
pub const PB_431: usize = 485262859849170944;

/// 432PB in bytes.
/// This constant represents 432 pb converted to bytes for use in size calculations.
pub const PB_432: usize = 486388759756013568;

/// 433PB in bytes.
/// This constant represents 433 pb converted to bytes for use in size calculations.
pub const PB_433: usize = 487514659662856192;

/// 434PB in bytes.
/// This constant represents 434 pb converted to bytes for use in size calculations.
pub const PB_434: usize = 488640559569698816;

/// 435PB in bytes.
/// This constant represents 435 pb converted to bytes for use in size calculations.
pub const PB_435: usize = 489766459476541440;

/// 436PB in bytes.
/// This constant represents 436 pb converted to bytes for use in size calculations.
pub const PB_436: usize = 490892359383384064;

/// 437PB in bytes.
/// This constant represents 437 pb converted to bytes for use in size calculations.
pub const PB_437: usize = 492018259290226688;

/// 438PB in bytes.
/// This constant represents 438 pb converted to bytes for use in size calculations.
pub const PB_438: usize = 493144159197069312;

/// 439PB in bytes.
/// This constant represents 439 pb converted to bytes for use in size calculations.
pub const PB_439: usize = 494270059103911936;

/// 440PB in bytes.
/// This constant represents 440 pb converted to bytes for use in size calculations.
pub const PB_440: usize = 495395959010754560;

/// 441PB in bytes.
/// This constant represents 441 pb converted to bytes for use in size calculations.
pub const PB_441: usize = 496521858917597184;

/// 442PB in bytes.
/// This constant represents 442 pb converted to bytes for use in size calculations.
pub const PB_442: usize = 497647758824439808;

/// 443PB in bytes.
/// This constant represents 443 pb converted to bytes for use in size calculations.
pub const PB_443: usize = 498773658731282432;

/// 444PB in bytes.
/// This constant represents 444 pb converted to bytes for use in size calculations.
pub const PB_444: usize = 499899558638125056;

/// 445PB in bytes.
/// This constant represents 445 pb converted to bytes for use in size calculations.
pub const PB_445: usize = 501025458544967680;

/// 446PB in bytes.
/// This constant represents 446 pb converted to bytes for use in size calculations.
pub const PB_446: usize = 502151358451810304;

/// 447PB in bytes.
/// This constant represents 447 pb converted to bytes for use in size calculations.
pub const PB_447: usize = 503277258358652928;

/// 448PB in bytes.
/// This constant represents 448 pb converted to bytes for use in size calculations.
pub const PB_448: usize = 504403158265495552;

/// 449PB in bytes.
/// This constant represents 449 pb converted to bytes for use in size calculations.
pub const PB_449: usize = 505529058172338176;

/// 450PB in bytes.
/// This constant represents 450 pb converted to bytes for use in size calculations.
pub const PB_450: usize = 506654958079180800;

/// 451PB in bytes.
/// This constant represents 451 pb converted to bytes for use in size calculations.
pub const PB_451: usize = 507780857986023424;

/// 452PB in bytes.
/// This constant represents 452 pb converted to bytes for use in size calculations.
pub const PB_452: usize = 508906757892866048;

/// 453PB in bytes.
/// This constant represents 453 pb converted to bytes for use in size calculations.
pub const PB_453: usize = 510032657799708672;

/// 454PB in bytes.
/// This constant represents 454 pb converted to bytes for use in size calculations.
pub const PB_454: usize = 511158557706551296;

/// 455PB in bytes.
/// This constant represents 455 pb converted to bytes for use in size calculations.
pub const PB_455: usize = 512284457613393920;

/// 456PB in bytes.
/// This constant represents 456 pb converted to bytes for use in size calculations.
pub const PB_456: usize = 513410357520236544;

/// 457PB in bytes.
/// This constant represents 457 pb converted to bytes for use in size calculations.
pub const PB_457: usize = 514536257427079168;

/// 458PB in bytes.
/// This constant represents 458 pb converted to bytes for use in size calculations.
pub const PB_458: usize = 515662157333921792;

/// 459PB in bytes.
/// This constant represents 459 pb converted to bytes for use in size calculations.
pub const PB_459: usize = 516788057240764416;

/// 460PB in bytes.
/// This constant represents 460 pb converted to bytes for use in size calculations.
pub const PB_460: usize = 517913957147607040;

/// 461PB in bytes.
/// This constant represents 461 pb converted to bytes for use in size calculations.
pub const PB_461: usize = 519039857054449664;

/// 462PB in bytes.
/// This constant represents 462 pb converted to bytes for use in size calculations.
pub const PB_462: usize = 520165756961292288;

/// 463PB in bytes.
/// This constant represents 463 pb converted to bytes for use in size calculations.
pub const PB_463: usize = 521291656868134912;

/// 464PB in bytes.
/// This constant represents 464 pb converted to bytes for use in size calculations.
pub const PB_464: usize = 522417556774977536;

/// 465PB in bytes.
/// This constant represents 465 pb converted to bytes for use in size calculations.
pub const PB_465: usize = 523543456681820160;

/// 466PB in bytes.
/// This constant represents 466 pb converted to bytes for use in size calculations.
pub const PB_466: usize = 524669356588662784;

/// 467PB in bytes.
/// This constant represents 467 pb converted to bytes for use in size calculations.
pub const PB_467: usize = 525795256495505408;

/// 468PB in bytes.
/// This constant represents 468 pb converted to bytes for use in size calculations.
pub const PB_468: usize = 526921156402348032;

/// 469PB in bytes.
/// This constant represents 469 pb converted to bytes for use in size calculations.
pub const PB_469: usize = 528047056309190656;

/// 470PB in bytes.
/// This constant represents 470 pb converted to bytes for use in size calculations.
pub const PB_470: usize = 529172956216033280;

/// 471PB in bytes.
/// This constant represents 471 pb converted to bytes for use in size calculations.
pub const PB_471: usize = 530298856122875904;

/// 472PB in bytes.
/// This constant represents 472 pb converted to bytes for use in size calculations.
pub const PB_472: usize = 531424756029718528;

/// 473PB in bytes.
/// This constant represents 473 pb converted to bytes for use in size calculations.
pub const PB_473: usize = 532550655936561152;

/// 474PB in bytes.
/// This constant represents 474 pb converted to bytes for use in size calculations.
pub const PB_474: usize = 533676555843403776;

/// 475PB in bytes.
/// This constant represents 475 pb converted to bytes for use in size calculations.
pub const PB_475: usize = 534802455750246400;

/// 476PB in bytes.
/// This constant represents 476 pb converted to bytes for use in size calculations.
pub const PB_476: usize = 535928355657089024;

/// 477PB in bytes.
/// This constant represents 477 pb converted to bytes for use in size calculations.
pub const PB_477: usize = 537054255563931648;

/// 478PB in bytes.
/// This constant represents 478 pb converted to bytes for use in size calculations.
pub const PB_478: usize = 538180155470774272;

/// 479PB in bytes.
/// This constant represents 479 pb converted to bytes for use in size calculations.
pub const PB_479: usize = 539306055377616896;

/// 480PB in bytes.
/// This constant represents 480 pb converted to bytes for use in size calculations.
pub const PB_480: usize = 540431955284459520;

/// 481PB in bytes.
/// This constant represents 481 pb converted to bytes for use in size calculations.
pub const PB_481: usize = 541557855191302144;

/// 482PB in bytes.
/// This constant represents 482 pb converted to bytes for use in size calculations.
pub const PB_482: usize = 542683755098144768;

/// 483PB in bytes.
/// This constant represents 483 pb converted to bytes for use in size calculations.
pub const PB_483: usize = 543809655004987392;

/// 484PB in bytes.
/// This constant represents 484 pb converted to bytes for use in size calculations.
pub const PB_484: usize = 544935554911830016;

/// 485PB in bytes.
/// This constant represents 485 pb converted to bytes for use in size calculations.
pub const PB_485: usize = 546061454818672640;

/// 486PB in bytes.
/// This constant represents 486 pb converted to bytes for use in size calculations.
pub const PB_486: usize = 547187354725515264;

/// 487PB in bytes.
/// This constant represents 487 pb converted to bytes for use in size calculations.
pub const PB_487: usize = 548313254632357888;

/// 488PB in bytes.
/// This constant represents 488 pb converted to bytes for use in size calculations.
pub const PB_488: usize = 549439154539200512;

/// 489PB in bytes.
/// This constant represents 489 pb converted to bytes for use in size calculations.
pub const PB_489: usize = 550565054446043136;

/// 490PB in bytes.
/// This constant represents 490 pb converted to bytes for use in size calculations.
pub const PB_490: usize = 551690954352885760;

/// 491PB in bytes.
/// This constant represents 491 pb converted to bytes for use in size calculations.
pub const PB_491: usize = 552816854259728384;

/// 492PB in bytes.
/// This constant represents 492 pb converted to bytes for use in size calculations.
pub const PB_492: usize = 553942754166571008;

/// 493PB in bytes.
/// This constant represents 493 pb converted to bytes for use in size calculations.
pub const PB_493: usize = 555068654073413632;

/// 494PB in bytes.
/// This constant represents 494 pb converted to bytes for use in size calculations.
pub const PB_494: usize = 556194553980256256;

/// 495PB in bytes.
/// This constant represents 495 pb converted to bytes for use in size calculations.
pub const PB_495: usize = 557320453887098880;

/// 496PB in bytes.
/// This constant represents 496 pb converted to bytes for use in size calculations.
pub const PB_496: usize = 558446353793941504;

/// 497PB in bytes.
/// This constant represents 497 pb converted to bytes for use in size calculations.
pub const PB_497: usize = 559572253700784128;

/// 498PB in bytes.
/// This constant represents 498 pb converted to bytes for use in size calculations.
pub const PB_498: usize = 560698153607626752;

/// 499PB in bytes.
/// This constant represents 499 pb converted to bytes for use in size calculations.
pub const PB_499: usize = 561824053514469376;

/// 500PB in bytes.
/// This constant represents 500 pb converted to bytes for use in size calculations.
pub const PB_500: usize = 562949953421312000;

/// 501PB in bytes.
/// This constant represents 501 pb converted to bytes for use in size calculations.
pub const PB_501: usize = 564075853328154624;

/// 502PB in bytes.
/// This constant represents 502 pb converted to bytes for use in size calculations.
pub const PB_502: usize = 565201753234997248;

/// 503PB in bytes.
/// This constant represents 503 pb converted to bytes for use in size calculations.
pub const PB_503: usize = 566327653141839872;

/// 504PB in bytes.
/// This constant represents 504 pb converted to bytes for use in size calculations.
pub const PB_504: usize = 567453553048682496;

/// 505PB in bytes.
/// This constant represents 505 pb converted to bytes for use in size calculations.
pub const PB_505: usize = 568579452955525120;

/// 506PB in bytes.
/// This constant represents 506 pb converted to bytes for use in size calculations.
pub const PB_506: usize = 569705352862367744;

/// 507PB in bytes.
/// This constant represents 507 pb converted to bytes for use in size calculations.
pub const PB_507: usize = 570831252769210368;

/// 508PB in bytes.
/// This constant represents 508 pb converted to bytes for use in size calculations.
pub const PB_508: usize = 571957152676052992;

/// 509PB in bytes.
/// This constant represents 509 pb converted to bytes for use in size calculations.
pub const PB_509: usize = 573083052582895616;

/// 510PB in bytes.
/// This constant represents 510 pb converted to bytes for use in size calculations.
pub const PB_510: usize = 574208952489738240;

/// 511PB in bytes.
/// This constant represents 511 pb converted to bytes for use in size calculations.
pub const PB_511: usize = 575334852396580864;

/// 512PB in bytes.
/// This constant represents 512 pb converted to bytes for use in size calculations.
pub const PB_512: usize = 576460752303423488;

/// 513PB in bytes.
/// This constant represents 513 pb converted to bytes for use in size calculations.
pub const PB_513: usize = 577586652210266112;

/// 514PB in bytes.
/// This constant represents 514 pb converted to bytes for use in size calculations.
pub const PB_514: usize = 578712552117108736;

/// 515PB in bytes.
/// This constant represents 515 pb converted to bytes for use in size calculations.
pub const PB_515: usize = 579838452023951360;

/// 516PB in bytes.
/// This constant represents 516 pb converted to bytes for use in size calculations.
pub const PB_516: usize = 580964351930793984;

/// 517PB in bytes.
/// This constant represents 517 pb converted to bytes for use in size calculations.
pub const PB_517: usize = 582090251837636608;

/// 518PB in bytes.
/// This constant represents 518 pb converted to bytes for use in size calculations.
pub const PB_518: usize = 583216151744479232;

/// 519PB in bytes.
/// This constant represents 519 pb converted to bytes for use in size calculations.
pub const PB_519: usize = 584342051651321856;

/// 520PB in bytes.
/// This constant represents 520 pb converted to bytes for use in size calculations.
pub const PB_520: usize = 585467951558164480;

/// 521PB in bytes.
/// This constant represents 521 pb converted to bytes for use in size calculations.
pub const PB_521: usize = 586593851465007104;

/// 522PB in bytes.
/// This constant represents 522 pb converted to bytes for use in size calculations.
pub const PB_522: usize = 587719751371849728;

/// 523PB in bytes.
/// This constant represents 523 pb converted to bytes for use in size calculations.
pub const PB_523: usize = 588845651278692352;

/// 524PB in bytes.
/// This constant represents 524 pb converted to bytes for use in size calculations.
pub const PB_524: usize = 589971551185534976;

/// 525PB in bytes.
/// This constant represents 525 pb converted to bytes for use in size calculations.
pub const PB_525: usize = 591097451092377600;

/// 526PB in bytes.
/// This constant represents 526 pb converted to bytes for use in size calculations.
pub const PB_526: usize = 592223350999220224;

/// 527PB in bytes.
/// This constant represents 527 pb converted to bytes for use in size calculations.
pub const PB_527: usize = 593349250906062848;

/// 528PB in bytes.
/// This constant represents 528 pb converted to bytes for use in size calculations.
pub const PB_528: usize = 594475150812905472;

/// 529PB in bytes.
/// This constant represents 529 pb converted to bytes for use in size calculations.
pub const PB_529: usize = 595601050719748096;

/// 530PB in bytes.
/// This constant represents 530 pb converted to bytes for use in size calculations.
pub const PB_530: usize = 596726950626590720;

/// 531PB in bytes.
/// This constant represents 531 pb converted to bytes for use in size calculations.
pub const PB_531: usize = 597852850533433344;

/// 532PB in bytes.
/// This constant represents 532 pb converted to bytes for use in size calculations.
pub const PB_532: usize = 598978750440275968;

/// 533PB in bytes.
/// This constant represents 533 pb converted to bytes for use in size calculations.
pub const PB_533: usize = 600104650347118592;

/// 534PB in bytes.
/// This constant represents 534 pb converted to bytes for use in size calculations.
pub const PB_534: usize = 601230550253961216;

/// 535PB in bytes.
/// This constant represents 535 pb converted to bytes for use in size calculations.
pub const PB_535: usize = 602356450160803840;

/// 536PB in bytes.
/// This constant represents 536 pb converted to bytes for use in size calculations.
pub const PB_536: usize = 603482350067646464;

/// 537PB in bytes.
/// This constant represents 537 pb converted to bytes for use in size calculations.
pub const PB_537: usize = 604608249974489088;

/// 538PB in bytes.
/// This constant represents 538 pb converted to bytes for use in size calculations.
pub const PB_538: usize = 605734149881331712;

/// 539PB in bytes.
/// This constant represents 539 pb converted to bytes for use in size calculations.
pub const PB_539: usize = 606860049788174336;

/// 540PB in bytes.
/// This constant represents 540 pb converted to bytes for use in size calculations.
pub const PB_540: usize = 607985949695016960;

/// 541PB in bytes.
/// This constant represents 541 pb converted to bytes for use in size calculations.
pub const PB_541: usize = 609111849601859584;

/// 542PB in bytes.
/// This constant represents 542 pb converted to bytes for use in size calculations.
pub const PB_542: usize = 610237749508702208;

/// 543PB in bytes.
/// This constant represents 543 pb converted to bytes for use in size calculations.
pub const PB_543: usize = 611363649415544832;

/// 544PB in bytes.
/// This constant represents 544 pb converted to bytes for use in size calculations.
pub const PB_544: usize = 612489549322387456;

/// 545PB in bytes.
/// This constant represents 545 pb converted to bytes for use in size calculations.
pub const PB_545: usize = 613615449229230080;

/// 546PB in bytes.
/// This constant represents 546 pb converted to bytes for use in size calculations.
pub const PB_546: usize = 614741349136072704;

/// 547PB in bytes.
/// This constant represents 547 pb converted to bytes for use in size calculations.
pub const PB_547: usize = 615867249042915328;

/// 548PB in bytes.
/// This constant represents 548 pb converted to bytes for use in size calculations.
pub const PB_548: usize = 616993148949757952;

/// 549PB in bytes.
/// This constant represents 549 pb converted to bytes for use in size calculations.
pub const PB_549: usize = 618119048856600576;

/// 550PB in bytes.
/// This constant represents 550 pb converted to bytes for use in size calculations.
pub const PB_550: usize = 619244948763443200;

/// 551PB in bytes.
/// This constant represents 551 pb converted to bytes for use in size calculations.
pub const PB_551: usize = 620370848670285824;

/// 552PB in bytes.
/// This constant represents 552 pb converted to bytes for use in size calculations.
pub const PB_552: usize = 621496748577128448;

/// 553PB in bytes.
/// This constant represents 553 pb converted to bytes for use in size calculations.
pub const PB_553: usize = 622622648483971072;

/// 554PB in bytes.
/// This constant represents 554 pb converted to bytes for use in size calculations.
pub const PB_554: usize = 623748548390813696;

/// 555PB in bytes.
/// This constant represents 555 pb converted to bytes for use in size calculations.
pub const PB_555: usize = 624874448297656320;

/// 556PB in bytes.
/// This constant represents 556 pb converted to bytes for use in size calculations.
pub const PB_556: usize = 626000348204498944;

/// 557PB in bytes.
/// This constant represents 557 pb converted to bytes for use in size calculations.
pub const PB_557: usize = 627126248111341568;

/// 558PB in bytes.
/// This constant represents 558 pb converted to bytes for use in size calculations.
pub const PB_558: usize = 628252148018184192;

/// 559PB in bytes.
/// This constant represents 559 pb converted to bytes for use in size calculations.
pub const PB_559: usize = 629378047925026816;

/// 560PB in bytes.
/// This constant represents 560 pb converted to bytes for use in size calculations.
pub const PB_560: usize = 630503947831869440;

/// 561PB in bytes.
/// This constant represents 561 pb converted to bytes for use in size calculations.
pub const PB_561: usize = 631629847738712064;

/// 562PB in bytes.
/// This constant represents 562 pb converted to bytes for use in size calculations.
pub const PB_562: usize = 632755747645554688;

/// 563PB in bytes.
/// This constant represents 563 pb converted to bytes for use in size calculations.
pub const PB_563: usize = 633881647552397312;

/// 564PB in bytes.
/// This constant represents 564 pb converted to bytes for use in size calculations.
pub const PB_564: usize = 635007547459239936;

/// 565PB in bytes.
/// This constant represents 565 pb converted to bytes for use in size calculations.
pub const PB_565: usize = 636133447366082560;

/// 566PB in bytes.
/// This constant represents 566 pb converted to bytes for use in size calculations.
pub const PB_566: usize = 637259347272925184;

/// 567PB in bytes.
/// This constant represents 567 pb converted to bytes for use in size calculations.
pub const PB_567: usize = 638385247179767808;

/// 568PB in bytes.
/// This constant represents 568 pb converted to bytes for use in size calculations.
pub const PB_568: usize = 639511147086610432;

/// 569PB in bytes.
/// This constant represents 569 pb converted to bytes for use in size calculations.
pub const PB_569: usize = 640637046993453056;

/// 570PB in bytes.
/// This constant represents 570 pb converted to bytes for use in size calculations.
pub const PB_570: usize = 641762946900295680;

/// 571PB in bytes.
/// This constant represents 571 pb converted to bytes for use in size calculations.
pub const PB_571: usize = 642888846807138304;

/// 572PB in bytes.
/// This constant represents 572 pb converted to bytes for use in size calculations.
pub const PB_572: usize = 644014746713980928;

/// 573PB in bytes.
/// This constant represents 573 pb converted to bytes for use in size calculations.
pub const PB_573: usize = 645140646620823552;

/// 574PB in bytes.
/// This constant represents 574 pb converted to bytes for use in size calculations.
pub const PB_574: usize = 646266546527666176;

/// 575PB in bytes.
/// This constant represents 575 pb converted to bytes for use in size calculations.
pub const PB_575: usize = 647392446434508800;

/// 576PB in bytes.
/// This constant represents 576 pb converted to bytes for use in size calculations.
pub const PB_576: usize = 648518346341351424;

/// 577PB in bytes.
/// This constant represents 577 pb converted to bytes for use in size calculations.
pub const PB_577: usize = 649644246248194048;

/// 578PB in bytes.
/// This constant represents 578 pb converted to bytes for use in size calculations.
pub const PB_578: usize = 650770146155036672;

/// 579PB in bytes.
/// This constant represents 579 pb converted to bytes for use in size calculations.
pub const PB_579: usize = 651896046061879296;

/// 580PB in bytes.
/// This constant represents 580 pb converted to bytes for use in size calculations.
pub const PB_580: usize = 653021945968721920;

/// 581PB in bytes.
/// This constant represents 581 pb converted to bytes for use in size calculations.
pub const PB_581: usize = 654147845875564544;

/// 582PB in bytes.
/// This constant represents 582 pb converted to bytes for use in size calculations.
pub const PB_582: usize = 655273745782407168;

/// 583PB in bytes.
/// This constant represents 583 pb converted to bytes for use in size calculations.
pub const PB_583: usize = 656399645689249792;

/// 584PB in bytes.
/// This constant represents 584 pb converted to bytes for use in size calculations.
pub const PB_584: usize = 657525545596092416;

/// 585PB in bytes.
/// This constant represents 585 pb converted to bytes for use in size calculations.
pub const PB_585: usize = 658651445502935040;

/// 586PB in bytes.
/// This constant represents 586 pb converted to bytes for use in size calculations.
pub const PB_586: usize = 659777345409777664;

/// 587PB in bytes.
/// This constant represents 587 pb converted to bytes for use in size calculations.
pub const PB_587: usize = 660903245316620288;

/// 588PB in bytes.
/// This constant represents 588 pb converted to bytes for use in size calculations.
pub const PB_588: usize = 662029145223462912;

/// 589PB in bytes.
/// This constant represents 589 pb converted to bytes for use in size calculations.
pub const PB_589: usize = 663155045130305536;

/// 590PB in bytes.
/// This constant represents 590 pb converted to bytes for use in size calculations.
pub const PB_590: usize = 664280945037148160;

/// 591PB in bytes.
/// This constant represents 591 pb converted to bytes for use in size calculations.
pub const PB_591: usize = 665406844943990784;

/// 592PB in bytes.
/// This constant represents 592 pb converted to bytes for use in size calculations.
pub const PB_592: usize = 666532744850833408;

/// 593PB in bytes.
/// This constant represents 593 pb converted to bytes for use in size calculations.
pub const PB_593: usize = 667658644757676032;

/// 594PB in bytes.
/// This constant represents 594 pb converted to bytes for use in size calculations.
pub const PB_594: usize = 668784544664518656;

/// 595PB in bytes.
/// This constant represents 595 pb converted to bytes for use in size calculations.
pub const PB_595: usize = 669910444571361280;

/// 596PB in bytes.
/// This constant represents 596 pb converted to bytes for use in size calculations.
pub const PB_596: usize = 671036344478203904;

/// 597PB in bytes.
/// This constant represents 597 pb converted to bytes for use in size calculations.
pub const PB_597: usize = 672162244385046528;

/// 598PB in bytes.
/// This constant represents 598 pb converted to bytes for use in size calculations.
pub const PB_598: usize = 673288144291889152;

/// 599PB in bytes.
/// This constant represents 599 pb converted to bytes for use in size calculations.
pub const PB_599: usize = 674414044198731776;

/// 600PB in bytes.
/// This constant represents 600 pb converted to bytes for use in size calculations.
pub const PB_600: usize = 675539944105574400;

/// 601PB in bytes.
/// This constant represents 601 pb converted to bytes for use in size calculations.
pub const PB_601: usize = 676665844012417024;

/// 602PB in bytes.
/// This constant represents 602 pb converted to bytes for use in size calculations.
pub const PB_602: usize = 677791743919259648;

/// 603PB in bytes.
/// This constant represents 603 pb converted to bytes for use in size calculations.
pub const PB_603: usize = 678917643826102272;

/// 604PB in bytes.
/// This constant represents 604 pb converted to bytes for use in size calculations.
pub const PB_604: usize = 680043543732944896;

/// 605PB in bytes.
/// This constant represents 605 pb converted to bytes for use in size calculations.
pub const PB_605: usize = 681169443639787520;

/// 606PB in bytes.
/// This constant represents 606 pb converted to bytes for use in size calculations.
pub const PB_606: usize = 682295343546630144;

/// 607PB in bytes.
/// This constant represents 607 pb converted to bytes for use in size calculations.
pub const PB_607: usize = 683421243453472768;

/// 608PB in bytes.
/// This constant represents 608 pb converted to bytes for use in size calculations.
pub const PB_608: usize = 684547143360315392;

/// 609PB in bytes.
/// This constant represents 609 pb converted to bytes for use in size calculations.
pub const PB_609: usize = 685673043267158016;

/// 610PB in bytes.
/// This constant represents 610 pb converted to bytes for use in size calculations.
pub const PB_610: usize = 686798943174000640;

/// 611PB in bytes.
/// This constant represents 611 pb converted to bytes for use in size calculations.
pub const PB_611: usize = 687924843080843264;

/// 612PB in bytes.
/// This constant represents 612 pb converted to bytes for use in size calculations.
pub const PB_612: usize = 689050742987685888;

/// 613PB in bytes.
/// This constant represents 613 pb converted to bytes for use in size calculations.
pub const PB_613: usize = 690176642894528512;

/// 614PB in bytes.
/// This constant represents 614 pb converted to bytes for use in size calculations.
pub const PB_614: usize = 691302542801371136;

/// 615PB in bytes.
/// This constant represents 615 pb converted to bytes for use in size calculations.
pub const PB_615: usize = 692428442708213760;

/// 616PB in bytes.
/// This constant represents 616 pb converted to bytes for use in size calculations.
pub const PB_616: usize = 693554342615056384;

/// 617PB in bytes.
/// This constant represents 617 pb converted to bytes for use in size calculations.
pub const PB_617: usize = 694680242521899008;

/// 618PB in bytes.
/// This constant represents 618 pb converted to bytes for use in size calculations.
pub const PB_618: usize = 695806142428741632;

/// 619PB in bytes.
/// This constant represents 619 pb converted to bytes for use in size calculations.
pub const PB_619: usize = 696932042335584256;

/// 620PB in bytes.
/// This constant represents 620 pb converted to bytes for use in size calculations.
pub const PB_620: usize = 698057942242426880;

/// 621PB in bytes.
/// This constant represents 621 pb converted to bytes for use in size calculations.
pub const PB_621: usize = 699183842149269504;

/// 622PB in bytes.
/// This constant represents 622 pb converted to bytes for use in size calculations.
pub const PB_622: usize = 700309742056112128;

/// 623PB in bytes.
/// This constant represents 623 pb converted to bytes for use in size calculations.
pub const PB_623: usize = 701435641962954752;

/// 624PB in bytes.
/// This constant represents 624 pb converted to bytes for use in size calculations.
pub const PB_624: usize = 702561541869797376;

/// 625PB in bytes.
/// This constant represents 625 pb converted to bytes for use in size calculations.
pub const PB_625: usize = 703687441776640000;

/// 626PB in bytes.
/// This constant represents 626 pb converted to bytes for use in size calculations.
pub const PB_626: usize = 704813341683482624;

/// 627PB in bytes.
/// This constant represents 627 pb converted to bytes for use in size calculations.
pub const PB_627: usize = 705939241590325248;

/// 628PB in bytes.
/// This constant represents 628 pb converted to bytes for use in size calculations.
pub const PB_628: usize = 707065141497167872;

/// 629PB in bytes.
/// This constant represents 629 pb converted to bytes for use in size calculations.
pub const PB_629: usize = 708191041404010496;

/// 630PB in bytes.
/// This constant represents 630 pb converted to bytes for use in size calculations.
pub const PB_630: usize = 709316941310853120;

/// 631PB in bytes.
/// This constant represents 631 pb converted to bytes for use in size calculations.
pub const PB_631: usize = 710442841217695744;

/// 632PB in bytes.
/// This constant represents 632 pb converted to bytes for use in size calculations.
pub const PB_632: usize = 711568741124538368;

/// 633PB in bytes.
/// This constant represents 633 pb converted to bytes for use in size calculations.
pub const PB_633: usize = 712694641031380992;

/// 634PB in bytes.
/// This constant represents 634 pb converted to bytes for use in size calculations.
pub const PB_634: usize = 713820540938223616;

/// 635PB in bytes.
/// This constant represents 635 pb converted to bytes for use in size calculations.
pub const PB_635: usize = 714946440845066240;

/// 636PB in bytes.
/// This constant represents 636 pb converted to bytes for use in size calculations.
pub const PB_636: usize = 716072340751908864;

/// 637PB in bytes.
/// This constant represents 637 pb converted to bytes for use in size calculations.
pub const PB_637: usize = 717198240658751488;

/// 638PB in bytes.
/// This constant represents 638 pb converted to bytes for use in size calculations.
pub const PB_638: usize = 718324140565594112;

/// 639PB in bytes.
/// This constant represents 639 pb converted to bytes for use in size calculations.
pub const PB_639: usize = 719450040472436736;

/// 640PB in bytes.
/// This constant represents 640 pb converted to bytes for use in size calculations.
pub const PB_640: usize = 720575940379279360;

/// 641PB in bytes.
/// This constant represents 641 pb converted to bytes for use in size calculations.
pub const PB_641: usize = 721701840286121984;

/// 642PB in bytes.
/// This constant represents 642 pb converted to bytes for use in size calculations.
pub const PB_642: usize = 722827740192964608;

/// 643PB in bytes.
/// This constant represents 643 pb converted to bytes for use in size calculations.
pub const PB_643: usize = 723953640099807232;

/// 644PB in bytes.
/// This constant represents 644 pb converted to bytes for use in size calculations.
pub const PB_644: usize = 725079540006649856;

/// 645PB in bytes.
/// This constant represents 645 pb converted to bytes for use in size calculations.
pub const PB_645: usize = 726205439913492480;

/// 646PB in bytes.
/// This constant represents 646 pb converted to bytes for use in size calculations.
pub const PB_646: usize = 727331339820335104;

/// 647PB in bytes.
/// This constant represents 647 pb converted to bytes for use in size calculations.
pub const PB_647: usize = 728457239727177728;

/// 648PB in bytes.
/// This constant represents 648 pb converted to bytes for use in size calculations.
pub const PB_648: usize = 729583139634020352;

/// 649PB in bytes.
/// This constant represents 649 pb converted to bytes for use in size calculations.
pub const PB_649: usize = 730709039540862976;

/// 650PB in bytes.
/// This constant represents 650 pb converted to bytes for use in size calculations.
pub const PB_650: usize = 731834939447705600;

/// 651PB in bytes.
/// This constant represents 651 pb converted to bytes for use in size calculations.
pub const PB_651: usize = 732960839354548224;

/// 652PB in bytes.
/// This constant represents 652 pb converted to bytes for use in size calculations.
pub const PB_652: usize = 734086739261390848;

/// 653PB in bytes.
/// This constant represents 653 pb converted to bytes for use in size calculations.
pub const PB_653: usize = 735212639168233472;

/// 654PB in bytes.
/// This constant represents 654 pb converted to bytes for use in size calculations.
pub const PB_654: usize = 736338539075076096;

/// 655PB in bytes.
/// This constant represents 655 pb converted to bytes for use in size calculations.
pub const PB_655: usize = 737464438981918720;

/// 656PB in bytes.
/// This constant represents 656 pb converted to bytes for use in size calculations.
pub const PB_656: usize = 738590338888761344;

/// 657PB in bytes.
/// This constant represents 657 pb converted to bytes for use in size calculations.
pub const PB_657: usize = 739716238795603968;

/// 658PB in bytes.
/// This constant represents 658 pb converted to bytes for use in size calculations.
pub const PB_658: usize = 740842138702446592;

/// 659PB in bytes.
/// This constant represents 659 pb converted to bytes for use in size calculations.
pub const PB_659: usize = 741968038609289216;

/// 660PB in bytes.
/// This constant represents 660 pb converted to bytes for use in size calculations.
pub const PB_660: usize = 743093938516131840;

/// 661PB in bytes.
/// This constant represents 661 pb converted to bytes for use in size calculations.
pub const PB_661: usize = 744219838422974464;

/// 662PB in bytes.
/// This constant represents 662 pb converted to bytes for use in size calculations.
pub const PB_662: usize = 745345738329817088;

/// 663PB in bytes.
/// This constant represents 663 pb converted to bytes for use in size calculations.
pub const PB_663: usize = 746471638236659712;

/// 664PB in bytes.
/// This constant represents 664 pb converted to bytes for use in size calculations.
pub const PB_664: usize = 747597538143502336;

/// 665PB in bytes.
/// This constant represents 665 pb converted to bytes for use in size calculations.
pub const PB_665: usize = 748723438050344960;

/// 666PB in bytes.
/// This constant represents 666 pb converted to bytes for use in size calculations.
pub const PB_666: usize = 749849337957187584;

/// 667PB in bytes.
/// This constant represents 667 pb converted to bytes for use in size calculations.
pub const PB_667: usize = 750975237864030208;

/// 668PB in bytes.
/// This constant represents 668 pb converted to bytes for use in size calculations.
pub const PB_668: usize = 752101137770872832;

/// 669PB in bytes.
/// This constant represents 669 pb converted to bytes for use in size calculations.
pub const PB_669: usize = 753227037677715456;

/// 670PB in bytes.
/// This constant represents 670 pb converted to bytes for use in size calculations.
pub const PB_670: usize = 754352937584558080;

/// 671PB in bytes.
/// This constant represents 671 pb converted to bytes for use in size calculations.
pub const PB_671: usize = 755478837491400704;

/// 672PB in bytes.
/// This constant represents 672 pb converted to bytes for use in size calculations.
pub const PB_672: usize = 756604737398243328;

/// 673PB in bytes.
/// This constant represents 673 pb converted to bytes for use in size calculations.
pub const PB_673: usize = 757730637305085952;

/// 674PB in bytes.
/// This constant represents 674 pb converted to bytes for use in size calculations.
pub const PB_674: usize = 758856537211928576;

/// 675PB in bytes.
/// This constant represents 675 pb converted to bytes for use in size calculations.
pub const PB_675: usize = 759982437118771200;

/// 676PB in bytes.
/// This constant represents 676 pb converted to bytes for use in size calculations.
pub const PB_676: usize = 761108337025613824;

/// 677PB in bytes.
/// This constant represents 677 pb converted to bytes for use in size calculations.
pub const PB_677: usize = 762234236932456448;

/// 678PB in bytes.
/// This constant represents 678 pb converted to bytes for use in size calculations.
pub const PB_678: usize = 763360136839299072;

/// 679PB in bytes.
/// This constant represents 679 pb converted to bytes for use in size calculations.
pub const PB_679: usize = 764486036746141696;

/// 680PB in bytes.
/// This constant represents 680 pb converted to bytes for use in size calculations.
pub const PB_680: usize = 765611936652984320;

/// 681PB in bytes.
/// This constant represents 681 pb converted to bytes for use in size calculations.
pub const PB_681: usize = 766737836559826944;

/// 682PB in bytes.
/// This constant represents 682 pb converted to bytes for use in size calculations.
pub const PB_682: usize = 767863736466669568;

/// 683PB in bytes.
/// This constant represents 683 pb converted to bytes for use in size calculations.
pub const PB_683: usize = 768989636373512192;

/// 684PB in bytes.
/// This constant represents 684 pb converted to bytes for use in size calculations.
pub const PB_684: usize = 770115536280354816;

/// 685PB in bytes.
/// This constant represents 685 pb converted to bytes for use in size calculations.
pub const PB_685: usize = 771241436187197440;

/// 686PB in bytes.
/// This constant represents 686 pb converted to bytes for use in size calculations.
pub const PB_686: usize = 772367336094040064;

/// 687PB in bytes.
/// This constant represents 687 pb converted to bytes for use in size calculations.
pub const PB_687: usize = 773493236000882688;

/// 688PB in bytes.
/// This constant represents 688 pb converted to bytes for use in size calculations.
pub const PB_688: usize = 774619135907725312;

/// 689PB in bytes.
/// This constant represents 689 pb converted to bytes for use in size calculations.
pub const PB_689: usize = 775745035814567936;

/// 690PB in bytes.
/// This constant represents 690 pb converted to bytes for use in size calculations.
pub const PB_690: usize = 776870935721410560;

/// 691PB in bytes.
/// This constant represents 691 pb converted to bytes for use in size calculations.
pub const PB_691: usize = 777996835628253184;

/// 692PB in bytes.
/// This constant represents 692 pb converted to bytes for use in size calculations.
pub const PB_692: usize = 779122735535095808;

/// 693PB in bytes.
/// This constant represents 693 pb converted to bytes for use in size calculations.
pub const PB_693: usize = 780248635441938432;

/// 694PB in bytes.
/// This constant represents 694 pb converted to bytes for use in size calculations.
pub const PB_694: usize = 781374535348781056;

/// 695PB in bytes.
/// This constant represents 695 pb converted to bytes for use in size calculations.
pub const PB_695: usize = 782500435255623680;

/// 696PB in bytes.
/// This constant represents 696 pb converted to bytes for use in size calculations.
pub const PB_696: usize = 783626335162466304;

/// 697PB in bytes.
/// This constant represents 697 pb converted to bytes for use in size calculations.
pub const PB_697: usize = 784752235069308928;

/// 698PB in bytes.
/// This constant represents 698 pb converted to bytes for use in size calculations.
pub const PB_698: usize = 785878134976151552;

/// 699PB in bytes.
/// This constant represents 699 pb converted to bytes for use in size calculations.
pub const PB_699: usize = 787004034882994176;

/// 700PB in bytes.
/// This constant represents 700 pb converted to bytes for use in size calculations.
pub const PB_700: usize = 788129934789836800;

/// 701PB in bytes.
/// This constant represents 701 pb converted to bytes for use in size calculations.
pub const PB_701: usize = 789255834696679424;

/// 702PB in bytes.
/// This constant represents 702 pb converted to bytes for use in size calculations.
pub const PB_702: usize = 790381734603522048;

/// 703PB in bytes.
/// This constant represents 703 pb converted to bytes for use in size calculations.
pub const PB_703: usize = 791507634510364672;

/// 704PB in bytes.
/// This constant represents 704 pb converted to bytes for use in size calculations.
pub const PB_704: usize = 792633534417207296;

/// 705PB in bytes.
/// This constant represents 705 pb converted to bytes for use in size calculations.
pub const PB_705: usize = 793759434324049920;

/// 706PB in bytes.
/// This constant represents 706 pb converted to bytes for use in size calculations.
pub const PB_706: usize = 794885334230892544;

/// 707PB in bytes.
/// This constant represents 707 pb converted to bytes for use in size calculations.
pub const PB_707: usize = 796011234137735168;

/// 708PB in bytes.
/// This constant represents 708 pb converted to bytes for use in size calculations.
pub const PB_708: usize = 797137134044577792;

/// 709PB in bytes.
/// This constant represents 709 pb converted to bytes for use in size calculations.
pub const PB_709: usize = 798263033951420416;

/// 710PB in bytes.
/// This constant represents 710 pb converted to bytes for use in size calculations.
pub const PB_710: usize = 799388933858263040;

/// 711PB in bytes.
/// This constant represents 711 pb converted to bytes for use in size calculations.
pub const PB_711: usize = 800514833765105664;

/// 712PB in bytes.
/// This constant represents 712 pb converted to bytes for use in size calculations.
pub const PB_712: usize = 801640733671948288;

/// 713PB in bytes.
/// This constant represents 713 pb converted to bytes for use in size calculations.
pub const PB_713: usize = 802766633578790912;

/// 714PB in bytes.
/// This constant represents 714 pb converted to bytes for use in size calculations.
pub const PB_714: usize = 803892533485633536;

/// 715PB in bytes.
/// This constant represents 715 pb converted to bytes for use in size calculations.
pub const PB_715: usize = 805018433392476160;

/// 716PB in bytes.
/// This constant represents 716 pb converted to bytes for use in size calculations.
pub const PB_716: usize = 806144333299318784;

/// 717PB in bytes.
/// This constant represents 717 pb converted to bytes for use in size calculations.
pub const PB_717: usize = 807270233206161408;

/// 718PB in bytes.
/// This constant represents 718 pb converted to bytes for use in size calculations.
pub const PB_718: usize = 808396133113004032;

/// 719PB in bytes.
/// This constant represents 719 pb converted to bytes for use in size calculations.
pub const PB_719: usize = 809522033019846656;

/// 720PB in bytes.
/// This constant represents 720 pb converted to bytes for use in size calculations.
pub const PB_720: usize = 810647932926689280;

/// 721PB in bytes.
/// This constant represents 721 pb converted to bytes for use in size calculations.
pub const PB_721: usize = 811773832833531904;

/// 722PB in bytes.
/// This constant represents 722 pb converted to bytes for use in size calculations.
pub const PB_722: usize = 812899732740374528;

/// 723PB in bytes.
/// This constant represents 723 pb converted to bytes for use in size calculations.
pub const PB_723: usize = 814025632647217152;

/// 724PB in bytes.
/// This constant represents 724 pb converted to bytes for use in size calculations.
pub const PB_724: usize = 815151532554059776;

/// 725PB in bytes.
/// This constant represents 725 pb converted to bytes for use in size calculations.
pub const PB_725: usize = 816277432460902400;

/// 726PB in bytes.
/// This constant represents 726 pb converted to bytes for use in size calculations.
pub const PB_726: usize = 817403332367745024;

/// 727PB in bytes.
/// This constant represents 727 pb converted to bytes for use in size calculations.
pub const PB_727: usize = 818529232274587648;

/// 728PB in bytes.
/// This constant represents 728 pb converted to bytes for use in size calculations.
pub const PB_728: usize = 819655132181430272;

/// 729PB in bytes.
/// This constant represents 729 pb converted to bytes for use in size calculations.
pub const PB_729: usize = 820781032088272896;

/// 730PB in bytes.
/// This constant represents 730 pb converted to bytes for use in size calculations.
pub const PB_730: usize = 821906931995115520;

/// 731PB in bytes.
/// This constant represents 731 pb converted to bytes for use in size calculations.
pub const PB_731: usize = 823032831901958144;

/// 732PB in bytes.
/// This constant represents 732 pb converted to bytes for use in size calculations.
pub const PB_732: usize = 824158731808800768;

/// 733PB in bytes.
/// This constant represents 733 pb converted to bytes for use in size calculations.
pub const PB_733: usize = 825284631715643392;

/// 734PB in bytes.
/// This constant represents 734 pb converted to bytes for use in size calculations.
pub const PB_734: usize = 826410531622486016;

/// 735PB in bytes.
/// This constant represents 735 pb converted to bytes for use in size calculations.
pub const PB_735: usize = 827536431529328640;

/// 736PB in bytes.
/// This constant represents 736 pb converted to bytes for use in size calculations.
pub const PB_736: usize = 828662331436171264;

/// 737PB in bytes.
/// This constant represents 737 pb converted to bytes for use in size calculations.
pub const PB_737: usize = 829788231343013888;

/// 738PB in bytes.
/// This constant represents 738 pb converted to bytes for use in size calculations.
pub const PB_738: usize = 830914131249856512;

/// 739PB in bytes.
/// This constant represents 739 pb converted to bytes for use in size calculations.
pub const PB_739: usize = 832040031156699136;

/// 740PB in bytes.
/// This constant represents 740 pb converted to bytes for use in size calculations.
pub const PB_740: usize = 833165931063541760;

/// 741PB in bytes.
/// This constant represents 741 pb converted to bytes for use in size calculations.
pub const PB_741: usize = 834291830970384384;

/// 742PB in bytes.
/// This constant represents 742 pb converted to bytes for use in size calculations.
pub const PB_742: usize = 835417730877227008;

/// 743PB in bytes.
/// This constant represents 743 pb converted to bytes for use in size calculations.
pub const PB_743: usize = 836543630784069632;

/// 744PB in bytes.
/// This constant represents 744 pb converted to bytes for use in size calculations.
pub const PB_744: usize = 837669530690912256;

/// 745PB in bytes.
/// This constant represents 745 pb converted to bytes for use in size calculations.
pub const PB_745: usize = 838795430597754880;

/// 746PB in bytes.
/// This constant represents 746 pb converted to bytes for use in size calculations.
pub const PB_746: usize = 839921330504597504;

/// 747PB in bytes.
/// This constant represents 747 pb converted to bytes for use in size calculations.
pub const PB_747: usize = 841047230411440128;

/// 748PB in bytes.
/// This constant represents 748 pb converted to bytes for use in size calculations.
pub const PB_748: usize = 842173130318282752;

/// 749PB in bytes.
/// This constant represents 749 pb converted to bytes for use in size calculations.
pub const PB_749: usize = 843299030225125376;

/// 750PB in bytes.
/// This constant represents 750 pb converted to bytes for use in size calculations.
pub const PB_750: usize = 844424930131968000;

/// 751PB in bytes.
/// This constant represents 751 pb converted to bytes for use in size calculations.
pub const PB_751: usize = 845550830038810624;

/// 752PB in bytes.
/// This constant represents 752 pb converted to bytes for use in size calculations.
pub const PB_752: usize = 846676729945653248;

/// 753PB in bytes.
/// This constant represents 753 pb converted to bytes for use in size calculations.
pub const PB_753: usize = 847802629852495872;

/// 754PB in bytes.
/// This constant represents 754 pb converted to bytes for use in size calculations.
pub const PB_754: usize = 848928529759338496;

/// 755PB in bytes.
/// This constant represents 755 pb converted to bytes for use in size calculations.
pub const PB_755: usize = 850054429666181120;

/// 756PB in bytes.
/// This constant represents 756 pb converted to bytes for use in size calculations.
pub const PB_756: usize = 851180329573023744;

/// 757PB in bytes.
/// This constant represents 757 pb converted to bytes for use in size calculations.
pub const PB_757: usize = 852306229479866368;

/// 758PB in bytes.
/// This constant represents 758 pb converted to bytes for use in size calculations.
pub const PB_758: usize = 853432129386708992;

/// 759PB in bytes.
/// This constant represents 759 pb converted to bytes for use in size calculations.
pub const PB_759: usize = 854558029293551616;

/// 760PB in bytes.
/// This constant represents 760 pb converted to bytes for use in size calculations.
pub const PB_760: usize = 855683929200394240;

/// 761PB in bytes.
/// This constant represents 761 pb converted to bytes for use in size calculations.
pub const PB_761: usize = 856809829107236864;

/// 762PB in bytes.
/// This constant represents 762 pb converted to bytes for use in size calculations.
pub const PB_762: usize = 857935729014079488;

/// 763PB in bytes.
/// This constant represents 763 pb converted to bytes for use in size calculations.
pub const PB_763: usize = 859061628920922112;

/// 764PB in bytes.
/// This constant represents 764 pb converted to bytes for use in size calculations.
pub const PB_764: usize = 860187528827764736;

/// 765PB in bytes.
/// This constant represents 765 pb converted to bytes for use in size calculations.
pub const PB_765: usize = 861313428734607360;

/// 766PB in bytes.
/// This constant represents 766 pb converted to bytes for use in size calculations.
pub const PB_766: usize = 862439328641449984;

/// 767PB in bytes.
/// This constant represents 767 pb converted to bytes for use in size calculations.
pub const PB_767: usize = 863565228548292608;

/// 768PB in bytes.
/// This constant represents 768 pb converted to bytes for use in size calculations.
pub const PB_768: usize = 864691128455135232;

/// 769PB in bytes.
/// This constant represents 769 pb converted to bytes for use in size calculations.
pub const PB_769: usize = 865817028361977856;

/// 770PB in bytes.
/// This constant represents 770 pb converted to bytes for use in size calculations.
pub const PB_770: usize = 866942928268820480;

/// 771PB in bytes.
/// This constant represents 771 pb converted to bytes for use in size calculations.
pub const PB_771: usize = 868068828175663104;

/// 772PB in bytes.
/// This constant represents 772 pb converted to bytes for use in size calculations.
pub const PB_772: usize = 869194728082505728;

/// 773PB in bytes.
/// This constant represents 773 pb converted to bytes for use in size calculations.
pub const PB_773: usize = 870320627989348352;

/// 774PB in bytes.
/// This constant represents 774 pb converted to bytes for use in size calculations.
pub const PB_774: usize = 871446527896190976;

/// 775PB in bytes.
/// This constant represents 775 pb converted to bytes for use in size calculations.
pub const PB_775: usize = 872572427803033600;

/// 776PB in bytes.
/// This constant represents 776 pb converted to bytes for use in size calculations.
pub const PB_776: usize = 873698327709876224;

/// 777PB in bytes.
/// This constant represents 777 pb converted to bytes for use in size calculations.
pub const PB_777: usize = 874824227616718848;

/// 778PB in bytes.
/// This constant represents 778 pb converted to bytes for use in size calculations.
pub const PB_778: usize = 875950127523561472;

/// 779PB in bytes.
/// This constant represents 779 pb converted to bytes for use in size calculations.
pub const PB_779: usize = 877076027430404096;

/// 780PB in bytes.
/// This constant represents 780 pb converted to bytes for use in size calculations.
pub const PB_780: usize = 878201927337246720;

/// 781PB in bytes.
/// This constant represents 781 pb converted to bytes for use in size calculations.
pub const PB_781: usize = 879327827244089344;

/// 782PB in bytes.
/// This constant represents 782 pb converted to bytes for use in size calculations.
pub const PB_782: usize = 880453727150931968;

/// 783PB in bytes.
/// This constant represents 783 pb converted to bytes for use in size calculations.
pub const PB_783: usize = 881579627057774592;

/// 784PB in bytes.
/// This constant represents 784 pb converted to bytes for use in size calculations.
pub const PB_784: usize = 882705526964617216;

/// 785PB in bytes.
/// This constant represents 785 pb converted to bytes for use in size calculations.
pub const PB_785: usize = 883831426871459840;

/// 786PB in bytes.
/// This constant represents 786 pb converted to bytes for use in size calculations.
pub const PB_786: usize = 884957326778302464;

/// 787PB in bytes.
/// This constant represents 787 pb converted to bytes for use in size calculations.
pub const PB_787: usize = 886083226685145088;

/// 788PB in bytes.
/// This constant represents 788 pb converted to bytes for use in size calculations.
pub const PB_788: usize = 887209126591987712;

/// 789PB in bytes.
/// This constant represents 789 pb converted to bytes for use in size calculations.
pub const PB_789: usize = 888335026498830336;

/// 790PB in bytes.
/// This constant represents 790 pb converted to bytes for use in size calculations.
pub const PB_790: usize = 889460926405672960;

/// 791PB in bytes.
/// This constant represents 791 pb converted to bytes for use in size calculations.
pub const PB_791: usize = 890586826312515584;

/// 792PB in bytes.
/// This constant represents 792 pb converted to bytes for use in size calculations.
pub const PB_792: usize = 891712726219358208;

/// 793PB in bytes.
/// This constant represents 793 pb converted to bytes for use in size calculations.
pub const PB_793: usize = 892838626126200832;

/// 794PB in bytes.
/// This constant represents 794 pb converted to bytes for use in size calculations.
pub const PB_794: usize = 893964526033043456;

/// 795PB in bytes.
/// This constant represents 795 pb converted to bytes for use in size calculations.
pub const PB_795: usize = 895090425939886080;

/// 796PB in bytes.
/// This constant represents 796 pb converted to bytes for use in size calculations.
pub const PB_796: usize = 896216325846728704;

/// 797PB in bytes.
/// This constant represents 797 pb converted to bytes for use in size calculations.
pub const PB_797: usize = 897342225753571328;

/// 798PB in bytes.
/// This constant represents 798 pb converted to bytes for use in size calculations.
pub const PB_798: usize = 898468125660413952;

/// 799PB in bytes.
/// This constant represents 799 pb converted to bytes for use in size calculations.
pub const PB_799: usize = 899594025567256576;

/// 800PB in bytes.
/// This constant represents 800 pb converted to bytes for use in size calculations.
pub const PB_800: usize = 900719925474099200;

/// 801PB in bytes.
/// This constant represents 801 pb converted to bytes for use in size calculations.
pub const PB_801: usize = 901845825380941824;

/// 802PB in bytes.
/// This constant represents 802 pb converted to bytes for use in size calculations.
pub const PB_802: usize = 902971725287784448;

/// 803PB in bytes.
/// This constant represents 803 pb converted to bytes for use in size calculations.
pub const PB_803: usize = 904097625194627072;

/// 804PB in bytes.
/// This constant represents 804 pb converted to bytes for use in size calculations.
pub const PB_804: usize = 905223525101469696;

/// 805PB in bytes.
/// This constant represents 805 pb converted to bytes for use in size calculations.
pub const PB_805: usize = 906349425008312320;

/// 806PB in bytes.
/// This constant represents 806 pb converted to bytes for use in size calculations.
pub const PB_806: usize = 907475324915154944;

/// 807PB in bytes.
/// This constant represents 807 pb converted to bytes for use in size calculations.
pub const PB_807: usize = 908601224821997568;

/// 808PB in bytes.
/// This constant represents 808 pb converted to bytes for use in size calculations.
pub const PB_808: usize = 909727124728840192;

/// 809PB in bytes.
/// This constant represents 809 pb converted to bytes for use in size calculations.
pub const PB_809: usize = 910853024635682816;

/// 810PB in bytes.
/// This constant represents 810 pb converted to bytes for use in size calculations.
pub const PB_810: usize = 911978924542525440;

/// 811PB in bytes.
/// This constant represents 811 pb converted to bytes for use in size calculations.
pub const PB_811: usize = 913104824449368064;

/// 812PB in bytes.
/// This constant represents 812 pb converted to bytes for use in size calculations.
pub const PB_812: usize = 914230724356210688;

/// 813PB in bytes.
/// This constant represents 813 pb converted to bytes for use in size calculations.
pub const PB_813: usize = 915356624263053312;

/// 814PB in bytes.
/// This constant represents 814 pb converted to bytes for use in size calculations.
pub const PB_814: usize = 916482524169895936;

/// 815PB in bytes.
/// This constant represents 815 pb converted to bytes for use in size calculations.
pub const PB_815: usize = 917608424076738560;

/// 816PB in bytes.
/// This constant represents 816 pb converted to bytes for use in size calculations.
pub const PB_816: usize = 918734323983581184;

/// 817PB in bytes.
/// This constant represents 817 pb converted to bytes for use in size calculations.
pub const PB_817: usize = 919860223890423808;

/// 818PB in bytes.
/// This constant represents 818 pb converted to bytes for use in size calculations.
pub const PB_818: usize = 920986123797266432;

/// 819PB in bytes.
/// This constant represents 819 pb converted to bytes for use in size calculations.
pub const PB_819: usize = 922112023704109056;

/// 820PB in bytes.
/// This constant represents 820 pb converted to bytes for use in size calculations.
pub const PB_820: usize = 923237923610951680;

/// 821PB in bytes.
/// This constant represents 821 pb converted to bytes for use in size calculations.
pub const PB_821: usize = 924363823517794304;

/// 822PB in bytes.
/// This constant represents 822 pb converted to bytes for use in size calculations.
pub const PB_822: usize = 925489723424636928;

/// 823PB in bytes.
/// This constant represents 823 pb converted to bytes for use in size calculations.
pub const PB_823: usize = 926615623331479552;

/// 824PB in bytes.
/// This constant represents 824 pb converted to bytes for use in size calculations.
pub const PB_824: usize = 927741523238322176;

/// 825PB in bytes.
/// This constant represents 825 pb converted to bytes for use in size calculations.
pub const PB_825: usize = 928867423145164800;

/// 826PB in bytes.
/// This constant represents 826 pb converted to bytes for use in size calculations.
pub const PB_826: usize = 929993323052007424;

/// 827PB in bytes.
/// This constant represents 827 pb converted to bytes for use in size calculations.
pub const PB_827: usize = 931119222958850048;

/// 828PB in bytes.
/// This constant represents 828 pb converted to bytes for use in size calculations.
pub const PB_828: usize = 932245122865692672;

/// 829PB in bytes.
/// This constant represents 829 pb converted to bytes for use in size calculations.
pub const PB_829: usize = 933371022772535296;

/// 830PB in bytes.
/// This constant represents 830 pb converted to bytes for use in size calculations.
pub const PB_830: usize = 934496922679377920;

/// 831PB in bytes.
/// This constant represents 831 pb converted to bytes for use in size calculations.
pub const PB_831: usize = 935622822586220544;

/// 832PB in bytes.
/// This constant represents 832 pb converted to bytes for use in size calculations.
pub const PB_832: usize = 936748722493063168;

/// 833PB in bytes.
/// This constant represents 833 pb converted to bytes for use in size calculations.
pub const PB_833: usize = 937874622399905792;

/// 834PB in bytes.
/// This constant represents 834 pb converted to bytes for use in size calculations.
pub const PB_834: usize = 939000522306748416;

/// 835PB in bytes.
/// This constant represents 835 pb converted to bytes for use in size calculations.
pub const PB_835: usize = 940126422213591040;

/// 836PB in bytes.
/// This constant represents 836 pb converted to bytes for use in size calculations.
pub const PB_836: usize = 941252322120433664;

/// 837PB in bytes.
/// This constant represents 837 pb converted to bytes for use in size calculations.
pub const PB_837: usize = 942378222027276288;

/// 838PB in bytes.
/// This constant represents 838 pb converted to bytes for use in size calculations.
pub const PB_838: usize = 943504121934118912;

/// 839PB in bytes.
/// This constant represents 839 pb converted to bytes for use in size calculations.
pub const PB_839: usize = 944630021840961536;

/// 840PB in bytes.
/// This constant represents 840 pb converted to bytes for use in size calculations.
pub const PB_840: usize = 945755921747804160;

/// 841PB in bytes.
/// This constant represents 841 pb converted to bytes for use in size calculations.
pub const PB_841: usize = 946881821654646784;

/// 842PB in bytes.
/// This constant represents 842 pb converted to bytes for use in size calculations.
pub const PB_842: usize = 948007721561489408;

/// 843PB in bytes.
/// This constant represents 843 pb converted to bytes for use in size calculations.
pub const PB_843: usize = 949133621468332032;

/// 844PB in bytes.
/// This constant represents 844 pb converted to bytes for use in size calculations.
pub const PB_844: usize = 950259521375174656;

/// 845PB in bytes.
/// This constant represents 845 pb converted to bytes for use in size calculations.
pub const PB_845: usize = 951385421282017280;

/// 846PB in bytes.
/// This constant represents 846 pb converted to bytes for use in size calculations.
pub const PB_846: usize = 952511321188859904;

/// 847PB in bytes.
/// This constant represents 847 pb converted to bytes for use in size calculations.
pub const PB_847: usize = 953637221095702528;

/// 848PB in bytes.
/// This constant represents 848 pb converted to bytes for use in size calculations.
pub const PB_848: usize = 954763121002545152;

/// 849PB in bytes.
/// This constant represents 849 pb converted to bytes for use in size calculations.
pub const PB_849: usize = 955889020909387776;

/// 850PB in bytes.
/// This constant represents 850 pb converted to bytes for use in size calculations.
pub const PB_850: usize = 957014920816230400;

/// 851PB in bytes.
/// This constant represents 851 pb converted to bytes for use in size calculations.
pub const PB_851: usize = 958140820723073024;

/// 852PB in bytes.
/// This constant represents 852 pb converted to bytes for use in size calculations.
pub const PB_852: usize = 959266720629915648;

/// 853PB in bytes.
/// This constant represents 853 pb converted to bytes for use in size calculations.
pub const PB_853: usize = 960392620536758272;

/// 854PB in bytes.
/// This constant represents 854 pb converted to bytes for use in size calculations.
pub const PB_854: usize = 961518520443600896;

/// 855PB in bytes.
/// This constant represents 855 pb converted to bytes for use in size calculations.
pub const PB_855: usize = 962644420350443520;

/// 856PB in bytes.
/// This constant represents 856 pb converted to bytes for use in size calculations.
pub const PB_856: usize = 963770320257286144;

/// 857PB in bytes.
/// This constant represents 857 pb converted to bytes for use in size calculations.
pub const PB_857: usize = 964896220164128768;

/// 858PB in bytes.
/// This constant represents 858 pb converted to bytes for use in size calculations.
pub const PB_858: usize = 966022120070971392;

/// 859PB in bytes.
/// This constant represents 859 pb converted to bytes for use in size calculations.
pub const PB_859: usize = 967148019977814016;

/// 860PB in bytes.
/// This constant represents 860 pb converted to bytes for use in size calculations.
pub const PB_860: usize = 968273919884656640;

/// 861PB in bytes.
/// This constant represents 861 pb converted to bytes for use in size calculations.
pub const PB_861: usize = 969399819791499264;

/// 862PB in bytes.
/// This constant represents 862 pb converted to bytes for use in size calculations.
pub const PB_862: usize = 970525719698341888;

/// 863PB in bytes.
/// This constant represents 863 pb converted to bytes for use in size calculations.
pub const PB_863: usize = 971651619605184512;

/// 864PB in bytes.
/// This constant represents 864 pb converted to bytes for use in size calculations.
pub const PB_864: usize = 972777519512027136;

/// 865PB in bytes.
/// This constant represents 865 pb converted to bytes for use in size calculations.
pub const PB_865: usize = 973903419418869760;

/// 866PB in bytes.
/// This constant represents 866 pb converted to bytes for use in size calculations.
pub const PB_866: usize = 975029319325712384;

/// 867PB in bytes.
/// This constant represents 867 pb converted to bytes for use in size calculations.
pub const PB_867: usize = 976155219232555008;

/// 868PB in bytes.
/// This constant represents 868 pb converted to bytes for use in size calculations.
pub const PB_868: usize = 977281119139397632;

/// 869PB in bytes.
/// This constant represents 869 pb converted to bytes for use in size calculations.
pub const PB_869: usize = 978407019046240256;

/// 870PB in bytes.
/// This constant represents 870 pb converted to bytes for use in size calculations.
pub const PB_870: usize = 979532918953082880;

/// 871PB in bytes.
/// This constant represents 871 pb converted to bytes for use in size calculations.
pub const PB_871: usize = 980658818859925504;

/// 872PB in bytes.
/// This constant represents 872 pb converted to bytes for use in size calculations.
pub const PB_872: usize = 981784718766768128;

/// 873PB in bytes.
/// This constant represents 873 pb converted to bytes for use in size calculations.
pub const PB_873: usize = 982910618673610752;

/// 874PB in bytes.
/// This constant represents 874 pb converted to bytes for use in size calculations.
pub const PB_874: usize = 984036518580453376;

/// 875PB in bytes.
/// This constant represents 875 pb converted to bytes for use in size calculations.
pub const PB_875: usize = 985162418487296000;

/// 876PB in bytes.
/// This constant represents 876 pb converted to bytes for use in size calculations.
pub const PB_876: usize = 986288318394138624;

/// 877PB in bytes.
/// This constant represents 877 pb converted to bytes for use in size calculations.
pub const PB_877: usize = 987414218300981248;

/// 878PB in bytes.
/// This constant represents 878 pb converted to bytes for use in size calculations.
pub const PB_878: usize = 988540118207823872;

/// 879PB in bytes.
/// This constant represents 879 pb converted to bytes for use in size calculations.
pub const PB_879: usize = 989666018114666496;

/// 880PB in bytes.
/// This constant represents 880 pb converted to bytes for use in size calculations.
pub const PB_880: usize = 990791918021509120;

/// 881PB in bytes.
/// This constant represents 881 pb converted to bytes for use in size calculations.
pub const PB_881: usize = 991917817928351744;

/// 882PB in bytes.
/// This constant represents 882 pb converted to bytes for use in size calculations.
pub const PB_882: usize = 993043717835194368;

/// 883PB in bytes.
/// This constant represents 883 pb converted to bytes for use in size calculations.
pub const PB_883: usize = 994169617742036992;

/// 884PB in bytes.
/// This constant represents 884 pb converted to bytes for use in size calculations.
pub const PB_884: usize = 995295517648879616;

/// 885PB in bytes.
/// This constant represents 885 pb converted to bytes for use in size calculations.
pub const PB_885: usize = 996421417555722240;

/// 886PB in bytes.
/// This constant represents 886 pb converted to bytes for use in size calculations.
pub const PB_886: usize = 997547317462564864;

/// 887PB in bytes.
/// This constant represents 887 pb converted to bytes for use in size calculations.
pub const PB_887: usize = 998673217369407488;

/// 888PB in bytes.
/// This constant represents 888 pb converted to bytes for use in size calculations.
pub const PB_888: usize = 999799117276250112;

/// 889PB in bytes.
/// This constant represents 889 pb converted to bytes for use in size calculations.
pub const PB_889: usize = 1000925017183092736;

/// 890PB in bytes.
/// This constant represents 890 pb converted to bytes for use in size calculations.
pub const PB_890: usize = 1002050917089935360;

/// 891PB in bytes.
/// This constant represents 891 pb converted to bytes for use in size calculations.
pub const PB_891: usize = 1003176816996777984;

/// 892PB in bytes.
/// This constant represents 892 pb converted to bytes for use in size calculations.
pub const PB_892: usize = 1004302716903620608;

/// 893PB in bytes.
/// This constant represents 893 pb converted to bytes for use in size calculations.
pub const PB_893: usize = 1005428616810463232;

/// 894PB in bytes.
/// This constant represents 894 pb converted to bytes for use in size calculations.
pub const PB_894: usize = 1006554516717305856;

/// 895PB in bytes.
/// This constant represents 895 pb converted to bytes for use in size calculations.
pub const PB_895: usize = 1007680416624148480;

/// 896PB in bytes.
/// This constant represents 896 pb converted to bytes for use in size calculations.
pub const PB_896: usize = 1008806316530991104;

/// 897PB in bytes.
/// This constant represents 897 pb converted to bytes for use in size calculations.
pub const PB_897: usize = 1009932216437833728;

/// 898PB in bytes.
/// This constant represents 898 pb converted to bytes for use in size calculations.
pub const PB_898: usize = 1011058116344676352;

/// 899PB in bytes.
/// This constant represents 899 pb converted to bytes for use in size calculations.
pub const PB_899: usize = 1012184016251518976;

/// 900PB in bytes.
/// This constant represents 900 pb converted to bytes for use in size calculations.
pub const PB_900: usize = 1013309916158361600;

/// 901PB in bytes.
/// This constant represents 901 pb converted to bytes for use in size calculations.
pub const PB_901: usize = 1014435816065204224;

/// 902PB in bytes.
/// This constant represents 902 pb converted to bytes for use in size calculations.
pub const PB_902: usize = 1015561715972046848;

/// 903PB in bytes.
/// This constant represents 903 pb converted to bytes for use in size calculations.
pub const PB_903: usize = 1016687615878889472;

/// 904PB in bytes.
/// This constant represents 904 pb converted to bytes for use in size calculations.
pub const PB_904: usize = 1017813515785732096;

/// 905PB in bytes.
/// This constant represents 905 pb converted to bytes for use in size calculations.
pub const PB_905: usize = 1018939415692574720;

/// 906PB in bytes.
/// This constant represents 906 pb converted to bytes for use in size calculations.
pub const PB_906: usize = 1020065315599417344;

/// 907PB in bytes.
/// This constant represents 907 pb converted to bytes for use in size calculations.
pub const PB_907: usize = 1021191215506259968;

/// 908PB in bytes.
/// This constant represents 908 pb converted to bytes for use in size calculations.
pub const PB_908: usize = 1022317115413102592;

/// 909PB in bytes.
/// This constant represents 909 pb converted to bytes for use in size calculations.
pub const PB_909: usize = 1023443015319945216;

/// 910PB in bytes.
/// This constant represents 910 pb converted to bytes for use in size calculations.
pub const PB_910: usize = 1024568915226787840;

/// 911PB in bytes.
/// This constant represents 911 pb converted to bytes for use in size calculations.
pub const PB_911: usize = 1025694815133630464;

/// 912PB in bytes.
/// This constant represents 912 pb converted to bytes for use in size calculations.
pub const PB_912: usize = 1026820715040473088;

/// 913PB in bytes.
/// This constant represents 913 pb converted to bytes for use in size calculations.
pub const PB_913: usize = 1027946614947315712;

/// 914PB in bytes.
/// This constant represents 914 pb converted to bytes for use in size calculations.
pub const PB_914: usize = 1029072514854158336;

/// 915PB in bytes.
/// This constant represents 915 pb converted to bytes for use in size calculations.
pub const PB_915: usize = 1030198414761000960;

/// 916PB in bytes.
/// This constant represents 916 pb converted to bytes for use in size calculations.
pub const PB_916: usize = 1031324314667843584;

/// 917PB in bytes.
/// This constant represents 917 pb converted to bytes for use in size calculations.
pub const PB_917: usize = 1032450214574686208;

/// 918PB in bytes.
/// This constant represents 918 pb converted to bytes for use in size calculations.
pub const PB_918: usize = 1033576114481528832;

/// 919PB in bytes.
/// This constant represents 919 pb converted to bytes for use in size calculations.
pub const PB_919: usize = 1034702014388371456;

/// 920PB in bytes.
/// This constant represents 920 pb converted to bytes for use in size calculations.
pub const PB_920: usize = 1035827914295214080;

/// 921PB in bytes.
/// This constant represents 921 pb converted to bytes for use in size calculations.
pub const PB_921: usize = 1036953814202056704;

/// 922PB in bytes.
/// This constant represents 922 pb converted to bytes for use in size calculations.
pub const PB_922: usize = 1038079714108899328;

/// 923PB in bytes.
/// This constant represents 923 pb converted to bytes for use in size calculations.
pub const PB_923: usize = 1039205614015741952;

/// 924PB in bytes.
/// This constant represents 924 pb converted to bytes for use in size calculations.
pub const PB_924: usize = 1040331513922584576;

/// 925PB in bytes.
/// This constant represents 925 pb converted to bytes for use in size calculations.
pub const PB_925: usize = 1041457413829427200;

/// 926PB in bytes.
/// This constant represents 926 pb converted to bytes for use in size calculations.
pub const PB_926: usize = 1042583313736269824;

/// 927PB in bytes.
/// This constant represents 927 pb converted to bytes for use in size calculations.
pub const PB_927: usize = 1043709213643112448;

/// 928PB in bytes.
/// This constant represents 928 pb converted to bytes for use in size calculations.
pub const PB_928: usize = 1044835113549955072;

/// 929PB in bytes.
/// This constant represents 929 pb converted to bytes for use in size calculations.
pub const PB_929: usize = 1045961013456797696;

/// 930PB in bytes.
/// This constant represents 930 pb converted to bytes for use in size calculations.
pub const PB_930: usize = 1047086913363640320;

/// 931PB in bytes.
/// This constant represents 931 pb converted to bytes for use in size calculations.
pub const PB_931: usize = 1048212813270482944;

/// 932PB in bytes.
/// This constant represents 932 pb converted to bytes for use in size calculations.
pub const PB_932: usize = 1049338713177325568;

/// 933PB in bytes.
/// This constant represents 933 pb converted to bytes for use in size calculations.
pub const PB_933: usize = 1050464613084168192;

/// 934PB in bytes.
/// This constant represents 934 pb converted to bytes for use in size calculations.
pub const PB_934: usize = 1051590512991010816;

/// 935PB in bytes.
/// This constant represents 935 pb converted to bytes for use in size calculations.
pub const PB_935: usize = 1052716412897853440;

/// 936PB in bytes.
/// This constant represents 936 pb converted to bytes for use in size calculations.
pub const PB_936: usize = 1053842312804696064;

/// 937PB in bytes.
/// This constant represents 937 pb converted to bytes for use in size calculations.
pub const PB_937: usize = 1054968212711538688;

/// 938PB in bytes.
/// This constant represents 938 pb converted to bytes for use in size calculations.
pub const PB_938: usize = 1056094112618381312;

/// 939PB in bytes.
/// This constant represents 939 pb converted to bytes for use in size calculations.
pub const PB_939: usize = 1057220012525223936;

/// 940PB in bytes.
/// This constant represents 940 pb converted to bytes for use in size calculations.
pub const PB_940: usize = 1058345912432066560;

/// 941PB in bytes.
/// This constant represents 941 pb converted to bytes for use in size calculations.
pub const PB_941: usize = 1059471812338909184;

/// 942PB in bytes.
/// This constant represents 942 pb converted to bytes for use in size calculations.
pub const PB_942: usize = 1060597712245751808;

/// 943PB in bytes.
/// This constant represents 943 pb converted to bytes for use in size calculations.
pub const PB_943: usize = 1061723612152594432;

/// 944PB in bytes.
/// This constant represents 944 pb converted to bytes for use in size calculations.
pub const PB_944: usize = 1062849512059437056;

/// 945PB in bytes.
/// This constant represents 945 pb converted to bytes for use in size calculations.
pub const PB_945: usize = 1063975411966279680;

/// 946PB in bytes.
/// This constant represents 946 pb converted to bytes for use in size calculations.
pub const PB_946: usize = 1065101311873122304;

/// 947PB in bytes.
/// This constant represents 947 pb converted to bytes for use in size calculations.
pub const PB_947: usize = 1066227211779964928;

/// 948PB in bytes.
/// This constant represents 948 pb converted to bytes for use in size calculations.
pub const PB_948: usize = 1067353111686807552;

/// 949PB in bytes.
/// This constant represents 949 pb converted to bytes for use in size calculations.
pub const PB_949: usize = 1068479011593650176;

/// 950PB in bytes.
/// This constant represents 950 pb converted to bytes for use in size calculations.
pub const PB_950: usize = 1069604911500492800;

/// 951PB in bytes.
/// This constant represents 951 pb converted to bytes for use in size calculations.
pub const PB_951: usize = 1070730811407335424;

/// 952PB in bytes.
/// This constant represents 952 pb converted to bytes for use in size calculations.
pub const PB_952: usize = 1071856711314178048;

/// 953PB in bytes.
/// This constant represents 953 pb converted to bytes for use in size calculations.
pub const PB_953: usize = 1072982611221020672;

/// 954PB in bytes.
/// This constant represents 954 pb converted to bytes for use in size calculations.
pub const PB_954: usize = 1074108511127863296;

/// 955PB in bytes.
/// This constant represents 955 pb converted to bytes for use in size calculations.
pub const PB_955: usize = 1075234411034705920;

/// 956PB in bytes.
/// This constant represents 956 pb converted to bytes for use in size calculations.
pub const PB_956: usize = 1076360310941548544;

/// 957PB in bytes.
/// This constant represents 957 pb converted to bytes for use in size calculations.
pub const PB_957: usize = 1077486210848391168;

/// 958PB in bytes.
/// This constant represents 958 pb converted to bytes for use in size calculations.
pub const PB_958: usize = 1078612110755233792;

/// 959PB in bytes.
/// This constant represents 959 pb converted to bytes for use in size calculations.
pub const PB_959: usize = 1079738010662076416;

/// 960PB in bytes.
/// This constant represents 960 pb converted to bytes for use in size calculations.
pub const PB_960: usize = 1080863910568919040;

/// 961PB in bytes.
/// This constant represents 961 pb converted to bytes for use in size calculations.
pub const PB_961: usize = 1081989810475761664;

/// 962PB in bytes.
/// This constant represents 962 pb converted to bytes for use in size calculations.
pub const PB_962: usize = 1083115710382604288;

/// 963PB in bytes.
/// This constant represents 963 pb converted to bytes for use in size calculations.
pub const PB_963: usize = 1084241610289446912;

/// 964PB in bytes.
/// This constant represents 964 pb converted to bytes for use in size calculations.
pub const PB_964: usize = 1085367510196289536;

/// 965PB in bytes.
/// This constant represents 965 pb converted to bytes for use in size calculations.
pub const PB_965: usize = 1086493410103132160;

/// 966PB in bytes.
/// This constant represents 966 pb converted to bytes for use in size calculations.
pub const PB_966: usize = 1087619310009974784;

/// 967PB in bytes.
/// This constant represents 967 pb converted to bytes for use in size calculations.
pub const PB_967: usize = 1088745209916817408;

/// 968PB in bytes.
/// This constant represents 968 pb converted to bytes for use in size calculations.
pub const PB_968: usize = 1089871109823660032;

/// 969PB in bytes.
/// This constant represents 969 pb converted to bytes for use in size calculations.
pub const PB_969: usize = 1090997009730502656;

/// 970PB in bytes.
/// This constant represents 970 pb converted to bytes for use in size calculations.
pub const PB_970: usize = 1092122909637345280;

/// 971PB in bytes.
/// This constant represents 971 pb converted to bytes for use in size calculations.
pub const PB_971: usize = 1093248809544187904;

/// 972PB in bytes.
/// This constant represents 972 pb converted to bytes for use in size calculations.
pub const PB_972: usize = 1094374709451030528;

/// 973PB in bytes.
/// This constant represents 973 pb converted to bytes for use in size calculations.
pub const PB_973: usize = 1095500609357873152;

/// 974PB in bytes.
/// This constant represents 974 pb converted to bytes for use in size calculations.
pub const PB_974: usize = 1096626509264715776;

/// 975PB in bytes.
/// This constant represents 975 pb converted to bytes for use in size calculations.
pub const PB_975: usize = 1097752409171558400;

/// 976PB in bytes.
/// This constant represents 976 pb converted to bytes for use in size calculations.
pub const PB_976: usize = 1098878309078401024;

/// 977PB in bytes.
/// This constant represents 977 pb converted to bytes for use in size calculations.
pub const PB_977: usize = 1100004208985243648;

/// 978PB in bytes.
/// This constant represents 978 pb converted to bytes for use in size calculations.
pub const PB_978: usize = 1101130108892086272;

/// 979PB in bytes.
/// This constant represents 979 pb converted to bytes for use in size calculations.
pub const PB_979: usize = 1102256008798928896;

/// 980PB in bytes.
/// This constant represents 980 pb converted to bytes for use in size calculations.
pub const PB_980: usize = 1103381908705771520;

/// 981PB in bytes.
/// This constant represents 981 pb converted to bytes for use in size calculations.
pub const PB_981: usize = 1104507808612614144;

/// 982PB in bytes.
/// This constant represents 982 pb converted to bytes for use in size calculations.
pub const PB_982: usize = 1105633708519456768;

/// 983PB in bytes.
/// This constant represents 983 pb converted to bytes for use in size calculations.
pub const PB_983: usize = 1106759608426299392;

/// 984PB in bytes.
/// This constant represents 984 pb converted to bytes for use in size calculations.
pub const PB_984: usize = 1107885508333142016;

/// 985PB in bytes.
/// This constant represents 985 pb converted to bytes for use in size calculations.
pub const PB_985: usize = 1109011408239984640;

/// 986PB in bytes.
/// This constant represents 986 pb converted to bytes for use in size calculations.
pub const PB_986: usize = 1110137308146827264;

/// 987PB in bytes.
/// This constant represents 987 pb converted to bytes for use in size calculations.
pub const PB_987: usize = 1111263208053669888;

/// 988PB in bytes.
/// This constant represents 988 pb converted to bytes for use in size calculations.
pub const PB_988: usize = 1112389107960512512;

/// 989PB in bytes.
/// This constant represents 989 pb converted to bytes for use in size calculations.
pub const PB_989: usize = 1113515007867355136;

/// 990PB in bytes.
/// This constant represents 990 pb converted to bytes for use in size calculations.
pub const PB_990: usize = 1114640907774197760;

/// 991PB in bytes.
/// This constant represents 991 pb converted to bytes for use in size calculations.
pub const PB_991: usize = 1115766807681040384;

/// 992PB in bytes.
/// This constant represents 992 pb converted to bytes for use in size calculations.
pub const PB_992: usize = 1116892707587883008;

/// 993PB in bytes.
/// This constant represents 993 pb converted to bytes for use in size calculations.
pub const PB_993: usize = 1118018607494725632;

/// 994PB in bytes.
/// This constant represents 994 pb converted to bytes for use in size calculations.
pub const PB_994: usize = 1119144507401568256;

/// 995PB in bytes.
/// This constant represents 995 pb converted to bytes for use in size calculations.
pub const PB_995: usize = 1120270407308410880;

/// 996PB in bytes.
/// This constant represents 996 pb converted to bytes for use in size calculations.
pub const PB_996: usize = 1121396307215253504;

/// 997PB in bytes.
/// This constant represents 997 pb converted to bytes for use in size calculations.
pub const PB_997: usize = 1122522207122096128;

/// 998PB in bytes.
/// This constant represents 998 pb converted to bytes for use in size calculations.
pub const PB_998: usize = 1123648107028938752;

/// 999PB in bytes.
/// This constant represents 999 pb converted to bytes for use in size calculations.
pub const PB_999: usize = 1124774006935781376;

/// 1000PB in bytes.
/// This constant represents 1000 pb converted to bytes for use in size calculations.
pub const PB_1000: usize = 1125899906842624000;

/// 1001PB in bytes.
/// This constant represents 1001 pb converted to bytes for use in size calculations.
pub const PB_1001: usize = 1127025806749466624;

/// 1002PB in bytes.
/// This constant represents 1002 pb converted to bytes for use in size calculations.
pub const PB_1002: usize = 1128151706656309248;

/// 1003PB in bytes.
/// This constant represents 1003 pb converted to bytes for use in size calculations.
pub const PB_1003: usize = 1129277606563151872;

/// 1004PB in bytes.
/// This constant represents 1004 pb converted to bytes for use in size calculations.
pub const PB_1004: usize = 1130403506469994496;

/// 1005PB in bytes.
/// This constant represents 1005 pb converted to bytes for use in size calculations.
pub const PB_1005: usize = 1131529406376837120;

/// 1006PB in bytes.
/// This constant represents 1006 pb converted to bytes for use in size calculations.
pub const PB_1006: usize = 1132655306283679744;

/// 1007PB in bytes.
/// This constant represents 1007 pb converted to bytes for use in size calculations.
pub const PB_1007: usize = 1133781206190522368;

/// 1008PB in bytes.
/// This constant represents 1008 pb converted to bytes for use in size calculations.
pub const PB_1008: usize = 1134907106097364992;

/// 1009PB in bytes.
/// This constant represents 1009 pb converted to bytes for use in size calculations.
pub const PB_1009: usize = 1136033006004207616;

/// 1010PB in bytes.
/// This constant represents 1010 pb converted to bytes for use in size calculations.
pub const PB_1010: usize = 1137158905911050240;

/// 1011PB in bytes.
/// This constant represents 1011 pb converted to bytes for use in size calculations.
pub const PB_1011: usize = 1138284805817892864;

/// 1012PB in bytes.
/// This constant represents 1012 pb converted to bytes for use in size calculations.
pub const PB_1012: usize = 1139410705724735488;

/// 1013PB in bytes.
/// This constant represents 1013 pb converted to bytes for use in size calculations.
pub const PB_1013: usize = 1140536605631578112;

/// 1014PB in bytes.
/// This constant represents 1014 pb converted to bytes for use in size calculations.
pub const PB_1014: usize = 1141662505538420736;

/// 1015PB in bytes.
/// This constant represents 1015 pb converted to bytes for use in size calculations.
pub const PB_1015: usize = 1142788405445263360;

/// 1016PB in bytes.
/// This constant represents 1016 pb converted to bytes for use in size calculations.
pub const PB_1016: usize = 1143914305352105984;

/// 1017PB in bytes.
/// This constant represents 1017 pb converted to bytes for use in size calculations.
pub const PB_1017: usize = 1145040205258948608;

/// 1018PB in bytes.
/// This constant represents 1018 pb converted to bytes for use in size calculations.
pub const PB_1018: usize = 1146166105165791232;

/// 1019PB in bytes.
/// This constant represents 1019 pb converted to bytes for use in size calculations.
pub const PB_1019: usize = 1147292005072633856;

/// 1020PB in bytes.
/// This constant represents 1020 pb converted to bytes for use in size calculations.
pub const PB_1020: usize = 1148417904979476480;

/// 1021PB in bytes.
/// This constant represents 1021 pb converted to bytes for use in size calculations.
pub const PB_1021: usize = 1149543804886319104;

/// 1022PB in bytes.
/// This constant represents 1022 pb converted to bytes for use in size calculations.
pub const PB_1022: usize = 1150669704793161728;

/// 1023PB in bytes.
/// This constant represents 1023 pb converted to bytes for use in size calculations.
pub const PB_1023: usize = 1151795604700004352;

/// 1024PB in bytes.
/// This constant represents 1024 pb converted to bytes for use in size calculations.
pub const PB_1024: usize = 1152921504606846976;
