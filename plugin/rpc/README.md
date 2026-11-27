<div align="center">

# hyperlane-plugin-rpc

**JSON-RPC 2.0 Server Plugin**

[![crates.io](https://img.shields.io/crates/v/hyperlane-plugin-rpc.svg?style=flat-square)](https://crates.io/crates/hyperlane-plugin-rpc)
[![docs.rs](https://img.shields.io/docsrs/hyperlane-plugin-rpc?style=flat-square)](https://docs.rs/hyperlane-plugin-rpc)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/crates/l/hyperlane-plugin-rpc.svg?style=flat-square)](../../LICENSE)

[Workspace Overview](../../README.md) · [API Docs](https://docs.rs/hyperlane-plugin-rpc/latest/) · [Agent Engineering Rules](../../AGENTS.md)

</div>

---

## Overview

Opt-in JSON-RPC 2.0 method router supporting batch execution, notifications, strict payload/batch size caps, and `RpcHook`.

This crate is a member of the [`hyperlane`](../../README.md) unified Rust 2024 workspace. All plugin crates are opt-in and transparent (`Status::Continue`) when unconfigured.

## Responsibilities & Module Layout

- `struct` / `trait` / `impl` — Method registry, batch dispatcher, and `RpcHook`

## Installation

```sh
cargo add hyperlane-plugin-rpc
```

## Verification

Run crate-scoped formatting, lint, and test gates from the workspace root:

```sh
cargo run -p hyperlane-cli -- fmt --check
cargo clippy -p hyperlane-plugin-rpc --all-features --all-targets -- -D warnings
cargo test -p hyperlane-plugin-rpc --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any public API, plugin contract, or module-boundary change per [`../../AGENTS.md`](../../AGENTS.md).
- Project standards and release workflows live in [`../../.agents/skills/hyperlane/SKILL.md`](../../.agents/skills/hyperlane/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
