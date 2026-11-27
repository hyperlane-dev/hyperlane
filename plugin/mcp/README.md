<div align="center">

# hyperlane-plugin-mcp

**Model Context Protocol (MCP) Server Plugin**

[![crates.io](https://flat.badgen.net/crates/v/hyperlane-plugin-mcp)](https://crates.io/crates/hyperlane-plugin-mcp)
[![docs.rs](https://img.shields.io/docsrs/hyperlane-plugin-mcp?style=flat-square)](https://docs.rs/hyperlane-plugin-mcp)
[![Rust 2024](https://img.shields.io/badge/edition-2024-orange.svg?style=flat-square)](https://www.rust-lang.org)
[![License: MIT](https://img.shields.io/badge/license-MIT-blue.svg?style=flat-square)](../../LICENSE)

[Workspace Overview](../../README.md) · [API Docs](https://docs.rs/hyperlane-plugin-mcp/latest/) · [Agent Engineering Rules](../../AGENTS.md)

</div>

---

## Overview

Opt-in Streamable HTTP Model Context Protocol server with tools/resources/prompts registries, `Mcp-Session-Id` management, and `Origin` DNS-rebinding protection.

This crate is a member of the [`hyperlane`](../../README.md) unified Rust 2024 workspace. All plugin crates are opt-in and transparent (`Status::Continue`) when unconfigured.

## Responsibilities & Module Layout

- `struct` / `fn` / `impl` — `McpHook`, JSON-RPC session handling, and tool/resource/prompt registries

## Installation

```sh
cargo add hyperlane-plugin-mcp
```

## Verification

Run crate-scoped formatting, lint, and test gates from the workspace root:

```sh
cargo run -p hyperlane-cli -- fmt --check
cargo clippy -p hyperlane-plugin-mcp --all-features --all-targets -- -D warnings
cargo test -p hyperlane-plugin-mcp --all-features
```

## Documentation & Governance

- Update this `README.md` alongside any public API, plugin contract, or module-boundary change per [`../../AGENTS.md`](../../AGENTS.md).
- Project standards and release workflows live in [`../../.agents/skills/hyperlane/SKILL.md`](../../.agents/skills/hyperlane/SKILL.md).

## License

This project is licensed under the MIT License. See the [LICENSE](../../LICENSE) file for details.

## Contact

For inquiries, please contact the author at [root@devhub.vip](mailto:root@devhub.vip).
