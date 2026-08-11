"""Local step-density distribution reconstructed from a sample."""

from __future__ import annotations

import math
from typing import TYPE_CHECKING

from .._normalize_zero import normalize_zero
from ..assumptions import Subject, check_cumulance, check_query_validity, check_validity
from ..dist_numerics import ceil_sqrt, lerp, lower_bound, midpoint, next_down, next_up, unit_ratio, upper_bound
from ..dist_sample_guard import check_no_assume_sorted, prepare
from ..measurement_unit import NUMBER_UNIT
from ..sample import Sample, _check_non_weighted
from .distribution import Distribution
from .measured_view import MeasuredView
from .neighbor_landscape import NeighborLandscape

if TYPE_CHECKING:
    from collections.abc import Sequence

    from ..measurement_unit import MeasurementUnit
    from ..rng import Rng


class NeighborDistModel(Distribution):
    """Local step-density distribution reconstructed from a sample (algorithm ``dist-neighbor-v1``).

    Each observation contributes a cell whose height reflects how tightly its nearest
    neighbors are packed, so the reconstruction follows local structure instead of imposing
    a single global shape. The window is indexed by k0 = ceil(sqrt(n)) with radius
    r = floor(k0 / 2); windows near the ends are truncated and the numerator is always the
    actual neighbor count.

    The construction is kNN-inspired but is not the Loftsgaarden-Quesenberry estimator:
    support is bounded by the observed range and the cell masses are explicitly normalized,
    so the density integrates to one.

    The model is immutable, so concurrent quantile/cumulative/density reads on a built model
    are safe.

    Queries come in a numeric and a measured flavour, exactly as every estimator does: the
    numeric methods take and return plain ``float`` vectors, and ``model.measured`` answers the
    same three questions in units.

    Example:
        >>> from pragmastat import NeighborDistModel
        >>> NeighborDistModel([1.0, 2.0, 3.0, 4.0, 5.0]).cumulative([1.0, 5.0])
        [0.0, 1.0]
    """

    ALGORITHM = "dist-neighbor-v1"

    def __init__(self, values: Sample | Sequence[float], *, assume_sorted: bool | None = None) -> None:
        """Builds the model from a measured sample or a native numeric vector.

        ``assume_sorted`` applies to the vector path only: a :class:`~pragmastat.sample.Sample`
        carries its own ascending promise, so supplying this argument alongside one is an
        error. Weighted samples are refused: this reconstruction has no weighted form, and
        silently ignoring the weights would answer a different question than the caller asked.
        A native vector gets the dimensionless unit.

        Args:
            values: A :class:`~pragmastat.sample.Sample`, or a fitting vector; must be
                non-empty, finite, of size at least two, with a finite representable range
                and pairwise distinct elements.
            assume_sorted: When true, the caller promises the input is already ascending
                and the internal copy is not sorted. Must be left unset when ``values`` is a
                Sample.

        Raises:
            AssumptionError: If the fitting vector violates validity, domain or distinctness,
                the sample is weighted, or ``assume_sorted`` is set alongside a Sample.
        """
        if isinstance(values, Sample):
            check_no_assume_sorted(assume_sorted)
            _check_non_weighted("x", values)
            # Sample construction already enforced validity, and its values are ascending.
            self._unit = values.unit
            self._build(prepare(values.sorted_values, assume_sorted=True))
            return
        check_validity(values, Subject.X)
        self._unit = NUMBER_UNIT
        self._build(prepare(values, bool(assume_sorted)))

    @property
    def unit(self) -> MeasurementUnit:
        """Unit of the observations this model was built from."""
        return self._unit

    @property
    def measured(self) -> MeasuredView:
        """The measured view of this model."""
        return MeasuredView(self)

    @property
    def cell_count(self) -> int:
        """Number of surviving cells in the reconstruction."""
        return len(self._p)

    @property
    def support_min(self) -> float:
        """Lower end of the support."""
        return self._b[0]

    @property
    def support_max(self) -> float:
        """Upper end of the support."""
        return self._b[-1]

    def quantile(self, cumulances: Sequence[float]) -> list[float]:
        """Value at each requested cumulance, in the order the cumulances were given."""
        check_query_validity(cumulances)
        check_cumulance(cumulances)
        return self._quantile_trusted(cumulances)

    def cumulative(self, values: Sequence[float]) -> list[float]:
        """Probability of not exceeding each requested value, in the order the values were given."""
        check_query_validity(values)
        return self._cumulative_trusted(values)

    def density(self, values: Sequence[float]) -> list[float]:
        """Density at each requested value, in the order the values were given."""
        check_query_validity(values)
        return self._density_trusted(values)

    def _quantile_trusted(self, cumulances: Sequence[float]) -> list[float]:
        """Query path for callers that have already run the assumptions in order."""
        return [self.quantile_at(float(c)) for c in cumulances]

    def _cumulative_trusted(self, values: Sequence[float]) -> list[float]:
        return [self.cumulative_at(float(v)) for v in values]

    def _density_trusted(self, values: Sequence[float]) -> list[float]:
        return [self.density_at(float(v)) for v in values]

    def sample(self, rng: Rng) -> float:
        return self.quantile_at(rng.uniform_float())

    def _build(self, x: list[float]) -> None:
        n = len(x)
        self._scale_index = ceil_sqrt(n)
        self._radius = self._scale_index // 2

        counts = [0] * n
        spans = [0.0] * n
        for i in range(n):
            left = max(0, i - self._radius)
            right = min(n - 1, i + self._radius)
            counts[i] = right - left + 1
            spans[i] = x[right] - x[left]

        e = [0.0] * (n + 1)
        e[0] = x[0]
        for i in range(1, n):
            e[i] = midpoint(x[i - 1], x[i])
        e[n] = x[n - 1]

        # Rounding can collapse the midpoint of two adjacent binary64 observations onto a
        # neighboring boundary. Such a cell carries no mass and is dropped; the surviving
        # cells keep the index of the observation they came from.
        boundaries = [e[0]]
        surviving = []
        for i in range(n):
            if e[i + 1] == e[i]:
                continue
            surviving.append(i)
            boundaries.append(e[i + 1])

        s = len(surviving)
        a = [0.0] * s
        for j in range(s):
            i = surviving[j]
            width = e[i + 1] - e[i]
            a[j] = counts[i] / n * (width / spans[i])

        total = 0.0
        for j in range(s):
            total += a[j]
        # A broken assume_sorted promise can make a cell wider than the window it normalizes
        # against, so its unnormalized mass overflows, or it can collapse every cell and leave a
        # zero total. Either way the divisor is unusable and NaN would reach the query kernels.
        # The sample itself satisfies domain(x) and distinctness(x), so this is not an assumption
        # violation: it is a numerical-conformance failure, reported through the same channel
        # unit_ratio uses. For a genuinely ascending sample the total is finite and strictly
        # positive. It is not bounded by 1: the total is a normalizer, not a probability. It sits
        # above 1 and decays toward it as n grows.
        if not math.isfinite(total) or total <= 0.0:
            raise RuntimeError(
                "Numerical conformance failure: the NeighborDistModel normalizer is not usable; "
                "this indicates an assume_sorted promise that the input does not satisfy"
            )

        # Reserve one representable step for every cell that has not been placed yet, so a
        # mass too small to register at this point in the prefix still advances the cumulative
        # value instead of flattening it into a plateau.
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

        self._b = boundaries
        self._f = f
        self._sources = surviving
        self._p = [f[j + 1] - f[j] for j in range(s)]
        self._d = [self._p[j] / (boundaries[j + 1] - boundaries[j]) for j in range(s)]

    def landscape(self) -> NeighborLandscape:
        """The staircase this model is: cell boundaries, cumulative values, masses and heights.

        Point evaluation answers what the density is at a value a caller picked; this answers
        what the density *is*, on the boundaries the model itself uses. Drawing the second from
        the first means guessing a grid and rebuilding the normalization on top of it, which is
        a second reconstruction of the same object and drifts from this one silently.
        """
        return NeighborLandscape(
            self._scale_index,
            self._radius,
            tuple(self._b),
            tuple(self._f),
            tuple(self._p),
            tuple(self._d),
            tuple(self._sources),
        )

    def quantile_at(self, p: float) -> float:
        """The value below which a fraction p of the reconstruction lies.

        NaN first, for the same reason LinearDistModel.quantile_at checks it: every branch
        below compares false against it, and the fallback would otherwise depend on how the
        platform converts a NaN cumulance to an index.
        """
        if math.isnan(p):
            return math.nan

        b, f, masses = self._b, self._f, self._p
        s = len(masses)
        # Clamped rather than compared: the contract answers a cumulance outside (0, 1)
        # with the endpoints of the support, and the vector path rejects one earlier.
        if p <= 0.0:
            value = b[0]
        elif p >= 1.0:
            value = b[s]
        else:
            j = lower_bound(f, 1, s + 1, p) - 1
            t = unit_ratio(p - f[j], masses[j])
            value = lerp(b[j], b[j + 1], t)

        # Every branch hands back a value built from the sample's own boundaries -- the clamped
        # ones return one verbatim, and lerp reproduces its endpoints exactly -- so a -0.0 among
        # the observations would otherwise reach the caller unnormalized.
        return normalize_zero(value)

    def cumulative_at(self, x: float) -> float:
        """Probability of not exceeding x."""
        if math.isnan(x):
            return math.nan

        b, f, p = self._b, self._f, self._p
        s = len(p)
        if x <= b[0]:
            return 0.0
        if x >= b[s]:
            return 1.0

        j = upper_bound(b, x) - 1
        t = unit_ratio(x - b[j], b[j + 1] - b[j])
        return lerp(f[j], f[j + 1], t)

    def density_at(self, x: float) -> float:
        """Density at x."""
        if math.isnan(x):
            return math.nan

        b, p = self._b, self._p
        s = len(p)
        if x < b[0] or x > b[s]:
            return 0.0

        j = s - 1 if x == b[s] else upper_bound(b, x) - 1
        return p[j] / (b[j + 1] - b[j])
