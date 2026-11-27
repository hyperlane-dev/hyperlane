<div align="center">

# hyperlane-plugin-gateway

**Reverse Proxy & API Gateway Plugin**

[![crates.io](https://img.shields.io/crates/v/hyperlane-plugin-gateway.svg?style=flat-square)](https://crates.io/crates/hyperlane-plugin-gateway)
[![docs.rs](https://img.shields.io/docsrs/hyperlane-plugin-gateway?style=flat-square)](https://docs.rs/hyperlane-plugin-gateway)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/crates/l/hyperlane-plugin-gateway.svg?style=flat-square)](../../LICENSE)

[Workspace Overview](../../README.md) · [API Docs](https://docs.rs/hyperlane-plugin-gateway/latest/) · [Agent Engineering Rules](../../AGENTS.md)

</div>

---

## Overview

Opt-in reverse proxy and API gateway backed by pooled `http-request` with load balancing, circuit breaking, token-bucket rate limiting, and RFC 9110 hop-by-hop header stripping.

This crate is a member of the [`hyperlane`](../../README.md) unified Rust 2024 workspace. All plugin crates are opt-in and transparent (`Status::Continue`) when unconfigured.

## Responsibilities & Module Layout

- `struct` / `trait` / `impl` — Upstream pools, health/circuit breakers, and `GatewayHook`

## Installation

```sh
cargo add hyperlane-plugin-gateway
```

## Verification

Run crate-scoped formatting, lint, and test gates from the workspace root:

```sh
cargo run -p hyperlane-cli -- fmt --check
cargo clippy -p hyperlane-plugin-gateway --all-features --all-targets -- -D warnings
cargo test -p hyperlane-plugin-gateway --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any public API, plugin contract, or module-boundary change per [`../../AGENTS.md`](../../AGENTS.md).
- Project standards and release workflows live in [`../../.agents/skills/hyperlane/SKILL.md`](../../.agents/skills/hyperlane/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
