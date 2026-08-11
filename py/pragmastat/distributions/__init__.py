"""Statistical distributions for sampling.

This module provides five parametric distributions for generating random samples:
- Uniform: uniform distribution on a bounded interval
- Additive: normal (Gaussian) distribution
- Multiplic: log-normal distribution
- Exp: exponential distribution
- Power: Pareto (power-law) distribution

It also provides a model reconstructed from a sample rather than from parameters:
- LinearDistModel: piecewise-linear (Hyndman-Fan type 7) reconstruction

All distributions produce identical sequences across all Pragmastat language
implementations when using the same seed.
"""

from .additive import Additive
from .distribution import Distribution
from .exp import Exp
from .linear_dist_model import LinearDistModel
from .linear_landscape import LinearLandscape
from .measured_view import MeasuredView
from .multiplic import Multiplic
from .power import Power
from .uniform import Uniform

__all__ = [
    "Additive",
    "LinearDistModel",
    "LinearLandscape",
    "Distribution",
    "Exp",
    "MeasuredView",
    "Multiplic",
    "Power",
    "Uniform",
]
