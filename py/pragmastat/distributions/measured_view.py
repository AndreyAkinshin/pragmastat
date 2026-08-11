"""The measured view shared by the distribution estimation models."""

from __future__ import annotations

from typing import TYPE_CHECKING

from ..dist_sample_guard import to_measurements, to_raw

if TYPE_CHECKING:
    from collections.abc import Sequence

    from ..measurement import Measurement
    from .linear_dist_model import LinearDistModel
    from .neighbor_dist_model import NeighborDistModel


class MeasuredView:
    """Measured queries against a built ``LinearDistModel`` or ``NeighborDistModel``.

    The metrology path lives behind ``model.measured`` rather than as a duck-typed branch of
    the numeric methods. A cumulance is a plain ``float`` in this port, so ``quantile`` would
    have to tell ``[0.5]`` (numeric) apart from ``[0.5]`` (measured) by the return type alone,
    which no runtime dispatch can do. ``cumulative`` and ``density`` could branch on
    :class:`~pragmastat.measurement.Measurement`, but a facade where one of the three
    operations sits somewhere else is worse than one that names its world once.

    Only quantile carries a unit -- the sample's own. cumulative answers in probabilities and
    density is a terminal value read, plotted, or compared against another density of the same
    model, never converted or combined, so both come back as plain numbers.
    """

    __slots__ = ("_model",)

    def __init__(self, model: LinearDistModel | NeighborDistModel) -> None:
        """Wraps a built model with the measured view over its numeric queries."""
        self._model = model

    def quantile(self, cumulances: Sequence[float]) -> list[Measurement]:
        """Values at the requested cumulances, in the unit of the sample."""
        return to_measurements(self._model.quantile(cumulances), self._model.unit)

    def cumulative(self, values: Sequence[Measurement]) -> list[float]:
        """Probabilities of not exceeding the requested measurements."""
        return self._model.cumulative(to_raw(values, self._model.unit))

    def density(self, values: Sequence[Measurement]) -> list[float]:
        """Densities at the requested measurements, as plain numbers."""
        return self._model.density(to_raw(values, self._model.unit))
