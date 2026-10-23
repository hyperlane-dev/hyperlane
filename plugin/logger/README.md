# hyperlane-plugin-logger

Writes the log of a server to the console, to a file, or to both, and can log
every request that the server answers.

```rust,ignore
LoggerPlugin::init(LoggerConfig {
    level: LevelFilter::Info,
    file: Some("server.log".to_owned()),
})?;
```

The request log is a middleware pair, so it is installed with the server. It
does nothing until the logger is configured, so a service that never calls
`init` writes no request log.

## Configuration

| field | meaning |
|---|---|
| `level` | the highest level that is written |
| `file` | the file the log is appended to, when there is one |

## The request log

Each answered request is written with its method, its path, its status and the
time it took. The time is measured by a request middleware that notes when the
request arrived, and read by a response middleware that writes the line.

## Order

The two middlewares use the order `100`, which leaves `0` to `99` for the
framework and for the application.
