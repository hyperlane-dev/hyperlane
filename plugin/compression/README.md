# hyperlane-plugin-compression

Answers a client with the encoding it asked for.

```rust,ignore
CompressionPlugin::init(CompressionConfig {
    min_size: 1024,
    encodings: vec![Compress::Gzip, Compress::Br],
})?;
```

## What it does

- reads the `accept-encoding` header of the request
- picks the first offered encoding the client accepts, in the order the
  configuration prefers them
- writes `content-encoding` on the answer, and the framework compresses the
  body when the answer is built, so the length of the answer is the length of
  the compressed body
- leaves an answer alone when the body is smaller than the configured size,
  when the content type is already compressed, when the answer already carries
  an encoding, or when the client asked for no encoding at all
- writes `vary: accept-encoding`, so that a cache keeps the answers of the
  different encodings apart

## Configuration

| field | meaning |
|---|---|
| `min_size` | the smallest body that is worth compressing |
| `encodings` | the encodings that are offered, in the order they are preferred |

## Order

The middleware uses the order `-110`. The plugins keep the negative orders, so
an application that gives its own middlewares positive orders runs after
them, and the header is written before the answer is built.
