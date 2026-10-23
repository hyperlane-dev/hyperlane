# The hyperlane plugin roadmap

A plugin is a crate whose hooks are registered into the framework's inventory.
The framework collects every registered hook when a server is built, so a
plugin becomes active by being a dependency of the application. This document
describes the contract a plugin follows, the standards it has to meet, and the
plugins that are planned, in the order they are worth building.

## The contract

The mechanism already exists; a plugin does not invent one.

- **Registration.** A plugin implements `ServerHook` and registers it with the
  `#[route]`, `#[request_middleware]` and `#[response_middleware]` attributes.
  Each attribute submits a `HookType` to the `inventory`, which is how the
  framework finds it.
- **Installation.** `#[hyperlane(server: Server)]` collects every submitted
  hook, checks that the explicit orders are unique, sorts by order and
  installs them. A plugin therefore needs no registration call of its own.
- **Order.** A hook carries an optional priority. Higher runs first, and two
  hooks of the same kind may not claim the same priority.
- **Configuration.** The registry is fixed at compile time, so a plugin's
  settings live in a process wide static, initialised before the server
  starts. The pattern is a `OnceLock` behind an `init` and a `get`, and the
  `GetOrInit` trait is the shape the application already uses.
- **Use in the application.** `#[import(path)]` brings the plugin's traits
  into scope without naming them, so an application talks to a plugin through
  its API and not through its internals.
- **Not configured means not in the way.** A hook that is registered but has
  no configuration returns `Status::Continue` and does nothing. A plugin that
  is a dependency but that the application did not configure must not change
  the server's behaviour.

The last point is the rule that shapes every plugin below: being a dependency
is not the same as being enabled.

## The standards a plugin has to meet

The repository's own gate checks every staged `.rs` and `.toml` file, so a
plugin is held to the same rules as the framework.

- **Layout.** A directory holds the keyword files only: `lib.rs`, `mod.rs`,
  `const.rs`, `fn.rs`, `struct.rs`, `enum.rs`, `impl.rs`, `trait.rs`, `type.rs`.
  The entry of a crate is `lib.rs`: the module declarations first, then the
  `pub use` and `pub(crate) use` re-exports, then the imports.
- **Visibility.** In `const.rs` and `static.rs` the `pub` items come before the
  `pub(crate)` ones and the private ones. A `pub` constant needs a reader
  outside its own module, which is why a plugin's settings are reached
  through an accessor rather than through a public field.
- **Strings.** Every literal lives in `const.rs`. The format string of a
  `format!` or a `println!` stays at the call site.
- **No panics.** `panic!`, `.expect()` and `.unwrap()` do not appear in
  production code. Errors travel as `Result`.
- **Types.** Every `let` binding and every closure parameter carries its type.
- **Fields.** A field is read through its accessor, never as `self.field`.
- **Documentation.** Every public item carries a doc comment, and a function
  that takes or returns something documents it under `# Arguments` and
  `# Returns`.
- **No `#[allow]`.**

## The plugins, in the order they are worth building

### P0 - the ground every service needs

| plugin | what it gives |
|---|---|
| `hyperlane-plugin-shutdown` | graceful shutdown: the framework already has the control hook, the plugin turns it into "configure it once and it is armed" |
| `hyperlane-plugin-logger` | logging, with a file backend |

Both are small, and the application already carries a working version of each,
so upstreaming them is a move rather than a rewrite. A service that cannot shut
down cleanly or cannot say what it is doing is not a service yet.

Both are in the tree now, and both are published as `false`: they are usable
from a path dependency today, and they wait for the release job to name them.

### P1 - what a web service is asked for every day

| plugin | what it gives |
|---|---|
| `hyperlane-plugin-cors` | cross origin headers and preflight |
| `hyperlane-plugin-compression` | gzip and brotli on the response |
| `hyperlane-plugin-static-files` | serving files, with the content type table the framework already carries |

The compression plugin is the cheapest of the three: `http-compress` is
already a dependency and the response builder already honours a
`Content-Encoding` header, so the plugin only has to set the header when the
client asked for it and the body is worth compressing.

### P2 - the data layer

| plugin | what it gives |
|---|---|
| `hyperlane-plugin-database` | the connection and auto creation traits, with no driver behind them |
| `hyperlane-plugin-postgresql` | the PostgreSQL driver |
| `hyperlane-plugin-mysql` | the MySQL driver |
| `hyperlane-plugin-redis` | the Redis client and its pool |

The application already has this shape, down to the trait names. The base
crate carries no driver, so it stays small; the driver crates carry theirs,
so they stay large and are published separately.

### P3 - running a service in production

| plugin | what it gives |
|---|---|
| `hyperlane-plugin-env` | configuration loaded from the environment |
| `hyperlane-plugin-metrics` | counters and timers in a scrapeable format |
| `hyperlane-plugin-rate-limit` | a limit per client or per route |

### P4 - the rest, when the need is proven

`hyperlane-plugin-message-queue`, `hyperlane-plugin-jwt`,
`hyperlane-plugin-tracing` and `hyperlane-plugin-multipart`.

## The shape of a plugin

```
plugin/<name>/
  Cargo.toml
  README.md
  src/
    lib.rs      the entry: module declarations, re-exports, the imports
    const.rs    every string literal
    struct.rs   the types
    enum.rs     the enumerations
    trait.rs    the traits the application talks to
    impl.rs     the implementations
    fn.rs       the free functions
    mod.rs      a submodule, when the plugin has more than one
  tests/
    mod.rs      the test entry: the modules and the crate under test
```

The entry of a crate is `lib.rs`. A `mod.rs` holds a submodule instead, and
carries the module declarations first, then the `pub use` and `pub(crate) use`
re-exports. Every keyword file below the entry starts with `use super::*;` and
reaches the crate's imports through it, so an import is written once, in the
entry, and never again.

A plugin declares its dependency on the framework and on nothing else, unless
it carries a driver. Its `Cargo.toml` carries `publish = false` until it is
ready to be published, and it is added to the workspace members, to the
workspace dependencies and to the release job's package list.

Its public surface is small on purpose: an `init` that takes the
configuration, a `get` that hands the configuration back, and the traits the
application calls. Everything else is private.

## What is still to decide

- **Publishing.** The websocket plugin is not in the release job's package
  list, so no plugin is published today. Until that is settled, the plugins
  are only usable from a path dependency, and the list has to grow before
  they are useful to anyone else.
- **The order of the middleware.** A plugin that adds a request middleware
  has to pick a priority, and two plugins that pick the same one collide.
  The priorities the framework reserves for itself have to be written down
  before the first plugin ships.
- **How much configuration is enough.** Every plugin that needs a setting
  needs it set before the server starts, and that is a second thing the
  application has to remember. A single configuration entry point would be
  better than one per plugin, and it is a decision, not an implementation.
