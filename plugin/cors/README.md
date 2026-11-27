# hyperlane-plugin-cors

High-performance Cross-Origin Resource Sharing (CORS) middleware plugin for Hyperlane.

- Precompiles static CORS header values once at initialization for zero-allocation hot-path application.
- Supports `AllowOrigin::Any`, `O(1)` `XxHash3_64` exact allowlists, domain suffix matching, and custom predicate functions.
- Enforces the Fetch credentials safety invariant (never emits `Access-Control-Allow-Origin: *` alongside `Access-Control-Allow-Credentials: true`).
- Answers preflight `OPTIONS` requests with `204 No Content` and merges `Vary: Origin` cleanly.
