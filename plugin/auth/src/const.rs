/// Context attribute key where [`crate::AuthHook`] stores the authenticated [`crate::AuthPrincipal`].
pub const ATTR_AUTH_PRINCIPAL: &str = "hyperlane_plugin_auth_principal";

/// Default `WWW-Authenticate` challenge header emitted on `401 Unauthorized`.
pub const DEFAULT_WWW_AUTHENTICATE: &str = "Bearer realm=\"hyperlane\"";

/// Default JWT clock-skew leeway in seconds.
pub const DEFAULT_JWT_LEEWAY_SECS: u64 = 60;

/// Supported JWT algorithm identifier (`HMAC-SHA256`).
pub const JWT_ALG_HS256: &str = "HS256";

/// Standard JWT `typ` header value.
pub const JWT_TYP_JWT: &str = "JWT";

/// Prefix for HTTP `Authorization: Bearer <token>` headers.
pub(crate) const PREFIX_BEARER: &str = "bearer ";

/// Prefix for HTTP `Authorization: Basic <base64>` headers.
pub(crate) const PREFIX_BASIC: &str = "basic ";

/// Base64URL alphabet (RFC 4648 Section 5, unpadded).
pub(crate) const BASE64URL_ALPHABET: &[u8; 64] =
    b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_";

/// Error description when credentials are absent from the request.
pub(crate) const MSG_AUTH_MISSING: &str = "missing authentication credentials";

/// Error description when token or credentials are malformed or fail signature verification.
pub(crate) const MSG_AUTH_INVALID: &str = "invalid authentication credentials";

/// Error description when JWT `exp` claim is in the past.
pub(crate) const MSG_AUTH_EXPIRED: &str = "authentication token expired";

/// Error description when JWT `nbf` claim is in the future.
pub(crate) const MSG_AUTH_NOT_YET_VALID: &str = "authentication token not yet valid";

/// Error description when JWT `alg` does not match `HS256`.
pub(crate) const MSG_AUTH_UNSUPPORTED_ALG: &str = "unsupported JWT algorithm";

/// Error description when JWT `iss` or `aud` does not match expected policy.
pub(crate) const MSG_AUTH_CLAIM_MISMATCH: &str = "JWT issuer or audience mismatch";

/// Error description when principal lacks required roles.
pub(crate) const MSG_AUTH_FORBIDDEN: &str = "insufficient permissions";

/// SHA-256 initial hash values `H0..H7` per FIPS 180-4.
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
