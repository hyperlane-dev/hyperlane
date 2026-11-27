<div align="center">

# http-constant

**Shared HTTP Protocol Constants**

[![crates.io](https://img.shields.io/crates/v/http-constant.svg?style=flat-square)](https://crates.io/crates/http-constant)
[![docs.rs](https://img.shields.io/docsrs/http-constant?style=flat-square)](https://docs.rs/http-constant)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/crates/l/http-constant.svg?style=flat-square)](../LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/http-constant/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

Canonical HTTP header names, methods, status phrases, MIME types, and protocol symbols shared across the entire Hyperlane workspace.

This crate is a member of the [`hyperlane`](../README.md) unified Rust 2024 workspace. All plugin crates are opt-in and transparent (`Status::Continue`) when unconfigured.

## Responsibilities & Module Layout

- `header` / `method` / `status` / `version` — Zero-allocation `&str` and byte-slice constants

## Installation

```sh
cargo add http-constant
```

## Verification

Run crate-scoped formatting, lint, and test gates from the workspace root:

```sh
cargo run -p hyperlane-cli -- fmt --check
cargo clippy -p http-constant --all-features --all-targets -- -D warnings
cargo test -p http-constant --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any public API, plugin contract, or module-boundary change per [`../AGENTS.md`](../AGENTS.md).
- Project standards and release workflows live in [`../.agents/skills/hyperlane/SKILL.md`](../.agents/skills/hyperlane/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
