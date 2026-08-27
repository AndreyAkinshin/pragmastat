"""Independent binary64 reference for the Distribution Estimation models.

This module is a second, independent implementation of ``dist-linear-v1`` and
``dist-neighbor-v1``. It is written directly from the normative algorithm
description and is deliberately *not* a port of any language implementation in
this repository: it exists so that the shipped fixtures are certified by
something other than the code they validate.

Two layers are provided:

``binary64`` layer
    Plain Python ``float`` arithmetic. CPython floats are IEEE-754 binary64 with
    round-to-nearest-ties-to-even, and ``+ - * /`` are correctly rounded, so this
    layer reproduces the normative operation graph exactly.

``exact`` layer
    The same operation graph evaluated over :class:`fractions.Fraction`. This
    gives the mathematically exact value of every intermediate quantity and is
    used to audit the binary64 layer (see :func:`audit_linear` and :func:`audit_neighbor`).

Nothing here imports the Pragmastat packages, so a bug in an implementation
cannot leak into the expected values.
"""

from __future__ import annotations

import math
import sys
from collections.abc import Sequence
from dataclasses import dataclass
from fractions import Fraction

# ---------------------------------------------------------------------------
# Normative binary64 helpers (manual/dist-linear, Algorithm)
# ---------------------------------------------------------------------------


def next_up(x: float) -> float:
    """Next representable binary64 value toward +Inf."""
    return math.nextafter(x, math.inf)


def next_down(x: float) -> float:
    """Next representable binary64 value toward -Inf."""
    return math.nextafter(x, -math.inf)


def binary64(k: int) -> float:
    """Exact integer-to-binary64 conversion for 0 <= k < 2**53."""
    if not 0 <= k < (1 << 53):
        raise ValueError(f"integer {k} is not exactly representable")
    return float(k)


def midpoint(a: float, b: float) -> float:
    """Overflow-safe midpoint of two finite binary64 values."""
    if a < 0.0 < b:
        return a / 2.0 + b / 2.0
    return a + (b - a) / 2.0


def lerp(a: float, b: float, t: float) -> float:
    """Canonical interpolation with exact endpoints."""
    if t == 0.0:
        return a
    if t == 1.0:
        return b
    if a < 0.0 < b:
        return (1.0 - t) * a + t * b
    return a + t * (b - a)


def unit_ratio(num: float, den: float) -> float:
    """Ratio clamped to [0, 1]; ordering logic must guarantee 0 <= num <= den."""
    t = num / den
    if math.isnan(t):
        raise ArithmeticError("numerical-conformance failure: unitRatio produced NaN")
    return min(1.0, max(0.0, t))


def normalize_zero(v: float) -> float:
    """A negative zero replaced by a positive one, every other value unchanged."""
    return 0.0 if v == 0.0 else v


