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

Three binaries run in containers:

| binary | what it is |
|---|---|
| `bench-server` | the server under test, written against the public API only |
| `bench-loadgen` | a closed-loop keep-alive HTTP/1.1 client that allocates nothing on the hot path |
| `bench-control` | a minimal server with no framework, one read and one write per request |

The control is the reference that makes the other two numbers readable. It
answers how much the machine itself allows, so a difference between two
variants can be separated from a difference in the machine's state. Without it
a throughput number says nothing: it could be the framework being slow, or the
machine being slow.

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

## Running it locally

The same script the workflow runs:

```sh
cd bench
./run-perf.sh build candidate ..
./run-perf.sh bench candidate baseline
./run-perf.sh report
```

`build` takes the build context, which holds two trees: `hyperlane/`, the
library under test, and `bench/`, the harness. To measure a change against the
committed state, build a second context from the base commit and copy the
harness into it:

```sh
mkdir -p /tmp/hyperlane-perf-baseline
git archive "$(git merge-base origin/master HEAD)" | tar -x -C /tmp/hyperlane-perf-baseline
cp -R bench /tmp/hyperlane-perf-baseline/
./run-perf.sh build baseline /tmp/hyperlane-perf-baseline
```

The knobs, all environment variables:

| variable | default | meaning |
|---|---|---|
| `PERF_CONNECTIONS` | `64 256` | the concurrency levels to measure |
| `PERF_DURATION` | `12` | the length of a measured window, in seconds |
| `PERF_WARMUP` | `4` | the length of the warm-up window, in seconds |
| `PERF_REPEATS` | `3` | how many times a level is measured |
| `PERF_SERVER_CPUS` | `4` | the CPU quota of each server container |
| `PERF_LOADGEN_CPUS` | `4` | the CPU quota of each load generator container |
| `PERF_RESULTS` | `bench/results` | where the raw rows are written |

On a GitHub hosted runner, which has four shared vCPUs, the defaults fit. On a
machine with more cores, raise the quotas: the measurement is only meaningful
while the load generator is not the bottleneck, which `docker stats` shows
directly.

The numbers a GitHub runner produces are not comparable with the numbers a
dedicated machine produces. The runner is a shared virtual machine, and the
control probe is what tells the two apart.

## In the pipeline

`.github/workflows/perf.yml` runs the harness on pull requests, on a schedule
and on demand. It builds the candidate from the pull request and the baseline
from the merge-base of `master`, measures both, and prints the table and the
verdict into the job log. It never fails a build because of the verdict.
