# The levers that did not work

Each entry is a mechanism that was tried, what it cost, and the number that
said so. They are written down so the next session does not spend a rebuild
and a measurement cycle rediscovering them.

## The lever was never the server

**The client was the bottleneck before the server was.** The first smoke
matrix reported 0 requests with 0 errors on the HTTP/2 cleartext row, which
reads as a broken server. It was the client: `run_http2` inferred the protocol
from whether a certificate authority was on the command line, and the runner
always passed one, so the cleartext row quietly spoke TLS. The server was
never asked the question. **Rule: the protocol is passed explicitly; nothing
is inferred from the presence of an unrelated argument.**

**An unpaced short-connection generator exhausts the ephemeral ports.** The
host's port range is 49152-65535, 16384 ports, and the kernel holds a closed
connection in TIME_WAIT for twice the maximum segment lifetime. A generator
that opens one connection per request spends the whole range in seconds and
then fails with `EADDRNOTAVAIL`, which reads as a server accept-loop defect
and is not one. The rate cap is 200 connections per second, and the short rows
use concurrency 16 and 64 rather than 64 and 256. **Rule: a short-connection
row is paced, and the port budget is part of the frozen parameters.**

**A TCP readiness probe cannot tell a bound HTTP/3 listener from a dead one.**
HTTP/3 is carried over UDP, so the probe answered "ready" for a server that
never bound its UDP socket. The readiness check reads the server's own
announcement instead, and the check is per protocol. **Rule: the readiness
marker is the server's own output, never a socket probe.**

## The levers that cost more than they returned

**Pooling the body buffer of the HTTP/2 and HTTP/3 response path.** The pump
allocated an 8 KiB read buffer per response, and the measurement showed it was
the largest single byte cost of the multiplexed path: 8192 of the 20906 bytes
per request. A thread-local pool of buffers removed it, and the byte count per
request fell to 12596. **Kept: the measurement confirmed the mechanism.**

**Rebuilding the nominated header list per response.** `connection_nominated_headers`
built a `Vec<String>` and a lowercased `String` per token on every response,
and the framer then tested every header against that list. The measurement put
it at six allocations per response. Replacing it with a borrowing predicate
removed the list. **Kept: the allocation count fell.**

**Re-validating a header value that is already valid.** The framer called
`http::HeaderValue::from_bytes(value.as_bytes())` on a value that was already
a `HeaderValue`, which re-parses and re-allocates it. The clone of the
original value is a refcount bump. **Kept: the allocation count fell.**

## The levers that were not measured

**A pooled reason phrase.** `Response::build` allocates the reason phrase on
every response whose phrase is empty or `OK`, which is every `200`. The
measurement put it at one allocation of two bytes per response, about five
percent of the HTTP/1.1 path's allocations and a rounding error in its bytes.
Rewriting `HttpStatus::phrase` to a borrowing accessor would touch a public
API of the type crate for a gain the measurement says is small. **Not
attempted: the ratio did not justify the change.**

**A head handed over without the HTTP/1.1 round trip.** The HTTP/2 and
HTTP/3 paths serialise the response as HTTP/1.1 bytes into a duplex pipe and
parse them back, so the head is written once and read once per response. This
is the largest suspected cost of the multiplexed path and the change that
would remove it is an architectural one, not an optimisation: the lifecycle
writes bytes and the framer reads them, and a direct handover would change
that contract. **Not attempted: it is a redesign, and the measured wins of the
three changes above were taken first.**

## The mistakes that cost the most time

**A `git checkout -- bench` destroyed the harness.** The harness was rewritten
in place and never committed, so a restore from `HEAD` deleted the protocol
modules, the runner and the certificate material. The library change survived
because it was in files the restore did not touch. **Rule: a rewritten harness
is committed before the next operation that can revert the tree, and a copy of
it lives outside the repository.**

**A `rm -rf "${context}/bench"` deleted the harness of the repository it was
building.** The runner synchronises the harness into the baseline context so
the two arms differ only in the library. The candidate's context IS the
repository, so the synchronisation removed the source it was about to copy.
The guard now compares the resolved harness path with the source path and
returns before touching anything. **Rule: a path that removes a directory is
checked against the source directory by resolved path, never by string.**

**Editing the runner while the runner was executing it.** Bash reads a script
by byte offset, so inserting text moves everything after it. The cohort that
was running died in a way that looked like a measurement failure. **Rule: the
instrument is frozen before the cohort starts, and the cohort is restarted
rather than continued after the instrument changes.**

**The arm order biased every pair.** The first arm started its server and its
load generator first, so it measured a slightly different machine state than
the second. With the candidate always first, the HTTP/1.1 rows -- which the
optimisation does not touch at all -- came out consistently negative: -17.3,
-3.5 and -2.9 percent, and -10.0, -12.9 and +2.0. A path that no change
touched cannot have regressed, so the bias was in the harness. The order is
now alternated across the repeats, and the same rows come out swinging in
both directions. **Rule: the order of the arms is a parameter of the
measurement, and it is alternated.**


## The harness silently re-resolved the dependencies

**The `|| cargo build --release` fallback hid a lock that did not match its
source.** The baseline context's source carries version 22.0.1 and the
candidate's carries 22.0.3, so the lock copied into the baseline context named
the wrong workspace members. The build tried `--locked`, failed, and fell back
to an unlocked build that resolved again. Nothing reported it: both arms
built, the images differed, and the measurement looked sound.

The arms happened to be safe, because every one of the 152 third-party pins
was identical in both. The fallback was a trap rather than an incident: the
next dependency to move would have separated the arms silently.

The fallback is gone, the lock is authoritative, and the runner rewrites the
workspace members of each context's lock before the build. The arms now
provably differ only in the library: zero harness content differences, and the
same 152 third-party pins on both sides. **Rule: a build that cannot honour
the lock must fail; it must not resolve again.**
