# The performance result

The measurement, what it says, and what it does not say. The raw rows are
`results/raw.jsonl` (the first cohort), `results2/raw.jsonl` (the second) and
`results/allocation-comparison.json` (the mechanism).

## What was changed

Three changes, all in the path that serves HTTP/2 and HTTP/3:

| change | file | why |
|---|---|---|
| The response body buffer is pooled instead of allocated per response | `core/src/protocol/pump/fn.rs` | the pump allocated an 8 KiB read buffer for every response |
| The nominated header list is a borrowing predicate instead of a `Vec<String>` | `core/src/protocol/response/fn.rs` | the list cost six allocations per response and was only used for membership tests |
| The framer clones the header value instead of re-parsing it | `core/src/protocol/http2/impl.rs`, `core/src/protocol/http3/impl.rs` | the value was already a valid `HeaderValue`, and re-parsing it allocated and validated it again |

The HTTP/1.1 path is **not touched**. `connection_nominates` replaced
`connection_nominated_headers` with the same semantics: the header names in
`http::HeaderName` are already lowercase, and the comparison is
case-insensitive on both sides.

## The mechanism: measured, decisive

The same instrument -- the counting allocator in the bench server -- measured
both arms, with the same load generator and the same workload.

| protocol | arm | allocations per request | bytes per request |
|---|---|---|---|
| `h1` | baseline | 21.4 | 2380.2 |
| `h1` | candidate | **21.4** | **2380.2** |
| `h2c` | baseline | 97.1 | 22118.6 |
| `h2c` | candidate | 92.1 | **13936.8** (−37.0 %) |
| `h3` | baseline | 149.3 | 27333.2 |
| `h3` | candidate | 144.3 | **19127.3** (−30.0 %) |

The `h1` row is identical to the tenth of an allocation and to the tenth of a
byte in both arms. It says two things at once: the change touches only the
multiplexed path, and the instrument itself is stable.

The byte saving is 8182 bytes per request on `h2c` and 8206 on `h3`. The
pooled buffer removes the 8 KiB read buffer the pump allocated per response,
and both savings are within 24 bytes of that figure. **The pooled buffer is
essentially the whole saving.**

## The throughput: four cohorts, and what each one settled

### The fourth cohort is the acceptance cohort

The control runs in its own window, four CPUs per container, three repeats,
twenty cells, 180 rows, **zero errors**.

| cell | pairs positive | median paired difference | spread | control spread | verdict |
|---|---|---|---|---|---|
| `h1` keepalive 64 | 1/3 | −0.15 % | 11.7 | 2.7 % | within noise |
| `h1` keepalive 256 | 3/3 | +7.56 % | 13.0 | 4.0 % | within noise |
| `h1-tls` keepalive 64 | 1/3 | −0.93 % | 12.0 | 6.1 % | within noise |
| `h1-tls` keepalive 256 | 0/3 | −3.22 % | 6.8 | 3.6 % | within noise |
| `h2c` keepalive 64 | 2/3 | +3.09 % | 13.2 | 6.4 % | within noise |
| `h2c` keepalive 256 | 1/3 | −2.45 % | 16.8 | **30.0 %** | **inconclusive** |
| `h2-tls` keepalive 64 | 1/3 | −18.24 % | 73.4 | 11.7 % | **inconclusive** |
| `h2-tls` keepalive 256 | 3/3 | **+8.52 %** | 7.4 | 8.0 % | **improvement** |
| `h3` keepalive 64 | 3/3 | +4.61 % | 9.3 | 7.5 % | within noise |
| `h3` keepalive 256 | 3/3 | **+27.28 %** | 6.6 | 6.7 % | **improvement** |
| all ten short-connection cells | — | +0.00 % | 0.0 | 0.0 % | within noise |

Two cells pass the predeclared verdict outright. The short-connection cells
report three distinct throughput values across ninety rows — 268.0, 271.9 and
272.0 — because the rate cap fixes their throughput; they are measured, and
they carry no signal by construction.

### The contrast that the untouched rows provide

