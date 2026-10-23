# hyperlane-plugin-static-files

Serves the files of a directory.

```rust,ignore
StaticFilesPlugin::init(StaticFilesConfig {
    root: PathBuf::from("./public"),
    index: String::from("index.html"),
})?;
server.route::<StaticFilesRoute>(&StaticFilesPlugin::pattern("/assets"));
```

## What it does

- reads the `path` parameter of the route, which carries the name of the file
  below the prefix
- answers with the file, and with the content type the framework's table names
  for its extension
- answers a file whose extension the framework does not know with a stream of
  bytes
- answers a path that climbs out of the root, and a path that names no file,
  with the status of a missing file
- answers a request for a directory with the index file

The route is registered by the application, because the path the files are
served under belongs to the application. The pattern of the route is built by
the plugin, so the name of the parameter stays the same.

## Configuration

| field | meaning |
|---|---|
| `root` | the directory the files are served from |
| `index` | the file that answers a request for a directory |

## Order

The plugin registers a route and no middleware, so it has no order. The
answer of a file is built and sent by the middlewares of the application, like
the answer of any other route.
