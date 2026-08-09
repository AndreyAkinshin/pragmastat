"""The cumulance additive and multiplic consume.

They are the two families whose quantile_at(0) is not a sample: additive returns negative
infinity there, multiplic returns the zero its own support excludes. The other three families'
quantile_at(0) is the finite value their support already includes, so they draw uniform_float()
directly and never call this.

A drawn 0 is lifted to the smallest positive subnormal. uniform_float() never returns 1, so the
other endpoint needs no guard.
"""

from typing import TYPE_CHECKING

from .._constants import SMALLEST_POSITIVE_SUBNORMAL

if TYPE_CHECKING:
    from ..rng import Rng


def sampling_cumulance(rng: "Rng") -> float:
    u = rng.uniform_float()
    return SMALLEST_POSITIVE_SUBNORMAL if u == 0.0 else u
