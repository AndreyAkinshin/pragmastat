from typing import Sequence, Union, NamedTuple
import numpy as np
from numpy.typing import NDArray
from .fast_center import _fast_center
from .fast_spread import _fast_spread
from .fast_shift import _fast_shift
from .pairwise_margin import pairwise_margin
from .assumptions import (
    check_validity,
    check_positivity,
    check_sparity,
    log,
)


class Bounds(NamedTuple):
    """Represents an interval with lower and upper bounds."""

    lower: float
    upper: float


def median(x: Union[Sequence[float], NDArray]) -> float:
    """
    Calculate the median of a sample.

    Args:
        x: Input sample.

    Returns:
        The median value.

    Raises:
        AssumptionError: If input is empty or contains NaN/Inf.
    """
    x = np.asarray(x)
    check_validity(x, "x")
    return float(np.median(x))


def center(x: Union[Sequence[float], NDArray]) -> float:
    """
    Estimate the central value using Hodges-Lehmann estimator.

    Calculates the median of all pairwise averages (x[i] + x[j])/2.
    More robust than the mean and more efficient than the median.

    Args:
        x: Input sample.

    Returns:
        Center estimate (median of pairwise averages).

    Raises:
        AssumptionError: If input is empty or contains NaN/Inf.
    """
    x = np.asarray(x)
    check_validity(x, "x")
    # Use fast O(n log n) algorithm
    return _fast_center(x.tolist())


def spread(x: Union[Sequence[float], NDArray]) -> float:
    """
    Estimate data dispersion using Shamos estimator.

    Calculates the median of all pairwise absolute differences |x[i] - x[j]|.
    More robust than standard deviation and more efficient than MAD.

    Assumptions:
        sparity(x) - sample must be non tie-dominant (Spread > 0)

    Args:
        x: Input sample.

    Returns:
        Spread estimate (median of pairwise absolute differences).

    Raises:
        AssumptionError: If sample is empty, contains NaN/Inf,
            or is tie-dominant.
    """
    x = np.asarray(x)
    # Check validity (priority 0)
    check_validity(x, "x")
    # Check sparity (priority 2)
    check_sparity(x, "x")
    # Use fast O(n log n) algorithm
    return _fast_spread(x.tolist())


def rel_spread(x: Union[Sequence[float], NDArray]) -> float:
    """
    Measure relative dispersion of a sample.

    Calculates the ratio of Spread to absolute Center.
    Robust alternative to the coefficient of variation.

    Assumptions:
        positivity(x) - all values must be strictly positive (ensures Center > 0)

    Args:
        x: Input sample.

    Returns:
        Relative spread (Spread / |Center|).

    Raises:
        AssumptionError: If sample is empty, contains NaN/Inf,
            or contains non-positive values.
    """
    x = np.asarray(x)
    # Check validity (priority 0)
    check_validity(x, "x")
    # Check positivity (priority 1)
    check_positivity(x, "x")
    # Calculate center (we know x is valid, center should succeed)
    center_val = _fast_center(x.tolist())
    # Calculate spread (using internal implementation since we already validated)
    spread_val = _fast_spread(x.tolist())
    # center_val is guaranteed positive because all values are positive
    return spread_val / abs(center_val)


def shift(
    x: Union[Sequence[float], NDArray], y: Union[Sequence[float], NDArray]
) -> float:
    """
    Measure the typical difference between elements of x and y.

    Calculates the median of all pairwise differences (x[i] - y[j]).
    Positive values mean x is typically larger, negative means y is typically larger.

    Args:
        x: First sample.
        y: Second sample.

    Returns:
        Shift estimate (median of pairwise differences).

    Raises:
        AssumptionError: If either input is empty or contains NaN/Inf.
    """
    x = np.asarray(x)
    y = np.asarray(y)
    check_validity(x, "x")
    check_validity(y, "y")
    # Use fast O((m+n) log L) algorithm instead of materializing all m*n differences
    return float(_fast_shift(x, y, p=0.5))


def ratio(
    x: Union[Sequence[float], NDArray], y: Union[Sequence[float], NDArray]
) -> float:
    """
    Measure how many times larger x is compared to y.

    Calculates the median of all pairwise ratios (x[i] / y[j]) via log-transformation.
    Equivalent to: exp(Shift(log(x), log(y)))
    For example, ratio = 1.2 means x is typically 20% larger than y.
    Uses fast O((m+n) log L) algorithm.

    Assumptions:
        positivity(x) - all values in x must be strictly positive
        positivity(y) - all values in y must be strictly positive

    Args:
        x: First sample.
        y: Second sample (must be strictly positive).

    Returns:
        Ratio estimate (median of pairwise ratios).

    Raises:
        AssumptionError: If either sample is empty, contains NaN/Inf,
            or contains non-positive values.
    """
    x = np.asarray(x)
    y = np.asarray(y)
    # Check validity for x (priority 0, subject x)
    check_validity(x, "x")
    # Check validity for y (priority 0, subject y)
    check_validity(y, "y")
    # Check positivity for x (priority 1, subject x)
    check_positivity(x, "x")
    # Check positivity for y (priority 1, subject y)
    check_positivity(y, "y")
    # Log-transform, compute shift, exp-transform back
    log_x = np.log(x)
    log_y = np.log(y)
    log_result = _fast_shift(log_x, log_y, p=0.5)
    return float(np.exp(log_result))


