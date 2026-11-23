# hyperlane-plugin-negotiate

Answers a response with the content coding the client accepts, and answers a
conditional request with `304 Not Modified` while a validator still matches.

```rust,ignore
use hyperlane_plugin_negotiate::{CompressMiddleware, ConditionalMiddleware};

server
    .response_middleware::<CompressMiddleware>()
    .response_middleware::<ConditionalMiddleware>()
    .response_middleware::<SendMiddleware>();
```

Both are response middlewares, so a route keeps building its response and
returning `Status::Continue`, and the middleware that writes the response onto
the stream runs last. A route that answers with `#[hyperlane::send]` writes the
bytes itself, which leaves nothing for a response middleware to negotiate.

## The content coding

The middleware reads every `Accept-Encoding` field of the request, picks the
coding the client wants most among the ones `http-compress` implements, and
declares it on the response. The response builder applies the coding and the
length it produces when the sender serializes the response, so the middleware
never touches the stream.

A response is left alone when it already declares a `Content-Encoding`, when it
is an event stream, when its status carries no body, and when its body is
shorter than the threshold. Every coded response also declares
`Vary: accept-encoding`, so a cache never answers a client with a coding it did
not ask for. A `Vary` the response already declares is extended rather than
replaced.

| knob | default | meaning |
|---|---|---|
| `min_length` | `256` | the body length below which a response is answered without a coding |

The threshold is process wide, because the framework registers a middleware by
its type and builds a hook per request, so a server cannot carry a configuration
of its own.

```rust,ignore
CompressMiddleware::init(CompressConfig::new(1_024))?;
```

## The conditional request

The middleware declares a strong entity-tag on every response that carries a
body and a safe method, unless the response already declares one, and answers
`304 Not Modified` when the request carries an `If-None-Match` that matches it.
The tag is the quoted hexadecimal digest of the body, so the same body always
produces the same tag, across processes and across restarts.

A `304` keeps the representation metadata, so the validator, the variance and
the content type of the answer survive it.

## Order

Register the coding middleware before the conditional middleware. The coding
middleware declares the variance, and the conditional middleware can then turn
the answer into a `304` that keeps it.
