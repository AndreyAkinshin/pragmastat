#!/usr/bin/env python3
"""Generate the Distribution Estimation reference fixtures.

Run with ``python3 tests/oracles/gen_dist_tests.py``.

The expected values come from :mod:`dist_reference`, an implementation written
directly from the normative algorithm description rather than ported from any
language in this repository. Every emitted case is additionally audited against
the same operation graph evaluated in exact rational arithmetic; a case whose
binary64 outputs reproduce the exact values bit for bit is labeled ``manual-rational``, and the
remaining ones are labeled ``generated-common``.
Generation fails if any output drifts further from the exact value than the construction's
documented quantization, so a silent numerical regression cannot be committed.

Only the standard library is used, except for the sampling sub-suites, which
import the already cross-language-verified Pragmastat ``Rng`` so that the
fixtures pin RNG consumption rather than re-deriving the generator.
"""

from __future__ import annotations

import json
import math
import os
import sys
from collections.abc import Iterable, Sequence
from fractions import Fraction
from pathlib import Path
from typing import Any

HERE = Path(__file__).resolve().parent
REPO = HERE.parent.parent
sys.path.insert(0, str(HERE))
sys.path.insert(0, str(REPO / "py"))

from dist_reference import (  # noqa: E402
    AssumptionViolation,
    LinearDistModel,
    NeighborDistModel,
    audit_linear,
    audit_neighbor,
    audit_neighbor_queries,
    next_down,
    next_up,
)
from pragmastat.rng import Rng  # noqa: E402

# Root the fixtures are written to, the repository unless PRAGMASTAT_TESTS_ROOT names somewhere
# else. The redirect is what lets tests:check:dist-fixtures regenerate into a scratch directory and
# compare, instead of writing over the tree seven language suites are reading. It carries the same
# name and the same meaning as the C# generator's, so one variable redirects every generator.
TESTS = Path(os.environ.get("PRAGMASTAT_TESTS_ROOT") or REPO) / "tests"
GENERATOR = "tests/oracles/gen_dist_tests.py"
MAX_LINEAR_ULP = 4.0
MAX_BOUNDARY_ULP = 0.5
MAX_QUANTIZATION_FACTOR = 4.0
MAX_NEIGHBOR_QUERY_ULP = 4.0

INF = math.inf
NAN = math.nan

# ---------------------------------------------------------------------------
# JSON encoding
# ---------------------------------------------------------------------------


def encode(value: Any) -> Any:
    """Encode floats, using the repository's string form for non-finite values."""
    if isinstance(value, bool):
        return value
    if isinstance(value, float):
        if math.isnan(value):
            return "NaN"
        if value == math.inf:
            return "Infinity"
        if value == -math.inf:
            return "-Infinity"
        return value
    if isinstance(value, int):
        return value
    if isinstance(value, (list, tuple)):
        return [encode(v) for v in value]
    if isinstance(value, dict):
        return {k: encode(v) for k, v in value.items()}
    return value


WRITTEN: set[Path] = set()


def write_case(directory: str, name: str, case: dict[str, Any]) -> None:
    path = TESTS / directory / f"{name}.json"
    # Every fixture name in this file comes either from a hand-written literal or from a loop that
    # formats one, and the two share a directory: `natural-{n}` is generated beside a hand-written
    # `demo-manual-5`. A collision would overwrite the earlier case and leave the tree one fixture
    # short, which nothing downstream can see: `tests:check:dist-fixtures` regenerates and diffs, and
    # a deterministic overwrite reproduces identically both times. The summary count would have
    # reported both writes while the tree held one file, so it read wrong in the direction that
    # hides this. Refused here, where the two namespaces actually meet, which is the same shape as
    # the C# generator's input builder, which throws on a duplicate fixture name.
    if path in WRITTEN:
        raise SystemExit(
            f"gen_dist_tests: two cases claim {directory}/{name}.json. A generated name has\n"
            "collided with a hand-written one (or with another generated one), and the second\n"
            "write would discard the first. Rename one of the two."
        )
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(encode(case), indent=2) + "\n")
    WRITTEN.add(path)


# ---------------------------------------------------------------------------
# Case builders
# ---------------------------------------------------------------------------

