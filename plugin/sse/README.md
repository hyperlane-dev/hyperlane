<div align="center">

# hyperlane-plugin-sse

**WHATWG Server-Sent Events (SSE) Plugin**

[![crates.io](https://img.shields.io/crates/v/hyperlane-plugin-sse.svg?style=flat-square)](https://crates.io/crates/hyperlane-plugin-sse)
[![docs.rs](https://img.shields.io/docsrs/hyperlane-plugin-sse?style=flat-square)](https://docs.rs/hyperlane-plugin-sse)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/crates/l/hyperlane-plugin-sse.svg?style=flat-square)](../../LICENSE)

[Workspace Overview](../../README.md) · [API Docs](https://docs.rs/hyperlane-plugin-sse/latest/) · [Agent Engineering Rules](../../AGENTS.md)

</div>

---

## Overview

Opt-in SSE streaming with CRLF-safe event framing, topic pub/sub, `Last-Event-ID` bounded replay buffers, and periodic heartbeats.

This crate is a member of the [`hyperlane`](../../README.md) unified Rust 2024 workspace. All plugin crates are opt-in and transparent (`Status::Continue`) when unconfigured.

## Responsibilities & Module Layout

- `struct` / `fn` / `impl` — Topic hub, replay ring buffer, and SSE response streaming

## Installation

```sh
cargo add hyperlane-plugin-sse
```

## Verification

Run crate-scoped formatting, lint, and test gates from the workspace root:

```sh
cargo run -p hyperlane-cli -- fmt --check
cargo clippy -p hyperlane-plugin-sse --all-features --all-targets -- -D warnings
cargo test -p hyperlane-plugin-sse --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any public API, plugin contract, or module-boundary change per [`../../AGENTS.md`](../../AGENTS.md).
- Project standards and release workflows live in [`../../.agents/skills/hyperlane/SKILL.md`](../../.agents/skills/hyperlane/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
