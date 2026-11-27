<div align="center">

# hyperlane-plugin-cors

**RFC-Compliant CORS Middleware Plugin**

[![crates.io](https://flat.badgen.net/crates/v/hyperlane-plugin-cors)](https://crates.io/crates/hyperlane-plugin-cors)
[![docs.rs](https://img.shields.io/docsrs/hyperlane-plugin-cors?style=flat-square)](https://docs.rs/hyperlane-plugin-cors)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](../../LICENSE)

[Workspace Overview](../../README.md) · [API Docs](https://docs.rs/hyperlane-plugin-cors/latest/) · [Agent Engineering Rules](../../AGENTS.md)

</div>

---

## Overview

Opt-in Cross-Origin Resource Sharing hook handling preflight `OPTIONS`, origin allowlists, credentials, exposed headers, and max-age policies.

This crate is a member of the [`hyperlane`](../../README.md) unified Rust 2024 workspace. All plugin crates are opt-in and transparent (`Status::Continue`) when unconfigured.

## Responsibilities & Module Layout

- `struct` / `enum` / `impl` — Origin/method/header policy builders and preflight short-circuit hook

## Installation

```sh
cargo add hyperlane-plugin-cors
```

## Verification

Run crate-scoped formatting, lint, and test gates from the workspace root:

```sh
cargo run -p hyperlane-cli -- fmt --check
cargo clippy -p hyperlane-plugin-cors --all-features --all-targets -- -D warnings
cargo test -p hyperlane-plugin-cors --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any public API, plugin contract, or module-boundary change per [`../../AGENTS.md`](../../AGENTS.md).
- Project standards and release workflows live in [`../../.agents/skills/hyperlane/SKILL.md`](../../.agents/skills/hyperlane/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