MODELS = {"linear": LinearDistModel, "neighbor": NeighborDistModel}
DIRS = {"linear": "dist-linear", "neighbor": "dist-neighbor"}

AUDIT_WORST = {
    "linear_ulp": 0.0,
    "neighbor_boundary_ulp": 0.0,
    "neighbor_factor": 0.0,
    "neighbor_query_ulp": 0.0,
}


def audit(
    model: str,
    obj: Any,
    label: str,
    cs: Sequence[float],
    vs: Sequence[float],
    op: str | None = None,
) -> None:
    """Compare the binary64 layer against exact arithmetic, or abort."""
    if model == "linear":
        worst = audit_linear(obj, cs, vs)
        AUDIT_WORST["linear_ulp"] = max(AUDIT_WORST["linear_ulp"], worst)
        if worst > MAX_LINEAR_ULP:
            raise SystemExit(f"{label}: output drifts {worst:.2f} ULP from exact arithmetic")
        return
    boundary_ulp, factor = audit_neighbor(obj)
    if op is not None:
        query_ulp = audit_neighbor_queries(obj, op, list(cs) + list(vs))
        AUDIT_WORST["neighbor_query_ulp"] = max(AUDIT_WORST["neighbor_query_ulp"], query_ulp)
        if query_ulp > MAX_NEIGHBOR_QUERY_ULP:
            raise SystemExit(
                f"{label}: query output drifts {query_ulp:.2f} ULP from the exact formula "
                f"over the same tables (limit {MAX_NEIGHBOR_QUERY_ULP})"
            )
    AUDIT_WORST["neighbor_boundary_ulp"] = max(AUDIT_WORST["neighbor_boundary_ulp"], boundary_ulp)
    AUDIT_WORST["neighbor_factor"] = max(AUDIT_WORST["neighbor_factor"], factor)
    if boundary_ulp > MAX_BOUNDARY_ULP:
        raise SystemExit(f"{label}: boundary drifts {boundary_ulp:.2f} ULP from exact midpoint")
    if factor > MAX_QUANTIZATION_FACTOR:
        raise SystemExit(
            f"{label}: landscape drifts {factor:.2f}*s*EPS from exact arithmetic (limit {MAX_QUANTIZATION_FACTOR})"
        )


def _oracle(kind: str, source: str) -> dict[str, str]:
    return {"kind": kind, "source": source}


def _exact_reproduction(model: str, obj: Any, op: str, y: Sequence[float]) -> bool:
    """True when the binary64 outputs match exact rational arithmetic exactly."""
    try:
        if model == "linear":
            for c in y:
                if op == "quantile":
                    if Fraction(obj.quantile_scalar(c)) != obj.exact_quantile(Fraction(c)):
                        return False
                elif op == "cumulative":
                    if Fraction(obj.cumulative_scalar(c)) != obj.exact_cumulative(Fraction(c)):
                        return False
                else:
                    got = obj.density_scalar(c)
                    if not math.isfinite(got):
                        return False
                    if Fraction(got) != obj.exact_density(Fraction(c)):
                        return False
            return True
        # strict=True on both: a shorter binary64 sequence than its exact counterpart would make
        # `any` scan only the prefix and answer "reproduces exactly" over a comparison that never
        # happened. The lengths agree by construction, so a mismatch is a defect worth raising on.
        if any(Fraction(g) != w for g, w in zip(obj.b, obj.exact_boundaries(), strict=True)):
            return False
        _, masses, _ = obj.exact_landscape()
        if any(Fraction(g) != w for g, w in zip(obj.p, masses, strict=True)):
            return False
        kernel = getattr(obj, f"exact_{op}")
        for value in y:
            got = getattr(obj, f"{op}_scalar")(value)
            if not math.isfinite(got):
                return False
            if Fraction(got) != kernel(Fraction(value)):
                return False
    except (OverflowError, ZeroDivisionError):
        return False
    return True


