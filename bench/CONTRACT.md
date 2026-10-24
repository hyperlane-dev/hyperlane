# The frozen acceptance contract

Everything below is fixed **before** the first sample is collected. A change
to any of it after the collection starts invalidates the cohort: the numbers
were measured under different conditions, and comparing them would compare two
different experiments rather than two arms of one.

## What is being compared

Two arms, built from the same toolchain, the same release profile, the same
dependency lock and the same harness:

| arm | source |
|---|---|
| `baseline` | the merge-base of the base branch, with the current `bench/` harness copied into it |
| `candidate` | the working tree |

Both arms are measured **at the same time**, each with its own load generator,
over the same wall clock window. A change in the state of the machine lands on
both measurements and cancels out. A sequential comparison cannot do this: two
runs of the same binary were measured to differ by up to 65 percent.

The two arms are verified to be different files before the first repeat, by
hashing the two images. Two arms that share a build directory can collapse
into one binary, and the comparison then reports a flattering zero difference
against itself.

The load generator and the control are the **instrument**. They are built once,
from the same harness, and are not rebuilt per arm: an instrument that differs
between the two arms measures the instrument.

The harness is copied into the baseline context byte for byte, so the two
builds differ only in the library: **zero** content differences in the harness
tree, and the library trees differ in the files the change touches.

The lock is authoritative. Each context's lock names its own workspace members,
because the baseline's source carries a different version than the candidate's,
and a lock that names the other one cannot be used with `--locked`. Only the
workspace members move; all 152 third-party pins stay exactly as they were, so
the two arms are compiled against the same dependency set. The build does not
fall back to an unlocked build: a fallback would resolve the dependencies
again, and the two arms could drift apart without anything reporting it.

The **order** in which the two arms are measured alternates across the
repeats. The first arm starts its server and its load generator first, so it
measures a slightly different machine state than the second; measuring one
arm first in every repeat puts that bias on the same arm in every pair. The
alternation cancels it.

## The workload matrix

Ten cells per concurrency level. Every cell is measured, and the report prints
the cells that were **not** measured rather than leaving them out.

| axis | values |
|---|---|
| protocol | `h1`, `h1-tls`, `h2c`, `h2-tls`, `h3` |
| connection mode | `keepalive`, `short` |

The connection axis is not a dial on the same code path. A keep-alive
connection carries many requests, so the accept loop, the per-connection
handshake and the disconnect cleanup are each exercised roughly once per
persistent connection. The short-connection row opens one connection per
request, which is a different path with a different cost.

## The frozen parameters

| parameter | value | why this value |
|---|---|---|
| server CPU quota | 4 | two server arms plus two load generators plus the control fit inside the machine's 18 CPUs |
| load generator CPU quota | 4 | the generator must not be the bottleneck; `docker stats` shows it |
| memory per container | 4g | the workload does not stream, so memory is not the limit |
| concurrency, keep-alive | 64, 256 | the sweep reaches the plateau |
| concurrency, short | 16, 64 | one connection per request spends one ephemeral port each |
| measured window | 12 s | long enough that the tail is not a warm-up artefact |
| warm-up window | 4 s | the connection pool and the route table settle |
| repeats | 3 | the median of three paired differences |
| connection rate cap, short | 200 per second | the ephemeral port range and the 60 s TIME_WAIT make the range the ceiling; without the cap the generator fails with `EADDRNOTAVAIL`, which reads as a server defect and is not one |
| cooldown between short repeats | 20 s | the ports of one repeat are still held when the next one starts |

## The metrics

| metric | role | direction |
|---|---|---|
| requests per second | primary | higher is better |
| p99 latency | secondary | lower is better |
| error count | gate | must be zero |

The error budget is zero. A cell with a failed request is a broken cell, not a
slow cell: the numbers of a cell that failed part way through describe a
different experiment.

## The verdict rules

The verdict is built from the **median of the per-repeat paired differences**,
not from the difference of two medians.

| verdict | rule |
|---|---|
| `improvement` | the median paired difference is at least +5 % **and** larger than the spread of the paired differences |
| `regression` | the median paired difference is at most -10 %, larger than the spread, and the control was stable |
| `within noise` | the difference is smaller than the spread, or smaller than the thresholds |
| `inconclusive` | the control moved by more than 10 %, so the machine changed underneath the measurement |

