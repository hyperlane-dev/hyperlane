# Original request targets

`Context::get_request_target() -> Option<String>` returns the unchanged request
path and query as received on the wire. Use it when building login return URLs,
canonical page redirects, or signatures that depend on query spelling and order.
Continue using `Request::get_path()` for route matching and the query helpers for
ordinary parameter lookup.

Inside a request hook:

```rust
let target: Option<String> = ctx.get_request_target();
```

Pass this value to the application's shared redirect API. Hyperlane preserves
the transport's original address without defining an application's login route.

## Protocol guarantees

- HTTP/1.1 retains the original request-line target after successful parsing.
- HTTP/2 retains the decoded `:path` URI before conversion to a query map.
- HTTP/3 retains the same original URI before QUIC stream dispatch.
- Middleware can rewrite the route path without changing the saved target.
- Repeated keys keep their original interleaving, and bare flags, empty values,
  literal plus signs, percent escapes and an empty `?` marker remain unchanged.
- The snapshot is set before request middleware and replaced for every request,
  including persistent HTTP/1.1 connections and multiplexed protocol streams.

The value lives in reserved request-local lifecycle metadata. A manually
constructed `Context` has no transport snapshot and returns `None`. Explicitly
clearing all context attributes also removes this metadata. Do not manufacture
an original target by rebuilding a query map when byte-exact behavior matters.

HTTP requests do not contain browser fragments. Applications must carry them in
client-side return URLs or restore a fragment inherited through an HTTP redirect.

## Compatibility

The existing public `Request` struct, its fields and its serialization remain
unchanged. No synthetic client header is trusted as the original target, and no
internal bookkeeping parameter is inserted into the user's query map.