def emit_query(
    model: str,
    op: str,
    name: str,
    x: Sequence[float],
    y: Sequence[float],
    *,
    assume_sorted: bool = False,
    source: str | None = None,
    oracle_kind: str | None = None,
) -> None:
    obj = MODELS[model](x, assume_sorted)
    output = getattr(obj, op)(y)

    audit(
        model,
        obj,
        f"{DIRS[model]}/{op}/{name}",
        list(y) if op == "quantile" else [],
        list(y) if op != "quantile" else [],
        op,
    )

    if oracle_kind is None:
        exact = _exact_reproduction(model, obj, op, y)
        oracle_kind = "manual-rational" if exact else "generated-common"
    case = {
        "input": {"x": list(x), "y": list(y)},
        "output": output,
        "tolerance": {"kind": "exact"},
        "oracle": _oracle(oracle_kind, source or GENERATOR),
    }
    if assume_sorted:
        case["input"]["assume_sorted"] = True
    write_case(f"{DIRS[model]}/{op}", name, case)


def emit_error(
    model: str,
    op: str,
    name: str,
    x: Sequence[float],
    y: Sequence[float],
    ident: str,
    subject: str,
    *,
    assume_sorted: bool = False,
    source: str = "manual/assumptions (hard assumption priority order)",
) -> None:
    """Emit an error fixture, asserting the reference raises exactly that violation."""
    try:
        getattr(MODELS[model](x, assume_sorted), op)(y)
    except AssumptionViolation as violation:
        got = (violation.id, violation.subject)
    else:
        raise SystemExit(f"{DIRS[model]}/{op}/{name}: expected {ident}({subject}), got success")
    if got != (ident, subject):
        raise SystemExit(f"{DIRS[model]}/{op}/{name}: expected {ident}({subject}), got {got[0]}({got[1]})")

    case: dict[str, Any] = {
        "input": {"x": list(x), "y": list(y)},
        "expected_error": {"id": ident, "subject": subject},
        "oracle": _oracle("manual-rational", source),
    }
    if assume_sorted:
        case["input"]["assume_sorted"] = True
    write_case(f"{DIRS[model]}/{op}", name, case)


def emit_error_all_ops(
    model: str,
    name: str,
    x: Sequence[float],
    y: Sequence[float],
    ident: str,
    subject: str,
    *,
    ops: Iterable[str] = ("quantile", "cumulative", "density"),
    source: str = "manual/assumptions (hard assumption priority order)",
) -> None:
    for op in ops:
        emit_error(model, op, name, x, y, ident, subject, source=source)


def emit_sample(model: str, name: str, x: Sequence[float], seed: str, count: int) -> None:
    obj = MODELS[model](x, assume_sorted=False)
    rng = Rng(seed)
    cumulances = [rng.uniform_float() for _ in range(count)]
    output = [obj.quantile_scalar(c) for c in cumulances]

    # Sampling draws through the same quantile_scalar kernel the quantile sub-suite audits, but
    # at RNG-drawn cumulances rather than the suite's hand-picked ones -- auditing here is what
    # catches a rounding defect that only a drawn cumulance happens to trigger.
    audit(model, obj, f"{DIRS[model]}/sample/{name}", cumulances, [], "quantile")

    write_case(
        f"{DIRS[model]}/sample",
        name,
        {
            "input": {"x": list(x), "seed": seed, "count": count},
            "output": output,
            "tolerance": {"kind": "exact"},
            "oracle": _oracle(
                "generated-common",
                f"{GENERATOR} composed with the cross-language verified Rng suite",
            ),
        },
    )


def emit_landscape(
    name: str, x: Sequence[float], *, oracle_kind: str = "generated-common", source: str = GENERATOR
) -> None:
    model = NeighborDistModel(x)
    audit("neighbor", model, f"dist-neighbor/landscape/{name}", [], [])
    write_case(
        "dist-neighbor/landscape",
        name,
        {
            "input": {"x": list(x)},
            "output": {
                "k0": model.k0,
                "radius": model.r,
                "boundaries": list(model.b),
                "cumulative": list(model.f),
                "masses": list(model.p),
                "densities": list(model.d),
                "sources": list(model.landscape.sources),
            },
            "tolerance": {"kind": "exact"},
            "oracle": _oracle(oracle_kind, source),
        },
    )


# ---------------------------------------------------------------------------
# Shared inputs
# ---------------------------------------------------------------------------

