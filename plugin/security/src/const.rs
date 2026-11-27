/// Default `Strict-Transport-Security` header value (1 year, includeSubDomains).
pub const DEFAULT_HSTS_VALUE: &str = "max-age=31536000; includeSubDomains";

/// Default `Content-Security-Policy` value for strict web defaults.
pub const DEFAULT_CSP_STRICT: &str =
    "default-src 'self'; frame-ancestors 'none'; object-src 'none'";

/// Default `X-Frame-Options` header value.
pub const DEFAULT_X_FRAME_OPTIONS: &str = "DENY";

/// Default `X-Content-Type-Options` header value.
pub const DEFAULT_X_CONTENT_TYPE_OPTIONS: &str = "nosniff";

/// Default `Referrer-Policy` header value.
pub const DEFAULT_REFERRER_POLICY: &str = "strict-origin-when-cross-origin";

/// Default `Permissions-Policy` header value disabling sensitive browser APIs.
pub const DEFAULT_PERMISSIONS_POLICY: &str = "geolocation=(), microphone=(), camera=()";

/// Default `Cross-Origin-Opener-Policy` header value.
pub const DEFAULT_COOP_VALUE: &str = "same-origin";

/// Default `Cross-Origin-Embedder-Policy` header value.
pub const DEFAULT_COEP_VALUE: &str = "require-corp";

/// Default `Cross-Origin-Resource-Policy` header value.
pub const DEFAULT_CORP_VALUE: &str = "same-origin";

/// Default double-submit CSRF cookie name.
pub const DEFAULT_CSRF_COOKIE_NAME: &str = "csrf_token";

/// Default CSRF request header name.
pub const DEFAULT_CSRF_HEADER_NAME: &str = "x-csrf-token";

/// Default maximum URI path length in bytes (8 KiB).
pub const DEFAULT_MAX_URI_BYTES: usize = 8 * 1024;

/// Default maximum request body size in bytes (`0` means defer to core `RequestConfig`).
pub const DEFAULT_SECURITY_MAX_BODY_BYTES: usize = 0;

/// Lowercase hexadecimal digits used for HMAC signature encoding.
pub(crate) const HEX_DIGITS: &[u8; 16] = b"0123456789abcdef";

/// Encoded dot traversal sequence (lowercase).
pub(crate) const ENCODED_DOT_LOWER: &str = "%2e";

/// Encoded NUL byte sequence.
pub(crate) const ENCODED_NUL: &str = "%00";

/// Encoded backslash sequence (lowercase).
pub(crate) const ENCODED_BACKSLASH_LOWER: &str = "%5c";

/// Violation message when request body exceeds configured limit.
pub(crate) const MSG_BODY_TOO_LARGE: &str = "request body exceeds security limit";

/// Violation message when request URI exceeds configured limit.
pub(crate) const MSG_URI_TOO_LONG: &str = "request URI exceeds security limit";

/// Violation message when `Host` header is missing or not allowlisted.
pub(crate) const MSG_INVALID_HOST: &str = "untrusted Host header";

/// Violation message when path traversal or NUL byte is detected.
pub(crate) const MSG_PATH_TRAVERSAL: &str = "path traversal sequence rejected";

/// Violation message when CSRF token is missing or invalid.
pub(crate) const MSG_CSRF_INVALID: &str = "invalid or missing CSRF token";

/// SHA-256 initial hash state `H0..H7` per FIPS 180-4.
pub(crate) const SHA256_INIT: [u32; 8] = [
    0x6a09_e667,
    0xbb67_ae85,
    0x3c6e_f372,
    0xa54f_f53a,
    0x510e_527f,
    0x9b05_688c,
    0x1f83_d9ab,
    0x5be0_cd19,
];

/// SHA-256 round constants `K[0..64]` per FIPS 180-4.
pub(crate) const SHA256_K: [u32; 64] = [
    0x428a_2f98,
    0x7137_4491,
    0xb5c0_fbcf,
    0xe9b5_dba5,
    0x3956_c25b,
    0x59f1_11f1,
    0x923f_82a4,
    0xab1c_5ed5,
    0xd807_aa98,
    0x1283_5b01,
    0x2431_85be,
    0x550c_7dc3,
    0x72be_5d74,
    0x80de_b1fe,
    0x9bdc_06a7,
    0xc19b_f174,
    0xe49b_69c1,
    0xefbe_4786,
    0x0fc1_9dc6,
    0x240c_a1cc,
    0x2de9_2c6f,
    0x4a74_84aa,
    0x5cb0_a9dc,
    0x76f9_88da,
    0x983e_5152,
    0xa831_c66d,
    0xb003_27c8,
    0xbf59_7fc7,
    0xc6e0_0bf3,
    0xd5a7_9147,
    0x06ca_6351,
    0x1429_2967,
    0x27b7_0a85,
    0x2e1b_2138,
    0x4d2c_6dfc,
    0x5338_0d13,
    0x650a_7354,
    0x766a_0abb,
    0x81c2_c92e,
    0x9272_2c85,
    0xa2bf_e8a1,
    0xa81a_664b,
    0xc24b_8b70,
    0xc76c_51a3,
    0xd192_e819,
    0xd699_0624,
    0xf40e_3585,
    0x106a_a070,
    0x19a4_c116,
    0x1e37_6c08,
    0x2748_774c,
    0x34b0_bcb5,
    0x391c_0cb3,
    0x4ed8_aa4a,
    0x5b9c_ca4f,
    0x682e_6ff3,
    0x748f_82ee,
    0x78a5_636f,
    0x84c8_7814,
    0x8cc7_0208,
    0x90be_fffa,
    0xa450_6ceb,
    0xbef9_a3f7,
    0xc671_78f2,
];
