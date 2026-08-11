"""The staircase a LinearDistModel is."""

from __future__ import annotations

from dataclasses import dataclass


@dataclass(frozen=True)
class LinearLandscape:
    """The staircase a :class:`LinearDistModel` is: cell boundaries, cumulative values and heights.

    The model interpolates the order statistics linearly, so its density is a staircase: one
    cell between each pair of adjacent observations, each carrying the same mass 1/(n-1) and
    therefore a height that is that mass over its width.

    Every field is an immutable tuple handed out by the model, so holding a landscape can never
    mutate it.
    """

    boundaries: tuple[float, ...]
    """The sorted observations, which are the cell boundaries; length cell_count + 1."""

    cumulative: tuple[float, ...]
    """Cumulative values at the boundaries, from 0 to exactly 1."""

    densities: tuple[float, ...]
    """Cell heights, mass divided by width; length cell_count."""

    @property
    def cell_count(self) -> int:
        """Number of cells, one fewer than the number of observations."""
        return len(self.densities)
