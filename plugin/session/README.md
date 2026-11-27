# hyperlane-plugin-session

High-concurrency HTTP session plugin for Hyperlane.

- 64-way `XxHash3_64` sharded bounded `InMemorySessionStore` plus a pluggable async `SessionStore` trait for Redis/database backends.
- Dirty-tracked read path: unmodified sessions perform zero store writes and emit zero redundant `Set-Cookie` headers.
- Built-in session-fixation defense (`regenerate_id()`), `destroy()` cookie clearing, and optional pure-Rust HMAC-SHA256 signed cookies via Hyperlane's `CookieBuilder` (`HttpOnly`, `Secure`, `SameSite`, `Max-Age`).
