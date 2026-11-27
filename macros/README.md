<div align="center">

# hyperlane-macros

**Hyperlane Route, Hook & Middleware Proc Macros**

[![crates.io](https://img.shields.io/crates/v/hyperlane-macros.svg?style=flat-square)](https://crates.io/crates/hyperlane-macros)
[![docs.rs](https://img.shields.io/docsrs/hyperlane-macros?style=flat-square)](https://docs.rs/hyperlane-macros)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/crates/l/hyperlane-macros.svg?style=flat-square)](../LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/hyperlane-macros/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

Procedural attribute macros (`#[route]`, `#[request_middleware]`, `#[response_middleware]`, `#[hyperlane]`, `#[methods]`, `#[send]`, `#[try_send]`, `#[host]`, `#[referer]`, etc.) eliminating handler boilerplate.

This crate is a member of the [`hyperlane`](../README.md) unified Rust 2024 workspace. All plugin crates are opt-in and transparent (`Status::Continue`) when unconfigured.

## Responsibilities & Module Layout

- `route` / `hook` / `send` / `filter` — Hygiene-safe attribute macros emitting absolute `::hyperlane::*` paths

## Installation

```sh
cargo add hyperlane-macros
```

## Verification

Run crate-scoped formatting, lint, and test gates from the workspace root:

```sh
cargo run -p hyperlane-cli -- fmt --check
cargo clippy -p hyperlane-macros --all-features --all-targets -- -D warnings
cargo test -p hyperlane-macros --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any public API, plugin contract, or module-boundary change per [`../AGENTS.md`](../AGENTS.md).
- Project standards and release workflows live in [`../.agents/skills/hyperlane/SKILL.md`](../.agents/skills/hyperlane/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
