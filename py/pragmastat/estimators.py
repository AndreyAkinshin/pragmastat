from typing import Sequence, Union, NamedTuple
import math
import numpy as np
from numpy.typing import NDArray
from .fast_center import _fast_center
from .fast_spread import _fast_spread
from .fast_shift import _fast_shift
from .pairwise_margin import pairwise_margin
from .signed_rank_margin import signed_rank_margin
from .min_misrate import min_achievable_misrate_one_sample
from ._fast_center_quantiles import fast_center_quantile_bounds
from .gauss_cdf import gauss_cdf
from .rng import Rng
from .assumptions import (
    check_validity,
    check_positivity,
    check_sparity,
    log,
    AssumptionError,
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
        AssumptionError: If misrate is out of range.
    """
    x = np.asarray(x)
    y = np.asarray(y)

    # Check validity for x
    check_validity(x, "x")
    # Check validity for y
    check_validity(y, "y")

    if math.isnan(misrate) or misrate < 0 or misrate > 1:
        raise AssumptionError.domain("misrate")

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
        AssumptionError: If misrate is out of range.
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


def _binomial_tail_probability(n: int, k: int) -> float:
    """Computes binomial tail probability: P(Bin(n, 0.5) <= k).

    Note: 2**n overflows float for n > 1024.
    """
    if k < 0:
        return 0.0
    if k >= n:
        return 1.0

    # Normal approximation with continuity correction for large n
    # (2**n overflows float for n > 1024)
    if n > 1023:
        mean = n / 2.0
        std = (n / 4.0) ** 0.5
        z = (k + 0.5 - mean) / std
        return gauss_cdf(z)

    sum_ = 0.0
    coef = 1.0  # C(n, i) starting with C(n, 0) = 1
    total = 2.0**n

    for i in range(k + 1):
        sum_ += coef
        coef = coef * (n - i) / (i + 1)

    return sum_ / total


def median_bounds(
    x: Union[Sequence[float], NDArray],
    misrate: float,
) -> Bounds:
    """
    Provides bounds on the Median with specified misclassification rate (median_bounds).

    Uses order statistics based on the binomial distribution to determine
    which sample values form the bounds.

    Args:
        x: Sample array
        misrate: Misclassification rate (probability that true median falls outside bounds)

    Returns:
        A Bounds object containing the lower and upper bounds

    Raises:
        AssumptionError: If sample is empty or contains NaN/Inf.
        AssumptionError: If misrate is below minimum achievable.
    """
    x = np.asarray(x)
    check_validity(x, "x")

    if math.isnan(misrate) or misrate < 0 or misrate > 1:
        raise AssumptionError.domain("misrate")

    n = len(x)

    if n < 2:
        raise AssumptionError.domain("x")

    sorted_x = sorted(x.tolist())

    # Validate misrate
    min_misrate = min_achievable_misrate_one_sample(n)
    if misrate < min_misrate:
        raise AssumptionError.domain("misrate")

    alpha = misrate / 2.0

    # Find the largest k where P(Bin(n,0.5) <= k) <= alpha
    lo = 0
    for k in range((n + 1) // 2):
        tail_prob = _binomial_tail_probability(n, k)
        if tail_prob <= alpha:
            lo = k
        else:
            break

    # Symmetric interval: hi = n - 1 - lo
    hi = n - 1 - lo

    if hi < lo:
        hi = lo
    if hi >= n:
        hi = n - 1

    return Bounds(sorted_x[lo], sorted_x[hi])


def center_bounds(
    x: Union[Sequence[float], NDArray],
    misrate: float,
) -> Bounds:
    """
    Provides exact bounds on the Center (Hodges-Lehmann pseudomedian) with specified misrate.

    Uses SignedRankMargin to determine which pairwise averages form the bounds.

    Args:
        x: Sample array
        misrate: Misclassification rate (probability that true center falls outside bounds)

    Returns:
        A Bounds object containing the lower and upper bounds

    Raises:
        AssumptionError: If sample is empty or contains NaN/Inf.
        AssumptionError: If misrate is below minimum achievable.
    """
    x = np.asarray(x)
    check_validity(x, "x")

    if math.isnan(misrate) or misrate < 0 or misrate > 1:
        raise AssumptionError.domain("misrate")

    n = len(x)

    if n < 2:
        raise AssumptionError.domain("x")

    # Validate misrate
    min_misrate = min_achievable_misrate_one_sample(n)
    if misrate < min_misrate:
        raise AssumptionError.domain("misrate")

    # Total number of pairwise averages (including self-pairs)
    total_pairs = n * (n + 1) // 2

    # Get signed-rank margin
    margin = signed_rank_margin(n, misrate)
    half_margin = min(margin // 2, (total_pairs - 1) // 2)

    # k_left and k_right are 1-based ranks
    k_left = half_margin + 1
    k_right = total_pairs - half_margin

    # Sort the input
    sorted_x = sorted(x.tolist())

    lo, hi = fast_center_quantile_bounds(sorted_x, k_left, k_right)
    return Bounds(lo, hi)


CENTER_BOUNDS_APPROX_ITERATIONS = 10000
CENTER_BOUNDS_APPROX_MAX_SUBSAMPLE = 5000
CENTER_BOUNDS_APPROX_DEFAULT_SEED = "center-bounds-approx"


def center_bounds_approx(
    x: Union[Sequence[float], NDArray],
    misrate: float,
    seed: str | None = None,
) -> Bounds:
    """
    Provides bootstrap-based nominal bounds for Center (Hodges-Lehmann pseudomedian).

    IMPORTANT: The misrate parameter specifies the NOMINAL (requested) coverage,
    not the actual coverage. The actual coverage of bootstrap percentile intervals
    can differ significantly from the nominal coverage.

    When requesting 95% confidence (misrate = 0.05), actual coverage is typically 85-92% for n < 30.
    Users requiring exact coverage should use center_bounds (if symmetry holds) or median_bounds.

    Args:
        x: Sample array
        misrate: Misclassification rate (probability that true center falls outside bounds)
        seed: Optional seed for deterministic results

    Returns:
        A Bounds object containing the lower and upper bounds

    Raises:
        AssumptionError: If sample is empty or contains NaN/Inf.
        AssumptionError: If n < 2 or misrate is below minimum achievable.
    """
    x = np.asarray(x)
    check_validity(x, "x")

    if math.isnan(misrate) or misrate < 0 or misrate > 1:
        raise AssumptionError.domain("misrate")

    n = len(x)
    if n < 2:
        raise AssumptionError.domain("x")

    min_misrate = max(
        2.0 / CENTER_BOUNDS_APPROX_ITERATIONS, min_achievable_misrate_one_sample(n)
    )
    if misrate < min_misrate:
        raise AssumptionError.domain("misrate")

    # Subsample if necessary
    m = min(n, CENTER_BOUNDS_APPROX_MAX_SUBSAMPLE)

    # Sort the input
    sorted_x = sorted(x.tolist())

    # Initialize RNG
    rng = Rng(seed if seed is not None else CENTER_BOUNDS_APPROX_DEFAULT_SEED)

    # Bootstrap iterations
    centers = []
    for _ in range(CENTER_BOUNDS_APPROX_ITERATIONS):
        sample = rng.resample(sorted_x, m)
        c = _fast_center(sample)
        centers.append(c)

    # Sort bootstrap centers
    centers.sort()

    # Extract percentile bounds
    alpha = misrate / 2.0
    lo_idx = int(math.floor(alpha * CENTER_BOUNDS_APPROX_ITERATIONS))
    hi_idx = int(math.ceil((1.0 - alpha) * CENTER_BOUNDS_APPROX_ITERATIONS)) - 1
    lo_idx = min(max(0, lo_idx), hi_idx)

    bootstrap_lo = centers[lo_idx]
    bootstrap_hi = centers[min(CENTER_BOUNDS_APPROX_ITERATIONS - 1, hi_idx)]

    # Scale bounds to full n using asymptotic sqrt(n) rate
    if m < n:
        center_val = _fast_center(sorted_x)
        scale_factor = (m / n) ** 0.5
        lo = center_val + (bootstrap_lo - center_val) / scale_factor
        hi = center_val + (bootstrap_hi - center_val) / scale_factor
        return Bounds(lo, hi)

    return Bounds(bootstrap_lo, bootstrap_hi)
