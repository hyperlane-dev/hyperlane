/// The default name for the session cookie.
pub const SESSION_COOKIE_NAME: &str = "session_id";

/// The default length of the session ID.
pub const SESSION_ID_LENGTH: usize = 32;

/// The default session timeout duration in seconds (30 minutes).
pub const SESSION_TIMEOUT_SECONDS: u64 = 1800;

/// The default session timeout duration in milliseconds (30 minutes).
pub const SESSION_TIMEOUT_MILLISECONDS: u64 = 1800000;

/// A string constant representing "session".
pub const SESSION: &str = "session";

/// The session data key used to store the user's ID.
pub const SESSION_USER_ID_KEY: &str = "user_id";

/// The session data key used to store the username.
pub const SESSION_USERNAME_KEY: &str = "username";

/// The session data key used to store the user's role.
pub const SESSION_USER_ROLE_KEY: &str = "user_role";

/// The session data key used to store the login timestamp.
pub const SESSION_LOGIN_TIME_KEY: &str = "login_time";

/// The session data key used to store the last access timestamp.
pub const SESSION_LAST_ACCESS_TIME_KEY: &str = "last_access_time";

/// The session data key used to store the client's IP address.
pub const SESSION_IP_ADDRESS_KEY: &str = "ip_address";

/// The session data key used to store the client's user agent string.
pub const SESSION_USER_AGENT_KEY: &str = "user_agent";

/// The session data key used to store the CSRF token.
pub const SESSION_CSRF_TOKEN_KEY: &str = "csrf_token";

/// The session data key used to store the user's language preference.
pub const SESSION_LANGUAGE_KEY: &str = "language";

/// The session data key used to store the user's timezone.
pub const SESSION_TIMEZONE_KEY: &str = "timezone";

/// The string representing an active session state.
pub const SESSION_STATE_ACTIVE: &str = "active";

/// The string representing an expired session state.
pub const SESSION_STATE_EXPIRED: &str = "expired";

/// The string representing an invalid session state.
pub const SESSION_STATE_INVALID: &str = "invalid";

/// The string representing a destroyed session state.
pub const SESSION_STATE_DESTROYED: &str = "destroyed";

/// The default interval in seconds for cleaning up expired sessions (5 minutes).
pub const SESSION_CLEANUP_INTERVAL_SECONDS: u64 = 300;

/// The maximum number of concurrent sessions allowed per user.
pub const MAX_SESSIONS_PER_USER: usize = 5;

/// The character set used for generating session IDs.
pub const SESSION_ID_CHARSET: &str =
    "ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789";

/// The byte representation of the session ID character set.
pub const SESSION_ID_CHARSET_BYTES: &[u8] = SESSION_ID_CHARSET.as_bytes();

/// The threshold in seconds after which a session should be regenerated (15 minutes).
pub const SESSION_REGENERATION_THRESHOLD_SECONDS: u64 = 900;

/// The prefix used for session flash message keys.
pub const SESSION_FLASH_MESSAGE_PREFIX: &str = "flash:";

/// The type for a successful flash message.
pub const SESSION_FLASH_SUCCESS: &str = "success";

/// The type for an error flash message.
pub const SESSION_FLASH_ERROR: &str = "error";

/// The type for a warning flash message.
pub const SESSION_FLASH_WARNING: &str = "warning";

/// The type for an informational flash message.
pub const SESSION_FLASH_INFO: &str = "info";

/// The name of the "remember me" cookie.
pub const SESSION_REMEMBER_ME_COOKIE_NAME: &str = "remember_me";

/// The length of the "remember me" token.
pub const SESSION_REMEMBER_ME_TOKEN_LENGTH: usize = 64;

/// The timeout duration in seconds for the "remember me" token (30 days).
pub const SESSION_REMEMBER_ME_TIMEOUT_SECONDS: u64 = 2592000;

/// The separator used between components of a session fingerprint.
pub const SESSION_FINGERPRINT_SEPARATOR: &str = "|";

/// The timeout in milliseconds for acquiring a session lock.
pub const SESSION_LOCK_TIMEOUT_MILLISECONDS: u64 = 5000;

/// The maximum number of concurrent accesses allowed for a single session.
pub const SESSION_CONCURRENT_ACCESS_LIMIT: usize = 10;