DEMO = [1.0, 3.0, 5.0, 7.0, 9.0]
PROPOSAL_LINEAR = "manual/dist-linear (worked example)"
PROPOSAL_NEIGHBOR = "manual/dist-neighbor (worked example)"

CEIL_SQRT_NS = [2, 3, 4, 5, 8, 9, 10, 15, 16, 17]

ASYMMETRIC = [0.0, 0.1, 0.15, 0.2, 5.0, 5.05, 5.1, 20.0]
WIDE = [1e-300, 1.0, 100.0, 1e100, 1e300]
ZERO_CROSSING = [-1.0, -0.25, 0.0, 0.25, 1.0]
# ZERO_CROSSING puts zero *on* an observation, so every interpolation interval has both endpoints
# on one side of it and lerp's sign-aware branch never fires; the sample pins the default branch
# instead. This one straddles zero inside a gap, which is the only way to reach the branch, and the
# branch matters because it is where the operation order changes from a + t*(b - a) to
# (1 - t)*a + t*b. Both samples are kept: they cover the two sides of the same decision.
ZERO_STRADDLE = [-1.0, -0.3, 0.7, 1.0]
# Zero as the smallest observation, spelled -0. Both zero samples above reach zero through
# arithmetic, which already yields +0; this is the only way a -0 gets as far as an answer, because
# the endpoint branches of every quantile kernel return the observation verbatim. One model then
# states that same observation twice: through Bounds as support.lower, and through the measured
# carrier as Quantile(0). Until each carrier canonicalized its own contents the two disagreed
# inside one object -- +0 from the interval, -0 from the query -- and nothing in the corpus could
# see it, since -0 == +0 makes the disagreement invisible to every comparison but a payload one.
NEGATIVE_ZERO_MIN = [-0.0, 1.0, 2.0]
ADJACENT = [1.0, next_up(1.0)]
TINY_GAP = [0.0, 1e-323, 1.0]
# A cluster whose cells carry less mass than one representable step of the cumulative array,
# placed beside a point far enough away to make them that small. The prefix reserves a step per
# remaining cell so such a cell still advances the cumulative value instead of flattening it into
# a plateau, and this is the only sample in the corpus that reaches the reservation. Without it,
# regenerating with the reservation deleted reproduces every other fixture byte for byte, so a
# port that omitted it, applied it in the wrong order, or floored the wrong operand passed the
# whole cross-language suite.
SUBULP_MASS = [1e-300 * k for k in range(1, 16)] + [1e300]


def anchors(n: int) -> list[float]:
    return [float(i) / float(n - 1) for i in range(n)]


def anchors_and_midpoints(n: int) -> list[float]:
    grid = anchors(n)
    mids = [(grid[i] + grid[i + 1]) / 2.0 for i in range(n - 1)]
    return sorted(grid + mids)


def probe_values(x: Sequence[float]) -> list[float]:
    """Observations, their neighbors, cell midpoints and points outside support."""
    values: list[float] = []
    for v in x:
        values.extend([math.nextafter(v, -math.inf), v, math.nextafter(v, math.inf)])
    values.extend(x[i] + (x[i + 1] - x[i]) / 2.0 for i in range(len(x) - 1))
    span = x[-1] - x[0]
    values.extend([x[0] - span, x[-1] + span])
    return values


def cumulance_grid(count: int = 21) -> list[float]:
    return [float(i) / float(count - 1) for i in range(count)]


# ---------------------------------------------------------------------------
# Suite construction
# ---------------------------------------------------------------------------


