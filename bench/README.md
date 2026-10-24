# The hyperlane performance harness

This directory holds the load test that measures how the HTTP server behaves
under concurrency. It exists because a performance claim is only worth what the
measurement behind it is worth, and because a measurement that cannot be
repeated is a rumour.

The harness is a separate Cargo workspace on purpose. It is not a member of the
root workspace, so `cargo test --workspace`, `cargo clippy --workspace` and
`cargo fmt --all` in the root never build it, and the release job's
`crate publish` cannot publish it. Every crate here carries `publish = false`
for the same reason.

## What it measures

Three binaries run in containers, with a local Rust runner orchestrating them:

| binary | what it is |
|---|---|
| `bench-server` | the server under test, written against the public API only, speaking the protocol the row names |
| `bench-loadgen` | a closed-loop client that speaks every protocol of the matrix, natively |
| `bench-control` | a minimal server with no framework, one read and one write per request |
| `bench-perf` | the local orchestrator, certificate generator and report renderer |

The control speaks the same protocol as the row it references. A control that
spoke HTTP/1.1 could not say what the machine allowed for HTTP/3, so the
control would be an incomparable number in every row but one.

The control is the reference that makes the other two numbers readable. It
answers how much the machine itself allows, so a difference between two
variants can be separated from a difference in the machine's state. Without it
a throughput number says nothing: it could be the framework being slow, or the
machine being slow.

## The matrix

The measurement is a grid, not a single row. The protocol axis is the wire
protocol the server speaks, and the connection axis is whether a connection
carries many requests or exactly one.

| | `keepalive` | `short` |
|---|---|---|
| `h1` | HTTP/1.1 over a plain socket | one connection per request |
| `h1-tls` | HTTP/1.1 over TLS | one connection per request |
| `h2c` | HTTP/2 with prior knowledge | one connection per request |
| `h2-tls` | HTTP/2 over TLS | one connection per request |
| `h3` | HTTP/3 over QUIC | one connection per request |

The connection axis is not a dial on the same code path. A keep-alive
connection carries many requests, so the accept loop, the per-connection
handshake and the disconnect cleanup are each exercised roughly once per
persistent connection. A short connection exercises them once per request,
and it spends one ephemeral port per request as well, so the generator caps
its connection rate: without the cap the kernel runs out of ports and the
run fails with `EADDRNOTAVAIL`, which reads as a server defect and is not one.

Every protocol is driven by its own client library. `curl` cannot speak
HTTP/2 without TLS, cannot speak HTTP/2 in cleartext, and cannot speak
HTTP/3 at all, so the harness uses `h2`, `h3` and `quinn` directly.

## How the comparison is made

Both variants run **at the same time**, each with its own load generator, over
the same wall clock window. A change in the state of the machine lands on both
measurements and cancels out.

A sequential comparison, where one variant runs and then the other does, was
tried and rejected: two runs of the same binary were measured to differ by up
to 65 percent, which is larger than any real difference the comparison is
meant to detect. The conclusion is therefore built from the **median of the
per-repeat paired differences**, not from the difference of two medians.

The verdict is printed. It never blocks a build. Only a broken harness does,
because a harness that fails quietly would report "no regression" for a server
that never answered.

| verdict | meaning |
|---|---|
| `improvement` | the median paired difference is at least +5 % and larger than the measured spread |
| `regression` | the median paired difference is at most -10 %, larger than the spread, and the control was stable |
| `within noise` | the difference is smaller than the spread, or smaller than the thresholds |
| `inconclusive` | the control moved, so the machine changed underneath the measurement |

## The certificate material

The TLS rows need a certificate chain. The material is **generated**, not
committed: a private key in a repository is a private key in every clone of
that repository, and this one is a throwaway that anyone could use to
impersonate the benchmark server.

```sh
cargo run -q -p bench-perf -- certificates certs
```

The generator makes a certificate authority and one leaf certificate. The leaf
carries the names the harness addresses the server by, and the clients verify
the chain against the authority explicitly: the authority is the trust
anchor, and the leaf is the end entity. The authority's signing key never
leaves the generator's temporary directory.

## Running it locally

The same Rust runner the workflow runs:

```sh
cd bench
cargo run -q -p bench-perf -- instrument ..
cargo run -q -p bench-perf -- build candidate ..
cargo run -q -p bench-perf -- bench candidate baseline
cargo run -q -p bench-perf -- report
```

`instrument` builds the load generator and the control. They are the
measuring instrument, so they are built once and are not rebuilt per arm: an
instrument that differs between the two arms measures the instrument.

`build` takes the build context, which holds two trees: `hyperlane/`, the
library under test, and `bench/`, the harness. To measure a change against the
committed state, build a second context from the base commit and copy the
harness into it:

```sh
mkdir -p /tmp/hyperlane-perf-baseline
git archive "$(git merge-base origin/master HEAD)" | tar -x -C /tmp/hyperlane-perf-baseline
cargo run -q -p bench-perf -- build baseline /tmp/hyperlane-perf-baseline
```

The build copies the current harness into the baseline context, so the two
images differ only in the library under test. The runner also checks that the
two images are different files before the first repeat: two arms that share a
build directory can collapse into one image, and the comparison would then
measure one build against itself.

The knobs, all environment variables:

| variable | default | meaning |
|---|---|---|
| `PERF_PROTOCOLS` | `h1 h1-tls h2c h2-tls h3` | the protocol rows to measure |
| `PERF_MODES` | `keepalive short` | the connection rows to measure |
| `PERF_CONNECTIONS` | `64 256` | the concurrency levels of the keep-alive rows |
| `PERF_CONNECTIONS_SHORT` | `16 64` | the concurrency levels of the short rows |
| `PERF_DURATION` | `12` | the length of a measured window, in seconds |
| `PERF_WARMUP` | `4` | the length of the warm-up window, in seconds |
| `PERF_REPEATS` | `3` | how many times a cell is measured |
| `PERF_RATE_SHORT` | `200` | the connection rate cap of the short rows, per second |
| `PERF_COOLDOWN_SHORT` | `20` | the pause between two short rows, in seconds |
| `PERF_SERVER_CPUS` | `4` | the CPU quota of each server container |
| `PERF_LOADGEN_CPUS` | `4` | the CPU quota of each load generator container |
| `PERF_MEMORY` | `4g` | the memory limit of each container |
| `PERF_RESULTS` | `bench/results` | where the raw rows are written |

The frozen parameters and verdict rules are fixed before the first sample,
because a cohort measured under changing conditions compares two experiments
rather than two arms of one.

On a GitHub hosted runner, which has four shared vCPUs, the defaults fit. On a
machine with more cores, raise the quotas: the measurement is only meaningful
while the load generator is not the bottleneck, which `docker stats` shows
directly.

The numbers a GitHub runner produces are not comparable with the numbers a
dedicated machine produces. The runner is a shared virtual machine, and the
control probe is what tells the two apart.

## In the pipeline

`.github/workflows/perf.yml` runs the harness on every branch push. It builds
the candidate from that commit and the baseline from its parent, measures both,
and prints the table and the verdict into the job log. It never fails a build
because of the verdict.
