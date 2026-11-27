<div align="center">

# hyperlane-plugin-cache

**Response & Keyed In-Memory Cache Plugin**

[![crates.io](https://flat.badgen.net/crates/v/hyperlane-plugin-cache)](https://crates.io/crates/hyperlane-plugin-cache)
[![docs.rs](https://img.shields.io/docsrs/hyperlane-plugin-cache?style=flat-square)](https://docs.rs/hyperlane-plugin-cache)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](../../LICENSE)

[Workspace Overview](../../README.md) · [API Docs](https://docs.rs/hyperlane-plugin-cache/latest/) · [Agent Engineering Rules](../../AGENTS.md)

</div>

---

## Overview

Opt-in HTTP response caching middleware with TTL eviction, cache-key customization, and safe-method cache controls.

This crate is a member of the [`hyperlane`](../../README.md) unified Rust 2024 workspace. All plugin crates are opt-in and transparent (`Status::Continue`) when unconfigured.

## Responsibilities & Module Layout

- `struct` / `trait` / `impl` — Cache store trait, TTL configuration, and response caching hooks

## Installation

```sh
cargo add hyperlane-plugin-cache
```

## Verification

Run crate-scoped formatting, lint, and test gates from the workspace root:

```sh
cargo run -p hyperlane-cli -- fmt --check
cargo clippy -p hyperlane-plugin-cache --all-features --all-targets -- -D warnings
cargo test -p hyperlane-plugin-cache --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any public API, plugin contract, or module-boundary change per [`../../AGENTS.md`](../../AGENTS.md).
- Project standards and release workflows live in [`../../.agents/skills/hyperlane/SKILL.md`](../../.agents/skills/hyperlane/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
