# hyperlane-plugin-static-files

High-concurrency static asset serving plugin for Hyperlane.

- Canonicalizes root and requested file paths to block `..`, `%2e%2e`, NUL bytes, backslash traversal, dotfiles, and symlink escapes.
- Uses Hyperlane's built-in `FileExtension` MIME table with precompressed `.br` and `.gz` sidecar negotiation (`Vary: Accept-Encoding`).
- 64-way `XxHash3_64` sharded bounded hot-file memory cache (`Arc<CachedFile>`) with `ETag` (`If-None-Match` -> `304 Not Modified`), RFC 9110 byte `Range` (`206 Partial Content` / `416 Range Not Satisfiable`), `HEAD`, directory index, and optional SPA fallback.
