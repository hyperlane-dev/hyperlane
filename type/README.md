<div align="center">

# http-type

**RFC 9110/9112 HTTP Wire Types & Builders**

[![crates.io](https://img.shields.io/crates/v/http-type.svg?style=flat-square)](https://crates.io/crates/http-type)
[![docs.rs](https://img.shields.io/docsrs/http-type?style=flat-square)](https://docs.rs/http-type)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/crates/l/http-type.svg?style=flat-square)](../LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/http-type/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

Wire-accurate `Request`, `Response`, `Stream`, `Cookie`, `MultipartFormData`, `WebSocket`, `Status`, and `RequestConfig` with CRLF/smuggling rejection and HEAD-aware response serialization.

This crate is a member of the [`hyperlane`](../README.md) unified Rust 2024 workspace. All plugin crates are opt-in and transparent (`Status::Continue`) when unconfigured.

## Responsibilities & Module Layout

- `request` / `response` — Wire parsers, builders, `matches_if_none_match`, and framing invariants
- `stream` / `websocket` / `cookie` / `multipart` — Transport stream wrappers and structured payload codecs

## Installation

```sh
cargo add http-type
```

## Verification

Run crate-scoped formatting, lint, and test gates from the workspace root:

```sh
cargo run -p hyperlane-cli -- fmt --check
cargo clippy -p http-type --all-features --all-targets -- -D warnings
cargo test -p http-type --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any public API, plugin contract, or module-boundary change per [`../AGENTS.md`](../AGENTS.md).
- Project standards and release workflows live in [`../.agents/skills/hyperlane/SKILL.md`](../.agents/skills/hyperlane/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
