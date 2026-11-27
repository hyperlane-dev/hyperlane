<div align="center">

# hyperlane-plugin-security

**HTTP Security Headers & CSRF Guard Plugin**

[![crates.io](https://img.shields.io/crates/v/hyperlane-plugin-security.svg?style=flat-square)](https://crates.io/crates/hyperlane-plugin-security)
[![docs.rs](https://img.shields.io/docsrs/hyperlane-plugin-security?style=flat-square)](https://docs.rs/hyperlane-plugin-security)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/crates/l/hyperlane-plugin-security.svg?style=flat-square)](../../LICENSE)

[Workspace Overview](../../README.md) · [API Docs](https://docs.rs/hyperlane-plugin-security/latest/) · [Agent Engineering Rules](../../AGENTS.md)

</div>

---

## Overview

Opt-in security middleware injecting CSP, HSTS, `X-Content-Type-Options`, `X-Frame-Options`, `Referrer-Policy`, and origin/CSRF protections.

This crate is a member of the [`hyperlane`](../../README.md) unified Rust 2024 workspace. All plugin crates are opt-in and transparent (`Status::Continue`) when unconfigured.

## Responsibilities & Module Layout

- `struct` / `enum` / `trait` / `impl` — Security policy presets and response header hook

## Installation

```sh
cargo add hyperlane-plugin-security
```

## Verification

Run crate-scoped formatting, lint, and test gates from the workspace root:

```sh
cargo run -p hyperlane-cli -- fmt --check
cargo clippy -p hyperlane-plugin-security --all-features --all-targets -- -D warnings
cargo test -p hyperlane-plugin-security --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any public API, plugin contract, or module-boundary change per [`../../AGENTS.md`](../../AGENTS.md).
- Project standards and release workflows live in [`../../.agents/skills/hyperlane/SKILL.md`](../../.agents/skills/hyperlane/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
