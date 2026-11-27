# hyperlane-plugin-shutdown

Graceful shutdown and connection-draining coordinator plugin for Hyperlane.

- Arms OS termination signals (`SIGINT` / `Ctrl-C`, and `SIGTERM` on Unix) directly onto Hyperlane's `ServerControlHook`.
- Provides lock-free RAII `ShutdownGuard` tracking so in-flight work completes cleanly before exit up to a bounded `drain_timeout_ms`.
- Executes extensible async `ShutdownCallback` hooks in strict LIFO registration order.
