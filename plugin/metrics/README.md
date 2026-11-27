# hyperlane-plugin-metrics

Zero-lock Prometheus / OpenMetrics 0.0.4 observability plugin for Hyperlane.

- Built-in lock-free atomic counters (2xx/3xx/4xx/5xx status classes, response bytes), in-flight gauge, and fixed-bucket request duration histogram.
- Extensible `MetricsRegistry` returning lock-free `Arc<Counter>`, `Arc<Gauge>`, and `Arc<Histogram>` handles for custom application metrics.
- Exposes a scrapeable `/metrics` endpoint with optional constant-time bearer token guard.
