<div align="center">

# http-request

**Sync & Async HTTP/1.1 + HTTP/2 Client**

[![crates.io](https://flat.badgen.net/crates/v/http-request)](https://crates.io/crates/http-request)
[![docs.rs](https://img.shields.io/docsrs/http-request?style=flat-square)](https://docs.rs/http-request)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](../LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/http-request/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

Ergonomic HTTP client supporting opt-in keep-alive pooling, TLS session resumption, custom PEM roots, bounded retries, multipart/form uploads, and HTTP/SOCKS5 proxies.

This crate is a member of the [`hyperlane`](../README.md) unified Rust 2024 workspace. All plugin crates are opt-in and transparent (`Status::Continue`) when unconfigured.

## Responsibilities & Module Layout

- `client` / `pool` / `tls` — Sync and Tokio-async HTTP/1.1 + HTTP/2 transports with redirect credential stripping

## Installation

```sh
cargo add http-request
```

## Verification

Run crate-scoped formatting, lint, and test gates from the workspace root:

```sh
cargo run -p hyperlane-cli -- fmt --check
cargo clippy -p http-request --all-features --all-targets -- -D warnings
cargo test -p http-request --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any public API, plugin contract, or module-boundary change per [`../AGENTS.md`](../AGENTS.md).
- Project standards and release workflows live in [`../.agents/skills/hyperlane/SKILL.md`](../.agents/skills/hyperlane/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