def build_linear() -> None:
    m = "linear"

    emit_query(m, "quantile", "demo-manual-5", DEMO, [0.375], source=PROPOSAL_LINEAR)
    emit_query(m, "quantile", "demo-endpoints-5", DEMO, [0.0, 1.0], source=PROPOSAL_LINEAR)
    emit_query(m, "quantile", "demo-grid-5", DEMO, cumulance_grid())
    emit_query(m, "cumulative", "demo-manual-5", DEMO, [4.0], source=PROPOSAL_LINEAR)
    emit_query(m, "cumulative", "demo-probes-5", DEMO, probe_values(DEMO))
    emit_query(m, "density", "demo-manual-5", DEMO, [4.0], source=PROPOSAL_LINEAR)
    emit_query(m, "density", "demo-probes-5", DEMO, probe_values(DEMO))

    for n in range(2, 11):
        x = [float(i) for i in range(1, n + 1)]
        emit_query(m, "quantile", f"natural-{n}", x, anchors_and_midpoints(n))
        emit_query(m, "cumulative", f"natural-{n}", x, probe_values(x))
        emit_query(m, "density", f"natural-{n}", x, probe_values(x))

    for op, y in (
        ("quantile", cumulance_grid()),
        ("cumulative", probe_values(DEMO)),
        ("density", probe_values(DEMO)),
    ):
        emit_query(m, op, "unsorted-reverse-5", list(reversed(DEMO)), y)
        emit_query(m, op, "unsorted-shuffle-5", [5.0, 1.0, 9.0, 3.0, 7.0], y)
        emit_query(m, op, "sorted-flag-5", DEMO, y, assume_sorted=True)
        emit_query(m, op, "edge-empty-query-5", DEMO, [])

    emit_query(m, "quantile", "edge-repeated-unordered-5", DEMO, [0.5, 0.1, 0.5, 0.9, 0.1])
    emit_query(m, "cumulative", "edge-repeated-unordered-5", DEMO, [5.0, 2.0, 5.0, 8.0, 2.0])
    emit_query(m, "density", "edge-repeated-unordered-5", DEMO, [5.0, 2.0, 5.0, 8.0, 2.0])
    emit_query(m, "cumulative", "edge-outside-cumulance-5", DEMO, [-1e6, 1e6])

    emit_query(m, "quantile", "boundary-adjacent-floats-2", ADJACENT, cumulance_grid())
    emit_query(m, "cumulative", "boundary-adjacent-floats-2", ADJACENT, probe_values(ADJACENT))
    emit_query(m, "density", "boundary-adjacent-floats-2", ADJACENT, probe_values(ADJACENT))
    emit_query(m, "quantile", "boundary-zero-crossing-5", ZERO_CROSSING, cumulance_grid())
    emit_query(m, "cumulative", "boundary-zero-crossing-5", ZERO_CROSSING, probe_values(ZERO_CROSSING))
    emit_query(m, "density", "boundary-zero-crossing-5", ZERO_CROSSING, probe_values(ZERO_CROSSING))
    emit_query(m, "quantile", "boundary-zero-straddle-4", ZERO_STRADDLE, cumulance_grid())
    emit_query(m, "cumulative", "boundary-zero-straddle-4", ZERO_STRADDLE, probe_values(ZERO_STRADDLE))
    emit_query(m, "density", "boundary-zero-straddle-4", ZERO_STRADDLE, probe_values(ZERO_STRADDLE))
    emit_query(m, "quantile", "boundary-negative-zero-3", NEGATIVE_ZERO_MIN, cumulance_grid())

    emit_query(m, "quantile", "extreme-wide-5", WIDE, cumulance_grid())
    emit_query(m, "cumulative", "extreme-wide-5", WIDE, probe_values(WIDE))
    emit_query(m, "density", "extreme-wide-5", WIDE, probe_values(WIDE))
    emit_query(m, "quantile", "extreme-range-2", [-1e308, 1e307], cumulance_grid())
    emit_query(m, "density", "extreme-overflow-2", [0.0, 5e-324], [0.0, 5e-324])
    emit_query(m, "quantile", "extreme-tiny-gap-3", TINY_GAP, cumulance_grid())
    emit_query(m, "density", "extreme-tiny-gap-3", TINY_GAP, probe_values(TINY_GAP))

    emit_sample(m, "demo-5", DEMO, "dist-linear-sample-demo", 12)
    emit_sample(m, "natural-10", [float(i) for i in range(1, 11)], "dist-linear-sample-natural", 12)
    emit_sample(m, "extreme-wide-5", WIDE, "dist-linear-sample-wide", 8)


