# hyperlane-plugin-tracing

Lock-free distributed tracing and correlation plugin for Hyperlane.

- Parses and propagates W3C Trace Context (`traceparent` / `tracestate`) and `X-Request-Id` with zero-mutex 128-bit trace ID and 64-bit span ID generation.
- Injects `traceparent`, `X-Request-Id`, and optional `Server-Timing` (`app;dur=<ms>`) headers onto responses.
- Provides an extensible `SpanExporter` trait receiving completed `SpanRecord` snapshots for OpenTelemetry/OTLP/Jaeger export.
