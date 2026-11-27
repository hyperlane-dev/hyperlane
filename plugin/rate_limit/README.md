# hyperlane-plugin-rate-limit

Ultra-high-concurrency rate-limiting plugin for Hyperlane.

- 64-way `XxHash3_64` sharded store with lock-free atomic GCRA and fixed-window counters (`RwLock::read` + atomic CAS on steady-state keys).
- Bounded key table with automatic stale-entry eviction to prevent memory exhaustion under spoofed-IP floods.
- Extensible `KeyExtractor` (built-in client IP, header, path+IP) and `RateLimitStore` traits for custom or distributed backends.
- Emits standard `RateLimit-Limit`, `RateLimit-Remaining`, `RateLimit-Reset`, and `Retry-After` headers.
