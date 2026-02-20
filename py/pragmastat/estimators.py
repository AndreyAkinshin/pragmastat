from typing import Sequence, Union, NamedTuple
import math
import warnings
import numpy as np
from numpy.typing import NDArray
from .fast_center import _fast_center
from .fast_spread import _fast_spread
from .fast_shift import _fast_shift
from .pairwise_margin import pairwise_margin
from .signed_rank_margin import signed_rank_margin
from .sign_margin import sign_margin_randomized
from .min_misrate import (
    min_achievable_misrate_one_sample,
    min_achievable_misrate_two_sample,
)
from ._fast_center_quantiles import fast_center_quantile_bounds
from .rng import Rng
from .assumptions import (
    check_validity,
    check_positivity,
    log,
    AssumptionError,
)


DEFAULT_MISRATE = 1e-3


class Bounds(NamedTuple):
    """Represents an interval with lower and upper bounds."""

    lower: float
    upper: float


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
    spread_val = _fast_spread(x.tolist())
    if spread_val <= 0:
        raise AssumptionError.sparity("x")
    return spread_val


def rel_spread(x: Union[Sequence[float], NDArray]) -> float:
    """
    Measure relative dispersion of a sample.

    .. deprecated::
        Use ``spread(x) / abs(center(x))`` instead.

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
    warnings.warn(
        "rel_spread is deprecated. Use spread(x) / abs(center(x)) instead.",
        DeprecationWarning,
        stacklevel=2,
    )
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


def _avg_spread(
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
    n = len(x)
    m = len(y)
    spread_x = _fast_spread(x.tolist())
    if spread_x <= 0:
        raise AssumptionError.sparity("x")
    spread_y = _fast_spread(y.tolist())
    if spread_y <= 0:
        raise AssumptionError.sparity("y")
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
    n = len(x)
    m = len(y)
    spread_x = _fast_spread(x.tolist())
    if spread_x <= 0:
        raise AssumptionError.sparity("x")
    spread_y = _fast_spread(y.tolist())
    if spread_y <= 0:
        raise AssumptionError.sparity("y")
    # Calculate shift (we know inputs are valid)
    shift_val = float(_fast_shift(x, y, p=0.5))
    avg_spread_val = (n * spread_x + m * spread_y) / (n + m)
    return shift_val / avg_spread_val


def shift_bounds(
    x: Union[Sequence[float], NDArray],
    y: Union[Sequence[float], NDArray],
    misrate: float = DEFAULT_MISRATE,
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

    min_misrate = min_achievable_misrate_two_sample(n, m)
    if misrate < min_misrate:
        raise AssumptionError.domain("misrate")

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
    misrate: float = DEFAULT_MISRATE,
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

    if math.isnan(misrate) or misrate < 0 or misrate > 1:
        raise AssumptionError.domain("misrate")

    min_misrate = min_achievable_misrate_two_sample(len(x), len(y))
    if misrate < min_misrate:
        raise AssumptionError.domain("misrate")

    # Log-transform samples (includes positivity check)
    log_x = log(x, "x")
    log_y = log(y, "y")

    # Delegate to shift_bounds in log-space
    log_bounds = shift_bounds(log_x, log_y, misrate)

    # Exp-transform back to ratio-space
    return Bounds(np.exp(log_bounds.lower), np.exp(log_bounds.upper))


def center_bounds(
    x: Union[Sequence[float], NDArray],
    misrate: float = DEFAULT_MISRATE,
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


def spread_bounds(
    x: Union[Sequence[float], NDArray],
    misrate: float = DEFAULT_MISRATE,
    seed: Union[str, None] = None,
) -> Bounds:
    """
    Provides distribution-free bounds for Spread using disjoint pairs with sign-test inversion.

    Args:
        x: Input sample.
        misrate: Misclassification rate (probability that true spread falls outside bounds).
        seed: Optional string seed for deterministic randomization.

    Returns:
        A Bounds object containing the lower and upper bounds.

    Raises:
        AssumptionError: If sample is empty or contains NaN/Inf.
        AssumptionError: If misrate is out of range or below minimum achievable.
        AssumptionError: If sample is tie-dominant.
    """
    x = np.asarray(x)
    check_validity(x, "x")

    if math.isnan(misrate) or misrate < 0 or misrate > 1:
        raise AssumptionError.domain("misrate")

    n = len(x)
    m = n // 2
    min_misrate_val = min_achievable_misrate_one_sample(m)
    if misrate < min_misrate_val:
        raise AssumptionError.domain("misrate")

    if n < 2:
        raise AssumptionError.sparity("x")
    if _fast_spread(x.tolist()) <= 0:
        raise AssumptionError.sparity("x")

    rng = Rng(seed) if seed is not None else Rng()
    margin = sign_margin_randomized(m, misrate, rng)
    half_margin = margin // 2
    max_half_margin = (m - 1) // 2
    if half_margin > max_half_margin:
        half_margin = max_half_margin

    k_left = half_margin + 1
    k_right = m - half_margin

    indices = list(range(n))
    shuffled = rng.shuffle(indices)
    diffs = sorted(
        abs(float(x[shuffled[2 * i]]) - float(x[shuffled[2 * i + 1]])) for i in range(m)
    )

    return Bounds(diffs[k_left - 1], diffs[k_right - 1])


def disparity_bounds(
    x: Union[Sequence[float], NDArray],
    y: Union[Sequence[float], NDArray],
    misrate: float = DEFAULT_MISRATE,
    seed: Union[str, None] = None,
) -> Bounds:
    """
    Provides distribution-free bounds for the Disparity estimator (Shift / AvgSpread)
    using Bonferroni combination of ShiftBounds and AvgSpreadBounds.

    Args:
        x: First input sample.
        y: Second input sample.
        misrate: Misclassification rate.
        seed: Optional string seed for deterministic randomization.

    Returns:
        A Bounds object containing the lower and upper bounds.

    Raises:
        AssumptionError: If inputs are invalid, misrate is out of range, or samples are tie-dominant.
    """
    x = np.asarray(x)
    y = np.asarray(y)

    # Check validity (priority 0)
    check_validity(x, "x")
    check_validity(y, "y")

    if math.isnan(misrate) or misrate < 0 or misrate > 1:
        raise AssumptionError.domain("misrate")

    n = len(x)
    m = len(y)
    if n < 2:
        raise AssumptionError.domain("x")
    if m < 2:
        raise AssumptionError.domain("y")

    min_shift = min_achievable_misrate_two_sample(n, m)
    min_x = min_achievable_misrate_one_sample(n // 2)
    min_y = min_achievable_misrate_one_sample(m // 2)
    min_avg = 2.0 * max(min_x, min_y)

    if misrate < min_shift + min_avg:
        raise AssumptionError.domain("misrate")

    extra = misrate - (min_shift + min_avg)
    alpha_shift = min_shift + extra / 2.0
    alpha_avg = min_avg + extra / 2.0

    if _fast_spread(x.tolist()) <= 0:
        raise AssumptionError.sparity("x")
    if _fast_spread(y.tolist()) <= 0:
        raise AssumptionError.sparity("y")

    sb = shift_bounds(x, y, alpha_shift)
    ab = _avg_spread_bounds(x, y, alpha_avg, seed=seed)

    la = ab.lower
    ua = ab.upper
    ls = sb.lower
    us = sb.upper

    if la > 0.0:
        r1 = ls / la
        r2 = ls / ua
        r3 = us / la
        r4 = us / ua
        return Bounds(min(r1, r2, r3, r4), max(r1, r2, r3, r4))

    if ua <= 0.0:
        if ls == 0.0 and us == 0.0:
            return Bounds(0.0, 0.0)
        if ls >= 0.0:
            return Bounds(0.0, math.inf)
        if us <= 0.0:
            return Bounds(-math.inf, 0.0)
        return Bounds(-math.inf, math.inf)

    # Default: ua > 0 and la <= 0
    if ls > 0.0:
        return Bounds(ls / ua, math.inf)
    if us < 0.0:
        return Bounds(-math.inf, us / ua)
    if ls == 0.0 and us == 0.0:
        return Bounds(0.0, 0.0)
    if ls == 0.0 and us > 0.0:
        return Bounds(0.0, math.inf)
    if ls < 0.0 and us == 0.0:
        return Bounds(-math.inf, 0.0)

    return Bounds(-math.inf, math.inf)


def _avg_spread_bounds(
    x: Union[Sequence[float], NDArray],
    y: Union[Sequence[float], NDArray],
    misrate: float = DEFAULT_MISRATE,
    seed: Union[str, None] = None,
) -> Bounds:
    """
    Provides distribution-free bounds for AvgSpread using Bonferroni combination.

    Args:
        x: First input sample.
        y: Second input sample.
        misrate: Misclassification rate (probability that true avg_spread falls outside bounds).
        seed: Optional string seed for deterministic randomization.

    Returns:
        A Bounds object containing the lower and upper bounds.

    Raises:
        AssumptionError: If sample is empty or contains NaN/Inf.
        AssumptionError: If misrate is out of range or below minimum achievable.
        AssumptionError: If either sample is tie-dominant.
    """
    x = np.asarray(x)
    y = np.asarray(y)
    check_validity(x, "x")
    check_validity(y, "y")

    if math.isnan(misrate) or misrate < 0 or misrate > 1:
        raise AssumptionError.domain("misrate")

    n = len(x)
    m = len(y)
    if n < 2:
        raise AssumptionError.domain("x")
    if m < 2:
        raise AssumptionError.domain("y")

    alpha = misrate / 2.0
    min_x = min_achievable_misrate_one_sample(n // 2)
    min_y = min_achievable_misrate_one_sample(m // 2)
    if alpha < min_x or alpha < min_y:
        raise AssumptionError.domain("misrate")

    if _fast_spread(x.tolist()) <= 0:
        raise AssumptionError.sparity("x")
    if _fast_spread(y.tolist()) <= 0:
        raise AssumptionError.sparity("y")

    bounds_x = spread_bounds(x, alpha, seed=seed)
    bounds_y = spread_bounds(y, alpha, seed=seed)

    weight_x = n / (n + m)
    weight_y = m / (n + m)

    return Bounds(
        weight_x * bounds_x.lower + weight_y * bounds_y.lower,
        weight_x * bounds_x.upper + weight_y * bounds_y.upper,
    )
