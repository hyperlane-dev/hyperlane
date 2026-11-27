---
name: hyperlane
description: 'Use when implementing or debugging the Hyperlane Rust HTTP framework: server routes, lifecycle hooks, Context, request targets, the plugin crates, or the perf harness. Publishing to ltpp.vip uses hyperlane-upload instead.'
license: MIT
metadata:
  hermes:
    related_skills: [rust-standards, git-standards]
---

# Hyperlane workspace standards and workflow

## Mandatory Standards

1. Load `rust-standards` before editing any `.rs` or `Cargo.toml` file. Resolve authoritative crate membership and versions from `Cargo.toml` / `cargo metadata --no-deps --format-version 1`.
2. Enforce core `ServerHook` and HTTP wire invariants (`AGENTS.md`):
  - `Server::route::<Handler>(path)` and middleware registrations are synchronous (`&mut Self`; never `.await`).
  - `Server::run().await` returns `Result<ServerControlHook, Box<ServerError>>`; propagate errors and never mask bind failures.
  - `ServerHook::new(stream, ctx)` / `handle(self, stream, ctx)` take `stream` first; use `ctx.build_response()` for HEAD-aware serialization; send bytes before returning `Status::Reject`.
  - Maintain RFC 9110/9112 wire conformance: bodyless 1xx/204/205/304 handling, bounded chunked decoding, `Expect: 100-continue`, request smuggling rejection, CRLF validation, decompression bomb caps (`64 MiB`), and keep-alive `Context` resets.

3. Enforce workspace boundaries & visibility:
  - Keep `hyperlane-core` internals (`listener`, `protocol`, `tls`) `pub(crate)` and put wire/transport fixes in `core/src/protocol/`, never in route handlers.
  - Keep public struct fields `pub(crate)` with generated `get_*`/`set_*` accessors; keep plugins opt-in and transparent (`Status::Continue`) when unconfigured.
  - Register every new publishable plugin in root `Cargo.toml` `[workspace] members` and `.github/workflows/rust.yml` `PACKAGES`.

4. Keep `hyperlane fmt` layered on `crate-cli`: `hyperlane fmt` MUST invoke `crate_cli::execute_unified_fmt` (Prettier-aligned `.md`/`.yml`/`.json`/`.toml` + derives + `cargo fmt`) first, then Hyperlane attribute normalization and `cargo fmt --all`.

## Verification Workflow

Run every gate locally before committing or pushing:

```sh
cargo run -p hyperlane-cli -- fmt
cargo run -p hyperlane-cli -- fmt
cargo run -p hyperlane-cli -- fmt --check
cargo fmt --all -- --check
cargo clippy --workspace --all-features --all-targets -- -D warnings
cargo test --workspace --all-features
cargo check --release --workspace --all-features
python3 ~/.agents/skills/rust-standards/scripts/staged_file_gate.py .
python3 ~/.agents/skills/rust-standards/scripts/verify_dep_order.py .
```

Always exercise real request/response wire bytes for protocol changes and reclaim all task-owned listeners/processes on exit.

## Release & Downstream Cascade Workflow

`hyperlane` is **Tier 2** (depends on published `ctares`; consumed by `hyperlane-quick-start`, `commerce-suite`, and `web-frameworks`):

1. Wait until upstream `ctares` is live on `crates.io`, then upgrade `crate-cli`, `color-output`, `lombok-macros`, and `tokio-broadcast` in root `Cargo.toml`.
2. Bump `[workspace.package].version` and internal `[workspace.dependencies]` pins in root `Cargo.toml`.
3. Run `hyperlane fmt` so `.github/workflows/*.yml` are pre-formatted (preventing CI's `GITHUB_TOKEN` from attempting workflow pushes during `publish`).
4. Validate commit/PR text via `verify_english_only.py`, push/merge to `master`, and wait until `.github/workflows/rust.yml` (`setup`, `check`, `tests`, `clippy`, `build`, `publish`, `release`) finishes green on `crates.io`.
5. Cascade upgrades to Tier 3 (`hyperlane-quick-start`, `commerce-suite`) and Tier 4 (`web-frameworks`).

## Documentation & Skill Governance

- Update root `README.md`, `<crate>/README.md`, `plugin/README.md`, and `AGENTS.md` in the same commit whenever adding/changing crates, plugins, wire invariants, CLI subcommands, or CI jobs.
- Keep `.agents/skills/hyperlane/SKILL.md` and `~/.agents/skills/hyperlane/SKILL.md` synchronized and strictly limited to standards, requirements, and workflows (`<= 160` lines); run `audit_skills.py` and `update_skill_lock.py` after edits.

## Reference Index

Consult cached references only as secondary lookup (source and compiler remain authoritative; ltpp.vip uploads use [hyperlane-upload](../hyperlane-upload/SKILL.md)): [Core](references/api-core.md) · [Macros](references/api-macros.md) · [HTTP Types](references/api-type.md) · [Client](references/api-request.md) · [Compression](references/api-compress.md) · [Constants](references/api-constant.md) · [Negotiate](references/plugin-negotiate.md) · [Plugins](references/plugins.md) · [Wire](references/wire-behaviour.md) · [WS](references/hyperlane-plugin-websocket.md) · [SSE](references/sse.md) · [Bench](references/bench.md) · [Release](references/release-bump-flow.md) · [Ownership](references/request-path-ownership.md) · [Visibility](references/field-visibility.md).