def avg_spread(
    x: Union[Sequence[float], NDArray], y: Union[Sequence[float], NDArray]
) -> float:
    """
    Measure the typical variability when considering both samples together.

    Computes the weighted average of individual spreads:
    (n * Spread(x) + m * Spread(y)) / (n + m).

    Assumptions:
        sparity(x) - first sample must be non tie-dominant (Spread > 0)
        sparity(y) - second sample must be non tie-dominant (Spread > 0)

    Args:
        x: First sample.
        y: Second sample.

    Returns:
        Weighted average of individual spreads.

    Raises:
        AssumptionError: If either sample is empty, contains NaN/Inf,
            or is tie-dominant.
    """
    x = np.asarray(x)
    y = np.asarray(y)
    # Check validity for x (priority 0, subject x)
    check_validity(x, "x")
    # Check validity for y (priority 0, subject y)
    check_validity(y, "y")
    # Check sparity for x (priority 2, subject x)
    check_sparity(x, "x")
    # Check sparity for y (priority 2, subject y)
    check_sparity(y, "y")
    n = len(x)
    m = len(y)
    # Calculate spreads (using internal implementation since we already validated)
    spread_x = _fast_spread(x.tolist())
    spread_y = _fast_spread(y.tolist())
    return (n * spread_x + m * spread_y) / (n + m)


def disparity(
    x: Union[Sequence[float], NDArray], y: Union[Sequence[float], NDArray]
) -> float:
    """
    Measure effect size: a normalized difference between x and y.

    Calculated as Shift / AvgSpread. Robust alternative to Cohen's d.

    Assumptions:
        sparity(x) - first sample must be non tie-dominant (Spread > 0)
        sparity(y) - second sample must be non tie-dominant (Spread > 0)

    Args:
        x: First sample.
        y: Second sample.

    Returns:
        Effect size (Shift / AvgSpread).

    Raises:
        AssumptionError: If either sample is empty, contains NaN/Inf,
            or is tie-dominant.
    """
    x = np.asarray(x)
    y = np.asarray(y)
    # Check validity for x (priority 0, subject x)
    check_validity(x, "x")
    # Check validity for y (priority 0, subject y)
    check_validity(y, "y")
    # Check sparity for x (priority 2, subject x)
    check_sparity(x, "x")
    # Check sparity for y (priority 2, subject y)
    check_sparity(y, "y")
    n = len(x)
    m = len(y)
    # Calculate shift (we know inputs are valid)
    shift_val = float(_fast_shift(x, y, p=0.5))
    # Calculate avg_spread (using internal implementation since we already validated)
    spread_x = _fast_spread(x.tolist())
    spread_y = _fast_spread(y.tolist())
    avg_spread_val = (n * spread_x + m * spread_y) / (n + m)
    return shift_val / avg_spread_val


def shift_bounds(
    x: Union[Sequence[float], NDArray],
    y: Union[Sequence[float], NDArray],
    misrate: float,
) -> Bounds:
    """
    Provides bounds on the Shift estimator with specified misclassification rate.

    The misrate represents the probability that the true shift falls outside
    the computed bounds. This is a pragmatic alternative to traditional confidence
    intervals for the Hodges-Lehmann estimator.

    Args:
        x: First sample
        y: Second sample
        misrate: Misclassification rate (probability that true shift falls outside bounds)

    Returns:
        A Bounds object containing the lower and upper bounds

    Raises:
        AssumptionError: If either sample is empty or contains NaN/Inf.
        ValueError: If misrate is out of range.
    """
    x = np.asarray(x)
    y = np.asarray(y)

    # Check validity for x
    check_validity(x, "x")
    # Check validity for y
    check_validity(y, "y")

    n = len(x)
    m = len(y)

    # Sort both arrays
    xs = sorted(x.tolist())
    ys = sorted(y.tolist())

    total = n * m

    # Special case: when there's only one pairwise difference, bounds collapse to a single value
    if total == 1:
        value = xs[0] - ys[0]
        return Bounds(value, value)

    margin = pairwise_margin(n, m, misrate)
    half_margin = min(margin // 2, (total - 1) // 2)
    k_left = half_margin
    k_right = (total - 1) - half_margin

    # Compute quantile positions
    denominator = total - 1 if total > 1 else 1
    p = [k_left / denominator, k_right / denominator]

    bounds = _fast_shift(xs, ys, p, assume_sorted=True)

    lower = min(bounds)
    upper = max(bounds)
    return Bounds(lower, upper)


def ratio_bounds(
    x: Union[Sequence[float], NDArray],
    y: Union[Sequence[float], NDArray],
    misrate: float,
) -> Bounds:
    """
    Provides bounds on the Ratio estimator with specified misclassification rate.

    Computes bounds via log-transformation and shift_bounds delegation:
    ratio_bounds(x, y, misrate) = exp(shift_bounds(log(x), log(y), misrate))

    Assumptions:
        positivity(x) - all values in x must be strictly positive
        positivity(y) - all values in y must be strictly positive

    Args:
        x: First sample (must be strictly positive)
        y: Second sample (must be strictly positive)
        misrate: Misclassification rate (probability that true ratio falls outside bounds)

    Returns:
        A Bounds object containing the lower and upper bounds

    Raises:
        AssumptionError: If either sample is empty, contains NaN/Inf,
            or contains non-positive values.
        ValueError: If misrate is out of range.
    """
    x = np.asarray(x)
    y = np.asarray(y)

    check_validity(x, "x")
    check_validity(y, "y")

    # Log-transform samples (includes positivity check)
    log_x = log(x, "x")
    log_y = log(y, "y")

    # Delegate to shift_bounds in log-space
    log_bounds = shift_bounds(log_x, log_y, misrate)

    # Exp-transform back to ratio-space
    return Bounds(np.exp(log_bounds.lower), np.exp(log_bounds.upper))
