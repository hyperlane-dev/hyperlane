<div align="center">

# hyperlane-core

**Multi-Protocol Async HTTP Server Core**

[![crates.io](https://img.shields.io/crates/v/hyperlane-core.svg?style=flat-square)](https://crates.io/crates/hyperlane-core)
[![docs.rs](https://img.shields.io/docsrs/hyperlane-core?style=flat-square)](https://docs.rs/hyperlane-core)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/crates/l/hyperlane-core.svg?style=flat-square)](../LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/hyperlane-core/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

Tokio-based HTTP/1.x, HTTP/2 (`h2c`/`h2`), and HTTP/3 (QUIC) server engine with radix routing, `Context`, `ServerHook` lifecycle, and Linux `io_uring` transport.

This crate is a member of the [`hyperlane`](../README.md) unified Rust 2024 workspace. All plugin crates are opt-in and transparent (`Status::Continue`) when unconfigured.

## Responsibilities & Module Layout

- `server` / `route` / `hook` / `context` / `config` / `error` — Public server API and lifecycle traits
- `listener` / `protocol` / `tls` (`pub(crate)`) — Internal wire framers, connection pumps, and transport engines

## Installation

```sh
cargo add hyperlane-core
```

## Verification

Run crate-scoped formatting, lint, and test gates from the workspace root:

```sh
cargo run -p hyperlane-cli -- fmt --check
cargo clippy -p hyperlane-core --all-features --all-targets -- -D warnings
cargo test -p hyperlane-core --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any public API, plugin contract, or module-boundary change per [`../AGENTS.md`](../AGENTS.md).
- Project standards and release workflows live in [`../.agents/skills/hyperlane/SKILL.md`](../.agents/skills/hyperlane/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