def build_neighbor() -> None:
    m = "neighbor"

    emit_query(m, "quantile", "demo-manual-5", DEMO, [0.0, 0.5, 1.0], source=PROPOSAL_NEIGHBOR)
    emit_query(m, "cumulative", "demo-manual-5", DEMO, [5.0], source=PROPOSAL_NEIGHBOR)
    emit_query(m, "density", "demo-manual-5", DEMO, [1.5, 5.0, 8.5, 9.0], source=PROPOSAL_NEIGHBOR)
    emit_query(m, "quantile", "demo-grid-5", DEMO, cumulance_grid())
    emit_query(m, "cumulative", "demo-probes-5", DEMO, probe_values(DEMO))
    emit_query(m, "density", "demo-probes-5", DEMO, probe_values(DEMO))

    for n in CEIL_SQRT_NS:
        x = [float(i) for i in range(1, n + 1)]
        emit_query(m, "quantile", f"natural-{n}", x, cumulance_grid())
        emit_query(m, "cumulative", f"natural-{n}", x, probe_values(x))
        emit_query(m, "density", f"natural-{n}", x, probe_values(x))
        emit_landscape(f"natural-{n}", x)

    for op, y in (
        ("quantile", cumulance_grid()),
        ("cumulative", probe_values(DEMO)),
        ("density", probe_values(DEMO)),
    ):
        emit_query(m, op, "unsorted-reverse-5", list(reversed(DEMO)), y)
        emit_query(m, op, "unsorted-shuffle-5", [5.0, 1.0, 9.0, 3.0, 7.0], y)
        emit_query(m, op, "sorted-flag-5", DEMO, y, assume_sorted=True)
        emit_query(m, op, "edge-empty-query-5", DEMO, [])

    emit_query(m, "quantile", "edge-repeated-unordered-5", DEMO, [0.5, 0.1, 0.5, 0.9, 0.1])
    emit_query(m, "cumulative", "edge-repeated-unordered-5", DEMO, [5.0, 2.0, 5.0, 8.0, 2.0])
    emit_query(m, "density", "edge-repeated-unordered-5", DEMO, [5.0, 2.0, 5.0, 8.0, 2.0])

    emit_query(m, "quantile", "asymmetric-gaps-8", ASYMMETRIC, cumulance_grid())
    emit_query(m, "cumulative", "asymmetric-gaps-8", ASYMMETRIC, probe_values(ASYMMETRIC))
    emit_query(m, "density", "asymmetric-gaps-8", ASYMMETRIC, probe_values(ASYMMETRIC))
    emit_landscape("asymmetric-gaps-8", ASYMMETRIC)

    # Three shapes of compression: the dropped cell is the first, the last, and an interior one.
    # Each shifts the source indices differently, so a bookkeeping error survives only if all
    # three are covered.
    zero_width_cases = {
        "boundary-zero-width-first-3": [1.0, next_up(1.0), 3.0],
        "boundary-zero-width-last-4": [0.0, 0.5, next_up(1.0), next_up(next_up(1.0))],
        "boundary-zero-width-interior-5": [
            0.0,
            next_down(1.0),
            1.0,
            next_up(1.0),
            10.0,
        ],
    }
    for name, sample in zero_width_cases.items():
        emit_query(m, "quantile", name, sample, cumulance_grid())
        emit_query(m, "cumulative", name, sample, probe_values(sample))
        emit_query(m, "density", name, sample, probe_values(sample))
        emit_landscape(name, sample)

    emit_query(m, "quantile", "boundary-adjacent-floats-2", ADJACENT, cumulance_grid())
    emit_landscape("boundary-adjacent-floats-2", ADJACENT)

    emit_query(m, "density", "extreme-overflow-3", TINY_GAP, probe_values(TINY_GAP))
    emit_query(m, "quantile", "extreme-overflow-3", TINY_GAP, cumulance_grid())
    emit_landscape("extreme-overflow-3", TINY_GAP)

    emit_query(m, "quantile", "extreme-wide-5", WIDE, cumulance_grid())
    emit_query(m, "cumulative", "extreme-wide-5", WIDE, probe_values(WIDE))
    emit_query(m, "density", "extreme-wide-5", WIDE, probe_values(WIDE))
    emit_landscape("extreme-wide-5", WIDE)

    emit_query(m, "quantile", "boundary-zero-crossing-5", ZERO_CROSSING, cumulance_grid())
    emit_landscape("boundary-zero-crossing-5", ZERO_CROSSING)

    # The same sample as the linear suite, for the same reason and against a different kernel: this
    # model reaches cumulance 0 through the leftmost landscape boundary rather than through x[0]
    # directly, and the two models share no quantile code in any port, so a case in one says
    # nothing about the other.
    emit_query(m, "quantile", "boundary-negative-zero-3", NEGATIVE_ZERO_MIN, cumulance_grid())

    emit_query(m, "cumulative", "extreme-subulp-mass-16", SUBULP_MASS, probe_values(SUBULP_MASS))
    emit_query(m, "quantile", "extreme-subulp-mass-16", SUBULP_MASS, cumulance_grid())
    emit_query(m, "density", "extreme-subulp-mass-16", SUBULP_MASS, probe_values(SUBULP_MASS))
    emit_landscape("extreme-subulp-mass-16", SUBULP_MASS)

    emit_sample(m, "demo-5", DEMO, "dist-neighbor-sample-demo", 12)
    emit_sample(m, "asymmetric-gaps-8", ASYMMETRIC, "dist-neighbor-sample-asymmetric", 12)
    emit_sample(m, "extreme-wide-5", WIDE, "dist-neighbor-sample-wide", 8)