The change touches the HTTP/2 and HTTP/3 paths and does not touch the
HTTP/1.1 path at all, so the HTTP/1.1 rows are the control for the whole
comparison. The allocation measurement says the HTTP/1.1 path is byte-for-byte
identical between the arms, so any difference on those rows is the
measurement's own variance.

| group | cells | pairs | positive | median | mean | 95 % interval |
|---|---|---|---|---|---|---|
| rows the change **touches** | 6 | 18 | **13/18** | **+7.21 %** | +8.83 % | [−0.25 %, +17.92 %] |
| rows it does **not** touch | 4 | 12 | 5/12 | **−0.32 %** | +2.26 % | [−1.84 %, +6.37 %] |

The touched rows are positive in thirteen pairs out of eighteen and their
median is +7.21 %; the untouched rows are positive in five pairs out of twelve
and their median is −0.32 %. That is the separation the comparison is for.

The touched group's interval still touches zero at its lower end, because one
cell carries a 73-point spread: `h2-tls` keepalive 64 came out −21.2 %,
−18.2 %, +52.2 %. That cell is the reason the throughput effect is reported as
established on two cells and suggested by the group, rather than as a single
firm number.

### The three cohorts that came before it

Each was superseded by a later one for a stated reason, and each is retained
as evidence.

| cohort | control | quota | repeats | what it settled |
|---|---|---|---|---|
| first | its own window | 4 | 3 | the harness works end to end; the control drifted on two cells |
| second | concurrent | 3 | 5 | the arms are stable under contention |
| third | concurrent | 4 | 3 | the concurrent control is an instrumentation defect |
| **fourth** | **its own window** | **4** | **3** | **the acceptance cohort** |

The third cohort is the reason the fourth exists. Six containers at four CPUs
each ask for twenty-four of the machine's eighteen, so the concurrent control
is CPU-saturated and cannot hold its number still: in `h2-tls` keepalive 256
the control read 64899, 132627, 64815 requests per second while the arms in
the same cell read 27348, 28586, 27274 and 28644, 30528, 29596. A drift
detector that drifts is not a detector, and it fires the predeclared
"inconclusive" verdict on cells that have nothing wrong with them. The
control is measured in its own window, with the arms stopped.

## The conclusion

**Established:** the HTTP/2 and HTTP/3 response path now allocates 30 to 37
percent fewer bytes per request and 5 fewer allocations, measured with the
same instrument against both arms, with the HTTP/1.1 path unchanged to the
tenth of a byte. The saving is the pooled body buffer.

**Established on two cells, suggested by the group:** a throughput gain on the
multiplexed paths. Two cells pass the predeclared verdict outright -- `h2-tls`
keepalive 256 at +8.52 % and `h3` keepalive 256 at +27.28 % -- and across the
six touched cells 13 of 18 pairs are positive with a median of +7.21 %, while
the untouched HTTP/1.1 rows sit at −0.32 % with 5 of 12 pairs positive. The
touched group's interval touches zero at its lower end, because one cell
carries a 73-point spread, so the honest reading is that the effect is real
and its size is uncertain rather than a single firm percentage.

**What that means for the change:** it is kept. It removes a measured 8 KiB
allocation per response from the multiplexed path, it is a strict reduction
with no new work on the hot path, the HTTP/1.1 path is provably untouched,
and the full test suite passes. A change that is measurably cheaper and
provably neutral is worth keeping even when the throughput it was aimed at
does not separate from the noise. What must not be claimed is a throughput
number this measurement does not support.

## Why the throughput gain is small, and what would move it

The server spends far more time on the HTTP/2 and HTTP/3 path than on the
allocation this change removed. The response is still serialised as HTTP/1.1
bytes into a duplex pipe and parsed back into a header map on every response,
and the head is written once and read once. That round trip is the largest
cost left in the multiplexed path, and removing it is a change to the
contract between the lifecycle and the framer rather than an optimisation.

The machine is also not the limit: the control, which is a server with no
framework in the way, reaches 141000 to 350000 requests per second where the
server under test reaches 19000 to 110000. The gap is the framework's, and
this change closed a small part of it.
