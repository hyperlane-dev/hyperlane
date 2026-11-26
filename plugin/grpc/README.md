# hyperlane-plugin-grpc

A gRPC over HTTP/2 plugin for the Hyperlane framework providing 5-byte length-prefixed framing, bounded compression/decompression, canonical gRPC status codes `0..=16`, percent-encoded `grpc-message`, `grpc-timeout` deadline enforcement, service routing, and an opt-in `GrpcHook` `ServerHook`.
