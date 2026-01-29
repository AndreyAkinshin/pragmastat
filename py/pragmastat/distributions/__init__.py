"""Statistical distributions for sampling.

This module provides five distributions for generating random samples:
- Uniform: uniform distribution on a bounded interval
- Additive: normal (Gaussian) distribution
- Multiplic: log-normal distribution
- Exp: exponential distribution
- Power: Pareto (power-law) distribution

All distributions produce identical sequences across all Pragmastat language
implementations when using the same seed.
"""

from .additive import Additive
from .distribution import Distribution
from .exp import Exp
from .multiplic import Multiplic
from .power import Power
from .uniform import Uniform

__all__ = [
    "Distribution",
    "Uniform",
    "Additive",
    "Multiplic",
    "Exp",
    "Power",
]
