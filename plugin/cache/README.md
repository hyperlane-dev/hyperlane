# hyperlane-plugin-cache

High-concurrency response and data caching plugin for Hyperlane.

- 64-way `XxHash3_64` sharded in-memory cache (`ShardedMemoryCache`) enforcing both entry-count and total-byte memory budgets with lazy and explicit TTL eviction.
- Built-in single-flight stampede protection (`get_or_load`) coalescing concurrent misses on the same key via `tokio::sync::Notify`.
- Pluggable async `CacheStore` trait for Redis, Memcached, or tiered storage backends.
- HTTP response caching hooks (`CacheRequestHook` / `CacheResponseHook`) honoring `Cache-Control`, `Vary`, `Age`, and `X-Cache: HIT / MISS / BYPASS`.
