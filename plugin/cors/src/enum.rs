use super::*;

/// Origin matching policy for [`CorsConfig`].
#[derive(Clone, Default)]
pub enum AllowOrigin {
    /// Permits any incoming origin (`*` without credentials, or reflected origin with credentials).
    #[default]
    Any,
    /// Permits only origins matching an `O(1)` `XxHash3_64` exact-match set (ASCII case-insensitive).
    Exact(HashSetXxHash3_64<String>),
    /// Permits origins whose host ends with any configured suffix (for example `".example.com"`).
    Suffix(Vec<String>),
    /// Evaluates incoming origins using a user-supplied predicate callback.
    Custom(OriginMatcher),
}
