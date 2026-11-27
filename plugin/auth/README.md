# hyperlane-plugin-auth

High-performance authentication and role-based access control (RBAC) plugin for Hyperlane.

- Pure-Rust FIPS 180-4 SHA-256, RFC 4231 HMAC-SHA256, RFC 4648 base64url, and constant-time comparison with zero extra cryptographic crate dependencies.
- HS256 JWT codec supporting `kid` key rotation, `exp`/`nbf`/`iat`/`iss`/`aud` validation with configurable leeway, and strict `alg == "HS256"` enforcement.
- Built-in `JwtAuthenticator`, `ApiKeyAuthenticator`, `BasicAuthenticator`, and `CompositeAuthenticator`, plus an extensible async `Authenticator` trait.
- Stores the verified `AuthPrincipal` in Hyperlane `Context` attributes and optionally enforces role requirements.
