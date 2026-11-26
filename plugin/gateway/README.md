# hyperlane-plugin-gateway

A reverse proxy and API gateway plugin for the Hyperlane framework powered by `http-request` with pooled HTTP/1.1 & HTTP/2 upstream connections, round-robin / least-connections / weighted load balancing, circuit breaking, per-route token-bucket rate limiting, RFC 9110 hop-by-hop header stripping, `X-Forwarded-*` injection, and an opt-in `GatewayHook` `ServerHook`.