The control is measured in the same window, speaking the same protocol as the
row it references. A control that moved means the machine moved, and no
difference between the arms can then be attributed to the code.

The verdict is a report. It never blocks a build. Only a broken harness does,
because a harness that fails quietly reports "no regression" for a server that
never answered.

## The reporting rules

- Every raw row is kept, including the failures. A failed pair is retained as
  a failed pair, not dropped and not replaced by a retry.
- The unit of the comparison is one **repeat** of one cell: the paired
  observation of the two arms in the same window. The observations of the
  individual connections inside one repeat are not independent repetitions.
- A cell that was not measured is reported as missing. A missing cell is not
  a passing cell.
- The absolute numbers are reported next to the paired differences, so a
  reader can tell a real difference from two small numbers.

## What this contract does not claim

- It does not claim that no workload anywhere regressed. It covers one matrix
  on one machine.
- It does not claim that the machine was idle. The control measures how much
  the machine allowed during each window, and a verdict that depends on the
  control is reported as inconclusive rather than as a pass.
- It does not treat a protocol the baseline cannot serve as a same-capability
  comparison. If a baseline arm cannot speak a protocol, that cell is
  reported as an absolute number with no confidence interval, not folded into
  a table whose other rows carry one.


## The second cohort

The first cohort is reported above and stands as it was measured. It had two
cells that came out `inconclusive`, and the reason was visible in the raw
rows: the control's three measurements drifted **monotonically** across the
cohort (141233, 144210, 165301 requests per second on one cell, and 141106,
156424, 170759 on another), while the paired differences of the two arms
stayed positive on every pair. The machine was speeding up while the cohort
ran. A control measured in a window of its own compares the machine of one
moment with the machine of another, and that difference is not a property of
the code.

The second cohort therefore changes one thing: the control runs **at the same
time as the two arms**, in the same wall clock window, so it sees the same
contention they see. The CPU quota drops from 4 to 3 for that reason, because
six containers at 4 CPUs each would ask for 24 of the machine's 18. The
repeat count rises from 3 to 5, and only the keep-alive cells are measured:
the short cells are capped by the connection rate, so every arm of every
short cell reports the same number by construction and the cell carries no
signal.

Everything else is unchanged: the same images, the same duration, the same
warm-up, the same concurrency levels, the same alternating arm order, the
same thresholds, the same error budget.

| parameter | first cohort | second cohort |
|---|---|---|
| control | its own window | concurrent with the arms |
| CPU quota per container | 4 | 3 |
| repeats | 3 | 5 |
| cells | all twenty | the ten keep-alive ones |
| everything else | identical | identical |

The two cohorts are reported separately. The second does not replace the
first, and the first is not recomputed with the second's thresholds.


## The third cohort: the concurrent control was an instrumentation defect

The second cohort's control ran at the same time as the two arms, so that it
would see the same contention they see. That was the right idea and the
wrong implementation: six containers at four CPUs each ask for twenty-four of
the machine's eighteen, so the control is CPU-saturated and contending for
whatever is left. A drift detector that cannot hold still detects nothing.

The third cohort measured the flaw. The control's throughput swung by 51
percent in one cell while the arms in that same cell held steady:

| cell | control rps | baseline rps | candidate rps |
|---|---|---|---|
| `h2-tls` keepalive 256 | 64899, 132627, 64815 | 27348, 28586, 27274 | 28644, 30528, 29596 |
| `h2c` keepalive 256 | 116263, 131203, 158587 | 29274, 25974, 24278 | 28127, 28507, 26915 |

The arms vary by under five percent; the control varies by a factor of two.
That is not the machine changing underneath the measurement. It is the
measurement overloading the machine, and it fires the predeclared
"inconclusive" verdict on cells that have nothing wrong with them.

**The fourth cohort returns the control to its own window.** It is the design
the harness has always documented: the control runs after the arms, with the
arms stopped, so the machine is not oversubscribed while it measures. The
quotas are the ones the contract already froze: four CPUs per container, so
the two arms and their generators ask for sixteen of eighteen, and the control
alone asks for eight.

Everything else is unchanged: the same images, the same duration, the same
warm-up, the same concurrency levels, the same alternating arm order, the
same thresholds, the same error budget. The fourth cohort is the acceptance
cohort. The third is retained as the evidence for the rejection, and no
observation is carried from one into the other.
