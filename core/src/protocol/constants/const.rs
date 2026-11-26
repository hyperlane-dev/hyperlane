/// Byte delimiting a quoted HTTP field member.
pub(crate) const HTTP_QUOTED_FIELD_QUOTE: u8 = b'"';

/// Byte escaping the next character inside a quoted HTTP field member.
pub(crate) const HTTP_QUOTED_FIELD_ESCAPE: u8 = b'\\';

/// Pseudo-header prefix used by HTTP/2 and HTTP/3 header maps.
pub(crate) const PSEUDO_HEADER_PREFIX: &str = ":";

/// The character separating the path from the query in a request target.
pub(crate) const QUESTION_MARK: char = '?';

/// The header naming the trailers section of a message.
pub(crate) const TRAILER_HEADER: &str = "trailer";

/// URI scheme used for cleartext HTTP.
pub(crate) const HTTP_SCHEME: &str = "http";

/// URI scheme used for TLS-protected HTTP.
pub(crate) const HTTPS_SCHEME: &str = "https";

/// Maximum size of a serialized response head accepted from the pipeline.
pub(crate) const RESPONSE_HEAD_MAX: usize = 16 * 1024;

/// The reason returned when a response head exceeds its bound.
pub(crate) const RESPONSE_HEAD_TOO_LONG: &str = "response head exceeds its bound";

/// The reason returned when the pipeline closed without writing a head.
pub(crate) const MISSING_RESPONSE_HEAD: &str = "response ended before a head was written";

/// The reason returned when a head announces a transfer coding the framing pump
/// cannot reproduce.
///
/// HTTP/2 and HTTP/3 do not carry a chunked body: the data frames are the
/// framing. Forwarding the chunk size lines as body bytes would hand the client
/// a body it must not have to decode, so the head is refused before any frame is
/// emitted and the stream can be reset with a real error.
pub(crate) const UNSUPPORTED_TRANSFER_ENCODING: &str =
    "response transfer-encoding is not supported on this protocol";

/// Static `text/html; charset=utf-8` media type for zero-allocation protocol header reuse.
pub(crate) const STATIC_CT_HTML_UTF8: &str = "text/html; charset=utf-8";

/// Static `text/plain` media type for zero-allocation protocol header reuse.
pub(crate) const STATIC_CT_PLAIN: &str = "text/plain";

/// Static `text/plain; charset=utf-8` media type for zero-allocation protocol header reuse.
pub(crate) const STATIC_CT_PLAIN_UTF8: &str = "text/plain; charset=utf-8";

/// Static `application/json` media type for zero-allocation protocol header reuse.
pub(crate) const STATIC_CT_JSON: &str = "application/json";

/// The reason returned when a status line carries no code the protocol allows.
pub(crate) const INVALID_RESPONSE_STATUS: &str = "response head has an unusable status code";

/// The reason returned when a header line is not `name: value`.
pub(crate) const MALFORMED_RESPONSE_HEADER: &str = "response head has a malformed header line";

/// The reason returned when the body is shorter than the announced length.
pub(crate) const TRUNCATED_RESPONSE_BODY: &str =
    "response ended before the announced content length arrived";

/// The reason returned when the body runs past the announced length.
pub(crate) const OVERFLOWING_RESPONSE_BODY: &str =
    "response carried more bytes than the announced content length";

/// The reason returned when a send stream is gone before capacity is granted.
pub(crate) const MISSING_SEND_CAPACITY: &str =
    "the send stream was closed before capacity was granted";

/// The smallest status code a protocol can carry.
pub(crate) const MIN_STATUS_CODE: usize = 100;

/// The largest status code a protocol can carry.
pub(crate) const MAX_STATUS_CODE: usize = 999;

/// Pre-encoded HTTP/1.1 200 OK default response head prefix up to the content length value.
pub(crate) const FAST_PROTOCOL_H11_200_PREFIX: &[u8] = b"HTTP/1.1 200 OK\r\ncontent-length:";

/// Pre-encoded HTTP/2 200 OK default response head prefix up to the content length value.
pub(crate) const FAST_PROTOCOL_H2_200_PREFIX: &[u8] = b"HTTP/2 200 OK\r\ncontent-length:";

/// Pre-encoded HTTP/3 200 OK default response head prefix up to the content length value.
pub(crate) const FAST_PROTOCOL_H3_200_PREFIX: &[u8] = b"HTTP/3 200 OK\r\ncontent-length:";

/// Pre-encoded default response head suffix emitted by `Response::build`.
pub(crate) const FAST_PROTOCOL_DEFAULT_SUFFIX: &[u8] =
    b"\r\nconnection:keep-alive\r\ncontent-type:text/html; charset=utf-8\r\n\r\n";

/// Static decimal strings for small content lengths (`0..=256`) avoiding `HeaderValue` heap allocation.
pub(crate) const STATIC_CONTENT_LENGTHS: [&str; 257] = [
    "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "10", "11", "12", "13", "14", "15", "16",
    "17", "18", "19", "20", "21", "22", "23", "24", "25", "26", "27", "28", "29", "30", "31", "32",
    "33", "34", "35", "36", "37", "38", "39", "40", "41", "42", "43", "44", "45", "46", "47", "48",
    "49", "50", "51", "52", "53", "54", "55", "56", "57", "58", "59", "60", "61", "62", "63", "64",
    "65", "66", "67", "68", "69", "70", "71", "72", "73", "74", "75", "76", "77", "78", "79", "80",
    "81", "82", "83", "84", "85", "86", "87", "88", "89", "90", "91", "92", "93", "94", "95", "96",
    "97", "98", "99", "100", "101", "102", "103", "104", "105", "106", "107", "108", "109", "110",
    "111", "112", "113", "114", "115", "116", "117", "118", "119", "120", "121", "122", "123",
    "124", "125", "126", "127", "128", "129", "130", "131", "132", "133", "134", "135", "136",
    "137", "138", "139", "140", "141", "142", "143", "144", "145", "146", "147", "148", "149",
    "150", "151", "152", "153", "154", "155", "156", "157", "158", "159", "160", "161", "162",
    "163", "164", "165", "166", "167", "168", "169", "170", "171", "172", "173", "174", "175",
    "176", "177", "178", "179", "180", "181", "182", "183", "184", "185", "186", "187", "188",
    "189", "190", "191", "192", "193", "194", "195", "196", "197", "198", "199", "200", "201",
    "202", "203", "204", "205", "206", "207", "208", "209", "210", "211", "212", "213", "214",
    "215", "216", "217", "218", "219", "220", "221", "222", "223", "224", "225", "226", "227",
    "228", "229", "230", "231", "232", "233", "234", "235", "236", "237", "238", "239", "240",
    "241", "242", "243", "244", "245", "246", "247", "248", "249", "250", "251", "252", "253",
    "254", "255", "256",
];
