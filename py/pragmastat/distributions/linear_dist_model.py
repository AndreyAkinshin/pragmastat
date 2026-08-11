"""Piecewise-linear distribution reconstructed from a sample."""

from __future__ import annotations

import math
from typing import TYPE_CHECKING

from .._normalize_zero import normalize_zero
from ..assumptions import Subject, check_cumulance, check_query_validity, check_validity
from ..dist_numerics import lerp, unit_ratio, upper_bound
from ..dist_sample_guard import check_no_assume_sorted, prepare
from ..measurement_unit import NUMBER_UNIT
from ..sample import Sample, _check_non_weighted
from .distribution import Distribution
from .linear_landscape import LinearLandscape
from .measured_view import MeasuredView

if TYPE_CHECKING:
    from collections.abc import Sequence

    from ..measurement_unit import MeasurementUnit
    from ..rng import Rng


class LinearDistModel(Distribution):
    """Piecewise-linear distribution reconstructed from a sample (algorithm ``dist-linear-v1``).

    The model interpolates the order statistics at the anchors (i-1)/(n-1), which is the
    Hyndman-Fan type 7 rule. It is immutable: the constructor copies and, unless the caller
    promises otherwise, sorts the input, so concurrent quantile/cumulative/density reads on
    a built model are safe.

    Support is [min(x), max(x)]. Queries come in a numeric and a measured flavour, exactly as
    every estimator does: the numeric methods take and return plain ``float`` vectors, and
    ``model.measured`` answers the same three questions in units.

    Example:
        >>> from pragmastat import LinearDistModel
        >>> LinearDistModel([1.0, 3.0, 5.0, 7.0, 9.0]).quantile([0.375])
        [4.0]
    """

    ALGORITHM = "dist-linear-v1"

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
                and the internal copy is not sorted. Passing an unsorted vector under this
                flag is a contract violation with an unspecified but terminating result.
                Must be left unset when ``values`` is a Sample.

        Raises:
            AssumptionError: If the fitting vector violates validity, domain or distinctness,
                the sample is weighted, or ``assume_sorted`` is set alongside a Sample.
        """
        if isinstance(values, Sample):
            check_no_assume_sorted(assume_sorted)
            _check_non_weighted("x", values)
            # Sample construction already enforced validity, and its values are ascending.
            self._unit = values.unit
            self._x = prepare(values.sorted_values, assume_sorted=True)
            return
        check_validity(values, Subject.X)
        self._unit = NUMBER_UNIT
        self._x = prepare(values, bool(assume_sorted))

    @property
    def unit(self) -> MeasurementUnit:
        """Unit of the observations this model was built from."""
        return self._unit

    @property
    def measured(self) -> MeasuredView:
        """The measured view of this model."""
        return MeasuredView(self)

    @property
    def size(self) -> int:
        """Number of observations backing the model."""
        return len(self._x)

    @property
    def support_min(self) -> float:
        """Lower end of the support."""
        return self._x[0]

    @property
    def support_max(self) -> float:
        """Upper end of the support."""
        return self._x[-1]

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

    def quantile_at(self, p: float) -> float:
        """The value below which a fraction p of the reconstruction lies.

        NaN first, for the same reason cumulative_at and density_at check it: every branch
        below compares false against it, and the fallback would otherwise depend on how the
        platform converts a NaN cumulance to an index.
        """
        if math.isnan(p):
            return math.nan

        x = self._x
        n = len(x)
        # Clamped rather than compared: the contract answers a cumulance outside (0, 1)
        # with the endpoints of the support, and the vector path rejects one earlier.
        if p <= 0.0:
            value = x[0]
        elif p >= 1.0:
            value = x[n - 1]
        else:
            r = (n - 1) * p
            j = math.floor(r)
            g = r - j
            value = lerp(x[j], x[j + 1], g)

        # Every branch hands back a value the sample supplied -- the clamped ones return an
        # order statistic verbatim, and lerp reproduces its endpoints exactly -- so a -0.0
        # among the observations would otherwise reach the caller unnormalized.
        return normalize_zero(value)

    def cumulative_at(self, x: float) -> float:
        """Probability of not exceeding x."""
        if math.isnan(x):
            return math.nan

        values = self._x
        n = len(values)
        if x <= values[0]:
            return 0.0
        if x >= values[n - 1]:
            return 1.0

        j = upper_bound(values, x) - 1
        t = unit_ratio(x - values[j], values[j + 1] - values[j])
        return (j + t) / (n - 1)

    def density_at(self, x: float) -> float:
        """Density at x."""
        if math.isnan(x):
            return math.nan

        values = self._x
        n = len(values)
        if x < values[0] or x > values[n - 1]:
            return 0.0

        j = n - 2 if x == values[n - 1] else upper_bound(values, x) - 1
        return 1.0 / (n - 1) / (values[j + 1] - values[j])

    def landscape(self) -> LinearLandscape:
        """The staircase this model is: cell boundaries, cumulative values and heights.

        Point evaluation answers what the density is at a value a caller picked; this answers
        what the density *is*, on the boundaries the model itself uses. Every height is the
        expression density_at evaluates inside the same cell, so the two agree bit for bit
        rather than approximately.
        """
        x = self._x
        n = len(x)
        # cumulative_at's general path for an exact order statistic x[i] always resolves index
        # j = i and interpolation fraction t = 0.0, so it reduces to exactly i / (n - 1) --
        # computing that directly skips an O(log n) binary search per boundary without changing
        # a single bit of the result.
        cumulative = [i / (n - 1) for i in range(n)]
        densities = [1.0 / (n - 1) / (x[j + 1] - x[j]) for j in range(n - 1)]
        return LinearLandscape(tuple(x), tuple(cumulative), tuple(densities))
