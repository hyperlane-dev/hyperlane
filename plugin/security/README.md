# hyperlane-plugin-security

Production HTTP security hardening plugin for Hyperlane.

- Prebuilds security response headers (`Strict-Transport-Security`, `Content-Security-Policy`, `X-Frame-Options`, `X-Content-Type-Options`, `Referrer-Policy`, `Permissions-Policy`, `COOP`, `COEP`, `CORP`) for zero-allocation hot-path injection.
- Stateless double-submit CSRF protection backed by pure-Rust FIPS 180-4 SHA-256 and RFC 4231 HMAC-SHA256 with constant-time signature verification.
- Built-in request guard rejecting oversized URIs/bodies, untrusted `Host` headers, and path-traversal encodings (`..`, `%2e%2e`, `%00`, backslashes).
- Extensible `SecurityRule` trait for custom WAF and application security policies.
