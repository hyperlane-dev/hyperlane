# hyperlane-plugin-health

Production-ready `/healthz`, `/livez`, and `/readyz` health probe plugin for Hyperlane.

- Pluggable async `HealthCheck` trait executed concurrently with bounded per-check timeout.
- Optional snapshot TTL caching to shield downstream databases from probe storms.
- Lock-free `AtomicBool` drain signaling (`mark_draining()`) so `/readyz` returns `503 Service Unavailable` during graceful shutdown while `/livez` stays `200 OK`.
