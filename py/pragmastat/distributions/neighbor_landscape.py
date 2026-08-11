"""The staircase a NeighborDistModel is."""

from __future__ import annotations

from dataclasses import dataclass


@dataclass(frozen=True)
class NeighborLandscape:
    """The staircase a NeighborDistModel is: cell boundaries, cumulative values, masses and heights.

    Point evaluation answers what the density is at a value a caller picked; this answers what
    the density *is*, on the boundaries the model itself uses. Drawing the second from the first
    means guessing a grid and rebuilding the normalization on top of it, which is a second
    reconstruction of the same object and drifts from this one silently.

    Attributes:
        scale_index: Smoothing scale index k0 = ceil(sqrt(n)).
        radius: Window radius r = floor(k0 / 2).
        boundaries: Strictly increasing cell boundaries, length ``cell_count + 1``.
        cumulative: Cumulative values at the boundaries, from 0 to exactly 1.
        masses: Strictly positive cell masses, length ``cell_count``.
        densities: Cell heights, mass divided by width.
        sources: Index of the sorted observation each surviving cell was derived from.
    """

    scale_index: int
    radius: int
    boundaries: tuple[float, ...]
    cumulative: tuple[float, ...]
    masses: tuple[float, ...]
    densities: tuple[float, ...]
    sources: tuple[int, ...]

    @property
    def cell_count(self) -> int:
        """Number of surviving cells."""
        return len(self.masses)
