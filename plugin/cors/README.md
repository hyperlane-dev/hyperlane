# hyperlane-plugin-cors

Answers the browsers that ask a server for another origin.

```rust,ignore
CorsPlugin::init(CorsConfig {
    allowed_origins: vec![String::from("https://app.example.com")],
    allowed_methods: vec![String::from("GET")],
    allowed_headers: vec![String::from("content-type")],
    max_age: Some(600),
    allow_credentials: true,
})?;
```

## What it does

- reads the `origin` header of the request
- leaves a request without an origin alone
- leaves a request from an origin that is not allowed alone
- writes `access-control-allow-origin`, `access-control-allow-methods`,
  `access-control-allow-headers`, `access-control-max-age` and
  `access-control-allow-credentials` on an allowed origin
- writes `vary: origin`, so that a cache does not answer one origin with the
  answer of another

The middleware writes the headers on the response, so every answer of the
server carries them, and a preflight request is answered with them as well.

## Configuration

| field | meaning |
|---|---|
| `allowed_origins` | the origins that are allowed, or one `*` for every origin |
| `allowed_methods` | the methods that are allowed |
| `allowed_headers` | the request headers that are allowed |
| `max_age` | the seconds a browser may cache the answer of a preflight request |
| `allow_credentials` | whether the answer may carry credentials |

## Order

The middleware uses the order `-110`. The plugins keep the negative orders, so
an application that gives its own middlewares positive orders runs after
them.
