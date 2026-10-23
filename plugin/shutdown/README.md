# hyperlane-plugin-shutdown

Turns the termination signals of the operating system into a graceful shutdown
of a hyperlane server.

The server already knows how to stop: the control hook it hands back has a
`shutdown` and a `wait`. What the framework does not do is watch for the
signals, so an application has to wire that itself. This plugin does it.

```rust,ignore
ShutdownPlugin::init(ShutdownConfig::default())?;
ShutdownPlugin::watch(&hook)?;
```

The plugin registers no hook of its own, so it changes nothing until an
application asks for it.

## What it does

- stops the server when the process is interrupted, and on the systems that
  have it, when the process is terminated
- runs the handler the application set, or writes a message when it set none
- asks the server to stop, and the application waits for it with the `wait`
  hook of the same control hook

## Configuration

| field | meaning |
|---|---|
| `watch_terminate` | whether the terminate signal also stops the server |
| `message` | the message the default handler writes |