def build_errors() -> None:
    for m in ("linear", "neighbor"):
        emit_error_all_ops(m, "error-empty-x", [], [0.5], "validity", "x")
        emit_error_all_ops(m, "error-nan-x", [1.0, NAN, 3.0], [0.5], "validity", "x")
        emit_error_all_ops(m, "error-inf-x", [1.0, INF, 3.0], [0.5], "validity", "x")
        emit_error_all_ops(m, "error-nan-y", DEMO, [0.5, NAN], "validity", "y")
        emit_error_all_ops(m, "error-inf-y", DEMO, [INF], "validity", "y")
        emit_error_all_ops(m, "error-single-x", [1.0], [0.5], "domain", "x")
        emit_error_all_ops(m, "error-range-overflow-x", [-1e308, 1e308], [0.5], "domain", "x")
        emit_error_all_ops(m, "error-ties-x", [1.0, 2.0, 2.0, 3.0], [0.5], "distinctness", "x")
        emit_error_all_ops(m, "error-signed-zero-ties-x", [-1.0, -0.0, 0.0, 1.0], [0.5], "distinctness", "x")
        emit_error_all_ops(m, "error-empty-query-still-validates-x", [1.0, 1.0], [], "distinctness", "x")
        # One sample, two broken assumptions: the range overflows and 1e308 repeats. The
        # hierarchy decides, so `domain` is reported and `distinctness` is not.
        emit_error_all_ops(
            m,
            "error-domain-before-distinctness-x",
            [1e308, -1e308, 1e308],
            [0.5],
            "domain",
            "x",
        )

        emit_error(m, "quantile", "error-cumulance-below", DEMO, [-0.1], "cumulance", "y")
        emit_error(m, "quantile", "error-cumulance-above", DEMO, [0.5, 1.5], "cumulance", "y")


def main() -> None:
    build_linear()
    build_neighbor()
    build_errors()

    kinds: dict[str, int] = {}
    for path in sorted(WRITTEN):
        payload = json.loads(path.read_text())
        kind = payload["oracle"]["kind"]
        kinds[kind] = kinds.get(kind, 0) + 1

    print(f"wrote {len(WRITTEN)} fixtures")
    for kind, count in sorted(kinds.items()):
        print(f"  oracle {kind}: {count}")
    print(
        "  binary64-vs-exact audit: linear "
        f"{AUDIT_WORST['linear_ulp']:.3f} ULP, neighbor boundaries "
        f"{AUDIT_WORST['neighbor_boundary_ulp']:.3f} ULP, neighbor landscape "
        f"{AUDIT_WORST['neighbor_factor']:.3f}*s*EPS, neighbor queries "
        f"{AUDIT_WORST['neighbor_query_ulp']:.3f} ULP"
    )


if __name__ == "__main__":
    main()