def ceil_sqrt(n: int) -> int:
    """Smallest k with k >= ceil(n / k), computed without forming k*k."""
    k = 1
    while k < -(-n // k):
        k += 1
    return k


def lower_bound(values: Sequence[float], key: float) -> int:
    """First index i with values[i] >= key."""
    lo, hi = 0, len(values)
    while lo < hi:
        mid = (lo + hi) // 2
        if values[mid] < key:
            lo = mid + 1
        else:
            hi = mid
    return lo


def upper_bound(values: Sequence[float], key: float) -> int:
    """First index i with values[i] > key."""
    lo, hi = 0, len(values)
    while lo < hi:
        mid = (lo + hi) // 2
        if values[mid] <= key:
            lo = mid + 1
        else:
            hi = mid
    return lo


# ---------------------------------------------------------------------------
# Exact-arithmetic mirrors of the same helpers
# ---------------------------------------------------------------------------


def q_midpoint(a: Fraction, b: Fraction) -> Fraction:
    return (a + b) / 2


def q_lerp(a: Fraction, b: Fraction, t: Fraction) -> Fraction:
    return a + t * (b - a)


# ---------------------------------------------------------------------------
# Validation (manual/assumptions)
# ---------------------------------------------------------------------------

VALIDITY = "validity"
DOMAIN = "domain"
DISTINCTNESS = "distinctness"
CUMULANCE = "cumulance"

SUBJECT_X = "x"
SUBJECT_Y = "y"


class AssumptionViolation(Exception):
    def __init__(self, ident: str, subject: str) -> None:
        super().__init__(f"{ident}({subject})")
        self.id = ident
        self.subject = subject


def check_validity_x(x: Sequence[float]) -> None:
    if len(x) == 0 or any(not math.isfinite(v) for v in x):
        raise AssumptionViolation(VALIDITY, SUBJECT_X)


def check_validity_y(y: Sequence[float]) -> None:
    if any(not math.isfinite(v) for v in y):
        raise AssumptionViolation(VALIDITY, SUBJECT_Y)


def check_cumulance(y: Sequence[float]) -> None:
    if any(not (0.0 <= c <= 1.0) for c in y):
        raise AssumptionViolation(CUMULANCE, SUBJECT_Y)


def prepare_sample(x: Sequence[float], assume_sorted: bool) -> list[float]:
    """Copy, sort if needed, then enforce domain(x) and distinctness(x)."""
    values = [float(v) for v in x]
    if not assume_sorted:
        values.sort()
    n = len(values)
    if n < 2:
        raise AssumptionViolation(DOMAIN, SUBJECT_X)
    span = values[n - 1] - values[0]
    if not math.isfinite(span):
        raise AssumptionViolation(DOMAIN, SUBJECT_X)
    for i in range(1, n):
        if values[i - 1] == values[i]:
            raise AssumptionViolation(DISTINCTNESS, SUBJECT_X)
    return values


# ---------------------------------------------------------------------------
# LinearDistModel (manual/dist-linear, Algorithm)
# ---------------------------------------------------------------------------


class LinearDistModel:
    """HF7 piecewise-linear reconstruction over a strictly increasing sample."""

    algorithm = "dist-linear-v1"

    def __init__(self, x: Sequence[float], assume_sorted: bool = False) -> None:
        check_validity_x(x)
        self.x = prepare_sample(x, assume_sorted)
        self.n = len(self.x)

    # -- scalar kernels ----------------------------------------------------

    def quantile_scalar(self, c: float) -> float:
        """The scalar quantile the ports publish as ``quantileAt`` and consume in ``draw``.

        Clamped rather than compared for equality, and normalized at the exit, because that is
        what the seven implementations do. Both differences are outside the vector query's reach
        -- it rejects a cumulance outside [0, 1] and its carrier canonicalizes the sign of a zero
        -- which is exactly why they belong here rather than in the caller.
        """
        x, n = self.x, self.n
        if c <= 0.0:
            value = x[0]
        elif c >= 1.0:
            value = x[n - 1]
        else:
            r = binary64(n - 1) * c
            j = math.floor(r)
            g = r - binary64(j)
            value = lerp(x[j], x[j + 1], g)
        return normalize_zero(value)

    def cumulative_scalar(self, v: float) -> float:
        x, n = self.x, self.n
        if v <= x[0]:
            return 0.0
        if v >= x[n - 1]:
            return 1.0
        j = upper_bound(x, v) - 1
        t = unit_ratio(v - x[j], x[j + 1] - x[j])
        return (binary64(j) + t) / binary64(n - 1)

    def density_scalar(self, v: float) -> float:
        x, n = self.x, self.n
        if v < x[0] or v > x[n - 1]:
            return 0.0
        j = n - 2 if v == x[n - 1] else upper_bound(x, v) - 1
        return (1.0 / binary64(n - 1)) / (x[j + 1] - x[j])

    # -- vector API --------------------------------------------------------

    def quantile(self, y: Sequence[float]) -> list[float]:
        check_validity_y(y)
        check_cumulance(y)
        return [self.quantile_scalar(c) for c in y]

    def cumulative(self, y: Sequence[float]) -> list[float]:
        check_validity_y(y)
        return [self.cumulative_scalar(v) for v in y]

    def density(self, y: Sequence[float]) -> list[float]:
        check_validity_y(y)
        return [self.density_scalar(v) for v in y]

    # -- exact mirror ------------------------------------------------------

    def exact_quantile(self, c: Fraction) -> Fraction:
        x = [Fraction(v) for v in self.x]
        n = self.n
        if c == 0:
            return x[0]
        if c == 1:
            return x[n - 1]
        r = (n - 1) * c
        j = math.floor(r)
        return q_lerp(x[j], x[j + 1], r - j)

    def exact_cumulative(self, v: Fraction) -> Fraction:
        x = [Fraction(t) for t in self.x]
        n = self.n
        if v <= x[0]:
            return Fraction(0)
        if v >= x[n - 1]:
            return Fraction(1)
        j = upper_bound(self.x, float(v)) - 1
        # Recover the true bracket in exact arithmetic (float search is only a hint).
        while j + 1 < n - 1 and x[j + 1] <= v:
            j += 1
        while j > 0 and x[j] > v:
            j -= 1
        return (j + (v - x[j]) / (x[j + 1] - x[j])) / (n - 1)

    def exact_density(self, v: Fraction) -> Fraction:
        x = [Fraction(t) for t in self.x]
        n = self.n
        if v < x[0] or v > x[n - 1]:
            return Fraction(0)
        if v == x[n - 1]:
            j = n - 2
        else:
            j = upper_bound(self.x, float(v)) - 1
            while j + 1 < n - 1 and x[j + 1] <= v:
                j += 1
            while j > 0 and x[j] > v:
                j -= 1
        return Fraction(1, n - 1) / (x[j + 1] - x[j])


# ---------------------------------------------------------------------------
# NeighborDistModel (manual/dist-neighbor, Algorithm)
# ---------------------------------------------------------------------------


@dataclass(frozen=True)
class NeighborLandscape:
    """Canonical step-density landscape: boundaries, CDF, masses, heights."""

    boundaries: tuple[float, ...]
    cumulative: tuple[float, ...]
    masses: tuple[float, ...]
    densities: tuple[float, ...]
    sources: tuple[int, ...]

    @property
    def cell_count(self) -> int:
        return len(self.masses)


class NeighborDistModel:
    """Local step density with truncated windows and half-width edge cells."""

    algorithm = "dist-neighbor-v1"

    def __init__(self, x: Sequence[float], assume_sorted: bool = False) -> None:
        check_validity_x(x)
        self.x = prepare_sample(x, assume_sorted)
        self.n = len(self.x)
        self._build()

    # -- construction ------------------------------------------------------

    def _windows(self) -> tuple[list[int], list[float]]:
        x, n = self.x, self.n
        k0 = ceil_sqrt(n)
        r = k0 // 2
        self.k0 = k0
        self.r = r
        counts: list[int] = []
        spans: list[float] = []
        for i in range(n):
            left = max(0, i - r)
            right = min(n - 1, i + r)
            counts.append(right - left + 1)
            spans.append(x[right] - x[left])
        return counts, spans

    def _build(self) -> None:
        x, n = self.x, self.n
        counts, spans = self._windows()

        e = [0.0] * (n + 1)
        e[0] = x[0]
        for i in range(1, n):
            e[i] = midpoint(x[i - 1], x[i])
        e[n] = x[n - 1]
        self.e = e

        b = [e[0]]
        source: list[int] = []
        for i in range(n):
            if e[i + 1] == e[i]:
                continue
            source.append(i)
            b.append(e[i + 1])

        s = len(source)
        a = [0.0] * s
        for j in range(s):
            i = source[j]
            width = e[i + 1] - e[i]
            a[j] = (binary64(counts[i]) / binary64(n)) * (width / spans[i])

        total = 0.0
        for j in range(s):
            total = total + a[j]

        cap = [0.0] * (s + 1)
        cap[s] = 1.0
        for j in range(s - 1, -1, -1):
            cap[j] = next_down(cap[j + 1])

        f = [0.0] * (s + 1)
        f[0] = 0.0
        for j in range(s - 1):
            raw = f[j] + a[j] / total
            f[j + 1] = min(cap[j + 1], max(next_up(f[j]), raw))
        f[s] = 1.0

        p = [f[j + 1] - f[j] for j in range(s)]
        d = [p[j] / (b[j + 1] - b[j]) for j in range(s)]

        self.counts = counts
        self.spans = spans
        self.a = a
        self.total = total
        self.landscape = NeighborLandscape(
            boundaries=tuple(b),
            cumulative=tuple(f),
            masses=tuple(p),
            densities=tuple(d),
            sources=tuple(source),
        )

    # -- convenience accessors --------------------------------------------

    @property
    def b(self) -> tuple[float, ...]:
        return self.landscape.boundaries

    @property
    def f(self) -> tuple[float, ...]:
        return self.landscape.cumulative

    @property
    def p(self) -> tuple[float, ...]:
        return self.landscape.masses

    @property
    def d(self) -> tuple[float, ...]:
        return self.landscape.densities

    @property
    def s(self) -> int:
        return self.landscape.cell_count

    # -- scalar kernels ----------------------------------------------------

    def density_scalar(self, v: float) -> float:
        b, s = self.b, self.s
        if v < b[0] or v > b[s]:
            return 0.0
        j = s - 1 if v == b[s] else upper_bound(b, v) - 1
        return self.p[j] / (b[j + 1] - b[j])

    def cumulative_scalar(self, v: float) -> float:
        b, f, s = self.b, self.f, self.s
        if v <= b[0]:
            return 0.0
        if v >= b[s]:
            return 1.0
        j = upper_bound(b, v) - 1
        t = unit_ratio(v - b[j], b[j + 1] - b[j])
        return lerp(f[j], f[j + 1], t)

    def quantile_scalar(self, c: float) -> float:
        """The scalar quantile the ports publish as ``quantileAt`` and consume in ``draw``.

        Clamped and normalized for the reason given on :meth:`LinearDistModel.quantile_scalar`;
        the two models are written the same way in every port.
        """
        b, f, s = self.b, self.f, self.s
        if c <= 0.0:
            value = b[0]
        elif c >= 1.0:
            value = b[s]
        else:
            j = lower_bound(f[1:], c)
            t = unit_ratio(c - f[j], self.p[j])
            value = lerp(b[j], b[j + 1], t)
        return normalize_zero(value)

    # -- vector API --------------------------------------------------------

    def quantile(self, y: Sequence[float]) -> list[float]:
        check_validity_y(y)
        check_cumulance(y)
        return [self.quantile_scalar(c) for c in y]

    def cumulative(self, y: Sequence[float]) -> list[float]:
        check_validity_y(y)
        return [self.cumulative_scalar(v) for v in y]

    def density(self, y: Sequence[float]) -> list[float]:
        check_validity_y(y)
        return [self.density_scalar(v) for v in y]

    # -- exact mirror ------------------------------------------------------

    def exact_window(self, i: int) -> tuple[int, Fraction]:
        """Neighbor count and span of window ``i``, re-derived from the radius.

        Deliberately not read back from the binary64 layer: an error in the
        window indices has to be visible to the audit, not inherited by it.
        """
        x = [Fraction(v) for v in self.x]
        left = max(0, i - self.r)
        right = min(self.n - 1, i + self.r)
        return right - left + 1, x[right] - x[left]

    def exact_boundaries(self) -> list[Fraction]:
        """Exact midpoints corresponding to the surviving binary64 boundaries.

        Which cells survive compression is a binary64 decision, so the exact
        mirror inherits the surviving set and only re-derives each boundary
        value. This isolates the rounding of ``midpoint`` from everything else.
        """
        x = [Fraction(v) for v in self.x]
        n = self.n
        exact = [x[0]]
        for i in self.landscape.sources:
            k = i + 1
            exact.append(x[n - 1] if k == n else q_midpoint(x[k - 1], x[k]))
        return exact

    def exact_landscape(self) -> tuple[list[Fraction], list[Fraction], list[Fraction]]:
        """Exact masses and densities for the binary64 boundaries.

        Boundaries enter as the exact rational value of the binary64 numbers the
        construction actually produced, so what is audited here is the window
        rule, the mass normalization and the CDF accumulation, not the boundary
        rounding that :meth:`exact_boundaries` covers.

        Cached because the exact Fraction sweep is the single most expensive step
        of fixture generation, and both audit_neighbor and _exact_reproduction
        (tests/oracles/gen_dist_tests.py) call it on the same, immutable object.
        """
        cached = getattr(self, "_exact_landscape_cache", None)
        if cached is not None:
            return cached
        b = [Fraction(v) for v in self.b]
        a: list[Fraction] = []
        for j, i in enumerate(self.landscape.sources):
            count, span = self.exact_window(i)
            a.append(Fraction(count, self.n) * ((b[j + 1] - b[j]) / span))

        total = sum(a, Fraction(0))
        masses = [v / total for v in a]
        densities = [masses[j] / (b[j + 1] - b[j]) for j in range(len(masses))]
        result = (b, masses, densities)
        self._exact_landscape_cache = result
        return result

    # -- exact query mirrors ----------------------------------------------
    #
    # These evaluate the query formulas exactly over the tables the model
    # actually holds. The quantization of those tables is a separate question,
    # audited by exact_landscape; folding it in here would charge the
    # interpolation for an error it did not make, and on a sample whose support
    # values are adjacent floats that charge is an entire cell.

    def _exact_tables(self) -> tuple[list[Fraction], list[Fraction], list[Fraction]]:
        return (
            [Fraction(v) for v in self.b],
            [Fraction(v) for v in self.f],
            [Fraction(v) for v in self.p],
        )

    def exact_density(self, v: Fraction) -> Fraction:
        b, _, p = self._exact_tables()
        if v < b[0] or v > b[self.s]:
            return Fraction(0)
        j = self.s - 1 if v == b[self.s] else upper_bound(self.b, float(v)) - 1
        return p[j] / (b[j + 1] - b[j])

    def exact_cumulative(self, v: Fraction) -> Fraction:
        b, f, _ = self._exact_tables()
        if v <= b[0]:
            return Fraction(0)
        if v >= b[self.s]:
            return Fraction(1)
        j = upper_bound(self.b, float(v)) - 1
        return f[j] + (f[j + 1] - f[j]) * (v - b[j]) / (b[j + 1] - b[j])

    def exact_quantile(self, c: Fraction) -> Fraction:
        b, f, p = self._exact_tables()
        if c == 0:
            return b[0]
        if c == 1:
            return b[self.s]
        j = lower_bound(self.f[1:], float(c))
        return b[j] + (c - f[j]) * (b[j + 1] - b[j]) / p[j]


# ---------------------------------------------------------------------------
# Auditing: binary64 layer vs exact layer
# ---------------------------------------------------------------------------


def ulp_at(value: float) -> float:
    """Spacing of the binary64 lattice around ``value``."""
    if value == 0.0:
        return next_up(0.0)
    return max(value - next_down(value), next_up(value) - value)


def ulp_distance(computed: float, exact: Fraction, scale: float | None = None) -> float:
    """How many binary64 steps separate ``computed`` from the exact value.

    ``scale`` overrides the lattice the distance is measured against. It matters
    wherever the result is a difference of larger quantities: interpolating
    across a zero crossing can land arbitrarily close to zero, and measuring the
    residue against its own vanishing ULP would report a huge distance for an
    error of a fraction of a step of the interval being interpolated.
    """
    return abs(Fraction(computed) - exact) / Fraction(ulp_at(scale if scale is not None else computed))


# Round-to-nearest overflows a finite computation to Infinity only once the exact value crosses
# the midpoint between the largest finite binary64 and Infinity -- half a ULP past DBL_MAX. A
# non-finite result closer to the origin than this is not a legitimate overflow; it is a bug
# masquerading as one, and every audit below checks the crossing explicitly rather than skipping
# non-finite results outright.
#
# ulp_at can't supply that last ULP: it takes the wider of the steps on either side of its
# argument, and the step above DBL_MAX is Infinity, not a spacing. The binade is symmetric at the
# top, so the step below is the same width and is what the true spacing actually is.
_DBL_MAX_ULP = sys.float_info.max - next_down(sys.float_info.max)
_OVERFLOW_THRESHOLD = Fraction(sys.float_info.max) + Fraction(_DBL_MAX_ULP) / 2


def assert_legitimate_overflow(got: float, want: Fraction, where: str) -> None:
    if abs(want) < _OVERFLOW_THRESHOLD:
        raise SystemExit(
            f"{where}: spurious {got!r} -- exact value {want} does not cross the overflow threshold"
        )


# The underflow mirror of _OVERFLOW_THRESHOLD: round-to-nearest flushes a finite computation to
# 0.0 only once the exact value falls at or under half the smallest subnormal. A 0.0 result closer
# to that boundary than this is not a legitimate underflow -- it is the same bug-masquerading-as-
# rounding that assert_legitimate_overflow catches at the other end of the range.
_SMALLEST_SUBNORMAL = sys.float_info.min * 2.0**-52
_UNDERFLOW_THRESHOLD = Fraction(_SMALLEST_SUBNORMAL) / 2


def assert_legitimate_underflow(want: Fraction, where: str) -> None:
    if abs(want) > _UNDERFLOW_THRESHOLD:
        raise SystemExit(
            f"{where}: spurious 0.0 -- exact value {want} does not fall under the underflow threshold"
        )


def audit_linear(model: LinearDistModel, cs: Sequence[float], vs: Sequence[float]) -> float:
    worst = 0.0
    for c in cs:
        got = model.quantile_scalar(c)
        want = model.exact_quantile(Fraction(c))
        if not math.isfinite(got):
            assert_legitimate_overflow(got, want, f"audit_linear quantile({c!r})")
            continue
        scale = None
        if 0.0 < c < 1.0:
            j = math.floor(binary64(model.n - 1) * c)
            scale = max(abs(model.x[j]), abs(model.x[j + 1]))
        worst = max(worst, float(ulp_distance(got, want, scale)))
    for v in vs:
        got = model.cumulative_scalar(v)
        want = model.exact_cumulative(Fraction(v))
        worst = max(worst, float(ulp_distance(got, want)))
        gotd = model.density_scalar(v)
        wantd = model.exact_density(Fraction(v))
        if gotd == 0.0:
            assert_legitimate_underflow(wantd, f"audit_linear density({v!r})")
            continue
        if not math.isfinite(gotd):
            assert_legitimate_overflow(gotd, wantd, f"audit_linear density({v!r})")
            continue
        worst = max(worst, float(ulp_distance(gotd, wantd)))
    return worst


def audit_neighbor_queries(model: NeighborDistModel, op: str, y: Sequence[float]) -> float:
    """Audit query outputs, in ULP, against the exact formulas over the same tables.

    The landscape audit covers how well the tables approximate the mathematics;
    this covers whether the queries read them correctly.
    """
    kernel = getattr(model, f"{op}_scalar")
    exact_kernel = getattr(model, f"exact_{op}")
    worst = 0.0
    for value in y:
        got = kernel(value)
        want = exact_kernel(Fraction(value))
        if op == "density" and got == 0.0:
            assert_legitimate_underflow(want, f"audit_neighbor_queries {op}({value!r})")
            continue
        if not math.isfinite(got):
            assert_legitimate_overflow(got, want, f"audit_neighbor_queries {op}({value!r})")
            continue
        scale = None
        if op == "quantile" and 0.0 < value < 1.0:
            j = lower_bound(model.f[1:], value)
            scale = max(abs(model.b[j]), abs(model.b[j + 1]))
        worst = max(worst, float(ulp_distance(got, want, scale)))
    return worst


def audit_neighbor(model: NeighborDistModel) -> tuple[float, float]:
    """Audit the landscape against exact arithmetic.

    Returns ``(boundary_ulp, quantization_factor)``.

    Boundaries are correctly rounded midpoints, so their deviation is at most
    half an ULP. Masses are differences of a CDF built by prefix accumulation
    and closed exactly at 1, which spends one rounding per cell; the natural
    yardstick is therefore ``s * EPS`` rather than an ULP of the mass itself.
    Densities are compared in mass-equivalent terms so that a narrow cell does
    not inflate the reported deviation.
    """
    _, masses, densities = model.exact_landscape()
    step = model.s * (2.0**-52)

    boundary_ulp = 0.0
    for got, want in zip(model.b, model.exact_boundaries(), strict=True):
        boundary_ulp = max(boundary_ulp, float(ulp_distance(got, want)))

    factor = 0.0
    for got, want in zip(model.p, masses, strict=True):
        factor = max(factor, float(abs(Fraction(got) - want)) / step)
    for j, (got, want) in enumerate(zip(model.d, densities, strict=True)):
        if math.isfinite(got):
            width = model.b[j + 1] - model.b[j]
            factor = max(factor, float(abs(Fraction(got) - want)) * width / step)
        else:
            assert_legitimate_overflow(got, want, f"audit_neighbor density[{j}]")
    return boundary_ulp, factor
