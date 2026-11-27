<div align="center">

# hyperlane-cli

**Hyperlane Developer CLI & Unified Formatter**

[![crates.io](https://img.shields.io/crates/v/hyperlane-cli.svg?style=flat-square)](https://crates.io/crates/hyperlane-cli)
[![docs.rs](https://img.shields.io/docsrs/hyperlane-cli?style=flat-square)](https://docs.rs/hyperlane-cli)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/crates/l/hyperlane-cli.svg?style=flat-square)](../LICENSE)

[Workspace Overview](../README.md) · [API Docs](https://docs.rs/hyperlane-cli/latest/) · [Agent Engineering Rules](../AGENTS.md)

</div>

---

## Overview

Command-line toolchain providing `hyperlane fmt` (delegating to `crate-cli` unified Prettier-aligned formatting followed by Hyperlane attribute normalization), `watch`, `new`, and `template` scaffolding.

This crate is a member of the [`hyperlane`](../README.md) unified Rust 2024 workspace. All plugin crates are opt-in and transparent (`Status::Continue`) when unconfigured.

## Responsibilities & Module Layout

- `fmt` — `execute_fmt` running `crate_cli::execute_unified_fmt` first then Hyperlane attribute formatting
- `watch` / `new` / `template` — Live cargo-watch dev loop and project/component scaffolding

## Installation

```sh
cargo add hyperlane-cli
```

## Verification

Run crate-scoped formatting, lint, and test gates from the workspace root:

```sh
cargo run -p hyperlane-cli -- fmt --check
cargo clippy -p hyperlane-cli --all-features --all-targets -- -D warnings
cargo test -p hyperlane-cli --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any public API, plugin contract, or module-boundary change per [`../AGENTS.md`](../AGENTS.md).
- Project standards and release workflows live in [`../.agents/skills/hyperlane/SKILL.md`](../.agents/skills/hyperlane/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
