#!/usr/bin/env python3
"""Turn the harness measurements into a table and a conclusion.

The measurement drives both variants at the same time, so the two numbers of
one repeat were taken under the same machine conditions. The conclusion is
built from the per-repeat paired differences, because a difference of two
medians is not the same thing: a sequential comparison was measured to differ
by up to 65 percent on identical code, while the paired one holds up.

The control server runs in the same window and answers how much the machine
itself allows. When the control moved, the machine moved, and a difference
between the variants cannot be attributed to the code.

This script never fails a build because of a verdict. It exits non-zero only
when the measurement itself is broken, so that a broken harness cannot be
mistaken for a result.
"""

import json
import statistics
import sys
from collections import defaultdict

# The median paired difference above which a change is called an improvement.
IMPROVEMENT_THRESHOLD = 5.0

# The median paired difference below which a change is called a regression.
REGRESSION_THRESHOLD = -10.0

# The share of the control's throughput that may move before the measurement is
# treated as inconclusive.
CONTROL_TOLERANCE = 10.0


def load(path):
    """Reads the measurement rows.

    Arguments:
        path - The file the rows are in.

    Returns:
        The rows, or an empty list when the file is missing.
    """
    rows = []
    try:
        with open(path, "r", encoding="utf-8") as handle:
            for line in handle:
                line = line.strip()
                if line:
                    rows.append(json.loads(line))
    except FileNotFoundError:
        return []
    return rows


def group(rows):
    """Groups the rows by concurrency level and variant.

    Arguments:
        rows - The measurement rows.

    Returns:
        The rows keyed by level and variant, and by repeat.
    """
    grouped = defaultdict(lambda: defaultdict(dict))
    for row in rows:
        grouped[row["connections"]][row["variant"]][row["repeat"]] = row["result"]
    return grouped


def median(values):
    """The median of the values.

    Arguments:
        values - The values.

    Returns:
        The median, or zero when there are no values.
    """
    if not values:
        return 0.0
    return float(statistics.median(values))


def describe(label, results):
    """Prints one line per variant.

    Arguments:
        label - The level the line belongs to.
        results - The results of one variant at that level.
    """
    rps = [result["rps"] for result in results.values()]
    p50 = [result["p50_us"] for result in results.values()]
    p99 = [result["p99_us"] for result in results.values()]
    errors = sum(result["errors"] for result in results.values())
    print(
        f"  {label:>10}  rps med={median(rps):>10.1f}  best={max(rps):>10.1f}  "
        f"worst={min(rps):>10.1f}  p50={median(p50):>7.0f}us  p99={median(p99):>8.0f}us  errors={errors}"
    )


def verdict_for(level, baseline, candidate, control):
    """The conclusion for one concurrency level.

    Arguments:
        level - The concurrency level.
        baseline - The baseline results, keyed by repeat.
        candidate - The candidate results, keyed by repeat.
        control - The control results, keyed by repeat.

    Returns:
        The verdict and the reason behind it.
    """
    shared = sorted(set(baseline) & set(candidate))
    if not shared:
        return "inconclusive", "the two variants were not measured together"
    deltas = []
    for repeat in shared:
        base = baseline[repeat]["rps"]
        cand = candidate[repeat]["rps"]
        if base > 0:
            deltas.append((cand - base) / base * 100.0)
    if not deltas:
        return "inconclusive", "no repeat produced a usable pair"
    median_delta = statistics.median(deltas)
    spread = max(deltas) - min(deltas)
    detail = (
        f"median paired difference {median_delta:+.2f}%, spread {spread:.1f} points"
    )
    if control:
        control_rps = [result["rps"] for result in control.values()]
        if len(control_rps) > 1:
            control_spread = (
                (max(control_rps) - min(control_rps)) / max(control_rps) * 100.0
            )
            if control_spread > CONTROL_TOLERANCE:
                return (
                    "inconclusive",
                    f"the control moved {control_spread:.1f}%, the machine changed",
                )
    if abs(median_delta) <= spread:
        return "within noise", detail
    if median_delta >= IMPROVEMENT_THRESHOLD:
        return "improvement", detail
    if median_delta <= REGRESSION_THRESHOLD:
        return "regression", detail
    return "within noise", detail


def main():
    """Prints the table and the conclusion.

    Returns:
        Zero when the measurement is usable, one when it is not.
    """
    if len(sys.argv) < 2:
        print("usage: summarize.py <raw.jsonl>", file=sys.stderr)
        return 2
    rows = load(sys.argv[1])
    if not rows:
        print("no measurements were found", file=sys.stderr)
        return 1
    grouped = group(rows)
    levels = sorted(grouped.keys())
    print("hyperlane performance report")
    print("===========================")
    for level in levels:
        per_variant = grouped[level]
        print(f"\nconcurrency {level}")
        for variant in ("baseline", "candidate", "control"):
            results = per_variant.get(variant, {})
            if results:
                describe(variant, results)
        baseline = per_variant.get("baseline", {})
        candidate = per_variant.get("candidate", {})
        control = per_variant.get("control", {})
        if not baseline or not candidate:
            print("  incomplete: the two variants were not measured together")
            continue
        verdict, reason = verdict_for(level, baseline, candidate, control)
        print(f"  => {verdict}: {reason}")
    print("")
    print("The verdict is a report. It never blocks a build.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
